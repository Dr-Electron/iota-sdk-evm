// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk_evm::Api;

use crate::{method::ApiMethod, response::Response, Result};

/// Call an api method.
pub(crate) async fn call_api_method_internal(api: &Api, method: ApiMethod) -> Result<Response> {
    let response = match method {
        ApiMethod::GetInfo => {
            Response::WaspInfo( api.info().await? )
        },
        ApiMethod::GetBalance { chain, address } => {
            Response::Assets( api.get_balance(&chain, address).await? )
        },
        ApiMethod::BuildAccountOutput { amount, mana } => todo!(),
    };

    Ok(response)
}
