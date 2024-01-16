import { OutputId } from '@iota/sdk';
import { RequestMetadata } from '../../utils/request-metadata';

export interface __GetInfoMethod__ {
    name: 'getInfo';
}

export interface __GetBalanceMethod__ {
    name: 'getBalance';
    data: {
        chain: string;
        address: string;
    };
}

export interface __GetReceiptMethod__ {
    name: 'getReceipt';
    data: {
        chain: string;
        requestId: OutputId;
    };
}

export interface __PostEstimateGasOnLedgerMethod__ {
    name: 'estimateGasOnLedger';
    data: {
        chain: string;
        json: object;
    };
}

export interface __PostEstimateGasOffLedgerMethod__ {
    name: 'estimateGasOffLedger';
    data: {
        chain: string;
        metadata: RequestMetadata;
    };
}
