import { RpcProvider, hash, num, uint256 } from "starknet";
import "dotenv/config";
import { ethers } from "ethers";
import { tokens, getLastBlockNumber, writeLastBlockNumber } from "./db";
import { EmittedEvent, Token } from "./models";
import { doTweet, refreshToken } from "./twitter";

const alchemyApiKey = process.env.ALCHEMY_API_KEY as string;
const coincapApiKey = process.env.COINCAP_API_KEY as string;
const provider = new RpcProvider({ nodeUrl: `https://starknet-mainnet.g.alchemy.com/v2/${alchemyApiKey}` });

async function main() {
  const lastBlock = await getLastBlockNumber();
  // We only proccess block that are "complete"
  const lastCompleteBlock = (await provider.getBlockNumber()) - 1;

  // No new block, nothing to proceed
  if (lastBlock >= lastCompleteBlock) {
    return;
  }

  // TODO Change when multi tokekn ==> will be wrong
  tokens.forEach(async (token) => {
    const events = await fetchAllEvent(token, lastBlock, lastCompleteBlock);
    if (events.length == 0) {
      return;
    }

    const eventsToTweet = events.filter((e) => {
      const amount1 = num.toBigInt(e.data[2]) + num.toBigInt(e.data[3]);
      return amount1 > token.threshold;
    });

    if (eventsToTweet.length == 0) {
      await refreshToken();
    } else {
      for (let index = 0; index < eventsToTweet.length; index++){
        const textToTweet = await getFormattedText(eventsToTweet[index], token);
        await doTweet(textToTweet);
      }
    }
  });
  writeLastBlockNumber(lastCompleteBlock);
}


async function fetchAllEvent(token: Token, lastBlock: number, lastCompleteBlock: number): Promise<EmittedEvent[]> {
  let allEvents: Array<EmittedEvent> = [];
  let continuationToken = "0";
  const selector = hash.getSelectorFromName(token.selector);
  while (continuationToken) {
    const response = await provider.getEvents({
      from_block: { block_number: lastBlock },
      to_block: { block_number: lastCompleteBlock },
      address: token.address,
      keys: [selector],
      chunk_size: 1000,
      continuation_token: continuationToken,
    });

    allEvents = allEvents.concat(response.events);
    continuationToken = response.continuation_token;
  }
  return allEvents;
}

async function getFormattedText(event: EmittedEvent, currentToken: Token): Promise<string> {
  const from = await getStarkNameOrAddress(event.data[0]);
  const to = await getStarkNameOrAddress(event.data[1]);
  const amount = lowHigh256ToNumber(event.data[2], event.data[3]);
  const rate = await tokenValueToNumber(currentToken.rateApiId);
  const usdValueLocalString = Math.round(amount * rate).toLocaleString();
  const amountFixed = amount.toFixed(3);

  // TODO Adding emoji before?
  let textToTweet = "";
  textToTweet += `${amountFixed} #${currentToken.symbol} ${currentToken.logo} (${usdValueLocalString} USD)`;
  textToTweet += "\n";
  textToTweet += `From: ${from} to: ${to}`;
  textToTweet += "\n";
  textToTweet += `https://starkscan.co/tx/${event.transaction_hash}`;
  return textToTweet;
}

async function getStarkNameOrAddress(address: string): Promise<string> {
  try {
    return await provider.getStarkName(address);
  } catch (e) {
    // console.log(e);
    return address.slice(0, 5) + "..." + address.slice(-4);
  }
}

function lowHigh256ToNumber(low: string, high: string): number {
  const amount = uint256.uint256ToBN({ low, high });
  // TODO decimals isn't used atm
  const formattedAmount = ethers.formatUnits(amount);
  return parseFloat(formattedAmount);
}

async function tokenValueToNumber(tokenName: string): Promise<number> {
  const tokenValue = await getTokenValue(tokenName);
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
