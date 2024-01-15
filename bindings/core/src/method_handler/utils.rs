// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::packable::PackableExt;
use iota_sdk_evm::{ethereum_agent_id, hname, ContractIdentity};

use crate::{method::UtilsMethod, response::Response, Result};

/// Call a utils method.
pub(crate) fn call_utils_method_internal(method: UtilsMethod) -> Result<Response> {
    let response = match method {
        UtilsMethod::EthereumAgentId { chain, address } => {
            match address {
                ContractIdentity::EVM(a) => {
                    Response::BytesArray(ethereum_agent_id(&chain, &a))
                },
                _ => {
                    Response::Panic("unimplemented".to_string())
                }
            }
        },
        UtilsMethod::Hname { name } => Response::Number(hname(&name)),
        UtilsMethod::SpecialEncode { metadata } => Response::SpecialEncoded(hex::encode(metadata.pack_to_vec()))
    };

    Ok(response)
}
