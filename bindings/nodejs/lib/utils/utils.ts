// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { callUtilsMethod } from '../bindings';
import { RequestMetadata } from '../types';
import { EvmAddress } from '../types/address';

/** Utils class for utils. */
export class Utils {
    static specialEncode(metadata: RequestMetadata): string {
        return callUtilsMethod({
            name: 'specialEncode',
            data: {
                metadata,
            },
        });
    }

    static hname(name: string): number {
        return callUtilsMethod({
            name: 'hname',
            data: {
                name,
            },
        });
    }

    /**
     * Generate a new mnemonic.
     */
    static ethereumAgentId(chain: string, address: EvmAddress): Uint8Array {
        return callUtilsMethod({
            name: 'ethereumAgentId',
            data: {
                chain,
                address,
            },
        });
    }
}
