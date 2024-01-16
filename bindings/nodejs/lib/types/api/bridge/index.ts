import type {
    __GetInfoMethod__,
    __GetBalanceMethod__,
    __GetReceiptMethod__,
    __PostEstimateGasOffLedgerMethod__,
    __PostEstimateGasOnLedgerMethod__,
} from './api';

export type __ApiMethods__ =
    | __GetInfoMethod__
    | __GetBalanceMethod__
    | __GetReceiptMethod__
    | __PostEstimateGasOffLedgerMethod__
    | __PostEstimateGasOnLedgerMethod__;
