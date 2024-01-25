// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod core;

pub use core::*;

use iota_sdk::packable::{
    error::{UnpackError, UnpackErrorExt},
    packer::Packer,
    unpacker::Unpacker,
    Packable,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct AgentId {
    chain_id: String,
    address: String,
}

impl AgentId {
    pub fn new(chain_id: String, address: String) -> Self {
        Self { chain_id, address }
    }
}

impl Packable for AgentId {
    type UnpackError = crate::Error;

    type UnpackVisitor = ();

    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        let mut bytes = [3_u8].to_vec();
        bytes.extend(hex::decode(&self.chain_id).expect("Invalid hex for chain id"));
        bytes.extend(hex::decode(&self.address).expect("Invalid hex for address"));
        packer.pack_bytes(bytes)
    }

    fn unpack<U: Unpacker, const VERIFY: bool>(
        unpacker: &mut U,
        visitor: &Self::UnpackVisitor,
    ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
        let id = u8::unpack::<_, VERIFY>(unpacker, visitor).coerce()?;
        match id {
            3 => {
                let mut bytes = vec![0u8; 28];
                unpacker.unpack_bytes(&mut bytes)?;
                let chain_id = hex::encode(bytes);

                let mut bytes = vec![0u8; 20];
                unpacker.unpack_bytes(&mut bytes)?;
                // let evm: EvmAddress = EvmAddress::try_from(&bytes)?;
                let address = hex::encode(bytes);
                Ok(AgentId::new(chain_id, address))
            }
            _ => panic!("invalid Agent id, requires 3"),
        }
    }
}
