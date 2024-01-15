import { Assets } from "../assets"
import { Utils } from "../../utils";
import { ContractIdentity } from "./contract-identity"

// import type { U64 } from "@iota/sdk"
import { MetadataFeature } from "@iota/sdk"
import { Contract } from "../contracts";



export class RequestMetadata {
    
    readonly senderContract: ContractIdentity;
    readonly targetContract: number;
    readonly targetEntryPoint: number;
    readonly gasBudget: bigint;
    readonly params: Map<string, Uint8Array> = new Map();
    readonly allowance: Assets = new Assets();

    constructor(senderContract: ContractIdentity, targetContract: Contract, targetEntryPoint: string, gasBudget: bigint) {
        this.senderContract = senderContract;
        this.targetContract = Utils.hname(targetContract);
        this.targetEntryPoint = Utils.hname(targetEntryPoint);;
        this.gasBudget = gasBudget;
    }

    asFeature(): MetadataFeature {
        return new MetadataFeature("0x" + Utils.specialEncode(this));
    }
}