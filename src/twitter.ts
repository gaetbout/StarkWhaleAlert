import { TwitterApi } from "twitter-api-v2";
import "dotenv/config";
import { getTwitterRefreshToken, writeTwitterRefreshToken } from "./db";
import { log } from ".";

const twitterClient = new TwitterApi({
  clientId: process.env.TWITTER_OAUTH2_CLIENT_ID as string,
  clientSecret: process.env.TWITTER_OAUTH2_CLIENT_SECRET as string,
});

// TODO If error here, should send an email
async function refreshToken() {
  try {
    const { refreshToken: newRefreshToken } = await twitterClient.refreshOAuth2Token(getTwitterRefreshToken());

    writeTwitterRefreshToken(newRefreshToken as string);
  } catch (e: any) {
    log("Error in refreshToken()");
    console.log(e);
  }
}
// TODO If error here, should send an email
async function tweet(tweetText: string) {
  try {
    const { client: refreshedClient, refreshToken: newRefreshToken } = await twitterClient.refreshOAuth2Token(
      getTwitterRefreshToken(),
    );

    writeTwitterRefreshToken(newRefreshToken as string);
    log(tweetText);
    await refreshedClient.v2.tweet(tweetText);
  } catch (e: any) {
    log("Error in refreshTokenAndTweet()");
    console.log(e);
  }
}

export { refreshToken, tweet };
