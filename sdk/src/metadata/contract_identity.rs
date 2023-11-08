// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use packable::error::{UnpackError, UnpackErrorExt};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::Error;

pub const NULL_KIND: u8 = 0;
pub const EVM_KIND: u8 = 1;
pub const ISC_KIND: u8 = 2;

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
            Self::Null => NULL_KIND,
            Self::EVM(_) => EVM_KIND,
            Self::ISC(_) => ISC_KIND,
        }
    }
}

impl packable::Packable for ContractIdentity {
    type UnpackError = Error;

    type UnpackVisitor = ();

    fn pack<P: packable::packer::Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        self.kind().pack(packer)?;
        packer.pack_bytes(hex::decode(format!("{:?}", self)).unwrap())
    }

    fn unpack<U: packable::unpacker::Unpacker, const VERIFY: bool>(
        unpacker: &mut U,
        visitor: &Self::UnpackVisitor,
    ) -> Result<Self, packable::error::UnpackError<Self::UnpackError, U::Error>> {
        Ok(match u8::unpack::<_, VERIFY>(unpacker, &()).coerce()? {
            NULL_KIND => Self::Null,
            EVM_KIND => {
                let mut bytes = vec![0u8; 20];
                unpacker.unpack_bytes(&mut bytes)?;
                // let evm: EvmAddress = EvmAddress::try_from(&bytes)?;
                Self::EVM(hex::encode(bytes))
            }
            ISC_KIND => Self::ISC(u32::unpack::<_, VERIFY>(unpacker, visitor).coerce()?.to_le()),
            k => return Err(UnpackError::Packable(Error::InvalidContractIdentityKind(k))),
        })
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
