// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will poll the node for info and a receipt
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example node
//! ```

use std::str::FromStr;

use iota_sdk::types::block::output::OutputId;
use iota_sdk_evm::{Api, Result, TESTNET_CHAIN_ADDRESS};
use url::Url;

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let wasp_url = std::env::var("WASP_NODE").unwrap();
    let api = Api::new(Url::parse(wasp_url.as_str()).unwrap());

    // Not working atm
    // println!("wasp node: '{:?}'", api.info().await?);

    let id = OutputId::from_str("0x49f2b03ff9fc646ffaf54a8da752ba50c8e112fac3ef82b06025d819be2b3d130000")?;
    let receipt = api.get_receipt(TESTNET_CHAIN_ADDRESS, id).await?;
    println!("{:?}", receipt);

    Ok(())
}
