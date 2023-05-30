import { json } from "starknet";
import { readFileSync, writeFileSync } from "fs";
import { AddressToName, Token } from "./models";
import { log } from "./logger";

// TODO Could use https://github.com/typicode/lowdb or even an actual DB
function getLastBlockNumber(): number {
  const jsonBlock = json.parse(readFileSync("./db/block.json").toString("ascii"));
  return jsonBlock.lastProcesssedBlockNumber;
}

function getTwitterRefreshToken(): string {
  const jsonBlock = json.parse(readFileSync("./db/block.json").toString("ascii"));
  return jsonBlock.twitterRefreshToken;
}

function writeLastBlockNumber(lastProcesssedBlockNumber: number) {
  const jsonBlock = json.parse(readFileSync("./db/block.json").toString("ascii"));
  jsonBlock.lastProcesssedBlockNumber = lastProcesssedBlockNumber;
  log(`Done ${lastProcesssedBlockNumber}`);
  writeFileSync("./db/block.json", json.stringify(jsonBlock));
}

function writeTwitterRefreshToken(twitterRefreshToken: string) {
  const jsonBlock = json.parse(readFileSync("./db/block.json").toString("ascii"));
  jsonBlock.twitterRefreshToken = twitterRefreshToken;
  writeFileSync("./db/block.json", json.stringify(jsonBlock));
}

const ETH = {
  address: "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
  decimals: 18,
  symbol: "ETH",
  selector: "Transfer",
  threshold: 2e19, // 20 eth
  logo: "♦",
  rateApiId: "ethereum",
};

const LAYER_SWAP = {
  address: "0x19252b1deef483477c4d30cfcc3e5ed9c82fafea44669c182a45a01b4fdb97a",
  name: "Layerswap",
};
const addressList: AddressToName[] = [LAYER_SWAP];
const tokens: Token[] = [ETH];

export {
  tokens,
  getLastBlockNumber,
  writeLastBlockNumber,
  getTwitterRefreshToken,
  writeTwitterRefreshToken,
  addressList,
};
