import { RpcProvider, constants, hash, num, uint256 } from "starknet";
import "dotenv/config";
import { EmittedEvent } from "../src/models";

const CONTRACT_ADDRESS = "0x053c91253bc9682c04929ca02ed00b3e423f6710d2ee7e0d5ebb06f3ecf368a8";
const SELECTOR = "Transfer";

let loopNumber = 0;

const nodeProviderAPIKey = process.env.NODE_PROVIDER_API_KEY as string;
export const provider = new RpcProvider({
  nodeUrl: `https://starknet-mainnet.infura.io/v3/${nodeProviderAPIKey}`,
  chainId: constants.StarknetChainId.SN_MAIN,
});

async function main() {
  const block_number = await provider.getBlockNumber();
  await recursiveFetch(block_number);
}

async function recursiveFetch(block_number: number, continuation_token = "0") {
  console.log(`Looped ${loopNumber} time(s), processed ${continuation_token || "0"} items`);
  loopNumber += 1;
  const transfer_selector = hash.getSelectorFromName(SELECTOR);
  const response = await provider.getEvents({
    from_block: { block_number: block_number - 10 },
    to_block: { block_number: block_number - 1 },
    address: CONTRACT_ADDRESS,
    keys: [transfer_selector],
    chunk_size: 1000,
    continuation_token,
  });

  const sortedEvents = response.events.sort((a, b) => {
    const amount1 = num.toBigInt(a.data[2]) + num.toBigInt(a.data[3]);
    const amount2 = num.toBigInt(b.data[2]) + num.toBigInt(b.data[3]);
    return Number(amount2 - amount1);
  });

  logNFirstItems(sortedEvents, 5);

  if (response.continuation_token) {
    recursiveFetch(block_number, response.continuation_token);
  }
}

function logNFirstItems(items: Array<EmittedEvent>, numberOfItemToLog = 1) {
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
  console.log(`\t${amount} (${amount.toString().length})`);
  console.log(`\t${event.block_number}`);
  console.log(`\thttps://starkscan.co/tx/${event.transaction_hash}`);
}
main();
