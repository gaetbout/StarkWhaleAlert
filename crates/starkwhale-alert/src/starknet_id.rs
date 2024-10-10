// This code was extracted from https://github.com/Th0rgal/starknet-rs/tree/feat/starknet-id
use starknet::{
    core::types::Felt,
    providers::{jsonrpc::HttpTransport, JsonRpcClient},
};

pub async fn address_to_domain(
    _rpc_client: JsonRpcClient<HttpTransport>,
    _address: Felt,
) -> Option<String> {
    // rpc_client
    //     .address_to_domain(address, MAINNET_CONTRACT)
    //     .await
    Option::None
}

#[cfg(test)]
mod tests {
    use super::address_to_domain;
    use crate::get_infura_client;
    use starknet::core::types::Felt;

    #[tokio::test]
    async fn test_starknet_id() {
        // stark
        let name = address_to_domain(
            get_infura_client(),
            Felt::from_hex("0x1f4055a52c859593e79988bfe998b536066805fe757522ece47945f46f6b6e7")
                .unwrap(),
        )
        .await
        .unwrap();
        assert_eq!(name, "stark.stark");
        // address_to_domain(
        //     get_infura_client(),
        //     Felt::from_hex("0x225bd17f4b4ede26c77673d8d3").unwrap(),
        // )
        // .await;
    }

    #[tokio::test]
    async fn test_starknet_id_fail() {
        // stark
        let should_be_none = address_to_domain(
            get_infura_client(),
            Felt::from_hex("0x225bd17f4b4ede26c77673d8d3").unwrap(),
        )
        .await;
        assert!(should_be_none.is_none());
    }
}
