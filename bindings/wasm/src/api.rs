// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use iota_sdk_evm_bindings_core::{
    call_api_method as rust_call_api_method,
    iota_sdk::{client::{Error as ClientError}, Url},
    Response, iota_sdk_evm::{Error, Api}
};
use tokio::sync::RwLock;
use wasm_bindgen::{prelude::wasm_bindgen, JsError};

use crate::{destroyed_err, map_err};

/// The Api method handler.
#[wasm_bindgen(js_name = ApiMethodHandler)]
pub struct ApiMethodHandler(Arc<RwLock<Option<Api>>>);

/// Creates a method handler with the given client options.
#[wasm_bindgen(js_name = createApi)]
pub async fn create_client(url: String) -> Result<ApiMethodHandler, JsError> {
    let api = Api::new(Url::parse(&url).map_err(|e| Error::ClientError(ClientError::Url(e)))?);

    Ok(ApiMethodHandler(Arc::new(RwLock::new(Some(api)))))
}

/// Necessary for compatibility with the node.js bindings.
#[wasm_bindgen(js_name = destroyApi)]
pub async fn destroy_api(method_handler: &ApiMethodHandler) -> Result<(), JsError> {
    method_handler.0.write().await.take();
    Ok(())
}

/// Handles a method, returns the response as a JSON-encoded string.
///
/// Returns an error if the response itself is an error or panic.
#[wasm_bindgen(js_name = callApiMethod)]
pub async fn call_api_method(method_handler: &ApiMethodHandler, method: String) -> Result<String, JsError> {
    let method = serde_json::from_str(&method).map_err(map_err)?;
    match &*method_handler.0.read().await {
        Some(api) => {
            let response = rust_call_api_method(api, method).await;
            let ser = serde_json::to_string(&response)?;
            match response {
                Response::Error(_) | Response::Panic(_) => Err(JsError::new(&ser)),
                _ => Ok(ser),
            }
        }
        None => Err(destroyed_err("Api")),
    }
}
