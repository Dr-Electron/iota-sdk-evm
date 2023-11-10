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
        block::{
            address::{Address, Bech32Address, Ed25519Address},
            output::{
                feature::{MetadataFeature, SenderFeature},
                unlock_condition::AddressUnlockCondition,
                BasicOutputBuilder, Feature, NativeToken, TokenId,
            },
        },
        ValidationParams,
    },
    wallet::ClientOptions,
    Wallet,
};
use iota_sdk_evm::{
    ethereum_agent_id, AgentId, ContractIdentity, Error, RequestMetadata, Result, ACCOUNTS, TESTNET_CHAIN_ADDRESS,
};
use packable::PackableExt;

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Setup Stronghold secret_manager
    let secret_manager = StrongholdSecretManager::builder()
        .password(std::env::var("STRONGHOLD_PASSWORD").unwrap())
        .build(std::env::var("STRONGHOLD_SNAPSHOT_PATH").unwrap())
        .unwrap();

    // Only required the first time, can also be generated with `manager.generate_mnemonic()?`
    let mnemonic = Mnemonic::from(std::env::var("MNEMONIC").unwrap());

    // The mnemonic only needs to be stored the first time
    // secret_manager
    // .store_mnemonic(mnemonic)
    // .await
    // .map_err(|e| crate::Error::SdkWallet(e.into()))?;

    let client_options = ClientOptions::new()
        .with_node(&std::env::var("NODE_URL").unwrap())
        .unwrap();

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
    let account_addrs = account.generate_ed25519_addresses(2, None).await?;
    let balance = account.sync(None).await?;
    let account_addr = &account_addrs[0];
    println!("Using addr: '{:?}'", account_addr.address());
    println!("Available balance: '{:?}'", balance);

    let protocol_parameters = account.client().get_protocol_parameters().await?;
    println!(
        "protocol_parameters: '{:?}'",
        protocol_parameters.bech32_hrp().to_string()
    );

    if balance.base_coin().available() > 0 {
        let protocol_parameters = account.client().get_protocol_parameters().await?;

        println!("Available balance: '{:?}'", balance.base_coin().available() / 2);

        let to_send = balance.base_coin().available() / 2;
        let metadata = deposit(to_send);

        let outputs = [BasicOutputBuilder::new_with_amount(to_send)
            .add_unlock_condition(AddressUnlockCondition::from(
                Bech32Address::from_str(TESTNET_CHAIN_ADDRESS)?.inner().clone(),
            ))
            .with_features([
                Feature::from(MetadataFeature::new(metadata.pack_to_vec())?),
                Feature::from(SenderFeature::new(account_addr.address().clone())),
            ])
            .finish()
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
        iota_sdk::client::request_funds_from_faucet(&std::env::var("FAUCET_URL").unwrap(), account_addr.address())
            .await
            .unwrap();
    }

    Ok(())
}

/// 0x00025e4b3c410fcc9dc096b10200809fb0b378
/// 0x 01 02 5e 4b 3c 00 00 00 00 00 00 00 00 01 00 00
fn deposit(amount: u64) -> RequestMetadata {
    let mut metadata = RequestMetadata::new(
        ContractIdentity::Null,
        ACCOUNTS.to_string(),
        "withdraw".to_string(),
        4999999,
    );
    metadata.allowance.set_base_tokens(amount);
    metadata
}

fn get_metadata(amount: u64) -> RequestMetadata {
    let mut metadata = RequestMetadata::new(
        ContractIdentity::Null,
        ACCOUNTS.to_string(),              // 1011572226,
        "transferAllowanceTo".to_string(), // 603251617,
        4999999,
    );
    metadata.params.insert(
        "a".to_string(),
        ethereum_agent_id(
            "42f7da9bdb55b3ec87e5ac1a1e6d88e16768663fde5eca3429eb6f579cc538ac".to_string(),
            "E913CAc59E0bA840039aDD645D5df83C294CC230".to_string(),
        ),
    );
    metadata.allowance.set_base_tokens(amount - 4999999);
    // metadata.allowance.add_native_token(
    // NativeToken::new(
    // TokenId::from_str("0x08e14c3499349cb8d2fd771e09829883e4ecfae02e6b09c9b6a0fb3c7504b4e2f40100000000")
    // .unwrap(),
    // 50,
    // )
    // .unwrap(),
    // );
    metadata
}
