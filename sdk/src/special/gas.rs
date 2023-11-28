// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::ops::Deref;

use iota_sdk::packable::{
    error::{UnpackError, UnpackErrorExt},
    packer::Packer,
    unpacker::Unpacker,
    Packable,
};
use serde::{Deserialize, Serialize};

use crate::U64Special;

#[derive(Debug, Default, Clone, Eq, PartialEq, PartialOrd, Hash, Serialize, Deserialize)]
pub struct Gas(U64Special);

impl From<u64> for Gas {
    fn from(value: u64) -> Self {
        Gas(value.into())
    }
}
impl From<U64Special> for Gas {
    fn from(value: U64Special) -> Self {
        Gas(value)
    }
}
impl Deref for Gas {
    type Target = U64Special;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Packable for Gas {
    type UnpackError = crate::Error;

    type UnpackVisitor = ();

    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        U64Special::from(*self.0 + 1).pack(packer)
    }

    fn unpack<U: Unpacker, const VERIFY: bool>(
        unpacker: &mut U,
        visitor: &Self::UnpackVisitor,
    ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
        let size = U64Special::unpack::<_, VERIFY>(unpacker, visitor).coerce()?;

        Ok(Self(U64Special::from(*size - 1)))
    }
}
