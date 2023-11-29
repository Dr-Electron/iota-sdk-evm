// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will create an evm transaction
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example basic
//! ```

use std::str::FromStr;

use instant::Duration;
use iota_sdk::{
    client::{
        constants::SHIMMER_COIN_TYPE,
        secret::{stronghold::StrongholdSecretManager, SecretManage, SecretManager},
        Client,
    },
    crypto::keys::bip39::Mnemonic,
    packable::PackableExt,
    types::block::{
        address::Bech32Address,
        output::{
            feature::{MetadataFeature, SenderFeature},
            unlock_condition::AddressUnlockCondition,
            BasicOutputBuilder, Feature,
        },
        payload::transaction::TransactionId,
        BlockId,
    },
    wallet::{account::types::AccountAddress, Account, ClientOptions},
    Wallet,
};
use iota_sdk_evm::{
    ethereum_agent_id, Api, ContractIdentity, EvmAddress, RequestMetadata, Result, ACCOUNTS, TESTNET_CHAIN_ADDRESS,
};
use url::Url;

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
    println!("Using mnemonic: {:?}", mnemonic);

    // secret_manager.store_mnemonic(mnemonic).await.unwrap();

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

    let evm_address = wallet
        .get_secret_manager()
        .read()
        .await
        .generate_evm_addresses(
            iota_sdk::client::api::GetAddressesOptions::default()
                .with_range(*account_addr.key_index()..*account_addr.key_index() + 1),
        )
        .await?;
    let bytes: [u8; 20] = prefix_hex::decode(&evm_address[0]).unwrap();
    let _evm_addr = EvmAddress::from(bytes);

    println!("Using evm address: {:?}", evm_address);

    let wasp_url = std::env::var("WASP_NODE").unwrap();
    let api = Api::new(Url::parse(wasp_url.as_str()).unwrap());

    println!("wasp node: '{:?}'", api.info().await?);

    if balance.base_coin().available() > 0 {
        // 225053825 glow -> 220.053826 SMR ( 4999999 gas fee + 0.01 fee on evm )

        println!("Available balance: '{:?}'", balance.base_coin().available() / 2);

        // 56171331 -> 56143231
        // = 28100 = 28000 + MIN_GAS_FEE

        let assets_pre = api.get_balance(TESTNET_CHAIN_ADDRESS, *account_addr.address()).await?;
        println!("EVM balance pre: '{:?}'", assets_pre);

        let to_send = 1000; //balance.base_coin().available() / 2;
        println!("Sending: '{:?}'", to_send);
        // let _ = send_to_evm(&account, to_send, account_addr, Some(&evm_addr)).await?;
        let _ = send_to_evm(&account, to_send, account_addr, None).await?;

        // Wasp node updates after at most 1 more milestone
        println!("await 1 milestone...");
        one_milestone(account.client()).await?;

        let assets_post = api.get_balance(TESTNET_CHAIN_ADDRESS, *account_addr.address()).await?;
        println!("EVM balance post: '{:?}'", assets_post);

        println!("------[ WITHDRAW ]---------");

        let _ = withdraw_from_evm(&account, assets_post.base_tokens, account_addr).await?;

        // Wasp node updates after at most 1 more milestone
        println!("await 1 milestone...");
        one_milestone(account.client()).await?;

        let assets_post = api.get_balance(TESTNET_CHAIN_ADDRESS, *account_addr.address()).await?;
        println!("EVM balance post withdraw: '{:?}'", assets_post);
    } else {
        println!("no available balance. top up at '{:?}'", account_addr.address());
        iota_sdk::client::request_funds_from_faucet(&std::env::var("FAUCET_URL").unwrap(), account_addr.address())
            .await
            .unwrap();
    }

    Ok(())
}

async fn withdraw_from_evm(account: &Account, amount: u64, from_addr: &AccountAddress) -> Result<BlockId> {
    let protocol_parameters = account.client().get_protocol_parameters().await?;
    let metadata = withdraw(amount);
    let outputs = [
        BasicOutputBuilder::new_with_minimum_storage_deposit(protocol_parameters.rent_structure().clone())
            .add_unlock_condition(AddressUnlockCondition::from(
                Bech32Address::from_str(TESTNET_CHAIN_ADDRESS)?.inner().clone(),
            ))
            .with_features([
                Feature::from(MetadataFeature::new(metadata.pack_to_vec())?),
                Feature::from(SenderFeature::new(from_addr.address().clone())),
            ])
            .finish()
            .unwrap()
            .into(),
    ];

    let transaction = account.send_outputs(outputs, None).await?;
    println!(
        "Transaction sent: {}/transaction/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        transaction.transaction_id
    );

    wait(account, &transaction.transaction_id).await
}

async fn send_to_evm(
    account: &Account,
    amount: u64,
    from_addr: &AccountAddress,
    to_address: Option<&EvmAddress>,
) -> Result<BlockId> {
    let protocol_parameters = account.client().get_protocol_parameters().await?;
    let metadata = match to_address {
        Some(a) => deposit_to(amount, a),
        None => deposit(amount),
    };

    let outputs = [
        BasicOutputBuilder::new_with_minimum_storage_deposit(protocol_parameters.rent_structure().clone())
            .add_unlock_condition(AddressUnlockCondition::from(
                Bech32Address::from_str(TESTNET_CHAIN_ADDRESS)?.inner().clone(),
            ))
            .with_features([
                Feature::from(MetadataFeature::new(metadata.pack_to_vec())?),
                Feature::from(SenderFeature::new(from_addr.address().clone())),
            ])
            .finish()
            .unwrap()
            .into(),
    ];

    let transaction = account.send_outputs(outputs, None).await?;
    println!(
        "Transaction sent: {}/transaction/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        transaction.transaction_id
    );

    wait(account, &transaction.transaction_id).await
}

async fn one_milestone(_client: &Client) -> Result<()> {
    let duration = Duration::from_secs(3);
    tokio::time::sleep(duration).await;
    Ok(())
}

async fn wait(account: &Account, tx: &TransactionId) -> Result<BlockId> {
    // Wait for transaction to get included
    let block_id = account.retry_transaction_until_included(tx, None, None).await?;

    println!(
        "Block included: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block_id
    );
    Ok(block_id)
}

fn withdraw(amount: u64) -> RequestMetadata {
    let mut metadata = RequestMetadata::new(
        ContractIdentity::Null,
        ACCOUNTS.to_string(),
        "withdraw".to_string(),
        500,
    );
    metadata.allowance.set_base_tokens(amount);
    metadata
}

fn deposit(amount: u64) -> RequestMetadata {
    let mut metadata = RequestMetadata::new(
        ContractIdentity::Null,
        ACCOUNTS.to_string(),
        "deposit".to_string(),
        iota_sdk_evm::MIN_GAS_FEE,
    );
    metadata.allowance.set_base_tokens(amount - iota_sdk_evm::MIN_GAS_FEE);

    metadata
}

fn deposit_to(amount: u64, address: &EvmAddress) -> RequestMetadata {
    let mut metadata = RequestMetadata::new(
        ContractIdentity::Null,
        ACCOUNTS.to_string(),
        "transferAllowanceTo".to_string(),
        iota_sdk_evm::MIN_GAS_FEE,
    );
    metadata.params.insert(
        "a".to_string(),
        ethereum_agent_id(
            "42f7da9bdb55b3ec87e5ac1a1e6d88e16768663fde5eca3429eb6f579cc538ac",
            address,
        ),
    );
    metadata.allowance.set_base_tokens(amount - iota_sdk_evm::MIN_GAS_FEE);

    metadata
}
