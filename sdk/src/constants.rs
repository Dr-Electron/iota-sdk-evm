// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// Every ISC chain is initialized with an instance of the Magic contract at this address
pub const ISC_MAGIC_ADDRESS: &'static str = "0x1074000000000000000000000000000000000000";

/// The ERC20 contract for base tokens is at this address:
pub const ISC_ERC20BASETOKENS_ADDRESS: &'static str = "0x1074010000000000000000000000000000000000";

/// The ERC721 contract for NFTs is at this address:
pub const ISC_ERC721_ADDRESS: &'static str = "0x1074030000000000000000000000000000000000";

// The base chain address from the testnet to which metadata tx should be send
pub const TESTNET_CHAIN_ADDRESS: &'static str = "rms1ppp00k5mmd2m8my8ukkp58nd3rskw6rx8l09aj35984k74uuc5u2cywn3ex";

pub const MIN_GAS_FEE: u64 = 100;
