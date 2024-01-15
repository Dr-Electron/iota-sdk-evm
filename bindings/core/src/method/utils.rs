// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use derivative::Derivative;
use iota_sdk_evm::{RequestMetadata, EvmAddress, ContractIdentity};
use serde::{Deserialize, Serialize};


/// Each public utils method.
#[derive(Clone, Derivative, Serialize, Deserialize)]
#[derivative(Debug)]
#[serde(tag = "name", content = "data", rename_all = "camelCase")]
#[non_exhaustive]
pub enum UtilsMethod {
    EthereumAgentId { chain: String, address: ContractIdentity },
    Hname { name: String },
    SpecialEncode { metadata: RequestMetadata },
}
