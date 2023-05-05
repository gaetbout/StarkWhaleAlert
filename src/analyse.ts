import { RpcProvider, hash, num, uint256 } from "starknet";
import "dotenv/config";

const CONTRACT_ADDRESS = "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7";
const alchemyApiKey = process.env.ALCHEMY_API_KEY!;
const provider = new RpcProvider({ nodeUrl: `https://starknet-mainnet.g.alchemy.com/v2/${alchemyApiKey}` });

// TODO Stats when is it a big tx?
async function main() {
  const block_number = await provider.getBlockNumber();
  const transfer_selector = hash.getSelectorFromName("Transfer");
  const response = await provider.getEvents({
    from_block: { block_number: block_number - 100 },
    to_block: { block_number: block_number - 1 },
    address: CONTRACT_ADDRESS,
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

  const from = max.data[0];
  const to = max.data[1];
  const amount = uint256.uint256ToBN({ low: max.data[2], high: max.data[3] });

  console.log(`From: ${from} to: ${to}`);
  console.log(`\t${amount}`);
  console.log(`\Find it here https://starkscan.co/tx/${max.transaction_hash}`);
}

main();
