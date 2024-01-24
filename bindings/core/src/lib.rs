// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Core library for iota-sdk bindings

mod error;
mod method;
mod method_handler;
mod panic;
mod response;

use std::fmt::{Formatter, Result as FmtResult};

use fern_logger::{logger_init, LoggerConfig, LoggerOutputConfigBuilder};
pub use iota_sdk;
use iota_sdk::client::secret::SecretManagerDto;
pub use iota_sdk_evm;

#[cfg(not(target_family = "wasm"))]
pub use self::method_handler::CallMethod;
pub use self::{
    error::{Error, Result},
    method::ApiMethod,
    method::UtilsMethod,
    method_handler::call_api_method,
    method_handler::call_utils_method,
    response::Response,
};

pub fn init_logger(config: String) -> std::result::Result<(), fern_logger::Error> {
    let output_config: LoggerOutputConfigBuilder =
        serde_json::from_str(&config).expect("invalid logger config");
    let config = LoggerConfig::build().with_output(output_config).finish();
    logger_init(config)
}

pub(crate) trait OmittedDebug {
    fn omitted_fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("<omitted>")
    }
}
impl OmittedDebug for String {}
impl OmittedDebug for SecretManagerDto {}
impl<T: OmittedDebug> OmittedDebug for Option<T> {
    fn omitted_fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Some(_) => f.write_str("Some(<omitted>)"),
            None => f.write_str("None"),
        }
    }
}
