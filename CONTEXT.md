# StarkWhaleAlert

A bot that watches Starknet for large token `Transfer`s and posts a tweet for each one that clears a per-token threshold. This file fixes the domain language so modules and types are named consistently.

## Transfers & events

**Transfer**:
A single ERC-20 `Transfer` of one token from one address to another. The raw on-chain form is an `EmittedEvent`; the decoded form is a `TransferEvent` carrying `from`, `to`, and `amount`.
_Avoid_: payment, tx (a Transfer is one event, not a whole transaction)

**Threshold**:
The per-token minimum `amount` a Transfer must clear to be worth alerting on. Held on `Token`.
_Avoid_: limit, minimum

**Multicall**:
A single transaction that emits more than one qualifying Transfer for the same token. Transfers are grouped by transaction hash, so a Multicall is "the Transfers sharing one tx hash."
_Avoid_: batch, group

**Rate**:
The USD price of one token unit, fetched from CoinMarketCap. `None` when a token has no `rate_api_id`, which renders as `???`.
_Avoid_: price, quote, value

## Composing the alert

**Alert**:
Everything needed to render one transaction's tweets: the token, the transaction hash, the Rate (fetched once), and the transaction's Resolved Transfers. The input to the Composer; pure data, no I/O.
_Avoid_: notification, message, post

**Resolved Transfer**:
A Transfer whose `from` and `to` have each been resolved to a display name, while still retaining the raw `Felt` addresses so the Composer can detect Bridges. The unit the Composer iterates over.
_Avoid_: enriched transfer, formatted transfer

**Composer**:
The deep module that turns an Alert into the ready-to-post tweet strings (`Vec<String>`). Owns the single/Multicall choice, the 280-character split, the Pool-rebalance filter, and Bridge wording. Pure and synchronous — it is the test surface.
_Avoid_: formatter, builder, renderer (when you mean the whole module)

**Party**:
One end (the `from` or the `to`) of a Transfer as it appears in a tweet: a Known Address name, a StarknetId domain, a shortened address, or a Bridge endpoint (`Ethereum L1` / `Starknet L2`).
_Avoid_: counterparty, account

**Bridge**:
A Transfer with the zero address at one end. To the zero address is a burn, from it is a mint. A single tweet phrases this as "bridged to Ethereum" / "bridged to Starknet"; a multicall line, which has no room for the phrasing, renders the zero end as the `Ethereum L1` endpoint.
_Avoid_: mint/burn (use Bridge in tweet-facing language)

**Pool rebalance**:
A Transfer whose Parties both resolve to an Ekubo name — Ekubo moving funds between its own positions, not a whale move. Filtered out of every Alert, single or Multicall.
_Avoid_: internal transfer, noise

## Addresses

**Known Address**:
A curated entry in the address book (`ADDRESS_LIST`) mapping an address suffix to a human name (exchanges, bridges, AMMs). The first naming strategy a Party tries.
_Avoid_: labelled address, tagged address

**Resolution**:
Turning a raw address into a Party name by trying, in order: Known Address → StarknetId domain → shortened `0x123...4567`. The only async step; it happens before the Composer runs.
_Avoid_: lookup, formatting
