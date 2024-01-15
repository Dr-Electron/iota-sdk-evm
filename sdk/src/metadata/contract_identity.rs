// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub use iota_sdk::crypto::signatures::secp256k1_ecdsa::EvmAddress;
use iota_sdk::packable::{
    error::{UnpackError, UnpackErrorExt},
    packer::Packer,
    unpacker::Unpacker,
    Packable, PackableExt,
};
use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

use crate::AgentId;

pub const NULL_KIND: u8 = 0;
pub const ISC_KIND: u8 = 1;
pub const EVM_KIND: u8 = 2;
pub const ETHEREUM_ADDRESS_KIND: u8 = 3;

#[derive(Eq, PartialEq, Clone)]
pub enum ContractIdentity {
    ///
    Null,
    ///
    ISC(u32),
    ///
    EVM(EvmAddress),
    ///
    ETH(AgentId),
}

impl ContractIdentity {
    /// Returns the kind of a [`ContractIdentity`].
    pub fn kind(&self) -> u8 {
        match self {
            Self::Null => NULL_KIND,
            Self::ISC(_) => ISC_KIND,
            Self::EVM(_) => EVM_KIND,
            Self::ETH(_) => ETHEREUM_ADDRESS_KIND,
        }
    }
}

impl Packable for ContractIdentity {
    type UnpackError = crate::Error;

    type UnpackVisitor = ();

    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        match self {
            ContractIdentity::ETH(agent) => agent.pack(packer),
            _ => {
                self.kind().pack(packer)?;
                packer.pack_bytes(hex::decode(format!("{:?}", self)).unwrap())
            }
        }
    }

    fn unpack<U: Unpacker, const VERIFY: bool>(
        unpacker: &mut U,
        visitor: &Self::UnpackVisitor,
    ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
        Ok(match u8::unpack::<_, VERIFY>(unpacker, &()).coerce()? {
            NULL_KIND => Self::Null,
            ISC_KIND => Self::ISC(u32::unpack::<_, VERIFY>(unpacker, visitor).coerce()?),
            EVM_KIND => {
                let mut bytes = [0_u8; 20];
                unpacker.unpack_bytes(&mut bytes)?;
                let evm = EvmAddress::try_from(bytes).map_err(|e| UnpackError::Packable(e.into()))?;
                Self::EVM(evm)
            }
            ETHEREUM_ADDRESS_KIND => Self::ETH(AgentId::unpack::<_, VERIFY>(unpacker, visitor)?), /* TODO split so doesnt need 3 */
            k => return Err(UnpackError::Packable(crate::Error::InvalidContractIdentityKind(k))),
        })
    }
}

impl core::fmt::Debug for ContractIdentity {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Null => Ok(()),
            Self::ISC(contract) => write!(f, "ISC({contract})"),
            Self::EVM(address) => format!("Evm({:?})", address).fmt(f),
            Self::ETH(agent) => format!("ETH({:?})", agent).fmt(f),
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
        s.serialize_str(&hex::encode(self.pack_to_vec()))
    }
}

impl<'de> Deserialize<'de> for ContractIdentity {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        ContractIdentity::unpack_unverified(hex::decode(s).unwrap()).map_err(|err| D::Error::custom(format!("{err}")))
    }
}

#[cfg(test)]
mod tests {
    use iota_sdk::packable::PackableExt;

    use crate::{hname, ContractIdentity, ACCOUNTS};

    const ISC: &str = "01025e4b3c";

    #[tokio::test]
    async fn unpack() {
        let evm = ContractIdentity::unpack_unverified(hex::decode(ISC).unwrap()).unwrap();
        matches!(evm, ContractIdentity::ISC(1011572226));
        assert_eq!(ContractIdentity::ISC(hname(ACCOUNTS)), evm);
    }
}
