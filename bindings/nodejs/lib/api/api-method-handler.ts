// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { errorHandle } from '..';
import { callApiMethod, createApi, destroyApi } from '../bindings';
import type { __ApiMethods__ } from '../types/api';

/**
 * The MethodHandler which sends the commands to the Rust side.
 */
export class ApiMethodHandler {
    methodHandler: any;

    /**
     * @param methodHandler The Rust method handler created in `ApiMethodHandler.create()`.
     */
    constructor(methodHandler: any) {
        this.methodHandler = methodHandler;
    }

    /**
     * @param options The client options.
     */
    static async create(url: String): Promise<ApiMethodHandler> {
        try {
            const methodHandler = await createApi(url);
            return new ApiMethodHandler(methodHandler);
        } catch (error: any) {
            throw errorHandle(error);
        }
    }

    async destroy(): Promise<void> {
        try {
            destroyApi(this.methodHandler);
        } catch (error: any) {
            throw errorHandle(error);
        }
    }

    /**
     * Call an api method.
     *
     * @param method The api method.
     * @returns A promise that resolves to a JSON string response holding the result of the api method.
     */
    async callMethod(method: __ApiMethods__): Promise<string> {
        return callApiMethod(this.methodHandler, JSON.stringify(method)).catch(
            (error: any) => {
                throw errorHandle(error);
            },
        );
    }
}
