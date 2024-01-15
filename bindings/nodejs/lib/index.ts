// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// Needed for class-transformer json deserialisation
import 'reflect-metadata';

export * from './api';
export * from './utils';
export * from './types';
export * from './logger';
export * as Constants from './constants';

// For future reference to see what we return from rust as a serialized string
export type Result = {
    // "error" | "panic", or other binding "Response" enum name, we consider "ok".
    type: string;
    // "panic" means payload is just a string, otherwise its the object below.
    payload: {
        // Ok: All method names from types/bridge/__name__.name
        // Not ok: all variants of iota_sdk_evm_bindings_core::Error type i.e block/client/wallet/
        type: string;
        // If "ok", json payload
        payload?: string;
        // If not "ok", json error
        error?: string;
    };
};

function errorHandle(error: any): Error {
    try {
        const err: Result = JSON.parse(error.message);
        if (!err.type) {
            return error;
        }

        if (err.type == 'panic') {
            // Panic example:
            // {"type":"panic","payload":"Client was destroyed"}
            return Error(err.payload.toString());
        } else if (err.type == 'error') {
            // Error example:
            // {"type":"error","payload":{"type":"client","error":"no healthy node available"}}
            // TODO: switch on type and create proper js errors https://github.com/iotaledger/iota-sdk/issues/1417
            return Error(err.payload.error);
        } else {
            return Error(
                'in ErrorHandle without a valid error object. Only call this in catch statements.',
            );
        }
    } catch (err: any) {
        // json error, SyntaxError, we must have send a non-json error
        return error;
    }
}

export { errorHandle };
