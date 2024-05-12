// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will mint some NFTs.
//!
//! Make sure that `STRONGHOLD_SNAPSHOT_PATH` and `WALLET_DB_PATH` already exist by
//! running the `./how_tos/accounts_and_addresses/create_account.rs` example!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example utility_create_nft
//! ```

use iota_sdk::{
    types::block::output::feature::Irc27Metadata,
    wallet::{MintNftParams, Result},
    Wallet,
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    for var in ["WALLET_DB_PATH", "STRONGHOLD_PASSWORD", "EXPLORER_URL"] {
        std::env::var(var).expect(&format!(".env variable '{var}' is undefined, see .env.example"));
    }

    // Create the wallet
    let wallet = Wallet::builder()
        .with_storage_path(&std::env::var("WALLET_DB_PATH").unwrap())
        .finish()
        .await?;

    // Get or create account alice
    let account = wallet.get_or_create_account("Alice").await?;

    // Ensure the account is synced after minting.
    account.sync(None).await?;

    // We send from the first address in the account.
    let sender_address = *account.addresses().await?[0].address();

    // Set the stronghold password
    wallet
        .set_stronghold_password(std::env::var("STRONGHOLD_PASSWORD").unwrap())
        .await?;

    let metadata = Irc27Metadata::new(
        "video/mp4",
        "https://ipfs.io/ipfs/QmPoYcVm9fx47YXNTkhpMEYSxCD3Bqh7PJYr7eo5YjLgiT"
            .parse()
            .unwrap(),
        "Shimmer OG NFT",
    )
    .with_description("The original Shimmer NFT");

    let nft_params = [MintNftParams::new()
        //.try_with_address(NFT1_OWNER_ADDRESS)?
        .try_with_sender(sender_address)?
        .try_with_issuer(sender_address)?
        .with_immutable_metadata(metadata.to_bytes())];

    let transaction = account.mint_nfts(nft_params, None).await?;
    println!("Transaction sent: {}", transaction.transaction_id);

    // Wait for transaction to get included
    let block_id = account
        .retry_transaction_until_included(&transaction.transaction_id, None, None)
        .await?;
    println!(
        "Block included: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block_id
    );
    println!("Minted NFT");

    Ok(())
}