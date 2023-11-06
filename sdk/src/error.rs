// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Error handling in iota-sdk-evm crate.

use std::fmt::Debug;

use iota_sdk::types::block::Error as SdkError;
use serde::{
    ser::{SerializeMap, Serializer},
    Serialize,
};

/// Type alias of `Result` in iota-client
pub type Result<T> = std::result::Result<T, Error>;

/// Error type of the iota client crate.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error("{expected}: reason {message}")]
    IO {
        expected: std::io::ErrorKind,
        message: &'static str,
    },
    #[error("Type {0} not valid for {1}")]
    InvalidType(u8, &'static str),

    #[error("iota Sdk error: {0}")]
    Sdk(SdkError),

    #[error("just a placeholder error")]
    Placeholder,
}

impl From<SdkError> for Error {
    fn from(value: SdkError) -> Self {
        Error::Sdk(value)
    }
}

// Serialize type with Display error
impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_map(Some(2))?;
        let mut kind_dbg = format!("{self:?}");
        // Convert first char to lowercase
        if let Some(r) = kind_dbg.get_mut(0..1) {
            r.make_ascii_lowercase();
        }
        // Split by whitespace for struct variants and split by `(` for tuple variants
        // Safe to unwrap because kind_dbg is never an empty string
        let kind = kind_dbg.split([' ', '(']).next().unwrap();
        seq.serialize_entry("type", &kind)?;
        seq.serialize_entry("error", &self.to_string())?;
        seq.end()
    }
}
