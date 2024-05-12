// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will create an evm transaction
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example send_native_token
//! ```

use std::{f32::MIN, str::FromStr};

use instant::Duration;
use iota_sdk::{
    client::{
        constants::SHIMMER_COIN_TYPE,
        secret::{stronghold::StrongholdSecretManager, SecretManager},
        Client,
    },
    crypto::keys::bip39::Mnemonic,
    packable::PackableExt,
    types::block::{
        address::Bech32Address,
        output::{
            feature::{MetadataFeature, SenderFeature},
            unlock_condition::AddressUnlockCondition,
            BasicOutputBuilder, Feature, NativeToken, TokenId,
        },
        payload::transaction::TransactionId,
        BlockId,
    },
    wallet::{account::types::AccountAddress, Account, ClientOptions},
    Wallet,
};
use iota_sdk_evm::{
    ethereum_agent_id, Api, ContractIdentity, EvmAddress, RequestMetadata, Result, ACCOUNTS, MIN_GAS_FEE, TESTNET_CHAIN_ADDRESS
};
use url::Url;

const CUSTOM_CHAIN_ADDRESS: &str = TESTNET_CHAIN_ADDRESS;

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Create the wallet
    let wallet = Wallet::builder()
        .with_storage_path(&std::env::var("WALLET_DB_PATH").unwrap())
        .finish()
        .await?;

    wallet.set_stronghold_password(std::env::var("STRONGHOLD_PASSWORD").unwrap()).await?;

    // Get or create a new account
    let account = wallet.get_or_create_account("Alice").await?;

    let balance = account.sync(None).await?;
    let account_addr = &account.addresses().await?[0];
    println!("Using addr: '{:?}'", account_addr.address());
    println!("{:?}", balance.base_coin().available());

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
    let parsed_url = Url::parse(wasp_url.as_str()).unwrap();
    let api = Api::new(parsed_url);

    // Not working atm
    // println!("wasp node: '{:?}'", api.info().await?);

    let token_balance = &balance.native_tokens()[0];
    println!("Token balance: '{:?}'", token_balance);

    let to_send = 1;
    if token_balance.available() > to_send.into() {
        println!("Available balance: '{:?}'", token_balance.available());

        let assets_pre = api.get_balance(CUSTOM_CHAIN_ADDRESS, *account_addr.address()).await?;
        println!("EVM balance pre: '{:?}'", assets_pre);

        println!("Sending: '{:?}'", to_send);
        // Send to an address
        // let _ = send_to_evm(&account, to_send, account_addr, Some(&evm_addr)).await?;
        let tokens = vec![NativeToken::new(token_balance.token_id().clone(), to_send).unwrap()];

        // Send on our own l2 linked account
        let _ = send_native_token_to_evm(&account, tokens,  account_addr, &_evm_addr).await?;

        // Wasp node updates after at most 1 more milestone
        println!("await 1 milestone...");
        one_milestone(account.client()).await?;

        println!("await 1 milestone...");
        let assets_post = api.get_balance(CUSTOM_CHAIN_ADDRESS, *account_addr.address()).await?;
        println!("EVM balance post: '{:?}'", assets_post);
    } else {
        println!("no available balance. top up at '{:?}'", account_addr.address());
        iota_sdk::client::request_funds_from_faucet(&std::env::var("FAUCET_URL").unwrap(), account_addr.address())
            .await
            .unwrap();
    }

    Ok(())
}

async fn send_native_token_to_evm(
    account: &Account,
    tokens: Vec<NativeToken>,
    from_addr: &AccountAddress,
    to_address: &EvmAddress,
) -> Result<BlockId> {
    let metadata = deposit_to(tokens.clone(), to_address);
    println!("metadata: '{:x?}'", metadata.pack_to_vec());
    panic!("stop here");

    let outputs = [
        BasicOutputBuilder::new_with_amount(1_000_000)
            .add_unlock_condition(AddressUnlockCondition::from(
                Bech32Address::from_str(CUSTOM_CHAIN_ADDRESS)?.inner().clone(),
            ))
            .with_features([
                Feature::from(MetadataFeature::new(metadata.pack_to_vec())?),
                Feature::from(SenderFeature::new(from_addr.address().clone())),
            ])
            .with_native_tokens(tokens)
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
    // we cheat
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

fn deposit_to(tokens: Vec<NativeToken>, address: &EvmAddress) -> RequestMetadata {
    let mut metadata = RequestMetadata::new(
        ContractIdentity::Null,
        ACCOUNTS.to_string(),
        "transferAllowanceTo".to_string(),
        10000,//MIN_GAS_FEE
    );
    metadata.params.insert(
        "a".to_string(),
        ethereum_agent_id(
            "97de24fe2e39737ff9d0fd1d7d9b50f9d4049badbc7ec49462cbb7e08dcb4535",
            address,
        ),
    );
    println!("tokens: '{:?}'", tokens);
    metadata.allowance.set_native_tokens(tokens);

    metadata
}
