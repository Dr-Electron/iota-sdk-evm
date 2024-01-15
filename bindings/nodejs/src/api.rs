// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use iota_sdk_evm_bindings_core::{
    call_api_method as rust_call_api_method,
    iota_sdk::{Url},
    ApiMethod, Response, iota_sdk_evm::Api,
};
use napi::{bindgen_prelude::External, Result};
use napi_derive::napi;
use tokio::sync::RwLock;

use crate::{build_js_error, destroyed_err, NodejsError};

pub type ApiMethodHandler = Arc<RwLock<Option<Api>>>;

#[napi(js_name = "createApi")]
pub async fn create_api(url: String) -> Result<External<ApiMethodHandler>> {
    let api = Api::new(Url::parse("https://archive.evm.testnet.shimmer.network").unwrap());//serde_json::from_str::<Url>(&url).map_err(NodejsError::new)?);
    Ok(External::new(Arc::new(RwLock::new(Some(api)))))
}

#[napi(js_name = "destroyApi")]
pub async fn destroy_api(api: External<ApiMethodHandler>) {
    *api.as_ref().write().await = None;
}

#[napi(js_name = "callApiMethod")]
pub async fn call_client_method(api: External<ApiMethodHandler>, method: String) -> Result<String> {
    let method = serde_json::from_str::<ApiMethod>(&method).map_err(NodejsError::new)?;

    match &*api.as_ref().read().await {
        Some(api) => {
            let response = rust_call_api_method(api, method).await;
            match response {
                Response::Error(_) | Response::Panic(_) => Err(build_js_error(response)),
                _ => Ok(serde_json::to_string(&response).map_err(NodejsError::new)?),
            }
        }
        None => Err(destroyed_err("Api")),
    }
}
