import { RpcProvider, constants } from "starknet";
import { doLogic } from "./default";

export * from "./db";
export * from "./default";
export * from "./formatter";
export * from "./logger";
export * from "./models";
export * from "./twitter";
export * from "./rateApi";

const nodeProviderAPIKey = process.env.NODE_PROVIDER_API_KEY as string;
export const provider = new RpcProvider({
  nodeUrl: `https://starknet-mainnet.infura.io/v3/${nodeProviderAPIKey}`,
  chainId: constants.StarknetChainId.SN_MAIN,
});

// TODO Could use nodecron instead of a cron based work or even serverless
// await doLogic();
