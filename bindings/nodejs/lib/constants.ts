// Every ISC chain is initialized with an instance of the Magic contract at this address
export const ISC_MAGIC_ADDRESS: string =
    '0x1074000000000000000000000000000000000000';

// The ERC20 contract for base tokens is at this address:
export const ISC_ERC20BASETOKENS_ADDRESS: string =
    '0x1074010000000000000000000000000000000000';

// The ERC721 contract for NFTs is at this address:
export const ISC_ERC721_ADDRESS: string =
    '0x1074030000000000000000000000000000000000';

// The base chain address from the testnet to which metadata tx should be sent
export const TESTNET_CHAIN_ADDRESS: string =
    'rms1ppp00k5mmd2m8my8ukkp58nd3rskw6rx8l09aj35984k74uuc5u2cywn3ex';

export const MIN_GAS_FEE: bigint = BigInt(100); // 0.0001 smr
