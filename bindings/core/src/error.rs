// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::{ser::SerializeMap, Serialize, Serializer};

/// Result type of the bindings core crate.
pub type Result<T> = std::result::Result<T, Error>;

/// Error type for the bindings core crate.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// Evm error.
    #[error("{0}")]
    Evm(#[from] iota_sdk_evm::Error),
    /// SerdeJson errors.
    #[error("{0}")]
    SerdeJson(#[from] serde_json::error::Error),
}

// Serialize type with Display error.
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
