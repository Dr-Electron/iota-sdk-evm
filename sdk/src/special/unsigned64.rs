// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::ops::Deref;

use packable::error::UnpackError;
use serde::{Deserialize, Serialize};

use crate::{size64_decode, size64_encode};

#[derive(Debug, Default, Clone, Eq, PartialEq, PartialOrd, Hash, Serialize, Deserialize)]
pub struct U64Special(u64);

impl From<u64> for U64Special {
    fn from(value: u64) -> Self {
        U64Special(value)
    }
}
impl Deref for U64Special {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl packable::Packable for U64Special {
    type UnpackError = crate::Error;

    type UnpackVisitor = ();

    fn pack<P: packable::packer::Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        packer.pack_bytes(size64_encode(**self))
    }

    fn unpack<U: packable::unpacker::Unpacker, const VERIFY: bool>(
        unpacker: &mut U,
        visitor: &Self::UnpackVisitor,
    ) -> Result<Self, packable::error::UnpackError<Self::UnpackError, U::Error>> {
        let byte_stream = || u8::unpack::<_, VERIFY>(unpacker, visitor).map_err(|e| crate::error::Error::Placeholder);

        Ok(U64Special(
            size64_decode(byte_stream).map_err(|e| UnpackError::Packable(e))?,
        ))
    }
}
