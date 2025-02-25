pub struct Token {
    pub address: &'static str,
    pub decimals: u32,
    pub symbol: &'static str,
    pub selector: &'static str, // This should be the String of the selector (Transfer, ...), not the HEX value
    pub threshold: u128,
    pub logo: &'static str,
    pub rate_api_id: Option<&'static str>,
}

pub const TOKENS: &[Token] = &[
    Token {
        address: "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
        decimals: 18,
        symbol: "ETH",
        selector: "Transfer",
        threshold: 900,
        logo: "♦",
        rate_api_id: Some("ethereum"),
    },
    Token {
        address: "0x053c91253bc9682c04929ca02ed00b3e423f6710d2ee7e0d5ebb06f3ecf368a8",
        decimals: 6,
        symbol: "USDC",
        selector: "Transfer",
        threshold: 1_800_000,
        logo: "$",
        rate_api_id: Some("usd-coin"),
    },
    Token {
        address: "0x068f5c6a61780768455de69077e07e89787839bf8166decfbf92b645209c0fb8",
        decimals: 6,
        symbol: "USDT",
        selector: "Transfer",
        threshold: 1_000_000,
        logo: "$",
        rate_api_id: Some("tether"),
    },
    Token {
        address: "0x00da114221cb83fa859dbdb4c44beeaa0bb37c7537ad5ae66fe5e0efd20e6eb3",
        decimals: 18,
        symbol: "DAI",
        selector: "Transfer",
        threshold: 100_000,
        logo: "D",
        rate_api_id: Some("multi-collateral-dai"),
    },
    Token {
        address: "0x04718f5a0fc34cc1af16a1cdee98ffb20c31f5cd61d6ab07201858f4287c938d",
        decimals: 18,
        symbol: "STRK",
        selector: "Transfer",
        threshold: 4_000_000,
        logo: "",
        rate_api_id: Some("starknet-token"),
    },
    Token {
        address: "0x03fe2b97c1fd336e750087d68b9b867997fd64a2661ff3ca5a7c771641e8e7ac",
        decimals: 8,
        symbol: "wBTC",
        selector: "Transfer",
        threshold: 5,
        logo: "B",
        rate_api_id: Some("wrapped-bitcoin"),
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
    AddressToName {
        address: "13c67ed78bc280887234fe5ed5e77272465317978ae86c25a71531d9332a2d",
        name: "Binance",
    },
    AddressToName {
        address: "517daba3622259ae4fffab72bb716d89c30df0994c6ab25ede61bd139639724",
        name: "Strk Provisions: StarkEx Claim",
    },
    // These addresses are coming from ZachXBT investigations telegram channel
    AddressToName {
        address: "213c67ed78bc280887234fe5ed5e77272465317978ae86c25a71531d9332a2d",
        name: "Binance",
    },
    AddressToName {
        address: "269ea391a9c99cb6cee43ff589169f547cbc48d7554fdfbbfa7f97f516da700",
        name: "OKX",
    },
    AddressToName {
        address: "76601136372fcdbbd914eea797082f7504f828e122288ad45748b0c8b0c9696",
        name: "Bybit",
    },
    AddressToName {
        address: "620102ea610be8518125cf2de850d0c4f5d0c5d81f969cff666fb53b05042d2",
        name: "Kraken",
    },
    AddressToName {
        address: "566ec9d06c79b1ca32970519715a27f066e76fac8971bbd21b96a50db826d90",
        name: "Kucoin",
    },
    AddressToName {
        address: "3fd14213a96e9d90563ebe1b224f357c6481a755ee6f046c8ce9acd9b8654a7",
        name: "HTX",
    },
    AddressToName {
        address: "69a7818562b608ce8c5d0039e7f6d1c6ee55f36978f633b151858d85c022d2f",
        name: "MEXC",
    },
    AddressToName {
        address: "e91830f84747f37692127b20d4e4f9b96482b1007592fee1d7c0136ee60e6d",
        name: "Gate",
    },
    AddressToName {
        address: "299b9008e2d3fa88de6d06781fc9f32f601b2626cb0efa8e8c19f2b17837ed1",
        name: "Bitget",
    },
    AddressToName {
        address: "4b555a99b585adf082754e5ea36e4202f13efa649e6ac16dfe8c0e217c454bc",
        name: "HitBTC",
    },
    AddressToName {
        address: "fb108ed29e1b5d82bb61a39a15bbab410543818bf7df9be3c0f5dd0d612cf3",
        name: "CoinEX",
    },
    AddressToName {
        address: "62b6edccf9d86aff918634e53f3fac9545a8bcf84bcb59a0a09f9194d18282d",
        name: "ChangeNow",
    },
    AddressToName {
        address: "786c463590ca32345e0118a0303a8f66af10882d7315ce282840feb5d6817f9",
        name: "XT",
    },
    AddressToName {
        address: "1a103074e6ea2f988b427c77e671207c20d6005d407a685eeee2e1f61028392",
        name: "Bitrue",
    },
    AddressToName {
        address: "4de639e634c071c3ce8b1c69fac0500aab5ddb25a08fd0f757176243e4c0467",
        name: "Bitmart",
    },
    // End of ZachXBT investigations telegram channel
];

pub struct AddressToName {
    pub address: &'static str,
    pub name: &'static str,
}
