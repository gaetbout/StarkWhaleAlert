pub struct Token {
    pub address: &'static str,
    pub decimals: u32,
    pub symbol: &'static str,
    pub selector: &'static str, // This should be the String of the selector (Transfer, ...), not the HEX value
    pub threshold: u128,
    pub logo: &'static str,
    pub rate_api_id: &'static str,
}

pub const TOKENS: &[Token] = &[
    Token {
        address: "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
        decimals: 18,
        symbol: "ETH",
        selector: "Transfer",
        threshold: 70,
        logo: "♦",
        rate_api_id: "ethereum",
    },
    Token {
        address: "0x053c91253bc9682c04929ca02ed00b3e423f6710d2ee7e0d5ebb06f3ecf368a8",
        decimals: 6,
        symbol: "USDC",
        selector: "Transfer",
        threshold: 100_000,
        logo: "$",
        rate_api_id: "usd-coin",
    },
    Token {
        address: "0x068f5c6a61780768455de69077e07e89787839bf8166decfbf92b645209c0fb8",
        decimals: 6,
        symbol: "USDT",
        selector: "Transfer",
        threshold: 100_000,
        logo: "$",
        rate_api_id: "tether",
    },
    Token {
        address: "0x00da114221cb83fa859dbdb4c44beeaa0bb37c7537ad5ae66fe5e0efd20e6eb3",
        decimals: 18,
        symbol: "DAI",
        selector: "Transfer",
        threshold: 100_000,
        logo: "D",
        rate_api_id: "multi-collateral-dai",
    },
];

pub const ADDRESS_LIST: &[AddressToName] = &[
    AddressToName {
        address: "19252b1deef483477c4d30cfcc3e5ed9c82fafea44669c182a45a01b4fdb97a",
        name: "Layerswap",
    },
    AddressToName {
        address: "4c0a5193d58f74fbace4b74dcf65481e734ed1714121bdc571da345540efa05",
        name: "zkLend: Market",
    },
    AddressToName {
        address: "5b021b6743c4f420e20786baa7fb9add1d711302c267afbc171252a74687376",
        name: "The Fucking Briq",
    },
    AddressToName {
        address: "1176a1bd84444c89232ec27754698e5d2e7e1a7f1539f12027f28b23ec9f3d8",
        name: "Starknet deployer",
    },
    AddressToName {
        address: "10884171baf1914edc28d7afb619b40a4051cfae78a094a55d230f19e944a28",
        name: "mySwap: AMM Swap",
    },
    AddressToName {
        address: "4d0390b777b424e43839cd1e744799f3de6c176c7e32c1812a41dbd9c19db6a",
        name: "JediSwap: ETH/USDC Pair",
    },
    AddressToName {
        address: "23c72abdf49dffc85ae3ede714f2168ad384cc67d08524732acea90df325",
        name: "10KSwap: ETH-USDC Pair",
    },
    AddressToName {
        address: "7b393627bd514d2aa4c83e9f0c468939df15ea3c29980cd8e7be3ec847795f0",
        name: "Orbiter Finance Bridge 1",
    },
    AddressToName {
        address: "6e18dd81378fd5240704204bccc546f6dfad3d08c4a3a44347bd274659ff328",
        name: "Orbiter Finance Bridge 4",
    },
    AddressToName {
        address: "64a24243f2aabae8d2148fa878276e6e6e452e3941b417f3c33b1649ea83e11",
        name: "Orbiter Finance Bridge 2",
    },
    AddressToName {
        address: "2e0af29598b407c8716b17f6d2795eca1b471413fa03fb145a5e33722184067",
        name: "Ekubo: Positions",
    },
    AddressToName {
        address: "4270219d365d6b017231b52e92b3fb5d7c8378b05e9abc97724537a80e93b0f",
        name: "AVNU: Exchange",
    },
    AddressToName {
        address: "4b3802058cdd4fc4e352e866e2eef5abde7d62e78116ac68b419654cbebc021",
        name: "Ekubo: Positions",
    },
    AddressToName {
        address: "1114c7103e12c2b2ecbd3a2472ba9c48ddcbf702b1c242dd570057e26212111",
        name: "mySwap: CL AMM Swap",
    },
    AddressToName {
        address: "259fec57cd26d27385cd8948d3693bbf26bed68ad54d7bdd1fdb901774ff0e8",
        name: "rhino.fi: Bridge",
    },
    AddressToName {
        address: "59a943ca214c10234b9a3b61c558ac20c005127d183b86a99a8f3c60a08b4ff",
        name: "Nostra: Interest Rate Model",
    },
    AddressToName {
        address: "5dd3d2f4429af886cd1a3b08289dbcea99a294197e9eb43b0e0325b4b",
        name: "Ekubo: Core",
    },
];

pub struct AddressToName {
    pub address: &'static str,
    pub name: &'static str,
}
