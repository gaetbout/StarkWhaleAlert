use bigdecimal::num_traits;
use num_traits::cast::ToPrimitive;
use starknet::{
    core::{
        types::{BlockId, BlockTag, FieldElement, FunctionCall},
        utils::get_selector_from_name,
    },
    providers::{jsonrpc::HttpTransport, JsonRpcClient, Provider},
};

pub const STARKNET_ID_CONTRACT_ADDRESS: FieldElement = FieldElement::from_mont([
    9876522541644636344,
    16204762974907305178,
    9525933456780166611,
    327799339589885214,
]);

pub async fn address_to_domain(
    rpc_client: JsonRpcClient<HttpTransport>,
    address: FieldElement,
) -> Option<String> {
    let response = rpc_client
        .call(
            FunctionCall {
                contract_address: STARKNET_ID_CONTRACT_ADDRESS,
                entry_point_selector: get_selector_from_name("address_to_domain").unwrap(),
                calldata: vec![address],
            },
            BlockId::Tag(BlockTag::Latest),
        )
        .await
        .unwrap();
    if response.len() == 1 && response[0] == FieldElement::ZERO {
        return None;
    }
    let mut domain = String::new();
    response.iter().skip(1).for_each(|value| {
        domain.push_str(decode(*value).as_str());
        domain.push('.');
    });
    domain.push_str("stark");
    Some(domain)
}

const BASIC_ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz0123456789-";
const BIG_ALPHABET: &str = "这来";

fn decode(mut felt: FieldElement) -> String {
    let mut decoded: String = String::new();
    let basic_plus = FieldElement::from(BASIC_ALPHABET.chars().count() + 1);
    let basic_len = FieldElement::from(BASIC_ALPHABET.chars().count());
    let big_plus = FieldElement::from(BIG_ALPHABET.chars().count() + 1);
    let big_len = FieldElement::from(BIG_ALPHABET.chars().count());
    let last_big = BIG_ALPHABET.chars().last().unwrap();
    while felt != FieldElement::ZERO {
        let code = felt % basic_plus;
        felt = felt.floor_div(basic_plus);
        if code == basic_len {
            let next_felt = felt.floor_div(big_plus);
            if next_felt == FieldElement::ZERO {
                let code2 = felt % big_plus;
                felt = next_felt;
                decoded.push(if code2 == FieldElement::ZERO {
                    BASIC_ALPHABET.chars().next().unwrap()
                } else {
                    last_big
                });
            } else {
                decoded.push(
                    BIG_ALPHABET
                        .chars()
                        .nth((felt % big_len).to_big_decimal(0).to_usize().unwrap())
                        .unwrap(),
                );
                felt = felt.floor_div(big_len);
            }
        } else {
            decoded.push(
                BASIC_ALPHABET
                    .chars()
                    .nth(code.to_big_decimal(0).to_usize().unwrap())
                    .unwrap(),
            );
        }
        let (decoded_str, k) = extract_stars(decoded.as_str());
        let mut decoded = String::from(decoded_str);
        if k != 0 {
            let star = last_big.to_string();
            if k % 2 == 0 {
                decoded.push_str(&str::repeat(&star, k / 2 - 1));
                decoded.push(BIG_ALPHABET.chars().next().unwrap());
                let mut basic_iter = BASIC_ALPHABET.chars();
                basic_iter.next();
                decoded.push(basic_iter.next().unwrap());
            } else {
                decoded.push_str(&str::repeat(&star, k / 2 + 1));
            }
        }
    }
    decoded
}

fn extract_stars(mut domain: &str) -> (&str, usize) {
    let mut k = 0;
    let last_char = BIG_ALPHABET.chars().last().unwrap();
    while domain.ends_with(last_char) {
        let mut chars = domain.chars();
        chars.next_back();
        domain = chars.as_str();
        k += 1;
    }
    (domain, k)
}

#[cfg(test)]
mod tests {
    use super::address_to_domain;
    use crate::get_infura_client;
    use starknet::core::types::FieldElement;

    #[tokio::test]
    async fn test_starknet_id() {
        // stark
        address_to_domain(
            get_infura_client(),
            FieldElement::from_hex_be(
                "0x1f4055a52c859593e79988bfe998b536066805fe757522ece47945f46f6b6e7",
            )
            .unwrap(),
        )
        .await;

        address_to_domain(
            get_infura_client(),
            FieldElement::from_hex_be("0x225bd17f4b4ede26c77673d8d3").unwrap(),
        )
        .await;
    }
}
