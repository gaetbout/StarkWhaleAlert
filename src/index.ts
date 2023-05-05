import { RpcProvider, hash, json, num, uint256 } from "starknet";
import "dotenv/config";
import { ethers } from "ethers";
import { tokens, getLastBlockNumber, writeLastBlockNumber } from "./db";

const alchemyApiKey = process.env.ALCHEMY_API_KEY!;
const coincapApiKey = process.env.COINCAP_API_KEY!;
const provider = new RpcProvider({ nodeUrl: `https://starknet-mainnet.g.alchemy.com/v2/${alchemyApiKey}` });

// TODO do a tweet
// TODO Pagination system if a LOT of transfer

async function main() {
  const lastBlockNumber = await getLastBlockNumber();
  const blockNumber = await provider.getBlockNumber();
  writeLastBlockNumber(blockNumber);
  tokens.forEach(async (token) => {
    const transferSelector = hash.getSelectorFromName("Transfer");
    const response = await provider.getEvents({
      from_block: { block_number: lastBlockNumber - 1 },
      to_block: { block_number: blockNumber - 1 },
      address: token.address,
      keys: [transferSelector],
      chunk_size: 1000,
    });
    if (response.events.length == 0) {
      return;
    }

    // TODO Need to filter et pas max
    const max = response.events.reduce((prev, current) => {
      const amount1 = num.toBigInt(prev.data[2]) + num.toBigInt(prev.data[3]);
      const amount2 = num.toBigInt(current.data[2]) + num.toBigInt(current.data[3]);
      return amount1 > amount2 ? prev : current;
    });

    console.log(max);

    const from = await getStarkNameOrAddress(max.data[0]);
    const to = await getStarkNameOrAddress(max.data[1]);
    const amount = fromUint256ToFloat(max.data[2], max.data[3]);
    const rate = await getTokenValueAsFloat("ethereum");
    const usdValue = amount * rate;

    console.log(`From: ${from} to: ${to}`);
    console.log(`\t${amount.toFixed(3)} #${token.symbol} ${token.logo} (${usdValue} USD)`);
    console.log(`\Find it here https://starkscan.co/tx/${max.transaction_hash}`);
  });
}

async function getStarkNameOrAddress(address: string) {
  try {
    return await provider.getStarkName(address);
  } catch (e) {
    // console.log(e);
    return address;
  }
}

function fromUint256ToFloat(low: string, high: string) {
  const amount = uint256.uint256ToBN({ low, high });
  // TODO decimals isn't used atm
  const formattedAmount = ethers.formatUnits(amount);
  return parseFloat(formattedAmount);
}

async function getTokenValueAsFloat(tokenName: string) {
  const tokenValue = await getTokenValue("ethereum");
  return parseFloat(tokenValue.data.rateUsd);
}
async function getTokenValue(tokenName: string) {
  try {
    const response = await fetch(`https://api.coincap.io/v2/rates/${tokenName}`, {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${coincapApiKey}`,
      },
    });
    if (!response.ok) {
      throw new Error(`HTTP error! Status: ${response.status}`);
    }
    return await response.json();
  } catch (error) {
    console.error(error);
  }
}

main();
