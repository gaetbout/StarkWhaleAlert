import { RpcProvider, constants } from "starknet";
import { doLogic } from "./default";

export * from "./db";
export * from "./default";
export * from "./formatter";
export * from "./logger";
export * from "./models";
export * from "./twitter";

const nodeProviderAPIKey = process.env.NODE_PROVIDER_API_KEY as string;
export const provider = new RpcProvider({
  nodeUrl: `https://starknet-mainnet.infura.io/v3/${nodeProviderAPIKey}`,
  chainId: constants.StarknetChainId.SN_MAIN,
});

await doLogic();
