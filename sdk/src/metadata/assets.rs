// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::types::block::output::{NativeToken, NftId};
use packable::Packable;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Default, Eq, PartialEq, Packable)]
pub struct Assets {
    base_tokens: u64,
    native_tokens: Option<Vec<NativeToken>>,
    nfts: Option<Vec<NftId>>,
}

impl Assets {
    // Getter for base_tokens
    pub fn get_base_tokens(&self) -> u64 {
        self.base_tokens
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
        self.base_tokens != 0
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
        self.base_tokens = value;
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
