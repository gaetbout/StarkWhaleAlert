import { TwitterApi } from "twitter-api-v2";
import "dotenv/config";
import express from "express";

const app = express();
// Instantiate with desired auth type (here's Bearer v2 auth)
const twitterClient = new TwitterApi({
  clientId: process.env.TWITTER_OAUTH2_CLIENT_ID as string,
  clientSecret: process.env.TWITTER_OAUTH2_CLIENT_SECRET as string,
});

const callbackURL = "http://127.0.0.1:3000/callback";
let codeVerifierSaved: string;
let storedState: string;
let refreshTokenSaved: string;

// STEP 1 - Auth URL
app.get("/login", async function (req, res) {
  const { url, codeVerifier, state } = twitterClient.generateOAuth2AuthLink(callbackURL, {
    scope: ["tweet.read", "tweet.write", "users.read", "offline.access"],
  });

  // store verifier
  codeVerifierSaved = codeVerifier;
  storedState = state;

  res.redirect(url);
});

// STEP 2 - Verify callback code, store access_token
app.get("/callback", async function (req, res) {
  const { state, code } = req.query;

  if (state !== storedState) {
    return res.status(400).send("Stored tokens do not match!");
  }

  const { client: loggedClient, refreshToken } = await twitterClient.loginWithOAuth2({
    code: code as string,
    codeVerifier: codeVerifierSaved,
    redirectUri: callbackURL,
  });

  refreshTokenSaved = refreshToken as string;

  const { data } = await loggedClient.v2.me();
  console.log(refreshTokenSaved);
  res.send(data);
});

async function doTweet(tweetText: string) {
  const { client: refreshedClient, refreshToken: newRefreshToken } = await twitterClient.refreshOAuth2Token(
    process.env.TWITTER_OAUTH2_REFRESH_TOKEN as string,
  );

  console.log(newRefreshToken); // DO WRITE THIS ONE IN CONFIG EVERY TIME IT CHANGES
  await refreshedClient.v2.tweet(tweetText);
}

doTweet("test");
// doListen();

async function doListen() {
  await app.listen(3000, () => {
    console.log(`Go here to login: http://127.0.0.1:3000/login`);
  });
}
