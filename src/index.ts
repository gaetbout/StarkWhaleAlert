import { RpcProvider, hash, num, uint256 } from "starknet";
import "dotenv/config";
import { ethers } from "ethers";
import { tokens, getLastBlockNumber, writeLastBlockNumber, addressList } from "./db";
import { EmittedEvent, Token } from "./models";
import { refreshToken, tweet } from "./twitter";
import { log } from "./logger";

const nodeProviderAPIKey = process.env.NODE_PROVIDER_API_KEY as string;
const coincapApiKey = process.env.COINCAP_API_KEY as string;
const provider = new RpcProvider({ nodeUrl: `https://starknet-mainnet.infura.io/v3/${nodeProviderAPIKey}` });

async function main() {
  log("Start", 0);
  const lastBlock = await getLastBlockNumber();
  // We only proccess block that are "complete"
  const lastCompleteBlock = (await provider.getBlockNumber()) - 1;

  // No new block, nothing to proceed
  if (lastBlock >= lastCompleteBlock) {
    log(`no block to process ${lastBlock} <=> ${lastCompleteBlock}`);
    return;
  }

  // TODO Create DB if not existing + Move twitter in its own file and only refresh if timeout rather than every 5 mn
  for (let tokenIndex = 0; tokenIndex < tokens.length; tokenIndex++) {
    const token = tokens[tokenIndex];
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
      for (let index = 0; index < eventsToTweet.length; index++) {
        const textToTweet = await getFormattedText(eventsToTweet[index], token);
        await tweet(textToTweet);
      }
    }
  }
  writeLastBlockNumber(lastCompleteBlock + 1);
  log("End", 0);
}

async function fetchAllEvent(token: Token, lastBlock: number, lastCompleteBlock: number): Promise<EmittedEvent[]> {
  let allEvents: Array<EmittedEvent> = [];
  let continuation_token = "0";
  const selector = hash.getSelectorFromName(token.selector);
  while (continuation_token) {
    const response = await fetchEvents(token, lastBlock, lastCompleteBlock, selector, continuation_token);
    allEvents = allEvents.concat(response.events);
    continuation_token = response.continuation_token;
  }
  return allEvents;
}

async function fetchEvents(
  token: Token,
  lastBlock: number,
  lastCompleteBlock: number,
  selector: string,
  continuation_token: string,
  retries = 0,
): Promise<any> {
  if (retries >= 3) {
    log("Too many failures...");
    process.exit(1);
  }
  try {
    const response = await provider.getEvents({
      from_block: { block_number: lastBlock },
      to_block: { block_number: lastCompleteBlock },
      address: token.address,
      keys: [selector],
      chunk_size: 1000,
      continuation_token,
    });
    return response;
  } catch (e: any) {
    log(`Failed to fetch ${retries}... Retrying`);
    await fetchEvents(token, lastBlock, lastCompleteBlock, selector, continuation_token, retries + 1);
  }
}

async function getFormattedText(event: EmittedEvent, currentToken: Token): Promise<string> {
  const from = await getStarkNameOrAddress(event.data[0]);
  const to = await getStarkNameOrAddress(event.data[1]);
  const amount = lowHigh256ToNumber(event.data[2], event.data[3]);
  const rate = await tokenValueToNumber(currentToken.rateApiId);
  const usdValueLocalString = Math.round(amount * rate).toLocaleString();
  const amountFixed = amount.toFixed(2);

  // TODO Adding emoji before?
  // TODO ugly logic this should definitely change
  let textToTweet = "";
  textToTweet += `${amountFixed} #${currentToken.symbol} ${currentToken.logo} (${usdValueLocalString} USD)`;
  textToTweet += "\n";
  if (to == "0x0") {
    textToTweet += `${from} bridged to Ethereum L1`;
  } else if (from == "0x0") {
    textToTweet += `${to} bridged to Starknet L2`;
  } else {
    textToTweet += `From ${from} to ${to}`;
  }
  textToTweet += "\n";
  textToTweet += `https://starkscan.co/tx/${event.transaction_hash}`;
  return textToTweet;
}

export async function getStarkNameOrAddress(address: string): Promise<string> {
  const el = addressList.find((e) => e.address == address);
  if (el) {
    return el.name;
  }
  try {
    return await provider.getStarkName(address);
  } catch (e) {
    // console.log(e);
    if (address == "0x0") {
      return address;
    }
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
