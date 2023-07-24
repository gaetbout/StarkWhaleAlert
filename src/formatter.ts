import { addressList, EmittedEvent, provider, Token, tokenValueToNumber } from ".";
import { num, uint256 } from "starknet";
import { ethers } from "ethers";

export async function getFormattedText(event: EmittedEvent, currentToken: Token): Promise<string> {
  const from = await getStarkNameOrAddress(event.data[0]);
  const to = await getStarkNameOrAddress(event.data[1]);
  const amount = lowHigh256ToNumber(currentToken, event.data[2], event.data[3]);
  const rate = await tokenValueToNumber(currentToken.rateApiId);
  const usdValueLocalString = Math.round(amount * rate).toLocaleString();
  // TODO toFixed ==> when usdc it is kinda useless
  // TODO There must be a better way to do this...
  const amountFixed = new Intl.NumberFormat("de-DE", {
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  }).format(amount);

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
  const el = addressList.find((e) => address.endsWith(e.address));
  if (el) {
    return el.name;
  }
  try {
    return await provider.getStarkName(address);
  } catch (e) {
    if (address == "0x0") {
      return address;
    }
    return address.slice(0, 5) + "..." + address.slice(-4);
  }
}

function lowHigh256ToNumber(token: Token, low: string, high: string): number {
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
