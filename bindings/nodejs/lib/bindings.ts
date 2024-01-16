// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { __UtilsMethods__ } from './types/utils';

// @ts-ignore: path is set to match runtime transpiled js path
import addon = require('../build/Release/index.node');
import { errorHandle } from '.';

const {
    callUtilsMethodRust,
    initLogger,
    createApi,
    destroyApi,
    callApiMethod,
} = addon;

const callUtilsMethod = (method: __UtilsMethods__): any => {
    try {
        const response = JSON.parse(
            callUtilsMethodRust(JSON.stringify(method)),
        );
        return response.payload;
    } catch (error: any) {
        throw errorHandle(error);
    }
};

export { initLogger, createApi, destroyApi, callApiMethod, callUtilsMethod };
