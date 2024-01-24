import { OutputId } from "@iota/sdk";
import { Assets } from "../assets";

export interface RentStructure {
    vByteFactorData: number;
    vByteCost: number;
    vByteFactorKey: number;
}

export interface Protocol {
    rentStructure: RentStructure;
    minPowScore: number;
    tokenSupply: string;
    networkName: string;
    belowMaxDepth: number;
    version: number;
    bech32Hrp: string;
}

export interface BaseToken {
    unit: string;
    decimals: number;
    name: string;
    tickerSymbol: string;
    subunit: string;
    useMetricPrefix: boolean;
}

export interface L1Params {
    protocol: Protocol;
    maxPayloadSize: number;
    baseToken: BaseToken;
}

export interface WaspInfo {
    peeringUrl: string;
    l1Params: L1Params;
    publicKey: number; // Assuming f64 maps to number in TypeScript
    version: string;
}

export interface GasBurned {
    code: number;
    gasBurned: number;
}

export interface NodeError {
    code: string;
    params: string[];
}

export interface Target {
    contractHName: string;
    functionHName: string;
}

export interface Request {
    allowance: Assets;
    callTarget: Target;
    fungibleTokens: Assets;
    gasBudget: string;
    isEVM: boolean;
    isOffLedger: boolean;
    nft: string | null;
    params: Record<string, number[]>;
    requestId: OutputId;
    senderAccount: string;
    targetAddress: string;
}

export interface ReceiptResponse {
    request: Request;
    rawError: NodeError;
    errorMessage: string;
    gasBudget: string;
    gasBurned: string;
    gasFeeCharged: string;
    storageDepositCharged: string;
    blockIndex: number;
    requestIndex: number;
    gasBurnLog: GasBurned[];
}
