// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use crypto::hashes::{blake2b::Blake2b256, Digest};
use iota_sdk::types::block::output::{NativeToken, TokenId};
use serde::{Deserialize, Serialize};

use crate::{Assets, ContractIdentity, Result, SimpleBufferCursor};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct RequestMetadata {
    sender_contract: ContractIdentity,
    target_contract: u32,
    target_entry_point: u32,
    gas_budget: u64,
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
            gas_budget,
        }
    }

    pub fn serialize(&self) -> Result<String> {
        let mut metadata = SimpleBufferCursor::default();
        metadata.write_uint8(self.sender_contract.kind());
        metadata.write_bytes(hex::decode(format!("{:?}", self.sender_contract)).unwrap().as_slice());
        metadata.write_uint32_le(self.target_contract);
        metadata.write_uint32_le(self.target_entry_point);
        metadata.write_uint64_special_encoding(self.gas_budget + 1);

        // params
        metadata.write_uint64_special_encoding(self.params.len() as u64);
        for entry in &self.params {
            let key_bytes = entry.0.as_bytes();
            metadata.write_uint64_special_encoding(key_bytes.len() as u64);
            metadata.write_bytes(key_bytes);
            metadata.write_uint64_special_encoding(entry.1.len() as u64);
            metadata.write_bytes(entry.1);
        }

        // assets
        let mut flags: u8 = 0;
        if self.allowance.has_base_tokens() {
            flags |= 0x80
        }
        if self.allowance.has_native_tokens() {
            flags |= 0x40
        }
        if self.allowance.has_nfts() {
            flags |= 0x20
        }
        metadata.write_uint8(flags);
        if self.allowance.has_base_tokens() {
            metadata.write_uint64_special_encoding(self.allowance.get_base_tokens())
        }
        if let Some(tokens) = self.allowance.get_native_tokens() {
            metadata.write_uint64_special_encoding(tokens.len() as u64);
            for token in tokens {
                metadata.write_bytes(&**token.token_id());
                metadata.write_u256_be(token.amount());
            }
        }
        if let Some(nfts) = self.allowance.get_nfts() {
            metadata.write_uint64_special_encoding(nfts.len() as u64);
            for nft in nfts {
                metadata.write_bytes(nft.as_slice());
            }
        }

        Ok(format!("{}", metadata.serialize()))
    }
}

pub async fn read_metadata(mut buffer: SimpleBufferCursor) -> Result<RequestMetadata> {
    let sender_contract = ContractIdentity::try_from(&mut buffer)?;

    let target_contract = buffer.read_uint32_le();
    let target_entry_point = buffer.read_uint32_le();
    let gas_budget = buffer.read_uint64_special_encoding()? - 1;

    let mut params = HashMap::new();
    let params_len = buffer.read_uint64_special_encoding()?;
    for _ in 0..params_len {
        let key_len = buffer.read_uint64_special_encoding()?;
        let key = buffer.read_bytes(key_len as usize);
        let entry_len = buffer.read_uint64_special_encoding()?;
        let entry = buffer.read_bytes(entry_len as usize);
        params.insert(String::from_utf8(key).unwrap(), entry);
    }

    let flags = buffer.next()?;
    let mut allowance = Assets::default();
    if flags & 0x80 != 0 {
        // base tokens
        allowance.set_base_tokens(buffer.read_uint64_special_encoding()?)
    }
    if flags & 0x40 != 0 {
        // native tokens
        let tokens_len = buffer.read_uint64_special_encoding()?;
        for _ in 0..tokens_len {
            let token_id_bytes = buffer.read_bytes(TokenId::LENGTH);
            let mut fixed_size_array = [0; TokenId::LENGTH];
            fixed_size_array.copy_from_slice(&token_id_bytes[..TokenId::LENGTH]);
            let amount = buffer.read_u256_be()?;

            allowance.add_native_token(NativeToken::new(TokenId::new(fixed_size_array), amount)?)
        }
    }
    if flags & 0x20 != 0 {
        // nfts
    }

    Ok(RequestMetadata {
        sender_contract,
        target_contract,
        target_entry_point,
        gas_budget,
        params,
        allowance,
    })
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
