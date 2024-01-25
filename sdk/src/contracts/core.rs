// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::types::block::output::{NftId, TokenScheme};

use crate::{AgentId, Gas, U256Special};

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

/// https://wiki.iota.org/wasp-evm/reference/core-contracts/overview/
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

/// The root contract is one of the core contracts on each IOTA Smart Contracts chain.
///
/// The root contract is responsible for the initialization of the chain.
/// It is the first smart contract deployed on the chain and, upon receiving the init request, bootstraps the state of
/// the chain. Deploying all of the other core contracts is a part of the state initialization.
///
/// The root contract also functions as a smart contract factory for the chain:
/// upon request, it deploys other smart contracts and maintains an on-chain registry of smart contracts in its state.
/// The contract registry keeps a list of contract records containing their respective name, hname, description, and
/// creator.
#[derive(Debug, Eq, PartialEq)]
pub enum RootContract {
    /// The constructor. Automatically called immediately after confirmation of the origin transaction and never called
    /// again. When executed, this function:
    Init,
    /// Deploys a non-EVM smart contract on the chain if the caller has deployment permission.
    DeployContract { ph: [u8; 32], nm: String, ds: String },
    /// The chain owner grants deploy permission to the agent ID.
    GrantDeployPermission { dp: AgentId },
    /// The chain owner revokes the deploy permission of the agent ID.
    RevokeDeployPermission(),
    /// By default, permissions are enabled (addresses need to be granted the right to deploy), but the chain owner can
    /// override this setting to allow anyone to deploy contracts on the chain.
    RequireDeployPermissions(),
}

#[derive(Debug, Eq, PartialEq)]
pub enum AccountsContract {
    /// A no-op that has the side effect of crediting any transferred tokens to the sender's account.
    Deposit,
    /// Moves tokens from the caller's on-chain account to the caller's L1 address. The number of tokens to be
    /// withdrawn must be specified via the allowance of the request.
    Withdraw,
    /// Transfers the specified allowance from the sender's L2 account to the given L2 account on the chain.
    TransferAllowanceTo { a: AgentId },
    /// Transfers the specified allowance from the sender SC's L2 account on the target chain to the sender SC's L2
    /// account on the origin chain.
    TransferAccountToChain { g: Gas },
    /// Creates a new foundry with the specified token scheme, and assigns the foundry to the sender.
    FoundryCreateNew { t: TokenScheme },
    /// Mints or destroys tokens for the given foundry, which must be controlled by the caller.
    FoundryModifySupply { s: u32, d: U256Special, y: bool },
    /// Destroys a given foundry output on L1, reimbursing the storage deposit to the caller. The foundry must be owned
    /// by the caller.
    FoundryDestroy { s: u32 },
    /// Mints an NFT with ImmutableData I that will be owned by the AgentID a
    MintNft {
        i: Vec<u8>,
        a: AgentId,
        c: Option<NftId>,
        w: Option<bool>,
    },
}
