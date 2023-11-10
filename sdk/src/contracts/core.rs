// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::AgentId;

/// Responsible for the initialization of the chain, maintains registry of deployed contracts.
pub const ROOT: &'static str = "root";

/// Manages the on-chain ledger of accounts.
pub const ACCOUNTS: &'static str = "accounts";
/// Responsible for the registry of binary objects of arbitrary size.
pub const BLOB: &'static str = "blob";
/// Keeps track of the blocks and receipts of requests that were processed by the chain.
pub const BLOCK_LOG: &'static str = "blocklog";
/// Handles the administrative functions of the chain. For example: rotation of the committee of validators of the
/// chain, fees and other chain-specific configurations.
pub const GOVERNANCE: &'static str = "governance";
/// Keeps a map of error codes to error messages templates. These error codes are used in request receipts.
pub const ERRORS: &'static str = "errors";
/// Provides the necessary infrastructure to accept Ethereum transactions and execute EVM code.
pub const EVM: &'static str = "evm";

pub enum CoreContracts {
    Root(RootContract),
    Accounts(AccountsContract),
}

impl CoreContracts {
    pub fn name(&self) -> &str {
        match self {
            CoreContracts::Root(_) => ROOT,
            CoreContracts::Accounts(_) => ACCOUNTS,
        }
    }
}

impl core::fmt::Debug for CoreContracts {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            CoreContracts::Root(c) => format!("{}({:?})", self.name(), c),
            CoreContracts::Accounts(c) => format!("{}({:?})", self.name(), c),
        }
        .fmt(f)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum RootContract {
    Init,
    DeployContract { ph: [u8; 32], nm: String, ds: String },
    GrantDeployPermission { dp: AgentId },
    RevokeDeployPermission(),
    RequireDeployPermissions(),
}

#[derive(Debug, Eq, PartialEq)]
pub enum AccountsContract {}
