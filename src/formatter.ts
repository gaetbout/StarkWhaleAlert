import { addressList, EmittedEvent, provider, Token } from ".";
import { num, uint256 } from "starknet";
import { ethers } from "ethers";

const coincapApiKey = process.env.COINCAP_API_KEY as string;

export async function getFormattedText(event: EmittedEvent, currentToken: Token): Promise<string> {
  const from = await getStarkNameOrAddress(event.data[0]);
  const to = await getStarkNameOrAddress(event.data[1]);
  const amount = lowHigh256ToNumber(currentToken, event.data[2], event.data[3]);
  const rate = await tokenValueToNumber(currentToken.rateApiId);
  const usdValueLocalString = Math.round(amount * rate).toLocaleString();
  const amountFixed = amount.toFixed();

  // TODO Adding emoji before?
  // TODO ugly logic this should definitely change
  let textToTweet = "";
  textToTweet += `${amountFixed} #${currentToken.symbol} ${currentToken.logo} (${usdValueLocalString} USD)`;
  textToTweet += "\n";
  if (to == "0x0") {
    textToTweet += `${from} bridged to Ethereum L1`;
  } else if (from == "0x0") {
    textToTweet += `${to} bridged to Starknet L2`;
  } else {
    textToTweet += `From ${from} to ${to}`;
  }
  textToTweet += "\n";
  textToTweet += `https://starkscan.co/tx/${event.transaction_hash}`;
  return textToTweet;
}

export async function getStarkNameOrAddress(address: string): Promise<string> {
  const el = addressList.find((e) => e.address == address);
  if (el) {
    return el.name;
  }
  try {
    return await provider.getStarkName(address);
  } catch (e) {
    // console.log(e);
    if (address == "0x0") {
      return address;
    }
    return address.slice(0, 5) + "..." + address.slice(-4);
  }
}

function lowHigh256ToNumber(token:Token, low: string, high: string): number {
  if (token.decimals == 18) {
    const amount = uint256.uint256ToBN({ low, high });
  // TODO decimals isn't used atm
  const formattedAmount = ethers.formatUnits(amount);
  return parseFloat(formattedAmount);
  } else {
    const amount: number = parseInt(num.hexToDecimalString(low));
    return amount / 1e6;
  }
}

async function tokenValueToNumber(tokenName: string): Promise<number> {
  // TODO Done temporary for USDC that doesn't have any value in the API
  try {
    const tokenValue = await getTokenValue(tokenName);
    return parseFloat(tokenValue.data.rateUsd);
  } catch (e: any) {
    return 1;
  }
}

async function getTokenValue(tokenName: string) {
  try {
    const response = await fetch(`https://api.coincap.io/v2/rates/${tokenName}`, {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${coincapApiKey}`,
      },
    });
    if (!response.ok) {
      throw new Error(`HTTP error! Status: ${response.status}`);
    }
    return await response.json();
  } catch (error) {
    console.error(error);
  }
}
