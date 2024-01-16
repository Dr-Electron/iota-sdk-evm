import { INativeToken, NftId } from '@iota/sdk';

export class Assets {
    public baseTokens: bigint = BigInt(0);

    public nativeTokens: INativeToken[] = [];

    public nfts: NftId[] = [];

    constructor() {}
}
