use bigdecimal::ToPrimitive;
use num_bigint::BigUint;
use num_format::{Locale, ToFormattedString};
use starknet::core::types::{EmittedEvent, Felt};
use std::ops::Div;

use crate::{api, consts::Token, consts::ADDRESS_LIST, get_infura_client, starknet_id, to_u256};

#[derive(Debug, Clone)]
pub struct TransferEvent {
    pub from: Felt,
    pub to: Felt,
    pub amount: BigUint,
}

impl Into<TransferEvent> for EmittedEvent {
    fn into(self) -> TransferEvent {
        let (from, to, amount_idx) = if self.keys.len() > 1 {
            (self.keys[1], self.keys[2], 0)
        } else {
            (self.data[0], self.data[1], 2)
        };
        let amount = to_u256(
            self.data[amount_idx].try_into().expect("Error: low"),
            self.data[amount_idx + 1].try_into().expect("Error: high"),
        );
        TransferEvent { from, to, amount }
    }
}

pub async fn get_formatted_text_for_transfer_events(
    emitted_events: &Vec<TransferEvent>,
    transaction_hash: Felt,
    token: &Token,
) -> String {
    let rate = get_rate(token).await;

    let mut text = format!("Multicall of #{}: \n", token.symbol);
    for event in emitted_events {
        let amount = to_rounded(event.amount.clone(), token.decimals);
        let amount_string = amount.to_u128().unwrap().to_formatted_string(&Locale::en);
        let usd_value = get_usd_value(amount, &rate);
        text += &format!(
            "- {}{} ({} $): {} -> {}\n",
            amount_string,
            token.logo,
            usd_value,
            format_address(event.from).await,
            format_address(event.to).await
        );
    }
    text += &format!("https://voyager.online/tx/{}", transaction_hash.to_hex());
    text
}

pub async fn get_formatted_text(
    transfer_event: TransferEvent,
    transaction_hash: Felt,
    token: &Token,
) -> String {
    let amount = to_rounded(transfer_event.amount, token.decimals);
    let amount_string = amount.to_u128().unwrap().to_formatted_string(&Locale::en);
    let rate = get_rate(token).await;
    let usd_value = get_usd_value(amount, &rate);

    let first_line = format!(
        "{:} #{} {} ({} USD)",
        amount_string, token.symbol, token.logo, usd_value
    );
    let second_line = if transfer_event.to == Felt::ZERO {
        format!(
            "{} bridged to Ethereum",
            format_address(transfer_event.from).await
        )
    } else if transfer_event.from == Felt::ZERO {
        format!(
            "{} bridged to Starknet",
            format_address(transfer_event.to).await
        )
    } else {
        format!(
            "From {} to {}",
            format_address(transfer_event.from).await,
            format_address(transfer_event.to).await
        )
    };

    let third_line = format!("https://voyager.online/tx/{}", transaction_hash.to_hex());
    format!("{}\n{}\n{}", first_line, second_line, third_line)
}

async fn get_rate(token: &Token) -> Option<BigUint> {
    match token.rate_api_id {
        Some(coin_id) => {
            let rate = api::fetch_coin(coin_id).await.unwrap();
            Some(BigUint::new(vec![(rate * 10000_f64).to_u32().unwrap()]))
        }
        None => None,
    }
}

fn get_usd_value(amount: BigUint, rate: &Option<BigUint>) -> String {
    if let Some(rate) = rate {
        let usd_value = (amount * rate).div(BigUint::new(vec![10000]));

        usd_value
            .to_u128()
            .unwrap()
            .to_formatted_string(&Locale::en)
    } else {
        "???".to_owned()
    }
}

fn to_rounded(amount: BigUint, pow: u32) -> BigUint {
    let power = 10_u128.pow(pow);
    let half_pow = power / 2;
    (amount + half_pow) / power
}

trait ToHex {
    fn to_hex(self) -> String;
}
impl ToHex for Felt {
    fn to_hex(self) -> String {
        format!("{:#x}", self)
    }
}

async fn format_address(address: Felt) -> String {
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

    use super::{format_address, get_formatted_text, to_rounded};
    use crate::{consts::TOKENS, to_u256};
    use starknet::core::types::{EmittedEvent, Felt};

    #[tokio::test]
    async fn test_get_formatted_text_bridge_to_starknet() {
        let keys = vec![Felt::from_hex(
            "0x99cd8bde557814842a3121e8ddfd433a539b8c9f14bf31ebf108d12e6196e9",
        )
        .unwrap()];
        let data = vec![
            Felt::from_hex("0x0").unwrap(),
            Felt::from_hex("0x6e14b28449c412a336e7a5a3473da083b9159e6845be4d02ee50f6095a5b3ce")
                .unwrap(),
            Felt::from_hex("0xe8d4a51000").unwrap(),
            Felt::from_hex("0x0").unwrap(),
        ];
        let transaction_hash =
            Felt::from_hex("0x732b09d901fb0075d283ac23cbaae4f8c486123a88a621eeaa05d0b5ddfb8d8")
                .unwrap();
        let emitted_event = EmittedEvent {
            from_address: Felt::from_hex(
                "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
            )
            .unwrap(),
            keys,
            data,
            block_hash: Some(
                Felt::from_hex(
                    "0x030905d20477c31ecc0951a8c7d2f8c91d16a2ce864aaad2730aa330e328dc6a",
                )
                .unwrap(),
            ),
            block_number: Some(237165),
            transaction_hash,
        };
        let response = get_formatted_text(emitted_event.into(), transaction_hash, &TOKENS[1]).await;
        println!("{response}");
        assert!(
            response
                == "1,000,000 #USDC $ (1,000,000 USD)\n0x6e1...b3ce bridged to Starknet L2\nhttps://starkscan.co/tx/0x732b09d901fb0075d283ac23cbaae4f8c486123a88a621eeaa05d0b5ddfb8d8",
            "Should be https://twitter.com/StarkWhaleAlert/status/1703701997629722850"
        );
    }

    #[tokio::test]
    async fn test_get_formatted_text_bridge_to_l1() {
        let keys = vec![Felt::from_hex(
            "0x99cd8bde557814842a3121e8ddfd433a539b8c9f14bf31ebf108d12e6196e9",
        )
        .unwrap()];
        let data = vec![
            Felt::from_hex("0x6e14b28449c412a336e7a5a3473da083b9159e6845be4d02ee50f6095a5b3ce")
                .unwrap(),
            Felt::from_hex("0x0").unwrap(),
            Felt::from_hex("0xe8d4a51000").unwrap(),
            Felt::from_hex("0x0").unwrap(),
        ];
        let transaction_hash =
            Felt::from_hex("0x732b09d901fb0075d283ac23cbaae4f8c486123a88a621eeaa05d0b5ddfb8d8")
                .unwrap();
        let emitted_event = EmittedEvent {
            from_address: Felt::from_hex(
                "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
            )
            .unwrap(),
            keys,
            data,
            block_hash: Some(
                Felt::from_hex(
                    "0x030905d20477c31ecc0951a8c7d2f8c91d16a2ce864aaad2730aa330e328dc6a",
                )
                .unwrap(),
            ),
            block_number: Some(237165),
            transaction_hash,
        };
        let response = get_formatted_text(emitted_event.into(), transaction_hash, &TOKENS[1]).await;
        assert!(
            response
                == "1,000,000 #USDC $ (1,000,000 USD)\n0x6e1...b3ce bridged to Ethereum L1\nhttps://starkscan.co/tx/0x732b09d901fb0075d283ac23cbaae4f8c486123a88a621eeaa05d0b5ddfb8d8",
            "Should be correct"
        );
    }

    #[tokio::test]
    async fn test_get_formatted_text_to_starknet_id() {
        let keys = vec![Felt::from_hex(
            "0x99cd8bde557814842a3121e8ddfd433a539b8c9f14bf31ebf108d12e6196e9",
        )
        .unwrap()];
        let data = vec![
            Felt::from_hex("0x6e14b249c412a336e7a5a3473da083b9159e6845be4d02ee50f6095a5b3ce")
                .unwrap(),
            Felt::from_hex("0x6e14b249c412a336e7a5a3473da083b9159e6845be4d02ee50f6095a5b3ce")
                .unwrap(),
            Felt::from_hex("0xe8d4a51000").unwrap(),
            Felt::from_hex("0x0").unwrap(),
        ];
        let transaction_hash =
            Felt::from_hex("0x732b09d901fb0075d283ac23cbaae4f8c486123a88a621eeaa05d0b5ddfb8d8")
                .unwrap();
        let emitted_event = EmittedEvent {
            from_address: Felt::from_hex(
                "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
            )
            .unwrap(),
            keys,
            data,
            block_hash: Some(
                Felt::from_hex(
                    "0x030905d20477c31ecc0951a8c7d2f8c91d16a2ce864aaad2730aa330e328dc6a",
                )
                .unwrap(),
            ),
            block_number: Some(237165),
            transaction_hash,
        };
        let response = get_formatted_text(emitted_event.into(), transaction_hash, &TOKENS[1]).await;
        assert!(
            response
                == "1,000,000 #USDC $ (1,000,000 USD)\nFrom 0x6e1...b3ce to 0x6e1...b3ce\nhttps://starkscan.co/tx/0x732b09d901fb0075d283ac23cbaae4f8c486123a88a621eeaa05d0b5ddfb8d8",
            "Should be correct"
        );
    }

    #[tokio::test]
    async fn test_format_address() {
        let response = format_address(
            Felt::from_hex("0x6e14b249c412a336e7a5a3473da083b9159e6845be4d02ee50f6095a5b3ce")
                .unwrap(),
        )
        .await;
        assert!(response == "0x6e1...b3ce", "Should be 0x6e1...b3ce");
    }

    #[tokio::test]
    async fn test_format_address_layer_swap() {
        let response = format_address(
            Felt::from_hex("0x019252b1deef483477c4d30cfcc3e5ed9c82fafea44669c182a45a01b4fdb97a")
                .unwrap(),
        )
        .await;
        assert!(response == "Layerswap", "Should be Layerswap");
    }

    #[tokio::test]
    async fn test_format_address_starknet_id() {
        let response = format_address(
            Felt::from_hex("0x1f4055a52c859593e79988bfe998b536066805fe757522ece47945f46f6b6e7")
                .unwrap(),
        )
        .await;
        assert!(response == "stark.stark", "Should be stark.stark");
    }

    #[test]
    fn test_rounding() {
        let bigint_close_to_84 = to_u256(83997000000000000000, 0);
        let eight_four = to_u256(84, 0);
        let rounded_84 = to_rounded(bigint_close_to_84, 18);
        assert!(rounded_84 == eight_four, "Should be 84");

        let bigint_close_to_83 = to_u256(83497000000000000000, 0);
        let eight_tree = to_u256(83, 0);
        let rounded_83 = to_rounded(bigint_close_to_83, 18);
        assert!(rounded_83 == eight_tree, "Should be 83");
    }
}
