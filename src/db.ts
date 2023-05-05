import { json } from "starknet";
import { readFileSync, writeFileSync } from "fs";

function getTokens() {
  const jsonTokens = json.parse(readFileSync("./db/tokens.json").toString("ascii"));
  return jsonTokens.tokens;
}

function getLastBlockNumber() {
  const jsonBlock = json.parse(readFileSync("./db/block.json").toString("ascii"));
  return jsonBlock.lastProcesssedBlockNumber;
}

function writeLastBlockNumber(lastProcesssedBlockNumber: number) {
  const jsonBlock = json.parse(readFileSync("./db/block.json").toString("ascii"));
  jsonBlock.lastProcesssedBlockNumber = lastProcesssedBlockNumber;
  writeFileSync("./db/block.json", json.stringify(jsonBlock));
}

type Token = {
  address: string;
  decimals: number;
  symbol: string;
  selector: string;
  limit: number;
  logo: string;
  rateApiId: string;
};

const ETH = {
  address: "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
  decimals: 18,
  symbol: "ETH",
  selector: "Transfer",
  limit: 5,
  logo: "Ξ",
  rateApiId: "ethereum",
};

const tokens: Token[] = [ETH];

export { tokens, getLastBlockNumber, writeLastBlockNumber };
