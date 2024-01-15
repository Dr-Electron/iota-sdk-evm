
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