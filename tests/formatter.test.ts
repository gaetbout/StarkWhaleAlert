import { num, uint256 } from "starknet";
import { log } from "../src/logger";
import { ethers } from "ethers";

log("lama");
log("lama", 1);


const a = lowHigh256ToNumber(6, "0x267788cf7", "12");

console.log(a);
console.log(a.toFixed(2));

function lowHigh256ToNumber(decimals:number, low: string, high: string): number {
    if (decimals == 18) {
      const amount = uint256.uint256ToBN({ low, high });
    // TODO decimals isn't used atm
    const formattedAmount = ethers.formatUnits(amount);
    return parseFloat(formattedAmount);
    } else {
      const amount: number = parseInt(num.hexToDecimalString(low));
      return amount / 1e6;
    }
  }