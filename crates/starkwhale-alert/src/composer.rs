use bigdecimal::ToPrimitive;
use num_bigint::BigUint;
use num_format::{Locale, ToFormattedString};
use starknet_rust::core::types::{EmittedEvent, Felt};
use std::ops::Div;

use crate::consts::Token;
use crate::to_u256;

const MAX_TWEET_LENGTH: usize = 280;

/// A decoded ERC-20 Transfer: who, to whom, how much. The on-chain `EmittedEvent`
/// turns into this via `into()`.
#[derive(Debug, Clone)]
pub struct TransferEvent {
    pub from: Felt,
    pub to: Felt,
    pub amount: BigUint,
}

impl From<EmittedEvent> for TransferEvent {
    fn from(event: EmittedEvent) -> Self {
        let (from, to, amount_idx) = if event.keys.len() > 1 {
            (event.keys[1], event.keys[2], 0)
        } else {
            (event.data[0], event.data[1], 2)
        };
        let amount = to_u256(
            event.data[amount_idx].try_into().expect("Error: low"),
            event.data[amount_idx + 1].try_into().expect("Error: high"),
        );
        TransferEvent { from, to, amount }
    }
}

/// A Transfer whose ends have been resolved to display names, while keeping the raw
/// addresses so the Composer can still detect Bridges (the zero address).
#[derive(Debug, Clone)]
pub struct ResolvedTransfer {
    pub from: Felt,
    pub to: Felt,
    pub from_name: String,
    pub to_name: String,
    pub amount: BigUint,
}

/// Everything one transaction needs to become tweets. Pure data — no I/O.
#[derive(Debug)]
pub struct Alert<'a> {
    pub token: &'a Token,
    pub tx: Felt,
    pub rate: Option<BigUint>,
    pub transfers: Vec<ResolvedTransfer>,
}

/// Turn an Alert into ready-to-post tweets. Pure and synchronous — this is the test surface.
///
/// Pool rebalances are dropped first, then the kept Transfers decide the shape:
/// none → nothing, one → a single tweet, many → a multicall tweet that degrades to
/// one tweet per Transfer when it would exceed the tweet length.
pub fn compose_tweets(alert: &Alert) -> Vec<String> {
    let kept: Vec<&ResolvedTransfer> = alert
        .transfers
        .iter()
        .filter(|t| !is_pool_rebalance(t))
        .collect();

    match kept.len() {
        0 => vec![],
        1 => vec![single_tweet(alert, kept[0])],
        _ => {
            let combined = multicall_tweet(alert, &kept);
            if combined.len() <= MAX_TWEET_LENGTH {
                vec![combined]
            } else {
                kept.iter().map(|t| single_tweet(alert, t)).collect()
            }
        }
    }
}

fn single_tweet(alert: &Alert, t: &ResolvedTransfer) -> String {
    let first_line = format!(
        "{} #{} {} ({} USD)",
        rounded_amount(t.amount.clone(), alert.token),
        alert.token.symbol,
        alert.token.logo,
        usd_value(t.amount.clone(), alert),
    );
    let second_line = if t.to == Felt::ZERO {
        format!("{} bridged to Ethereum", t.from_name)
    } else if t.from == Felt::ZERO {
        format!("{} bridged to Starknet", t.to_name)
    } else {
        format!("From {} to {}", t.from_name, t.to_name)
    };
    format!("{}\n{}\n{}", first_line, second_line, hex(alert.tx))
}

fn multicall_tweet(alert: &Alert, transfers: &[&ResolvedTransfer]) -> String {
    let mut text = format!("Multicall of #{}: \n", alert.token.symbol);
    for t in transfers {
        text += &format!(
            "- {}{} ({} $): {} -> {}\n",
            rounded_amount(t.amount.clone(), alert.token),
            alert.token.logo,
            usd_value(t.amount.clone(), alert),
            render_party(t.from, &t.from_name),
            render_party(t.to, &t.to_name),
        );
    }
    text += &hex(alert.tx);
    text
}

/// One end of a Transfer as it appears in a multicall line: the resolved name, or the
/// `Ethereum L1` Bridge endpoint when the end is the zero address.
fn render_party<'a>(addr: Felt, name: &'a str) -> &'a str {
    if addr == Felt::ZERO {
        "Ethereum L1"
    } else {
        name
    }
}

/// Ekubo moving funds between its own positions — both ends are an Ekubo name.
fn is_pool_rebalance(t: &ResolvedTransfer) -> bool {
    t.from_name.contains("kubo") && t.to_name.contains("kubo")
}

fn rounded_amount(amount: BigUint, token: &Token) -> String {
    to_rounded(amount, token.decimals)
        .to_u128()
        .unwrap()
        .to_formatted_string(&Locale::en)
}

fn usd_value(amount: BigUint, alert: &Alert) -> String {
    let rounded = to_rounded(amount, alert.token.decimals);
    if let Some(rate) = &alert.rate {
        (rounded * rate)
            .div(BigUint::new(vec![10000]))
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

fn hex(felt: Felt) -> String {
    format!("{:#x}", felt)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::consts::TOKENS;

    const PARTY: &str = "0x6e1...b3ce";
    const ADDR: &str = "0x6e14b28449c412a336e7a5a3473da083b9159e6845be4d02ee50f6095a5b3ce";
    const TX: &str = "0x732b09d901fb0075d283ac23cbaae4f8c486123a88a621eeaa05d0b5ddfb8d8";

    // TOKENS[1] is USDC.e: 6 decimals, "$" logo.
    fn usdc() -> &'static Token {
        &TOKENS[1]
    }

    fn addr() -> Felt {
        Felt::from_hex(ADDR).unwrap()
    }

    fn tx() -> Felt {
        Felt::from_hex(TX).unwrap()
    }

    // 1,000,000 USDC.e (raw = 1e12 with 6 decimals).
    fn transfer(from: Felt, from_name: &str, to: Felt, to_name: &str) -> ResolvedTransfer {
        ResolvedTransfer {
            from,
            to,
            from_name: from_name.to_owned(),
            to_name: to_name.to_owned(),
            amount: to_u256(1_000_000_000_000, 0),
        }
    }

    // Rate is price * 10000; $1.00 → 10000.
    fn alert(transfers: Vec<ResolvedTransfer>) -> Alert<'static> {
        Alert {
            token: usdc(),
            tx: tx(),
            rate: Some(BigUint::new(vec![10000])),
            transfers,
        }
    }

    #[test]
    fn single_bridge_to_starknet() {
        let out = compose_tweets(&alert(vec![transfer(Felt::ZERO, "", addr(), PARTY)]));
        assert_eq!(
            out,
            vec![format!(
                "1,000,000 #USDC.e $ (1,000,000 USD)\n{PARTY} bridged to Starknet\n{TX}"
            )]
        );
    }

    #[test]
    fn single_bridge_to_ethereum() {
        let out = compose_tweets(&alert(vec![transfer(addr(), PARTY, Felt::ZERO, "")]));
        assert_eq!(
            out,
            vec![format!(
                "1,000,000 #USDC.e $ (1,000,000 USD)\n{PARTY} bridged to Ethereum\n{TX}"
            )]
        );
    }

    #[test]
    fn single_from_to() {
        let out = compose_tweets(&alert(vec![transfer(addr(), PARTY, addr(), PARTY)]));
        assert_eq!(
            out,
            vec![format!(
                "1,000,000 #USDC.e $ (1,000,000 USD)\nFrom {PARTY} to {PARTY}\n{TX}"
            )]
        );
    }

    #[test]
    fn missing_rate_renders_question_marks() {
        let mut a = alert(vec![transfer(addr(), PARTY, addr(), PARTY)]);
        a.rate = None;
        assert!(compose_tweets(&a)[0].starts_with("1,000,000 #USDC.e $ (??? USD)"));
    }

    #[test]
    fn pool_rebalance_is_dropped() {
        let out = compose_tweets(&alert(vec![transfer(
            addr(),
            "Ekubo: Core",
            addr(),
            "Ekubo: Positions",
        )]));
        assert_eq!(out, Vec::<String>::new());
    }

    #[test]
    fn multicall_under_limit_is_one_tweet() {
        let out = compose_tweets(&alert(vec![
            transfer(addr(), PARTY, addr(), PARTY),
            transfer(addr(), PARTY, Felt::ZERO, ""),
        ]));
        assert_eq!(out.len(), 1);
        assert!(out[0].starts_with("Multicall of #USDC.e: \n"));
        // Bridge end renders as the L1 endpoint, not a sliced zero address.
        assert!(out[0].contains("-> Ethereum L1\n"));
        assert!(out[0].ends_with(TX));
    }

    #[test]
    fn multicall_filtered_to_one_renders_as_single() {
        let out = compose_tweets(&alert(vec![
            transfer(addr(), "Ekubo: Core", addr(), "Ekubo: Positions"),
            transfer(addr(), PARTY, addr(), PARTY),
        ]));
        assert_eq!(out.len(), 1);
        assert!(out[0].contains("From "));
        assert!(!out[0].contains("Multicall"));
    }

    #[test]
    fn multicall_over_limit_splits_into_single_tweets() {
        let transfers: Vec<ResolvedTransfer> = (0..5)
            .map(|_| transfer(addr(), PARTY, addr(), PARTY))
            .collect();
        let out = compose_tweets(&alert(transfers));
        assert_eq!(out.len(), 5);
        assert!(out
            .iter()
            .all(|t| t.contains("From ") && !t.contains("Multicall")));
    }

    #[test]
    fn rounding_halves_up() {
        // 83.997 → 84, 83.497 → 83 (18 decimals).
        assert_eq!(
            to_rounded(to_u256(83997000000000000000, 0), 18),
            to_u256(84, 0)
        );
        assert_eq!(
            to_rounded(to_u256(83497000000000000000, 0), 18),
            to_u256(83, 0)
        );
    }
}
