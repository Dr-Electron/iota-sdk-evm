// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Assets, WaspInfo } from '../types';
import { ApiMethodHandler } from './api-method-handler';

import { Bech32Address } from '@iota/sdk';

/** The Client to interact with nodes. */
export class Api {
    private methodHandler: ApiMethodHandler;

    /**
     * @param methodHandler The Rust method handler created in `ApiMethodHandler.create()`.
     */
    constructor(methodHandler: ApiMethodHandler) {
        this.methodHandler = methodHandler;
    }

    /**
     * @param options The client options.
     */
    static async create(url: String): Promise<Api> {
        return new Api(await ApiMethodHandler.create(url));
    }
    async destroy(): Promise<void> {
        return this.methodHandler.destroy();
    }

    /**
     * Get the node information together with the url of the used node.
     */
    async getInfo(): Promise<WaspInfo> {
        const response = await this.methodHandler.callMethod({
            name: 'getInfo',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Returns the balance of an l1 address available for l2 transfers.
     */
    async getBalance(chain: string, address: Bech32Address): Promise<Assets> {
        const response = await this.methodHandler.callMethod({
            name: 'getBalance',
            data: {
                chain,
                address
            }
        });

        return JSON.parse(response).payload;
    }
}
