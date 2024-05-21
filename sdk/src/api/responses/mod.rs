// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use iota_sdk::types::block::output::OutputId;
use serde::{Deserialize, Serialize};

use crate::AssetsDto;

/// Describes a receipt.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReceiptResponse {
    pub request: Request,
    pub raw_error: Option<NodeError>,
    pub error_message: Option<String>,
    pub gas_budget: String,
    pub gas_burned: String,
    pub gas_fee_charged: String,
    pub storage_deposit_charged: String,
    pub block_index: u32,
    pub request_index: u32,
    pub gas_burn_log: Vec<GasBurned>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    allowance: AssetsDto,
    call_target: Target,
    fungible_tokens: AssetsDto,
    gas_budget: String,
    #[serde(rename = "isEVM")]
    is_evm: bool,
    is_off_ledger: bool,
    nft: Option<String>,
    params: HashMap<String, Vec<u8>>,
    request_id: OutputId,
    sender_account: String,
    target_address: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Target {
    #[serde(rename = "contractHName")]
    contract_hname: String,
    #[serde(rename = "functionHName")]
    function_hname: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeError {
    code: String,
    params: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GasBurned {
    code: u8,
    gas_burned: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WaspInfo {
    peering_url: String,
    l1_params: L1Params,
    public_key: f64,
    version: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct L1Params {
    protocol: Protocol,
    max_payload_size: u32,
    base_token: BaseToken,
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
struct RentStructure {
    v_byte_factor_data: u32,
    v_byte_cost: u32,
    v_byte_factor_key: u32,
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
