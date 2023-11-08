// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::types::block::output::{NativeToken, NftId, TokenId};
use packable::error::{UnpackError, UnpackErrorExt};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{U256Special, U64Special};

pub const BASE_TOKEN_FLAG: u8 = 0x80;
pub const NATIVE_TOKENS_FLAG: u8 = 0x40;
pub const NFTS_FLAG: u8 = 0x20;

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Assets {
    base_tokens: U64Special,
    native_tokens: Option<Vec<NativeToken>>,
    nfts: Option<Vec<NftId>>,
}

impl Assets {
    // Getter for base_tokens
    pub fn get_base_tokens(&self) -> u64 {
        *self.base_tokens
    }

    // Getter for native_tokens
    pub fn get_native_tokens(&self) -> Option<&Vec<NativeToken>> {
        self.native_tokens.as_ref()
    }

    // Getter for nfts
    pub fn get_nfts(&self) -> Option<&Vec<NftId>> {
        self.nfts.as_ref()
    }

    // Check if base_tokens is present
    pub fn has_base_tokens(&self) -> bool {
        *self.base_tokens != 0
    }

    // Check if native_tokens is present
    pub fn has_native_tokens(&self) -> bool {
        self.native_tokens.as_ref().map_or(false, |tokens| !tokens.is_empty())
    }

    // Check if nfts is present
    pub fn has_nfts(&self) -> bool {
        self.nfts.as_ref().map_or(false, |nfts| !nfts.is_empty())
    }

    // Setter for base_tokens
    pub fn set_base_tokens(&mut self, value: u64) {
        self.base_tokens = value.into();
    }

    // Add a single native token
    pub fn add_native_token(&mut self, token: NativeToken) {
        if let Some(tokens) = &mut self.native_tokens {
            tokens.push(token);
        } else {
            self.native_tokens = Some(vec![token]);
        }
    }

    // Replace the native_tokens vec
    pub fn set_native_tokens(&mut self, tokens: Vec<NativeToken>) {
        self.native_tokens = Some(tokens);
    }

    // Add a single NftId
    pub fn add_nft(&mut self, nft: NftId) {
        if let Some(nfts) = &mut self.nfts {
            nfts.push(nft);
        } else {
            self.nfts = Some(vec![nft]);
        }
    }

    // Replace the nfts vec
    pub fn set_nfts(&mut self, nfts: Vec<NftId>) {
        self.nfts = Some(nfts);
    }

    pub fn flags(&self) -> u8 {
        let mut flags: u8 = 0;
        if self.has_base_tokens() {
            flags |= BASE_TOKEN_FLAG
        }
        if self.has_native_tokens() {
            flags |= NATIVE_TOKENS_FLAG
        }
        if self.has_nfts() {
            flags |= NFTS_FLAG
        }
        flags
    }
}

impl packable::Packable for Assets {
    type UnpackError = crate::Error;

    type UnpackVisitor = ();

    fn pack<P: packable::packer::Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        self.flags().pack(packer)?;
        if self.has_base_tokens() {
            U64Special::pack(&self.base_tokens, packer)?;
        }
        if let Some(tokens) = self.get_native_tokens() {
            U64Special::pack(&(tokens.len() as u64).into(), packer)?;
            for token in tokens {
                token.token_id().pack(packer)?;
                U256Special::from(token.amount()).pack(packer)?;
            }
        }
        if let Some(nfts) = self.get_nfts() {
            U64Special::pack(&(nfts.len() as u64).into(), packer)?;
            for nft in nfts {
                nft.pack(packer)?;
            }
        }
        Ok(())
    }

    fn unpack<U: packable::unpacker::Unpacker, const VERIFY: bool>(
        unpacker: &mut U,
        visitor: &Self::UnpackVisitor,
    ) -> Result<Self, packable::error::UnpackError<Self::UnpackError, U::Error>> {
        let flags = u8::unpack::<_, VERIFY>(unpacker, &()).coerce()?;
        let mut assets = Assets::default();
        if flags & BASE_TOKEN_FLAG != 0 {
            // base tokens
            let tokens = U64Special::unpack::<_, VERIFY>(unpacker, visitor)?;
            assets.base_tokens = tokens;
        }
        if flags & NATIVE_TOKENS_FLAG != 0 {
            // native tokens
            let tokens_len = *U64Special::unpack::<_, VERIFY>(unpacker, visitor)?;
            for _ in 0..tokens_len {
                let token_id =
                    TokenId::unpack::<_, VERIFY>(unpacker, visitor).map_packable_err(|e| crate::Error::Placeholder)?;
                let amount = U256Special::unpack::<_, VERIFY>(unpacker, visitor)
                    .map_packable_err(|e| crate::Error::Placeholder)?;

                assets
                    .add_native_token(NativeToken::new(token_id, *amount).map_err(|e| UnpackError::Packable(e.into()))?)
            }
        }
        if flags & NFTS_FLAG != 0 {
            // nfts
            let nfts_len = *U64Special::unpack::<_, VERIFY>(unpacker, visitor)?;
            for _ in 0..nfts_len {
                let nft =
                    NftId::unpack::<_, VERIFY>(unpacker, visitor).map_packable_err(|e| crate::Error::Placeholder)?;
                assets.add_nft(nft);
            }
        }

        Ok(assets)
    }
}

impl Serialize for Assets {
    fn serialize<S>(&self, s: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let ser = format!("{:0>2}{:?}", 0, self);
        s.serialize_str(&ser)
    }
}

impl<'de> Deserialize<'de> for Assets {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Assets::default())
    }
}
