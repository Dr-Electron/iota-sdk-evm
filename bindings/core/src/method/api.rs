// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use derivative::Derivative;
use iota_sdk::{
    types::block::{address::Bech32Address, output::OutputId},
};
use iota_sdk_evm::RequestMetadata;
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
    GetBalance {
        chain: String,
        address: Bech32Address,
    },
    /// Expected response: [`Receipt`](crate::Response::Receipt)
    EstimateGasOnLedger {
        chain: String,
        json: serde_json::Value,
    },
    /// Expected response: [`Receipt`](crate::Response::Receipt)
    EstimateGasOffLedger {
        chain: String,
        metadata: RequestMetadata,
    },
    /// Expected response: [`Receipt`](crate::Response::Receipt)
    #[serde(rename_all = "camelCase")]
    GetReceipt { chain: String, request_id: OutputId },
}
