use std::{
    collections::HashMap,
    path::PathBuf,
    str::FromStr,
    time::{Duration, SystemTime},
};

use iota_sdk::types::block::output::{NativeToken, TokenId};
use serde;
use serde_json;

mod buffer;
mod error;
mod metadata;

pub use buffer::*;
pub use error::*;
pub use metadata::*;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

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

    let metadata_ser = metadata.serialize()?;
    println!("{:?}", metadata_ser);

    let actual_metadata_ser = "00025e4b3ca1e3f423914e0101613503e14c3499349cb8d2fd771e09829883e4ecfae02e6b09c9b6a0fb3c7504b4e2f4e913cac59e0ba840039add645d5df83c294cc230400108e14c3499349cb8d2fd771e09829883e4ecfae02e6b09c9b6a0fb3c7504b4e2f401000000000132";
    assert_eq!(actual_metadata_ser, metadata_ser);

    let buffer_cursor = SimpleBufferCursor::from(hex::decode(actual_metadata_ser).unwrap());
    let new_meta = read_metadata(buffer_cursor).await?;
    println!("{:?}", new_meta);

    assert_eq!(metadata, new_meta);

    Ok(())
}
