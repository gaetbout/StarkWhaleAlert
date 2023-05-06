type Token = {
  address: string;
  decimals: number;
  symbol: string;
  selector: string;
  threshold: number;
  logo: string;
  rateApiId: string;
};

type EmittedEvent = {
  from_address: string;
  keys: Array<string>;
  data: Array<string>;
  block_hash: string;
  block_number: number;
  transaction_hash: string;
};
export { Token, EmittedEvent };
