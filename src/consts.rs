pub struct Token {
    pub address: &'static str,
    pub decimals: u8,
    pub symbol: &'static str,
    pub selector: &'static str, // This should be the String of the selector (Transfer, ...), not the HEX value
    pub threshold: u128,
    pub logo: &'static str,
    pub rate_api_id: &'static str,
}

pub const TOKENS: &[Token] = &[ETH, USDC, USDT];
pub const ADDRESS_LIST: &[AddressToName] = &[
    LAYER_SWAP,
    ZKLEND_MARKET,
    BRIQ_FACTORY,
    STARKNET_DEPLOYER,
    MY_SWAP_AMM,
    JEDI_SWAP_ETH_USDC,
    TENK_SWAP_ETH_USD,
    ORBITER_FINANCE_BRIDGE_1,
    ORBITER_FINANCE_BRIDGE_2,
    ORBITER_FINANCE_BRIDGE_4,
];

pub struct AddressToName {
    pub address: &'static str,
    pub name: &'static str,
}

const ETH: Token = Token {
    address: "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
    decimals: 18,
    symbol: "ETH",
    selector: "Transfer",
    threshold: 70,
    logo: "♦",
    rate_api_id: "ethereum",
};

pub const USDC: Token = Token {
    address: "0x053c91253bc9682c04929ca02ed00b3e423f6710d2ee7e0d5ebb06f3ecf368a8",
    decimals: 6,
    symbol: "USDC",
    selector: "Transfer",
    threshold: 100_000,
    logo: "$",
    rate_api_id: "usd-coin",
};

const USDT: Token = Token {
    address: "0x068f5c6a61780768455de69077e07e89787839bf8166decfbf92b645209c0fb8",
    decimals: 6,
    symbol: "USDT",
    selector: "Transfer",
    threshold: 100_000,
    logo: "$",
    rate_api_id: "tether",
};

pub const LAYER_SWAP: AddressToName = AddressToName {
    address: "19252b1deef483477c4d30cfcc3e5ed9c82fafea44669c182a45a01b4fdb97a",
    name: "Layerswap",
};

const ZKLEND_MARKET: AddressToName = AddressToName {
    address: "4c0a5193d58f74fbace4b74dcf65481e734ed1714121bdc571da345540efa05",
    name: "zkLend: Market",
};

const BRIQ_FACTORY: AddressToName = AddressToName {
    address: "5b021b6743c4f420e20786baa7fb9add1d711302c267afbc171252a74687376",
    name: "The Fucking Briq",
};

const STARKNET_DEPLOYER: AddressToName = AddressToName {
    address: "1176a1bd84444c89232ec27754698e5d2e7e1a7f1539f12027f28b23ec9f3d8",
    name: "Starknet deployer",
};

const MY_SWAP_AMM: AddressToName = AddressToName {
    address: "10884171baf1914edc28d7afb619b40a4051cfae78a094a55d230f19e944a28",
    name: "mySwap: AMM Swap",
};

const JEDI_SWAP_ETH_USDC: AddressToName = AddressToName {
    address: "4d0390b777b424e43839cd1e744799f3de6c176c7e32c1812a41dbd9c19db6a",
    name: "JediSwap: ETH/USDC Pair",
};

const TENK_SWAP_ETH_USD: AddressToName = AddressToName {
    address: "23c72abdf49dffc85ae3ede714f2168ad384cc67d08524732acea90df325",
    name: "10KSwap: ETH-USDC Pair",
};

const ORBITER_FINANCE_BRIDGE_1: AddressToName = AddressToName {
    address: "7b393627bd514d2aa4c83e9f0c468939df15ea3c29980cd8e7be3ec847795f0",
    name: "Orbiter Finance Bridge 1",
};

const ORBITER_FINANCE_BRIDGE_4: AddressToName = AddressToName {
    address: "6e18dd81378fd5240704204bccc546f6dfad3d08c4a3a44347bd274659ff328",
    name: "Orbiter Finance Bridge 4",
};

const ORBITER_FINANCE_BRIDGE_2: AddressToName = AddressToName {
    address: "64a24243f2aabae8d2148fa878276e6e6e452e3941b417f3c33b1649ea83e11",
    name: "Orbiter Finance Bridge 2",
};
