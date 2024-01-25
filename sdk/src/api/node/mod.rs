// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// TODO use sdk
pub(crate) mod http_client;

/// Structs for nodes
use std::fmt::Debug;

use instant::Duration;
use iota_sdk::{
    client::node_manager::node::Node,
    packable::PackableExt,
    types::block::{address::Bech32Address, output::OutputId},
};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use url::Url;

use self::http_client::HttpClient;
use crate::{AssetsDto, ReceiptResponse, RequestMetadata, Result, WaspInfo};

/// Api (eventually) based on
/// https://editor.swagger.io/?url=https://raw.githubusercontent.com/iotaledger/wasp/develop/clients/apiclient/api/openapi.yaml
pub struct Api {
    node: Node,
    http_client: HttpClient,
}

impl Api {
    pub fn new(url: Url) -> Self {
        Self {
            node: url.into(),
            http_client: HttpClient::new("evm_sdk".to_string()),
        }
    }

    fn get_timeout(&self) -> Duration {
        Duration::from_secs(10)
    }

    /// Returns private information about this node.
    /// GET /v1/node/info
    pub async fn info(&self) -> Result<WaspInfo> {
        let path = &format!("v1/node/info");

        self.get_request(path, None, true, true).await
    }

    /// Returns the balance of an l1 address available for l2 transfers.
    /// GET /v1/chains/{chain}/core/accounts/account/{address}/balance
    pub async fn get_balance(&self, chain: &str, address: Bech32Address) -> Result<AssetsDto> {
        let path = &format!("v1/chains/{chain}/core/accounts/account/{address}/balance");

        self.get_request(path, None, true, true).await
    }

    /// Estimates gas for a given on-ledger ISC request
    /// POST /v1/chains/{chainID}/estimategas-onledger
    pub async fn estimate_gas_on_ledger(&self, chain: &str, json: Value) -> Result<ReceiptResponse> {
        let path = &format!("v1/chains/{chain}/estimategas-onledger");
        let body = serde_json::json!({
            "outputBytes": json
        });
        self.post_request(path, None, body, true, true).await
    }

    /// Estimates gas for a given off-ledger ISC request
    /// POST /v1/chains/{chainID}/estimategas-offledger
    pub async fn estimate_gas_off_ledger(&self, chain: &str, metadata: &RequestMetadata) -> Result<ReceiptResponse> {
        let path = &format!("v1/chains/{chain}/estimategas-offledger");
        let body = serde_json::json!({
            "requestBytes": prefix_hex::encode(metadata.pack_to_vec())
        });

        self.post_request(path, None, body, true, true).await
    }

    /// Get a receipt from a request ID
    /// GET /v1/chains/{chainID}/receipts/{requestID}
    pub async fn get_receipt(&self, chain: &str, request_id: OutputId) -> Result<ReceiptResponse> {
        let path = &format!("v1/chains/{chain}/receipts/{request_id}");

        self.get_request(path, None, true, true).await
    }

    pub(crate) async fn post_request<T: DeserializeOwned + Debug + Serialize>(
        &self,
        path: &str,
        query: Option<&str>,
        json: serde_json::Value,
        _need_quorum: bool,
        _prefer_permanode: bool,
    ) -> Result<T> {
        let mut node = self.node.clone();
        node.url.set_path(path);
        node.url.set_query(query);
        if let Some(auth) = &node.auth {
            if let Some((name, password)) = &auth.basic_auth_name_pwd {
                node.url
                    .set_username(name)
                    .map_err(|_| iota_sdk::client::Error::UrlAuth("username"))?;
                node.url
                    .set_password(Some(password))
                    .map_err(|_| iota_sdk::client::Error::UrlAuth("password"))?;
            }
        }

        let res = self.http_client.post_json(node, self.get_timeout(), json).await;
        match res {
            Ok(r) => r.into_json().await.map_err(|e| crate::Error::ClientError(e.into())),
            Err(e) => Err(crate::Error::ClientError(e.into())),
        }
    }

    pub(crate) async fn get_request<T: DeserializeOwned + Debug + Serialize>(
        &self,
        path: &str,
        query: Option<&str>,
        _need_quorum: bool,
        _prefer_permanode: bool,
    ) -> Result<T> {
        let mut node = self.node.clone();
        node.url.set_path(path);
        node.url.set_query(query);
        if let Some(auth) = &node.auth {
            if let Some((name, password)) = &auth.basic_auth_name_pwd {
                node.url
                    .set_username(name)
                    .map_err(|_| iota_sdk::client::Error::UrlAuth("username"))?;
                node.url
                    .set_password(Some(password))
                    .map_err(|_| iota_sdk::client::Error::UrlAuth("password"))?;
            }
        }

        let res = self.http_client.get_bytes(node, self.get_timeout()).await;
        match res {
            Ok(r) => r.into_json().await.map_err(|e| crate::Error::ClientError(e.into())),
            Err(e) => Err(crate::Error::ClientError(e.into())),
        }
    }
}

pub(crate) fn query_tuples_to_query_string(
    tuples: impl IntoIterator<Item = Option<(&'static str, String)>>,
) -> Option<String> {
    let query = tuples
        .into_iter()
        .filter_map(|tuple| tuple.map(|(key, value)| format!("{}={}", key, value)))
        .collect::<Vec<_>>();

    if query.is_empty() { None } else { Some(query.join("&")) }
}
