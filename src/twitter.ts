import { TwitterApi } from "twitter-api-v2";
import "dotenv/config";
import { getTwitterRefreshToken, writeTwitterRefreshToken } from "./db";

const twitterClient = new TwitterApi({
  clientId: process.env.TWITTER_OAUTH2_CLIENT_ID as string,
  clientSecret: process.env.TWITTER_OAUTH2_CLIENT_SECRET as string,
});

async function refreshToken() {
  console.log(getTwitterRefreshToken());
  try {
    const { refreshToken: newRefreshToken } = await twitterClient.refreshOAuth2Token(getTwitterRefreshToken());

    writeTwitterRefreshToken(newRefreshToken as string);
  } catch (e: any) {
    console.log(`${new Date().toISOString()} - Error in refreshToken()`);
    console.log(e);
  }
}
async function tweet(tweetText: string) {
  try {
    const { client: refreshedClient, refreshToken: newRefreshToken } = await twitterClient.refreshOAuth2Token(
      getTwitterRefreshToken(),
    );

    writeTwitterRefreshToken(newRefreshToken as string);
    await refreshedClient.v2.tweet(tweetText);
  } catch (e: any) {
    // TODO If e.data.status == 429  Store tweet until reset time (e.rateLimit.reset)
    console.log(`${new Date().toISOString()} - Error in refreshTokenAndTweet()`);
    console.log(e);
  }
}

export { refreshToken, tweet };
