// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{Error, Result, SimpleBufferCursor};

#[derive(Eq, PartialEq)]
pub enum ContractIdentity {
    ///
    Null,
    ///
    EVM(String),
    ///
    ISC(u32),
}

impl ContractIdentity {
    /// Returns the kind of a [`ContractIdentity`].
    pub fn kind(&self) -> u8 {
        match self {
            Self::Null => 0,
            Self::EVM(_) => 1,
            Self::ISC(_) => 2,
        }
    }

    pub fn from1(buffer: &mut SimpleBufferCursor) -> Result<Self> {
        match buffer.next()? {
            0 => Ok(ContractIdentity::Null),
            1 => Ok(ContractIdentity::Null),
            2 => Ok(ContractIdentity::Null),
            t => Err(Error::InvalidType(t, "ContractIdentity")),
        }
    }
}

impl TryFrom<&mut SimpleBufferCursor> for ContractIdentity {
    type Error = Error;
    fn try_from(value: &mut SimpleBufferCursor) -> Result<Self> {
        ContractIdentity::from1(value)
    }
}

impl core::fmt::Debug for ContractIdentity {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Null => Ok(()),
            Self::EVM(address) => address.fmt(f),
            Self::ISC(contract) => contract.to_be_bytes().fmt(f),
        }
    }
}

impl Default for ContractIdentity {
    fn default() -> Self {
        ContractIdentity::Null
    }
}

impl Serialize for ContractIdentity {
    fn serialize<S>(&self, s: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let ser = format!("{:0>2}{:?}", self.kind(), self);
        s.serialize_str(&ser)
    }
}

impl<'de> Deserialize<'de> for ContractIdentity {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(ContractIdentity::default())
    }
}
