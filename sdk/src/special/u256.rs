// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::ops::Deref;

use iota_sdk::U256;
use packable::error::UnpackErrorExt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Eq, PartialEq, PartialOrd, Hash, Serialize, Deserialize)]
pub struct U256Special(U256);

impl From<U256> for U256Special {
    fn from(value: U256) -> Self {
        U256Special(value)
    }
}
impl Deref for U256Special {
    type Target = U256;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl packable::Packable for U256Special {
    type UnpackError = crate::Error;

    type UnpackVisitor = ();

    fn pack<P: packable::packer::Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        let bytes: [u8; 32] = self.0.into();
        let first_non_zero_index = bytes.iter().position(|&x| x != 0).unwrap_or(32);
        let size = 32 - (first_non_zero_index as u8);
        size.pack(packer)?;
        packer.pack_bytes(&bytes[first_non_zero_index..])
    }

    fn unpack<U: packable::unpacker::Unpacker, const VERIFY: bool>(
        unpacker: &mut U,
        visitor: &Self::UnpackVisitor,
    ) -> Result<Self, packable::error::UnpackError<Self::UnpackError, U::Error>> {
        let size = u8::unpack::<_, VERIFY>(unpacker, visitor).coerce()?;

        let mut bytes = vec![0_u8];
        for _ in 0..size {
            bytes.push(u8::unpack::<_, VERIFY>(unpacker, &()).coerce()?);
        }
        Ok(U256::from_big_endian(&bytes).into())
    }
}
