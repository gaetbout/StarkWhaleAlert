type Token = {
  address: string;
  decimals: number;
  symbol: string;
  selector: string; // This should be the String of the selector (Transfer, ...), not the HEX value
  threshold: number;
  logo: string;
  rateApiId: string;
};

type EmittedEvent = {
  data: Array<string>;
  keys: Array<string>;
  from_address: string;
  block_hash: string;
  block_number: number;
  transaction_hash: string;
};

type AddressToName = {
  address: string;
  name: string;
};
export { Token, EmittedEvent, AddressToName };
