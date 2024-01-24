// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk_evm::Api;

use crate::{method::ApiMethod, response::Response, Result};

/// Call an api method.
pub(crate) async fn call_api_method_internal(api: &Api, method: ApiMethod) -> Result<Response> {
    let response = match method {
        ApiMethod::GetInfo => Response::WaspInfo(api.info().await?),
        ApiMethod::GetBalance { chain, address } => {
            Response::Assets(api.get_balance(&chain, address).await?)
        }
        ApiMethod::EstimateGasOnLedger { chain, json } => {
            Response::Receipt(api.estimate_gas_on_ledger(&chain, json).await?)
        }
        ApiMethod::EstimateGasOffLedger { chain, metadata } => {
            Response::Receipt(api.estimate_gas_off_ledger(&chain, &metadata).await?)
        }
        ApiMethod::GetReceipt { chain, request_id } => {
            Response::Receipt(api.get_receipt(&chain, request_id).await?)
        }
    };

    Ok(response)
}
