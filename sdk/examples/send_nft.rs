// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will create an evm transaction
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example send_nft
//! ```

use std::{panic, str::FromStr};

use instant::Duration;
use iota_sdk::{
    client::Client,
    packable::PackableExt,
    types::block::{
        address::Bech32Address,
        output::{
            feature::{IssuerFeature, MetadataFeature, SenderFeature}, unlock_condition::AddressUnlockCondition, BasicOutputBuilder, Feature, NativeToken, NftId, NftOutput, NftOutputBuilder, Output
        },
        payload::transaction::TransactionId,
        BlockId,
    },
    wallet::{account::types::AccountAddress, Account},
    Wallet,
};
use iota_sdk_evm::{
    ethereum_agent_id, Api, ContractIdentity, EvmAddress, RequestMetadata, Result, ACCOUNTS, TESTNET_CHAIN_ADDRESS
};
use prefix_hex::ToHexPrefixed;
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

    let nft_ids = &balance.nfts();
    if !nft_ids.is_empty() {
        let nft_id = nft_ids[0];
        println!("NFT ID: '{:?}'", nft_id);

        let assets_pre = api.get_balance(CUSTOM_CHAIN_ADDRESS, *account_addr.address()).await?;
        let agent_id = &ethereum_agent_id(
            "42f7da9bdb55b3ec87e5ac1a1e6d88e16768663fde5eca3429eb6f579cc538ac",
            &_evm_addr,
        );
        let assets_pre = api.get_balance_l2(CUSTOM_CHAIN_ADDRESS, agent_id.to_hex_prefixed()).await?;
        println!("EVM balance pre: '{:?}'", assets_pre);
        
        // Send on our own l2 linked account
        let _ = send_nft_to_evm(&account, nft_id,  account_addr, &_evm_addr).await?;

        // Wasp node updates after at most 1 more milestone
        println!("await 1 milestone...");
        one_milestone(account.client()).await?;

        println!("await 1 milestone...");
        let assets_post = api.get_balance(CUSTOM_CHAIN_ADDRESS, *account_addr.address()).await?;
        let assets_post = api.get_balance_l2(CUSTOM_CHAIN_ADDRESS, agent_id.to_hex_prefixed()).await?;
        println!("EVM balance post: '{:?}'", assets_post);
    } else {
        println!("No NFTs found, please create one first.");
    }

    Ok(())
}

async fn send_nft_to_evm(
    account: &Account,
    nft_id: NftId,
    from_addr: &AccountAddress,
    to_address: &EvmAddress,
) -> Result<BlockId> {
    let output_data = account.clone().unspent_nft_output(&nft_id).await?.unwrap();
    
    // Unwrap is safe here because we checked that the nft id exists
    let nft = if let Output::Nft(nft_output) = output_data.output {
        nft_output
    } else {
        panic!("NFT not found")
    };

    //let protocol_parameters = account.client().get_protocol_parameters().await?;
    let metadata = deposit_to(vec![nft_id.clone()], to_address);
    //println!("Metadata: '{:x?}'", metadata.pack_to_vec());
    //panic!("stop here");
    println!("NFT ID: '{:?}'", nft_id);

    let outputs = [
        NftOutputBuilder::new_with_amount(1_000_000, nft_id)//new_with_minimum_storage_deposit(protocol_parameters.rent_structure().clone(), nft_id)
            .add_unlock_condition(AddressUnlockCondition::from(
                Bech32Address::from_str(CUSTOM_CHAIN_ADDRESS)?.inner().clone(),
            ))
            // TODO: Copy all needed features and check where to add the metadata
            .with_features([
                Feature::from(MetadataFeature::new(metadata.pack_to_vec())?),
                Feature::from(SenderFeature::new(from_addr.address().clone())),
            ]
            )
            .with_immutable_features(nft.immutable_features().clone())
            .finish()
            .unwrap()
            .into(),
        /*BasicOutputBuilder::new_with_amount(1_000_000)
            .add_unlock_condition(AddressUnlockCondition::from(
                Bech32Address::from_str(CUSTOM_CHAIN_ADDRESS)?.inner().clone(),
            ))
            .with_features([
                Feature::from(MetadataFeature::new(metadata.pack_to_vec())?),
                Feature::from(SenderFeature::new(from_addr.address().clone())),
            ])
            .finish()
            .unwrap()
            .into(),*/
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

fn deposit_to(nft_ids: Vec<NftId>, address: &EvmAddress) -> RequestMetadata {
    let mut metadata = RequestMetadata::new(
        ContractIdentity::Null,
        ACCOUNTS.to_string(),
        "transferAllowanceTo".to_string(),
        1_000_000,//MIN_GAS_FEE
    );
    metadata.params.insert(
        "a".to_string(),
        ethereum_agent_id(
            // TODO: Use static value
            "42f7da9bdb55b3ec87e5ac1a1e6d88e16768663fde5eca3429eb6f579cc538ac",
            address,
        ),
    );
    println!("NFT IDs: '{:?}'", nft_ids);
    metadata.allowance.set_nfts(nft_ids);

    metadata
}
