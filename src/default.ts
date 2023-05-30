import { hash, num } from "starknet";
import "dotenv/config";
import {
  EmittedEvent,
  Token,
  refreshToken,
  tweet,
  log,
  tokens,
  getLastBlockNumber,
  writeLastBlockNumber,
  getFormattedText,
  provider,
} from ".";

export async function main() {
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

main();
