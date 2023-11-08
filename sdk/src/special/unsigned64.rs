// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::ops::Deref;

use packable::error::UnpackError;
use serde::{Deserialize, Serialize};

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

/// size64_encode uses a simple variable length encoding scheme
/// It takes groups of 7 bits per byte, and encodes if there will be a next group
/// by setting the 0x80 bit. Since most numbers are small, this will result in
/// significant storage savings, with values < 128 occupying only a single byte,
/// and values < 16384 only 2 bytes.
pub fn size64_encode(mut s: u64) -> Vec<u8> {
    let mut result = Vec::new();
    loop {
        let mut byte = (s & 0x7F) as u8;
        s >>= 7;
        if s != 0 {
            byte |= 0x80;
        }
        result.push(byte);
        if s == 0 {
            break;
        }
    }
    result
}

pub fn size64_decode<R>(mut read_byte: R) -> crate::Result<u64>
where
    R: FnMut() -> crate::Result<u8>,
{
    let mut value: u64 = 0;
    for shift in (0..64).step_by(7) {
        let mut byte = read_byte()?;
        let is_last_byte = byte & 0x80 == 0;
        byte &= 0x7F;
        value |= (byte as u64) << shift;
        if is_last_byte {
            return Ok(value);
        }
    }
    Err(crate::Error::IO {
        expected: std::io::ErrorKind::Other,
        message: "size64 overflow",
    })
}
