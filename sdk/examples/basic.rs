// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will create an evm transaction
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example basic
//! ```

use std::str::FromStr;

use iota_sdk::{
    client::{
        constants::SHIMMER_COIN_TYPE,
        secret::{stronghold::StrongholdSecretManager, SecretManager},
        Client,
    },
    crypto::keys::bip39::Mnemonic,
    types::{
        block::output::{
            feature::MetadataFeature, unlock_condition::AddressUnlockCondition, BasicOutputBuilder, Feature,
            NativeToken, TokenId,
        },
        ValidationParams,
    },
    wallet::ClientOptions,
    Wallet,
};
use iota_sdk_evm::{ethereum_agent_id, ContractIdentity, Error, RequestMetadata, Result};
use packable::PackableExt;

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Setup Stronghold secret_manager
    let secret_manager = StrongholdSecretManager::builder()
        .password(std::env::var("STRONGHOLD_PASSWORD").unwrap())
        .build(std::env::var("STRONGHOLD_SNAPSHOT_PATH").unwrap())
        .map_err(|e| crate::Error::SdkWallet(e.into()))?;

    // Only required the first time, can also be generated with `manager.generate_mnemonic()?`
    let mnemonic = Mnemonic::from(std::env::var("MNEMONIC").unwrap());

    // The mnemonic only needs to be stored the first time
    /*secret_manager
        .store_mnemonic(mnemonic)
        .await
        .map_err(|e| crate::Error::SdkWallet(e.into()))?;*/

    let client_options = ClientOptions::new().with_node(&std::env::var("NODE_URL").unwrap())?;

    // Create the wallet
    let wallet = Wallet::builder()
        .with_secret_manager(SecretManager::Stronghold(secret_manager))
        .with_storage_path(&std::env::var("WALLET_DB_PATH").unwrap())
        .with_client_options(client_options)
        .with_coin_type(SHIMMER_COIN_TYPE)
        .finish()
        .await?;

    // Get or create a new account
    let account = wallet.get_or_create_account("Alice").await?;
    let balance = account.sync(None).await?;
    let account_addr = &account.generate_ed25519_addresses(1, None).await?[0];
    println!("Using addr: '{:?}'", account_addr.address());

    let protocol_parameters = account.client().get_protocol_parameters().await?;
    println!(
        "protocol_parameters: '{:?}'",
        protocol_parameters.bech32_hrp().to_string()
    );

    //let meta = "00025e4b3ca1e3f42300010161350342f7da9bdb55b3ec87e5ac1a1e6d88e16768663fde5eca3429eb6f579cc538acb67c72b84e8bced681478241686d30c2a1ddd8d680f0f3d62f";
    //let new_meta = RequestMetadata::unpack_unverified(hex::decode(meta).unwrap()).unwrap();
    //println!("{:?}", new_meta);

    if balance.base_coin().available() > 0 {
        let protocol_parameters = account.client().get_protocol_parameters().await?;
        let metadata = format!("0x{}", hex::encode(get_metadata().pack_to_vec()));

        let outputs = [BasicOutputBuilder::new_with_amount(1)
            .with_minimum_storage_deposit(*protocol_parameters.rent_structure())
            .add_unlock_condition(AddressUnlockCondition::from(account_addr.address().inner().clone()))
            .with_features([Feature::from(MetadataFeature::new(get_metadata().pack_to_vec())?)])
            .finish_with_params(ValidationParams::default().with_protocol_parameters(protocol_parameters.clone()))
            .unwrap()
            .into()];


        let transaction = account.send_outputs(outputs, None).await?;
        println!(
            "Transaction sent: {}/transaction/{}",
            std::env::var("EXPLORER_URL").unwrap(),
            transaction.transaction_id
        );

        // Wait for transaction to get included
        let block_id = account
            .retry_transaction_until_included(&transaction.transaction_id, None, None)
            .await?;

        println!(
            "Block included: {}/block/{}",
            std::env::var("EXPLORER_URL").unwrap(),
            block_id
        );
    } else {
        println!("no available balance. top up at '{:?}'", account_addr.address());
        iota_sdk::client::request_funds_from_faucet(&std::env::var("FAUCET_URL").unwrap(), account_addr.address()).await?;
    }

    Ok(())
}

fn get_metadata() -> RequestMetadata {
    let mut metadata = RequestMetadata::new(
        ContractIdentity::Null,
        "accounts".to_string(),            // 1011572226,
        "transferAllowanceTo".to_string(), // 603251617,
        10000,
    );
    metadata.params.insert(
        "a".to_string(),
        ethereum_agent_id(
            "e14c3499349cb8d2fd771e09829883e4ecfae02e6b09c9b6a0fb3c7504b4e2f4".to_string(),
            "E913CAc59E0bA840039aDD645D5df83C294CC230".to_string(),
        ),
    );
    metadata.allowance.add_native_token(
        NativeToken::new(
            TokenId::from_str("0x08e14c3499349cb8d2fd771e09829883e4ecfae02e6b09c9b6a0fb3c7504b4e2f40100000000")
                .unwrap(),
            50,
        )
        .unwrap(),
    );
    metadata
}
