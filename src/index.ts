import { Client } from "twitter-api-sdk";
import { RpcProvider, hash, num, shortString, uint256 } from "starknet";
import "dotenv/config";
import { ethers } from "ethers";

const ETH_ADDRESS = "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7";
const ETH_DECIMALS = 18;
const client = new Client(process.env.BEARER_TOKEN!);
const alchemyApiKey = process.env.ALCHEMY_API_KEY!;
// const provider = new Provider({ sequencer: { network: constants.NetworkName.SN_MAIN } });
const provider = new RpcProvider({ nodeUrl: `https://starknet-mainnet.g.alchemy.com/v2/${alchemyApiKey}` });

// TODO do a tweet
// TODO remember each block inbetween multiple boots
// TODO Stats when is it a big tx?
// TODO Add way of store each token + transfer_selector + decimals
// TODO Pagination system if a LOT of transfer
async function main() {
  const block_number = await provider.getBlockNumber();
  const transfer_selector = hash.getSelectorFromName("Transfer");
  const response = await provider.getEvents({
    from_block: { block_number: block_number - 100 },
    to_block: { block_number },
    address: ETH_ADDRESS,
    keys: [transfer_selector],
    chunk_size: 100,
  });

  response.events
    // .filter((e) => e.keys[0] == transfer_selector)
    .forEach((e) => {
      const from = e.data[0];
      const to = e.data[1];
      const amount = num.toBigInt(e.data[2]) + num.toBigInt(e.data[3]);
      // console.log(`From: ${from} to: ${to} amount: ${ethers.formatUnits(amount, ETH_DECIMALS)}`);
      console.log(`${ethers.formatUnits(amount, ETH_DECIMALS)}`);
      // console.log(`\tTransaction hash: ${ e.transaction_hash }`);
      // console.log(`\Find it here https://starkscan.co/tx/${ e.transaction_hash }`);
    });

  const arr = response.events
    // .filter((e) => e.keys[0] == transfer_selector)
    .map((e) => {
      const amount = num.toBigInt(e.data[2]) + num.toBigInt(e.data[3]);
      return parseFloat(ethers.formatUnits(amount, ETH_DECIMALS));
    });
  console.log(`Max BAD FILTER: ${Math.max(...arr)}`);

  const max = response.events.reduce((prev, current) => {
    const amount1 = num.toBigInt(prev.data[2]) + num.toBigInt(prev.data[3]);
    const amount2 = num.toBigInt(current.data[2]) + num.toBigInt(current.data[3]);
    return amount1 > amount2 ? prev : current;
  });

  console.log(max);
  
  const from = await getStarkNameOrAddress(max.data[0]);
  const to = await getStarkNameOrAddress(max.data[1]);
  console.log(`From: ${from} to: ${to}`);
  // const tweet = await client.tweets.findTweetById("20");
  // if (tweet.data) {
  //   console.log(tweet.data.text);
  // } else {
  //   console.log("Nothing");
  // }
}

async function getStarkNameOrAddress(address: string) {
  try {
    return await provider.getStarkName(address);
  } catch ( e ) {
    return address
  }
}

main();
