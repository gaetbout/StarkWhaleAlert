# StarkWhaleAlert

[![Twitter URL](https://img.shields.io/twitter/url.svg?label=Follow%20%40StarkWhaleAlert&style=social&url=https%3A%2F%2Ftwitter.com%2FStarkWhaleAlert)]( https://twitter.com/StarkWhaleAlert)

This is a script aimed to track all transfers of certains tokens above a specified threshold done on Starknet.  
Whenever a `Transfer` matching the criteria is detected, this bot will post a Tweet.

## Overview

Here is how the script works:
1. Fetch last processed block number from a file and the last block produced by the network. 
   - If no new block ⇒ Stop
   - If it is too much behind ⇒ Stop (can happen that something went wrong).
   - Otherwise ⇒ Continue
2. For each token:
   1. Get all `Transfer` events from last processed block to current network block.
   2. Filter events to keep the ones above the threshold:
      - If none ⇒ Proceed to next token
      - If any ⇒ Tweet
3. Set last processed block to last block produced by the network


To create the tweet the text needs to be formatted involving multiple steps:
- Format the token value 
- Fetch it's USD value and format it 
- Format the `from` and `to`:
  - Can be someone bridging to Ethereum L1
  - Can be someone bridging to Starknet L2
  - Can be someone owning a StarknetId
  - Can be someone belonging to the list of known addresses, e.g.: Layerswap, Jediswap, ...
  - If none of the previous ⇒ Shorten the address to have something like `0x123...4567`
- Put a reference to the transaction involving the transfer (link to a block explorer)


## Installation
**Prerequisite: have Rust and Cargo installed**   

Build both binaries:
```shell
cargo build
```

Run a specific binary using: 
```shell
cargo run --bin bin_name
```

Build for a linux environment from a mac using: 
```shell
cargo build --bin starkwhale_alert --release --target=x86_64-unknown-linux-gnu
```

## Setup 

### Twitter
Start by running once the project:
```shell
cargo run --bin starkwhale_alert
```
This will take care of creating all relevant files in the db folder (and also the folder)

First create a new project on:  
https://developer.twitter.com/en/portal/dashboard  
Remember to save the `CLIENT_ID` and `CLIENT_SECRET` somehwere, you'll need it later.  
In the **App info** section fill the Callback URI with `http://127.0.0.1:3000/callback`.  
You can fill the required `Website URL` with anything you want it doesn't matter.

Then run:
```shell
cargo run --bin twitter_login
```
You can now open http://127.0.0.1:3000/login in your browser and authorize the application. It should redirect you to another page, copy the whole page and paste it in the [token file](./db/token.json).

### Setup the .env
This script is using: 
- Infura as a node provider
- Coincap to fetch the USD value of a token
- Here is where you paste the twitter `CLIENT_ID` and `CLIENT_SECRET`

Feel free to tweak the code to use another RPC or API to fetch the USD value.

### Run the bot
Once everything is setup you can try the bot using:
```shell
cargo run --bin starkwhale_alert
```

If you want to build the binary, add the release flag: 
```shell
cargo build --bin starkwhale_alert --release
```
You can now find the binary at `./target/release/starkwhale_alert`

The first time you run the script, the block might be out of date and it'll tell you to restart the script using an option:
```shell
./starkwhale_alert -s
```
This will update the block file to the latest block.

After this step you are all setup.  
I personally setup nodecron to run this script every 5 minutes.
