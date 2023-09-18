use starknet::core::types::{EmittedEvent, FieldElement};

use crate::{api::Token, to_u256};

fn get_formatted_text(emitted_event: EmittedEvent, token: Token) -> String {
    // TODO Update "from" and "to" to use starknet ID and reduce ID
    let from = emitted_event.data[0];
    let to = emitted_event.data[1];
    let amount = to_u256(
        emitted_event.data[2].try_into().unwrap(),
        emitted_event.data[3].try_into().unwrap(),
    );

    let first_line = format!(
        "{} #{} {} (usd value) USD",
        amount, token.symbol, token.logo
    );
    let second_line = if to == FieldElement::ZERO {
        format!("{} bridged to Ethereum L1", from)
    } else if from == FieldElement::ZERO {
        format!("{} bridged to Starknet L2", to)
    } else {
        format!("From ${} to ${}", from, to)
    };

    let third_line = format!("https://starkscan.co/tx/{}", emitted_event.transaction_hash);
    format!("{}\n{}\n{}", first_line, second_line, third_line)
}

#[cfg(test)]
mod tests {
    use starknet::core::types::{EmittedEvent, FieldElement};

    use crate::ETH;

    use super::get_formatted_text;
    #[test]
    fn test_get_formatted_text() {
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
        let response = get_formatted_text(emitted_event, ETH);
        println!("LAMA\n{response}");
        assert!(
            response
                == "1.000.000,00 #USDC $ (1,000,326 USD)
        0x6e1...b3ce bridged to Starknet L2
        https://starkscan.co/tx/0x732b09d901fb0075d283ac23cbaae4f8c486123a88a621eeaa05d0b5ddfb8d8",
            "Should be https://twitter.com/StarkWhaleAlert/status/1703701997629722850"
        );
    }
}
