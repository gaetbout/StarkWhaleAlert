import { RpcProvider, constants, hash, num, uint256 } from "starknet";
import "dotenv/config";

type EmittedEvent = {
  data: Array<string>;
  keys: Array<string>;
  from_address: string;
  block_hash: string;
  block_number: number;
  transaction_hash: string;
};

const CONTRACT_ADDRESS = "0x068f5c6a61780768455de69077e07e89787839bf8166decfbf92b645209c0fb8";
const SELECTOR = "Transfer";
const DECIMALS = BigInt(1e6);

let loopNumber = 0;

const nodeProviderAPIKey = process.env.NODE_PROVIDER_API_KEY as string;
export const provider = new RpcProvider({
  nodeUrl: `https://starknet-mainnet.infura.io/v3/${nodeProviderAPIKey}`,
  chainId: constants.StarknetChainId.SN_MAIN,
});
async function main() {
  const block_number = await provider.getBlockNumber();
  await fetchAllEvents(block_number);
}

async function fetchAllEvents(block_number: number) {
  const events = await recursiveFetch(block_number);

  const sortedEvents = events.sort((a, b) => {
    const amount1 = num.toBigInt(a.data[2]) + num.toBigInt(a.data[3]);
    const amount2 = num.toBigInt(b.data[2]) + num.toBigInt(b.data[3]);
    return Number(amount2 - amount1);
  });

  logNFirstItems(sortedEvents, 10);
}

async function recursiveFetch(block_number: number, continuation_token = "0"): Promise<EmittedEvent[]> {
  console.log(`Looped ${loopNumber} time(s), processed ${continuation_token} items`);
  loopNumber += 1;
  const transfer_selector = hash.getSelectorFromName(SELECTOR);
  const response = await provider.getEvents({
    from_block: { block_number: block_number - 1000 },
    to_block: { block_number: block_number - 1 },
    address: CONTRACT_ADDRESS,
    keys: [[transfer_selector]],
    chunk_size: 1000,
    continuation_token,
  });

  if (response.continuation_token) {
    const tmp = await recursiveFetch(block_number, response.continuation_token);
    return response.events.concat(tmp);
  } else {
    return response.events;
  }
}
function logNFirstItems(items: Array<EmittedEvent>, numberOfItemToLog: number) {
  console.log(`${items.length} events`);
  for (let i = 0; i < numberOfItemToLog; i++) {
    console.log(`${i} ======================================================`);
    logItem(items[i]);
    console.log();
  }
}

function logItem(event: EmittedEvent) {
  const from = event.data[0];
  const to = event.data[1];
  const amount = uint256.uint256ToBN({ low: event.data[2], high: event.data[3] });

  console.log(`\tFrom: ${from} to: ${to}`);
  console.log(`\t${amount} (${amount.toString().length}) - ${amount / DECIMALS}`);
  console.log(`\t${event.block_number}`);
  console.log(`\thttps://starkscan.co/tx/${event.transaction_hash}`);
}
main();
