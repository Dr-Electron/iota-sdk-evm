// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use crypto::hashes::{blake2b::Blake2b256, Digest};
use iota_sdk::types::block::output::{NativeToken, TokenId};
use packable::error::{UnpackError, UnpackErrorExt};
use serde::{Deserialize, Serialize};

use crate::{Assets, ContractIdentity, SimpleBufferCursor, U64Special};

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
        target_contract: String,
        target_entry_point: String,
        gas_budget: u64,
    ) -> Self {
        RequestMetadata {
            sender_contract: sender_contract,
            target_contract: hname(&target_contract),       // 1011572226,
            target_entry_point: hname(&target_entry_point), // 603251617,
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
