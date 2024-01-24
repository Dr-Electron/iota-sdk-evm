/* eslint-disable @typescript-eslint/no-var-requires */

import {
    Api,
    Constants,
    initLogger,
} from '@iota/sdk-evm';

import {
    initLogger as sdkInitLogger,
} from '@iota/sdk';

// Run with command:
// yarn run-example ./basic/node.ts

// This example uses secrets in environment variables for simplicity which should not be done in production.
require('dotenv').config({ path: '.env' });

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

        // Not currently working
        // console.log(await api.getInfo());

        let api = await Api.create(process.env.WASP_NODE as string);
        console.log(await api.getReceipt(Constants.TESTNET_CHAIN_ADDRESS, "0x49f2b03ff9fc646ffaf54a8da752ba50c8e112fac3ef82b06025d819be2b3d130000"))

    } catch (error) {
        console.error(error);
    }
}

void run().then(() => process.exit());
