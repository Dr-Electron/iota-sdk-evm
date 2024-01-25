// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

const console = require('console');
const { Api, initLogger } = require('@iota/sdk-evm-wasm/node');

async function run() {
    initLogger();

    let api = await Api.create(process.env.WASP_NODE);

    try {
        const nodeInfo = await api.getInfo();
        console.log('Node info: ', nodeInfo);

        console.log(await api.getReceipt(Constants.TESTNET_CHAIN_ADDRESS, "0x49f2b03ff9fc646ffaf54a8da752ba50c8e112fac3ef82b06025d819be2b3d130000"))
    } catch (error) {
        console.error('Error: ', error);
    }
}

void run().then(() => process.exit());
