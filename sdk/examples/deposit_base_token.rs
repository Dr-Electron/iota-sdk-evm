// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will create an evm transaction
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example deposit_base_token
//! ```

use std::{
    str::FromStr,
    time::{SystemTime, UNIX_EPOCH},
};

use instant::Duration;
use iota_sdk::{
    client::Client,
    packable::PackableExt,
    types::block::{
        address::Bech32Address,
        output::{
            feature::{MetadataFeature, SenderFeature},
            unlock_condition::{AddressUnlockCondition, ExpirationUnlockCondition},
            BasicOutputBuilder, Feature,
        },
        payload::transaction::TransactionId,
        BlockId,
    },
    wallet::{account::types::AccountAddress, Account},
    Wallet,
};
use iota_sdk_evm::{
    ethereum_agent_id, Api, ContractIdentity, EvmAddress, RequestMetadata, Result, ACCOUNTS, MIN_GAS_FEE,
    TESTNET_CHAIN_ADDRESS,
};
use url::Url;

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Create the wallet
    let wallet = Wallet::builder()
        .with_storage_path(&std::env::var("WALLET_DB_PATH").unwrap())
        .finish()
        .await?;

    wallet
        .set_stronghold_password(std::env::var("STRONGHOLD_PASSWORD").unwrap())
        .await?;

    // Get or create a new account
    let account = wallet.get_or_create_account("Alice").await?;
    // let account_addrs = account.generate_ed25519_addresses(2, None).await?;

    let balance = account.sync(None).await?;
    let account_addr = &account.addresses().await?[0];
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

    if balance.base_coin().available() > 0 {
        println!("Available balance: '{:?}'", balance.base_coin().available());

        let assets_pre = api.get_balance(TESTNET_CHAIN_ADDRESS, *account_addr.address()).await?;
        println!("EVM balance pre: '{:?}'", assets_pre);

        let to_send = 1_000_000;
        println!("Sending: '{:?}'", to_send);

        // Send on our own l2 linked account
        let _ = send_to_evm(&account, to_send, account_addr, None).await?;

        // Wasp node updates after at most 1 more milestone
        println!("await 1 milestone...");
        one_milestone(account.client()).await?;

        let assets_post = api.get_balance(TESTNET_CHAIN_ADDRESS, *account_addr.address()).await?;
        println!("EVM balance post: '{:?}'", assets_post);
    } else {
        println!("no available balance. top up at '{:?}'", account_addr.address());
        iota_sdk::client::request_funds_from_faucet(&std::env::var("FAUCET_URL").unwrap(), account_addr.address())
            .await
            .unwrap();
    }

    Ok(())
}

async fn send_to_evm(
    account: &Account,
    amount: u64,
    from_addr: &AccountAddress,
    to_address: Option<&EvmAddress>,
) -> Result<BlockId> {
    let metadata = match to_address {
        Some(a) => deposit_to(amount, a),
        None => deposit(amount),
    };

    // Get current unix timestamp
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let outputs = [BasicOutputBuilder::new_with_amount(amount)
        .add_unlock_condition(AddressUnlockCondition::from(
            Bech32Address::from_str(TESTNET_CHAIN_ADDRESS)?.inner().clone(),
        ))
        .add_unlock_condition(ExpirationUnlockCondition::new(
            from_addr.address(),
            (now + 3600).try_into().unwrap(),
        )?)
        .with_features([
            Feature::from(MetadataFeature::new(metadata.pack_to_vec())?),
            Feature::from(SenderFeature::new(from_addr.address().clone())),
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

fn deposit(amount: u64) -> RequestMetadata {
    let mut metadata = RequestMetadata::new(
        ContractIdentity::Null,
        ACCOUNTS.to_string(),
        "deposit".to_string(),
        MIN_GAS_FEE * 100,
    );
    metadata.allowance.set_base_tokens(amount - iota_sdk_evm::MIN_GAS_FEE);

    metadata
}

fn deposit_to(amount: u64, address: &EvmAddress) -> RequestMetadata {
    let mut metadata = RequestMetadata::new(
        ContractIdentity::Null,
        ACCOUNTS.to_string(),
        "transferAllowanceTo".to_string(),
        MIN_GAS_FEE * 100,
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
