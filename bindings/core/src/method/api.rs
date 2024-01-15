// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use derivative::Derivative;
use iota_sdk::{
    utils::serde::{option_string, string}, types::block::address::Bech32Address,
};
use serde::{Deserialize, Serialize};

/// Each public api method.
#[derive(Clone, Derivative, Serialize, Deserialize)]
#[derivative(Debug)]
#[serde(tag = "name", content = "data", rename_all = "camelCase")]
#[non_exhaustive]
pub enum ApiMethod {
    /// Expected response: [`WaspInfo`](crate::Response::WaspInfo)
    GetInfo,
    /// Expected response: [`Assets`](crate::Response::Assets)
    GetBalance {chain: String, address: Bech32Address },
    /// Build an AccountOutput.
    /// Expected response: [`Output`](crate::Response::Output)
    #[allow(missing_docs)]
    #[serde(rename_all = "camelCase")]
    BuildAccountOutput {
        // If not provided, minimum amount will be used
        #[serde(default, with = "option_string")]
        amount: Option<u64>,
        // TODO: Determine if `default` is wanted here
        #[serde(default, with = "string")]
        mana: u64,
    },
}
