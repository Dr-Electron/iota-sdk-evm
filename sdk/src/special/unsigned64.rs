// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::ops::Deref;

use iota_sdk::packable::{
    error::{UnpackError, UnpackErrorExt},
    packer::Packer,
    unpacker::Unpacker,
    Packable,
};
use serde::{de, Deserialize, Deserializer, Serializer};

#[derive(Debug, Default, Clone, Eq, PartialEq, PartialOrd, Hash)]
pub struct U64Special(u64);

impl serde::Serialize for U64Special {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        s.serialize_str(&format!("0x{:x}", &self.0))
    }
}

impl<'de> Deserialize<'de> for U64Special {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let binding = String::deserialize(deserializer)?;
        let without_prefix = binding.trim_start_matches("0x");
        let value = u64::from_str_radix(without_prefix, 16).map_err(de::Error::custom)?;
        Ok(U64Special(value))
    }
}

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

impl Packable for U64Special {
    type UnpackError = crate::Error;

    type UnpackVisitor = ();

    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        packer.pack_bytes(size64_encode(**self))
    }

    fn unpack<U: Unpacker, const VERIFY: bool>(
        unpacker: &mut U,
        visitor: &Self::UnpackVisitor,
    ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
        let byte_stream = || {
            u8::unpack::<_, VERIFY>(unpacker, visitor).coerce().map_err(
                |_: UnpackError<Self::UnpackError, U::Error>| crate::Error::IO {
                    expected: std::io::ErrorKind::InvalidData,
                    message: "failed to unpack a byte",
                },
            )
        };

        Ok(U64Special(size64_decode(byte_stream).map_err(UnpackError::Packable)?))
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
