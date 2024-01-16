/* eslint-disable @typescript-eslint/no-var-requires */

import {
    Api,
    Utils,
    initLogger,
    Constants,
    EvmAddress,
    NullIdentity,
    RequestMetadata,
    Contract,
} from '@iota/sdk-evm';

import {
    Wallet,
    Account,
    SecretManager,
    WalletOptions,
    CoinType,
    SenderFeature,
    Utils as SdkUtils,
    TransactionId,
    Client,
    initLogger as sdkInitLogger,
    AccountAddress,
} from '@iota/sdk';
import { AddressUnlockCondition, BlockId } from '@iota/sdk';
// Run with command:
// yarn run-example ./basic/basic.ts

//

// This example uses secrets in environment variables for simplicity which should not be done in production.
require('dotenv').config({ path: '.env' });

// TODO: Use UnitsHelper.MAGNITUDE_MAP["M"]
const ONE_MI = BigInt(1000000);

async function run(): Promise<void> {
    try {
        initLogger();
        sdkInitLogger();
        for (const envVar of [
            'MNEMONIC',
            'STRONGHOLD_SNAPSHOT_PATH',
            'STRONGHOLD_PASSWORD',
            'WALLET_DB_PATH',
            'NODE_URL',
            'WASP_NODE',
        ]) {
            if (!(envVar in process.env)) {
                throw new Error(
                    `.env ${envVar} is undefined, see .env.example`,
                );
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
        //await secretManager.storeMnemonic(process.env.MNEMONIC!);

        const walletOptions: WalletOptions = {
            storagePath: process.env.WALLET_DB_PATH,
            clientOptions: {
                nodes: [process.env.NODE_URL!],
                ignoreNodeHealth: true,
            },
            coinType: CoinType.Shimmer,
            secretManager: strongholdSecretManager,
        };

        let wallet = new Wallet(walletOptions);

        let accounts = await wallet.getAccounts();
        let account;
        if (accounts.length == 0) {
            account = await wallet.createAccount({});
        } else {
            account = accounts[0];
        }

        let balance = await account.sync();

        //let accountAddrs = await account.generateEd25519Addresses(2);
        let accountAddrs = await account.addresses();
        let accountAddr = accountAddrs[0];
        console.log(`Using addr: '${accountAddr.address}'`);

        // Prefixed with 0x
        let evm_address = await secretManager.generateEvmAddresses({
            range: {
                start: accountAddr.keyIndex,
                end: accountAddr.keyIndex + 1,
            },
        });

        console.log(`Using evm address: '${evm_address}'`);

        let api = await Api.create(process.env.WASP_NODE as string);
        let client = await wallet.getClient();

        if (balance.baseCoin.available > ONE_MI) {
            console.log(`Available balance: '${balance.baseCoin.available}'`);

            let assetsPre = await api.getBalance(
                Constants.TESTNET_CHAIN_ADDRESS,
                accountAddr.address,
            );
            console.log('EVM balance pre:', assetsPre);

            console.log(`Sending: '${ONE_MI}'`);
            let _blockId = await sendToEVM(
                account,
                client,
                ONE_MI,
                accountAddr,
            );

            // Wasp node updates after at most 1 more milestone
            console.log('await 1 milestone...');
            await oneMilestone(await wallet.getClient());
            let balancePost = await account.sync();
            console.log(
                `Available balance POST: '${balancePost.baseCoin.available}'`,
            );
            console.log(
                `Available balance DIFF: '${
                    balance.baseCoin.available - balancePost.baseCoin.available
                }'`,
            );

            let assetsPost = await api.getBalance(
                Constants.TESTNET_CHAIN_ADDRESS,
                accountAddr.address,
            );
            console.log('EVM balance POST: ', assetsPost);
            console.log(
                'EVM balance DIFF: ',
                assetsPost.baseTokens - assetsPre.baseTokens,
            );

            console.log('------[ WITHDRAW ]---------');

            _blockId = await withdrawFromEvm(
                account,
                api,
                client,
                BigInt(assetsPre.baseTokens),
                accountAddr,
            );

            // Wasp node updates after at most 1 more milestone
            console.log('await 1 milestone...');
            await oneMilestone(await wallet.getClient());
            let balancePost2 = await account.sync();
            console.log(
                `Available balance POST2: '${balancePost2.baseCoin.available}'`,
            );
            console.log(
                `Available balance DIFF: '${
                    balancePost2.baseCoin.available - balance.baseCoin.available
                }'`,
            );

            let assetsPost2 = await api.getBalance(
                Constants.TESTNET_CHAIN_ADDRESS,
                accountAddr.address,
            );
            console.log('EVM balance post withdraw: ', assetsPost2);
            console.log(
                'EVM balance DIFF: ',
                assetsPost2.baseTokens - assetsPre.baseTokens,
            );
        } else {
            console.log('no available balance. top up at', accountAddr.address);
            client.requestFundsFromFaucet(
                process.env.FAUCET_URL!,
                accountAddr.address,
            );
        }
    } catch (err) {
        console.error(err);
    }
}

// Example translation for withdraw function
async function withdrawFromEvm(
    account: Account,
    api: Api,
    client: Client,
    amount: bigint,
    fromAddr: AccountAddress,
): Promise<BlockId> {
    const metadata = withdraw(amount);
    console.log(metadata);

    //let gasFee = await api.estimateGasOffLedger(Constants.TESTNET_CHAIN_ADDRESS, metadata);
    //console.log(gasFee);

    let outputs = [
        await client.buildBasicOutput({
            unlockConditions: [
                new AddressUnlockCondition(
                    SdkUtils.parseBech32Address(
                        Constants.TESTNET_CHAIN_ADDRESS,
                    ),
                ),
            ],
            features: [
                metadata.asFeature(),
                new SenderFeature(
                    SdkUtils.parseBech32Address(fromAddr.address),
                ),
            ],
        }),
    ];
    console.log(outputs);
    const minDeposit = await client.minimumRequiredStorageDeposit(outputs[0]);
    console.log(minDeposit);
    metadata.allowance.baseTokens += BigInt(minDeposit);
    console.log(metadata);
    console.log(metadata.asFeature());

    outputs = [
        await client.buildBasicOutput({
            amount: BigInt(minDeposit) + Constants.MIN_GAS_FEE, // Use gasFee instead
            unlockConditions: [
                new AddressUnlockCondition(
                    SdkUtils.parseBech32Address(
                        Constants.TESTNET_CHAIN_ADDRESS,
                    ),
                ),
            ],
            features: [
                metadata.asFeature(),
                new SenderFeature(
                    SdkUtils.parseBech32Address(fromAddr.address),
                ),
            ],
        }),
    ];
    console.log(outputs);

    const transaction = await account.sendOutputs(outputs);
    console.log(transaction);
    console.log(
        'Transaction sent:',
        `${process.env.EXPLORER_URL}/${transaction.transactionId}`,
    );

    return wait(account, transaction.transactionId);
}

// Example translation for withdraw function

function withdraw(amount: bigint): RequestMetadata {
    const metadata = new RequestMetadata(
        NullIdentity,
        Contract.Accounts,
        'withdraw',
        Constants.MIN_GAS_FEE * BigInt(100),
    );
    metadata.allowance.baseTokens = amount;
    return metadata;
}

async function sendToEVM(
    account: Account,
    client: Client,
    amount: bigint,
    fromAddr: AccountAddress,
    toAddress?: EvmAddress,
): Promise<BlockId> {
    const metadata = toAddress ? depositTo(amount, toAddress) : deposit(amount);

    let outputs = [
        await client.buildBasicOutput({
            amount: metadata.allowance.baseTokens,
            unlockConditions: [
                new AddressUnlockCondition(
                    SdkUtils.parseBech32Address(
                        Constants.TESTNET_CHAIN_ADDRESS,
                    ),
                ),
            ],
            features: [
                metadata.asFeature(),
                new SenderFeature(
                    SdkUtils.parseBech32Address(fromAddr.address),
                ),
            ],
        }),
    ];

    const transaction = await account.sendOutputs(outputs);
    console.log(
        `Transaction sent: ${process.env.EXPLORER_URL}/transaction/${transaction.transactionId}`,
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
    return blockId;
}

function deposit(amount: bigint): RequestMetadata {
    const metadata = new RequestMetadata(
        NullIdentity,
        Contract.Accounts,
        'deposit',
        Constants.MIN_GAS_FEE * BigInt(100),
    );
    metadata.allowance.baseTokens = amount;

    return metadata;
}

function depositTo(amount: bigint, address: EvmAddress): RequestMetadata {
    const metadata = new RequestMetadata(
        NullIdentity,
        Contract.Accounts,
        'transferAllowanceTo',
        Constants.MIN_GAS_FEE * BigInt(100),
    );
    metadata.params.set(
        'a',
        Utils.ethereumAgentId(
            '42f7da9bdb55b3ec87e5ac1a1e6d88e16768663fde5eca3429eb6f579cc538ac',
            address,
        ),
    );
    metadata.allowance.baseTokens = amount;

    return metadata;
}

void run().then(() => process.exit());
