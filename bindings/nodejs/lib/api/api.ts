// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Assets, ReceiptResponse, RequestMetadata, WaspInfo } from '../types';
import { ApiMethodHandler } from './api-method-handler';

import { Bech32Address, OutputId } from '@iota/sdk';

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
                address,
            },
        });

        return JSON.parse(response).payload;
    }

    async estimateGasOnLedger(
        chain: string,
        json: object,
    ): Promise<ReceiptResponse> {
        const response = await this.methodHandler.callMethod({
            name: 'estimateGasOnLedger',
            data: {
                chain,
                json,
            },
        });

        return JSON.parse(response).payload;
    }

    async estimateGasOffLedger(
        chain: string,
        metadata: RequestMetadata,
    ): Promise<ReceiptResponse> {
        const response = await this.methodHandler.callMethod({
            name: 'estimateGasOffLedger',
            data: {
                chain,
                metadata,
            },
        });

        return JSON.parse(response).payload;
    }

    async getReceipt(
        chain: string,
        requestId: OutputId,
    ): Promise<ReceiptResponse> {
        const response = await this.methodHandler.callMethod({
            name: 'getReceipt',
            data: {
                chain,
                requestId,
            },
        });

        return JSON.parse(response).payload;
    }
}
