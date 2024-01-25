// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

export * from './api';
export * from './address';
export * from './contracts';
export * from './assets';
export * from './utils';
export * from './logger-config';

/**
 * Response from the message interface
 */
export interface Response<T> {
    type: string;
    payload: T;
}
