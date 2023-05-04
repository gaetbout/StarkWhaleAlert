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
// TODO Add way of store each token address + transfer_selector + decimals + HIT limit + SYMBOL + LOGO
// TODO Should use some oracle to show actual value
// TODO Pagination system if a LOT of transfer
async function main() {
  const block_number = await provider.getBlockNumber();
  const transfer_selector = hash.getSelectorFromName("Transfer");
  const response = await provider.getEvents({
    from_block: { block_number: block_number - 100 },
    to_block: { block_number:  block_number - 1 },
    address: ETH_ADDRESS,
    keys: [transfer_selector],
    chunk_size: 1000,
  });

  // TODO Need to filter et pas max
  const max = response.events.reduce((prev, current) => {
    const amount1 = num.toBigInt(prev.data[2]) + num.toBigInt(prev.data[3]);
    const amount2 = num.toBigInt(current.data[2]) + num.toBigInt(current.data[3]);
    return amount1 > amount2 ? prev : current;
  });

  console.log(max);

  const from = await getStarkNameOrAddress(max.data[0]);
  const to = await getStarkNameOrAddress(max.data[1]);
  const amount = fromUint256ToFormattedNumber(max.data[2], max.data[3]);

  console.log(`From: ${from} to: ${to}`);
  console.log(`\t${amount}`);
  console.log(`\Find it here https://starkscan.co/tx/${max.transaction_hash}`);
}

async function getStarkNameOrAddress(address: string) {
  try {
    return await provider.getStarkName(address);
  } catch (e) {
    // console.log(e);
    return address;
  }
}

async function doTweet() {
  // const tweet = await client.tweets.findTweetById("20");
  // if (tweet.data) {
  //   console.log(tweet.data.text);
  // } else {
  //   console.log("Nothing");
  // }
}

function fromUint256ToFormattedNumber(low: string, high: string) {
  const amount = uint256.uint256ToBN({low,high});
  // TODO decimals isn't used atm
  const formattedAmount = ethers.formatUnits(amount);
  const parsedNum = parseFloat(formattedAmount);
  const roundedNum = parsedNum.toFixed(3);
  return roundedNum.toString();
}
main();
