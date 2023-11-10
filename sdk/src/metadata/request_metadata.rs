// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use crypto::hashes::{blake2b::Blake2b256, Digest};
use packable::error::UnpackErrorExt;
use serde::{Deserialize, Serialize};

use crate::{Assets, ContractIdentity, U64Special};

/// https://wiki.iota.org/wasp-evm/reference/core-contracts/overview/
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct RequestMetadata {
    sender_contract: ContractIdentity,
    target_contract: u32,
    target_entry_point: u32,
    gas_budget: U64Special,
    pub params: HashMap<String, Vec<u8>>,
    pub allowance: Assets,
}

impl RequestMetadata {
    pub fn new(
        sender_contract: ContractIdentity,
        target_contract: impl Into<Option<String>>,
        target_entry_point: impl Into<Option<String>>,
        gas_budget: u64,
    ) -> Self {
        RequestMetadata {
            sender_contract,
            target_contract: target_contract.into().map_or(0, |tc| hname(&tc)),
            target_entry_point: target_entry_point.into().map_or(0, |tep| hname(&tep)),
            params: Default::default(),
            allowance: Assets::default(),
            gas_budget: gas_budget.into(),
        }
    }
}

impl packable::Packable for RequestMetadata {
    type UnpackError = crate::Error;

    type UnpackVisitor = ();

    fn pack<P: packable::packer::Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        self.sender_contract.pack(packer)?;

        self.target_contract.to_le_bytes().pack(packer)?;
        self.target_entry_point.to_le_bytes().pack(packer)?;
        Into::<U64Special>::into(*self.gas_budget + 1).pack(packer)?;

        U64Special::pack(&(self.params.len() as u64).into(), packer)?;
        for entry in &self.params {
            U64Special::pack(&(entry.0.as_bytes().len() as u64).into(), packer)?;
            packer.pack_bytes(entry.0)?;
            U64Special::pack(&(entry.1.len() as u64).into(), packer)?;
            packer.pack_bytes(entry.1)?;
        }

        self.allowance.pack(packer)?;
        Ok(())
    }

    fn unpack<U: packable::unpacker::Unpacker, const VERIFY: bool>(
        unpacker: &mut U,
        visitor: &Self::UnpackVisitor,
    ) -> Result<Self, packable::error::UnpackError<Self::UnpackError, U::Error>> {
        let sender_contract = ContractIdentity::unpack::<U, VERIFY>(unpacker, visitor)?;

        let target_contract = u32::unpack::<_, VERIFY>(unpacker, visitor).coerce()?.to_le();
        let target_entry_point = u32::unpack::<_, VERIFY>(unpacker, visitor).coerce()?.to_le();
        let gas_budget = (*U64Special::unpack::<_, VERIFY>(unpacker, visitor)? - 1).into();

        let mut params = HashMap::new();
        let params_len = *U64Special::unpack::<_, VERIFY>(unpacker, visitor)?;
        for _ in 0..params_len {
            let key_len = *U64Special::unpack::<_, VERIFY>(unpacker, visitor)?;
            let mut key = vec![0u8; key_len.try_into().unwrap()];
            unpacker.unpack_bytes(&mut key)?;
            let entry_len = *U64Special::unpack::<_, VERIFY>(unpacker, visitor)?;
            let mut entry = vec![0u8; entry_len.try_into().unwrap()];
            unpacker.unpack_bytes(&mut entry)?;
            params.insert(String::from_utf8(key).unwrap(), entry);
        }

        let allowance = Assets::unpack::<U, VERIFY>(unpacker, visitor)?;
        Ok(RequestMetadata {
            sender_contract,
            target_contract,
            target_entry_point,
            gas_budget,
            params,
            allowance,
        })
    }
}

/// Takes a chain ID and an address as input, converts them from
/// hexadecimal to bytes, and returns the concatenated bytes prepended with a 3 signifying the type.
/// The AgentID key in the parameters has to be `a`.
///
/// Arguments:
///
/// * `chain_id`: The `chain_id` parameter represents the ID of the Ethereum blockchain network.
/// * `address`: The `address` parameter is a hexadecimal string representing the Ethereum address.
pub fn ethereum_agent_id(chain_id: String, address: String) -> Vec<u8> {
    let mut bytes = [3_u8].to_vec();
    bytes.extend(hex::decode(chain_id).expect("Invalid hex for chain id"));
    bytes.extend(hex::decode(address).expect("Invalid hex for address"));

    bytes
}

/// `hname` takes a UTF8 string as input, calculates its Blake2b256 hash, and returns
/// the first 4 bytes of the hash as a u32 using LE encoding.
pub fn hname(name: &str) -> u32 {
    let hash_result = Blake2b256::digest(name);
    let hash_bytes: [u8; 4] = hash_result[0..4].try_into().expect("slice with incorrect length");
    let hash_u32 = u32::from_le_bytes(hash_bytes);
    hash_u32
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use iota_sdk::types::block::output::{NativeToken, TokenId};
    use packable::PackableExt;

    use crate::{ethereum_agent_id, hname, ContractIdentity, RequestMetadata, ACCOUNTS};

    const SER: &str = "00025e4b3ca1e3f423914e0101613503e14c3499349cb8d2fd771e09829883e4ecfae02e6b09c9b6a0fb3c7504b4e2f4e913cac59e0ba840039add645d5df83c294cc230400108e14c3499349cb8d2fd771e09829883e4ecfae02e6b09c9b6a0fb3c7504b4e2f401000000000132";
    const SER_ISC: &str = "01025e4b3c0000000000000000010000";

    #[tokio::test]
    async fn hnames() {
        assert_eq!(hname(ACCOUNTS), 1011572226);
        assert_eq!(hname("transferAllowanceTo"), 603251617);
    }

    #[tokio::test]
    async fn serde() {
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(&serde_json::to_string(&get_metadata()).unwrap()).unwrap(),
            serde_json::json!({
                "sender_contract": "00",
                "target_contract": 1011572226,
                "target_entry_point": 603251617,
                "gas_budget": 10000,
                "params": {
                    "a": [3,225,76,52,153,52,156,184,210,253,119,30,9,130,152,131,228,236,250,224,46,107,9,201,182,160,251,60,117,4,180,226,244,233,19,202,197,158,11,168,64,3,154,221,100,93,93,248,60,41,76,194,48]
                },
                "allowance": {
                    "base_tokens": 0,
                    "native_tokens": [{
                        "id": "0x08e14c3499349cb8d2fd771e09829883e4ecfae02e6b09c9b6a0fb3c7504b4e2f40100000000",
                        "amount": "0x32"
                    }],
                    "nfts": serde_json::Value::Null
                }
            })
        )
    }

    #[tokio::test]
    async fn pack() {
        let metadata = get_metadata();
        let buf = metadata.pack_to_vec();
        assert_eq!(hex::decode(SER).unwrap(), buf);
    }

    #[tokio::test]

    async fn unpack() {
        let new_meta = RequestMetadata::unpack_unverified(hex::decode(SER).unwrap()).unwrap();
        assert_eq!(get_metadata(), new_meta);
    }

    #[tokio::test]
    async fn unpack_ics() {
        let new_meta = RequestMetadata::unpack_unverified(hex::decode(SER_ISC).unwrap()).unwrap();
        assert_eq!(
            RequestMetadata::new(ContractIdentity::ISC(hname(ACCOUNTS)), None, None, 0),
            new_meta
        );
    }

    fn get_metadata() -> RequestMetadata {
        let mut metadata = RequestMetadata::new(
            ContractIdentity::Null,
            ACCOUNTS.to_string(),
            "transferAllowanceTo".to_string(),
            10000,
        );
        metadata.params.insert(
            "a".to_string(),
            ethereum_agent_id(
                "e14c3499349cb8d2fd771e09829883e4ecfae02e6b09c9b6a0fb3c7504b4e2f4".to_string(),
                "E913CAc59E0bA840039aDD645D5df83C294CC230".to_string(),
            ),
        );
        metadata.allowance.add_native_token(
            NativeToken::new(
                TokenId::from_str("0x08e14c3499349cb8d2fd771e09829883e4ecfae02e6b09c9b6a0fb3c7504b4e2f40100000000")
                    .unwrap(),
                50,
            )
            .unwrap(),
        );
        metadata
    }
}
