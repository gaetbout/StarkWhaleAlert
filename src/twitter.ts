import { TwitterApi } from "twitter-api-v2";
import "dotenv/config";
import { getTwitterRefreshToken, writeTwitterRefreshToken } from "./db";

const twitterClient = new TwitterApi({
  clientId: process.env.TWITTER_OAUTH2_CLIENT_ID as string,
  clientSecret: process.env.TWITTER_OAUTH2_CLIENT_SECRET as string,
});

async function refreshToken() {
  const { refreshToken: newRefreshToken } = await twitterClient.refreshOAuth2Token(getTwitterRefreshToken());
  writeTwitterRefreshToken(newRefreshToken as string);
}

async function doTweet(tweetText: string) {
  const { client: refreshedClient, refreshToken: newRefreshToken } = await twitterClient.refreshOAuth2Token(
    getTwitterRefreshToken(),
  );
  writeTwitterRefreshToken(newRefreshToken as string);
  await refreshedClient.v2.tweet(tweetText);
}

export { refreshToken, doTweet };
