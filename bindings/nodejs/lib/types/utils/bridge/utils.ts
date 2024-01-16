import { RequestMetadata } from '../..';
import { EvmAddress } from '../../address';

export interface __EthereumAgentIdMethod__ {
    name: 'ethereumAgentId';
    data: {
        chain: string;
        address: EvmAddress;
    };
}

export interface __SpecialEncodeMethod__ {
    name: 'specialEncode';
    data: {
        metadata: RequestMetadata;
    };
}

export interface __HnameMethod__ {
    name: 'hname';
    data: {
        name: string;
    };
}
