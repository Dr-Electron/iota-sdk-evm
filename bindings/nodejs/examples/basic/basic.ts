/* eslint-disable @typescript-eslint/no-var-requires */

import { Api, Utils, initLogger, Constants, EvmAddress, NullIdentity, RequestMetadata, Contract } from "@iota/sdk-evm";

import { Wallet, Account, SecretManager, WalletOptions, CoinType, Address, SenderFeature, MetadataFeature, TransactionId, Client, AccountAddress, Bech32Address, Ed25519Address } from '@iota/sdk';
import { AddressUnlockCondition, BlockId } from '@iota/sdk';

// Run with command:
// yarn run-example ./basic/basic.ts

// 

// This example uses secrets in environment variables for simplicity which should not be done in production.
require('dotenv').config({ path: '.env' });

async function run(): Promise<void> {
    try {
        initLogger();
        for (const envVar of ['MNEMONIC', 'STRONGHOLD_SNAPSHOT_PATH', 'STRONGHOLD_PASSWORD', 'WALLET_DB_PATH', 'NODE_URL']) {
            if (!(envVar in process.env)) {
                throw new Error(`.env ${envVar} is undefined, see .env.example`);
            }
        }

        const strongholdSecretManager = {
            stronghold: {
                snapshotPath: process.env.STRONGHOLD_SNAPSHOT_PATH,
                password: process.env.STRONGHOLD_PASSWORD,
            },
        };
        const secretManager = new SecretManager(strongholdSecretManager);
        console.log('Using mnemonic:', process.env.MNEMONIC);

        // Store the mnemonic in the Stronghold snapshot, this needs to be done only the first time.
        // The mnemonic can't be retrieved from the Stronghold file, so make a backup in a secure place!
        // await secretManager.storeMnemonic(process.env.MNEMONIC!);

        const walletOptions: WalletOptions = {
            storagePath: process.env.WALLET_DB_PATH,
            clientOptions: {
                nodes: [process.env.NODE_URL!],
                ignoreNodeHealth: true
            },
            coinType: CoinType.IOTA,
            secretManager: strongholdSecretManager,
        };

        let wallet = new Wallet(walletOptions);

        let account = (await wallet.getAccounts())[0];
        let accountAddrs = await account.generateEd25519Addresses(2);

        let balance = await account.sync();
        let accountAddr = accountAddrs[0];
        console.log(`Using addr: '${accountAddr.address}'`);

        let evm_address = await secretManager
            .generateEvmAddresses({
                range: {
                    start: accountAddr.keyIndex,
                    end: accountAddr.keyIndex + 1,
                }
            });
        //let bytes prefix_hex::decode(&evm_address[0]).unwrap();
        //let _evm_addr = new EvmAddress(bytes);

        console.log(`Using evm address: '${evm_address}'`);

        let api = await Api.create(process.env.WASP_NODE as string);
        let client = await wallet.getClient();

        if (balance.baseCoin.available > 0) {
            // 225053825 glow -> 220.053826 SMR ( 4999999 gas fee + 0.01 fee on evm )

            console.log(`Available balance: '${balance.baseCoin.available / BigInt(2)}'`);

            // 56171331 -> 56143231
            // = 28100 = 28000 + MIN_GAS_FEE

            let assetsPre = await api.getBalance(Constants.TESTNET_CHAIN_ADDRESS, accountAddr.address);
            console.log('EVM balance pre:' , assetsPre);

            let to_send = BigInt(1000); //balance.base_coin().available() / BigInt(2);
            console.log(`Sending: '${to_send}'`);
            // let _ = send_to_evm(&account, to_send, accountAddr, Some(&evm_addr));
            let _blockId = await sendToEVM(account, client, to_send, new Ed25519Address(accountAddr.address));

            // Wasp node updates after at most 1 more milestone
            console.log("await 1 milestone...");
            await oneMilestone(await wallet.getClient());

            let assetsPost = await api.getBalance(Constants.TESTNET_CHAIN_ADDRESS, accountAddr.address);
            console.log(`EVM balance post: '${assetsPost}'`);

            console.log("------[ WITHDRAW ]---------");

            _blockId = await withdrawFromEvm(account, client, assetsPost.baseTokens, new Ed25519Address(accountAddr.address));

            // Wasp node updates after at most 1 more milestone
            console.log("await 1 milestone...");
            await oneMilestone(await wallet.getClient());

            assetsPost = await api.getBalance(Constants.TESTNET_CHAIN_ADDRESS, accountAddr.address);
            console.log('EVM balance post withdraw: ', assetsPost);

        } else {
            console.log('no available balance. top up at', accountAddr.address);
            client.requestFundsFromFaucet(process.env.FAUCET_URL!, accountAddr.address);
        }
    } catch (err) {
        console.error(err);
    }
}

// Example translation for withdraw function
async function withdrawFromEvm(account: Account, client: Client, amount: bigint, fromAddr: Address): Promise<BlockId> {
    const metadata = withdraw(amount);
    
    const outputs = [
        await client.buildBasicOutput({
            unlockConditions: [new AddressUnlockCondition(
                new Ed25519Address(Constants.TESTNET_CHAIN_ADDRESS),
            )],
            features: [
                    metadata.asFeature(),
                    new SenderFeature(fromAddr),
                ],
        })
    ];

    const transaction = await account.sendOutputs(outputs);
    console.log('Transaction sent:', `${process.env.EXPLORER_URL}/${transaction.transactionId}`);

    return wait(account, transaction.transactionId);
}

async function sendToEVM(
    account: Account,
    client: Client,
    amount: bigint,
    fromAddr: Address,
    toAddress?: EvmAddress
): Promise<BlockId> {
    console.log("sendToEVM")
    const metadata = toAddress
        ? depositTo(amount, toAddress)
        : deposit(amount);
    console.log(metadata)
    
    const outputs = [
        await client.buildBasicOutput({
            unlockConditions: [new AddressUnlockCondition(
                new Ed25519Address(Constants.TESTNET_CHAIN_ADDRESS),
            )],
            features: [
                    metadata.asFeature(),
                    new SenderFeature(fromAddr),
                ],
        })
    ];
    console.log(outputs)

    const transaction = await account.sendOutputs(outputs);
    console.log(
        `Transaction sent: ${process.env.EXPLORER_URL}/transaction/${
        transaction.transactionId
        }`
    );

    return wait(account, transaction.transactionId);
}

async function oneMilestone(client: Client): Promise<void> {
    const duration = 3; // seconds
    await new Promise((resolve) => setTimeout(resolve, duration * 1000));
}

async function wait(account: Account, tx: TransactionId): Promise<BlockId> {
    // Wait for transaction to get included
    let blockId = await account.retryTransactionUntilIncluded(tx);

    console.log(`Block included: ${process.env.EXPLORER_URL}/block/${blockId}`);
    return blockId
}

// Example translation for withdraw function
function withdraw(amount: bigint): RequestMetadata {
    const metadata = new RequestMetadata(
        NullIdentity,
        Contract.Account,
        'withdraw',
        BigInt(500),
    );
    metadata.allowance.baseTokens = amount;
    return metadata;
}

function deposit(amount: bigint): RequestMetadata {
    const metadata = new RequestMetadata(
        NullIdentity,
        Contract.Account,
        'deposit',
        Constants.MIN_GAS_FEE,
    );
    metadata.allowance.baseTokens = amount - Constants.MIN_GAS_FEE;

    return metadata;
}

function depositTo(amount: bigint, address: EvmAddress): RequestMetadata {
    const metadata = new RequestMetadata(
        NullIdentity,
        Contract.Account,
        'transferAllowanceTo',
        Constants.MIN_GAS_FEE,
    );
    metadata.params.set(
        'a',
        Utils.ethereumAgentId(
            '42f7da9bdb55b3ec87e5ac1a1e6d88e16768663fde5eca3429eb6f579cc538ac',
            address,
        ),
    );
    metadata.allowance.baseTokens = amount - Constants.MIN_GAS_FEE;

    return metadata;
}


void run().then(() => process.exit());
