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

pub const MIN_GAS_FEE: u64 = 100; // 0.0001 smr

// use crate::RequestMetadata;
//
// EstimateGasOffLedger executes the given on-ledger request without committing
// any changes in the ledger. It returns the amount of gas consumed.
// if use_max_balance is `true` the request will be executed as if the sender had enough base tokens to cover the
// maximum gas allowed WARNING: Gas estimation is just an "estimate", there is no guarantee that the real call will bear
// the same cost, due to the turing-completeness of smart contracts pub fn estimate_gas_off_ledger(
// ch: &mut Chain,
// req: &RequestMetadata,
// key_pair: Option<&cryptolib::KeyPair>,
// use_max_balance: bool,
// ) -> Result<(u64, u64)> {
// let mut req_copy = req.clone();
// if use_max_balance {
// req_copy.with_gas_budget(0);
// }
// let key_pair = key_pair.unwrap_or_else(|| ch.originator_private_key.as_ref().unwrap());
// let r = req_copy.new_request_off_ledger(ch, key_pair);
// let res = ch.estimate_gas(r)?;
// let gas_burned = res.receipt.gas_burned;
// let gas_fee_charged = res.receipt.gas_fee_charged;
// let error = ch.resolve_vm_error(res.receipt.error).as_go_error()?;
// Ok((gas_burned, gas_fee_charged))
// }
//
// EstimateNeededStorageDeposit estimates the amount of base tokens that will be
// needed to add to the request (if any) in order to cover for the storage
// deposit.
// pub fn estimate_needed_storage_deposit(
// ch: &mut Chain,
// req: &RequestMetadata,
// key_pair: Option<&cryptolib::KeyPair>,
// ) -> Result<u64> {
// let out = transaction::make_request_transaction_output(ch.request_transaction_params(req, key_pair));
// let storage_deposit = parameters::L1().protocol.rent_structure.min_rent(out);
//
// let req_deposit = req.ftokens.map_or(0, |ftokens| ftokens.base_tokens);
//
// if req_deposit >= storage_deposit {
// 0
// } else {
// storage_deposit - req_deposit
// }
// }
