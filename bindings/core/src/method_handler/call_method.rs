// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::pin::Pin;

use futures::Future;
use iota_sdk_evm::Api;
use crate::{
    method::{ApiMethod, UtilsMethod},
    method_handler::{
        api::call_api_method_internal, utils::call_utils_method_internal
    },
    panic::{convert_async_panics, convert_panics},
    response::Response
};

pub trait CallMethod {
    type Method;

    // This uses a manual async_trait-like impl because it's not worth it to import the lib for one trait
    fn call_method<'a>(&'a self, method: Self::Method) -> Pin<Box<dyn Future<Output = Response> + 'a>>;
}

impl CallMethod for Api {
    type Method = ApiMethod;

    fn call_method<'a>(&'a self, method: Self::Method) -> Pin<Box<dyn Future<Output = Response> + 'a>> {
        Box::pin(call_api_method(self, method))
    }
}

/// Call an api method.
pub async fn call_api_method(api: &Api, method: ApiMethod) -> Response {
    log::debug!("Api method: {method:?}");
    let result = convert_async_panics(|| async { call_api_method_internal(api, method).await }).await;

    let response = result.unwrap_or_else(Response::Error);

    log::debug!("Api response: {response:?}");
    response
}

/// Call a utils method.
pub fn call_utils_method(method: UtilsMethod) -> Response {
    log::debug!("Utils method: {method:?}");
    let result = convert_panics(|| call_utils_method_internal(method));

    let response = result.unwrap_or_else(Response::Error);

    log::debug!("Utils response: {response:?}");
    response
}
