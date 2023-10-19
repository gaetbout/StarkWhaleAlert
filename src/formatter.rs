use bigdecimal::ToPrimitive;
use num_bigint::BigUint;
use starknet::core::types::{EmittedEvent, FieldElement};

use crate::{
    api::{fetch_coin, Token},
    to_u256,
};

pub async fn get_formatted_text(emitted_event: EmittedEvent, token: &Token) -> String {
    // TODO Update "from" and "to" to use starknet ID and reduce ID
    // or resolve through the array
    let from = emitted_event.data[0];
    let to = emitted_event.data[1];
    let amount = to_u256(
        emitted_event.data[2].try_into().unwrap(),
        emitted_event.data[3].try_into().unwrap(),
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
    let rate = fetch_coin(token.rate_api_id).await.unwrap();
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
        let from_hex = format!("{:#x}", from);
        format!(
            "{}...{} bridged to Ethereum L1",
            &from_hex[0..5],
            &from_hex[from_hex.len() - 4..]
        )
    } else if from == FieldElement::ZERO {
        let to_hex = format!("{:#x}", to);
        format!(
            "{}...{} bridged to Starknet L2",
            &to_hex[0..5],
            &to_hex[to_hex.len() - 4..]
        )
    } else {
        let from_hex = format!("{:#x}", from);
        let to_hex = format!("{:#x}", to);

        format!(
            "From {}...{} to {}...{}",
            &from_hex[0..5],
            &from_hex[from_hex.len() - 4..],
            &to_hex[0..5],
            &to_hex[from_hex.len() - 4..]
        )
    };

    let third_line = format!(
        "https://starkscan.co/tx/{}",
        format!("{:#x}", emitted_event.transaction_hash)
    );
    format!("{}\n{}\n{}", first_line, second_line, third_line)
}

#[cfg(test)]
mod tests {
    use starknet::core::types::{EmittedEvent, FieldElement};

    use crate::consts::USDC;

    use super::get_formatted_text;

    #[tokio::test]
    async fn test_get_formatted_text() {
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
}
