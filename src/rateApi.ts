const coincapApiKey = process.env.COINCAP_API_KEY as string;

export async function tokenValueToNumber(tokenName: string): Promise<number> {
  try {
    const tokenValue = await getTokenValue(tokenName);
    return parseFloat(tokenValue.data.priceUsd);
  } catch (e) {
    console.error(e);
    return 1;
  }
}

async function getTokenValue(tokenName: string) {
  try {
    const response = await fetch(`https://api.coincap.io/v2/assets/${tokenName}`, {
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
