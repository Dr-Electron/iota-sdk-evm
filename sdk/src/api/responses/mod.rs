// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct RentStructure {
    v_byte_factor_data: u32,
    v_byte_cost: u32,
    v_byte_factor_key: u32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Protocol {
    rent_structure: RentStructure,
    min_pow_score: u32,
    token_supply: String,
    network_name: String,
    below_max_depth: u32,
    version: u32,
    bech32_hrp: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct BaseToken {
    unit: String,
    decimals: u32,
    name: String,
    ticker_symbol: String,
    subunit: String,
    use_metric_prefix: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct L1Params {
    protocol: Protocol,
    max_payload_size: u32,
    base_token: BaseToken,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WaspInfo {
    peering_url: String,
    l1_params: L1Params,
    public_key: f64,
    version: String,
}
