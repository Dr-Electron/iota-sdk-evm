// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use derivative::Derivative;
use iota_sdk::types::block::address::Bech32Address;
use iota_sdk_evm::{AssetsDto, WaspInfo};
use serde::Serialize;

use crate::Error;

/// The response message.
#[derive(Serialize, Derivative)]
#[derivative(Debug)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
#[non_exhaustive]
pub enum Response {
    /// Response for:
    /// - [`GetInfo`](crate::method::Api::GetInfo)
    WaspInfo(WaspInfo),
    /// Response for:
    /// - [`Balance`](crate::method::Api::Balance)
    Assets(AssetsDto),
    /// Response for:
    /// - [`Hname`](crate::method::Utils::Hname)
    Number(u32),
    /// Response for:
    /// - [`SpecialEncode`](crate::method::Utils::SpecialEncode)
    SpecialEncoded(String),
    /// Response for:
    /// - [`EthereumAgentId`](crate::method::Utils::EthereumAgentId)
    BytesArray(Vec<u8>),
    /// Response for:
    /// - [`GenerateEd25519Addresses`](crate::method::api::GenerateEd25519Addresses)
    GeneratedEd25519Addresses(Vec<Bech32Address>),
    Ok,
    /// Response for any method that returns an error.
    Error(Error),
    /// Response for any method that panics.
    Panic(String),
}
