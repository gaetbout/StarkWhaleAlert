use bigdecimal::ToPrimitive;
use num_bigint::BigUint;
use starknet::core::types::{EmittedEvent, FieldElement};

use crate::{api, api::Token, consts::ADDRESS_LIST, get_infura_client, starknet_id, to_u256};

pub async fn get_formatted_text(emitted_event: EmittedEvent, token: &Token) -> String {
    let from = emitted_event.data[0];
    let to = emitted_event.data[1];
    let amount = to_u256(
        emitted_event.data[2].try_into().expect("Error: low"),
        emitted_event.data[3].try_into().expect("Error: high"),
    );
    let amount = amount / 10_u128.pow(token.decimals.into());
    let amount_string = amount
        .to_string()
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
        .join(".");
    let rate = api::fetch_coin(token.rate_api_id).await.unwrap();
    let rate = BigUint::new(vec![rate.to_u32().unwrap()]);
    let usd_value = amount * rate;
    let usd_value_string = usd_value
        .to_string()
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
        .join(".");

    let first_line = format!(
        "{:} #{} {} ({} USD)",
        amount_string, token.symbol, token.logo, usd_value_string
    );
    let second_line = if to == FieldElement::ZERO {
        format!("{} bridged to Ethereum L1", format_address(from).await)
    } else if from == FieldElement::ZERO {
        format!("{} bridged to Starknet L2", format_address(to).await)
    } else {
        format!(
            "From {} to {}",
            format_address(from).await,
            format_address(to).await
        )
    };

    let third_line = format!(
        "https://starkscan.co/tx/{}",
        emitted_event.transaction_hash.to_hex()
    );
    format!("{}\n{}\n{}", first_line, second_line, third_line)
}

trait ToHex {
    fn to_hex(self) -> String;
}
impl ToHex for FieldElement {
    fn to_hex(self) -> String {
        format!("{:#x}", self)
    }
}

async fn format_address(address: FieldElement) -> String {
    let address_as_hex = address.to_hex();
    let named_address = ADDRESS_LIST
        .iter()
        .find(|item| address_as_hex.ends_with(item.address));
    if let Some(address_to_name) = named_address {
        return address_to_name.name.to_string();
    };
    let starknet_id = starknet_id::address_to_domain(get_infura_client(), address).await;
    match starknet_id {
        Some(name) => name,
        None => {
            format!(
                "{}...{}",
                &address_as_hex[0..5],
                &address_as_hex[address_as_hex.len() - 4..],
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{format_address, get_formatted_text};
    use crate::consts::USDC;
    use starknet::core::types::{EmittedEvent, FieldElement};

    #[tokio::test]
    async fn test_get_formatted_text_bridge_to_starknet() {
        let keys = vec![FieldElement::from_hex_be(
            "0x99cd8bde557814842a3121e8ddfd433a539b8c9f14bf31ebf108d12e6196e9",
        )
        .unwrap()];
        let data = vec![
            FieldElement::from_hex_be("0x0").unwrap(),
            FieldElement::from_hex_be(
                "0x6e14b28449c412a336e7a5a3473da083b9159e6845be4d02ee50f6095a5b3ce",
            )
            .unwrap(),
            FieldElement::from_hex_be("0xe8d4a51000").unwrap(),
            FieldElement::from_hex_be("0x0").unwrap(),
        ];
        let emitted_event = EmittedEvent {
            from_address: FieldElement::from_hex_be(
                "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
            )
            .unwrap(),
            keys,
            data,
            block_hash: FieldElement::from_hex_be(
                "0x030905d20477c31ecc0951a8c7d2f8c91d16a2ce864aaad2730aa330e328dc6a",
            )
            .unwrap(),
            block_number: 237165,
            transaction_hash: FieldElement::from_hex_be(
                "0x732b09d901fb0075d283ac23cbaae4f8c486123a88a621eeaa05d0b5ddfb8d8",
            )
            .unwrap(),
        };
        let response = get_formatted_text(emitted_event, &USDC).await;
        assert!(
            response
                == "1.000.000 #USDC $ (1.000.000 USD)\n0x6e1...b3ce bridged to Starknet L2\nhttps://starkscan.co/tx/0x732b09d901fb0075d283ac23cbaae4f8c486123a88a621eeaa05d0b5ddfb8d8",
            "Should be https://twitter.com/StarkWhaleAlert/status/1703701997629722850"
        );
    }

    #[tokio::test]
    async fn test_get_formatted_text_bridge_to_l1() {
        let keys = vec![FieldElement::from_hex_be(
            "0x99cd8bde557814842a3121e8ddfd433a539b8c9f14bf31ebf108d12e6196e9",
        )
        .unwrap()];
        let data = vec![
            FieldElement::from_hex_be(
                "0x6e14b28449c412a336e7a5a3473da083b9159e6845be4d02ee50f6095a5b3ce",
            )
            .unwrap(),
            FieldElement::from_hex_be("0x0").unwrap(),
            FieldElement::from_hex_be("0xe8d4a51000").unwrap(),
            FieldElement::from_hex_be("0x0").unwrap(),
        ];
        let emitted_event = EmittedEvent {
            from_address: FieldElement::from_hex_be(
                "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
            )
            .unwrap(),
            keys,
            data,
            block_hash: FieldElement::from_hex_be(
                "0x030905d20477c31ecc0951a8c7d2f8c91d16a2ce864aaad2730aa330e328dc6a",
            )
            .unwrap(),
            block_number: 237165,
            transaction_hash: FieldElement::from_hex_be(
                "0x732b09d901fb0075d283ac23cbaae4f8c486123a88a621eeaa05d0b5ddfb8d8",
            )
            .unwrap(),
        };
        let response = get_formatted_text(emitted_event, &USDC).await;
        assert!(
            response
                == "1.000.000 #USDC $ (1.000.000 USD)\n0x6e1...b3ce bridged to Ethereum L1\nhttps://starkscan.co/tx/0x732b09d901fb0075d283ac23cbaae4f8c486123a88a621eeaa05d0b5ddfb8d8",
            "Should be correct"
        );
    }

    #[tokio::test]
    async fn test_get_formatted_text_to_starknet_id() {
        let keys = vec![FieldElement::from_hex_be(
            "0x99cd8bde557814842a3121e8ddfd433a539b8c9f14bf31ebf108d12e6196e9",
        )
        .unwrap()];
        let data = vec![
            FieldElement::from_hex_be(
                "0x6e14b249c412a336e7a5a3473da083b9159e6845be4d02ee50f6095a5b3ce",
            )
            .unwrap(),
            FieldElement::from_hex_be(
                "0x6e14b249c412a336e7a5a3473da083b9159e6845be4d02ee50f6095a5b3ce",
            )
            .unwrap(),
            FieldElement::from_hex_be("0xe8d4a51000").unwrap(),
            FieldElement::from_hex_be("0x0").unwrap(),
        ];
        let emitted_event = EmittedEvent {
            from_address: FieldElement::from_hex_be(
                "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
            )
            .unwrap(),
            keys,
            data,
            block_hash: FieldElement::from_hex_be(
                "0x030905d20477c31ecc0951a8c7d2f8c91d16a2ce864aaad2730aa330e328dc6a",
            )
            .unwrap(),
            block_number: 237165,
            transaction_hash: FieldElement::from_hex_be(
                "0x732b09d901fb0075d283ac23cbaae4f8c486123a88a621eeaa05d0b5ddfb8d8",
            )
            .unwrap(),
        };
        let response = get_formatted_text(emitted_event, &USDC).await;
        assert!(
            response
                == "1.000.000 #USDC $ (1.000.000 USD)\nFrom 0x6e1...b3ce to 0x6e1...b3ce\nhttps://starkscan.co/tx/0x732b09d901fb0075d283ac23cbaae4f8c486123a88a621eeaa05d0b5ddfb8d8",
            "Should be correct"
        );
    }

    #[tokio::test]
    async fn test_format_address() {
        let response = format_address(
            FieldElement::from_hex_be(
                "0x6e14b249c412a336e7a5a3473da083b9159e6845be4d02ee50f6095a5b3ce",
            )
            .unwrap(),
        )
        .await;
        assert!(response == "0x6e1...b3ce", "Should be 0x6e1...b3ce");
    }

    #[tokio::test]
    async fn test_format_address_layer_swap() {
        let response = format_address(
            FieldElement::from_hex_be(
                "0x019252b1deef483477c4d30cfcc3e5ed9c82fafea44669c182a45a01b4fdb97a",
            )
            .unwrap(),
        )
        .await;
        assert!(response == "Layerswap", "Should be Layerswap");
    }

    #[tokio::test]
    async fn test_format_address_starknet_id() {
        let response = format_address(
            FieldElement::from_hex_be(
                "0x1f4055a52c859593e79988bfe998b536066805fe757522ece47945f46f6b6e7",
            )
            .unwrap(),
        )
        .await;
        assert!(response == "stark.stark", "Should be stark.stark");
    }
}
