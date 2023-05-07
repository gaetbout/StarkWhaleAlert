import { TwitterApi } from "twitter-api-v2";
import "dotenv/config";
import express from "express";
import { writeTwitterRefreshToken } from "./db";

const app = express();
// Instantiate with desired auth type (here's Bearer v2 auth)
const twitterClient = new TwitterApi({
  clientId: process.env.TWITTER_OAUTH2_CLIENT_ID as string,
  clientSecret: process.env.TWITTER_OAUTH2_CLIENT_SECRET as string,
});

const callbackURL = "http://127.0.0.1:3000/callback";
let codeVerifierSaved: string;
let storedState: string;

// STEP 1 - Auth URL
app.get("/login", async function (req, res) {
  const { url, codeVerifier, state } = twitterClient.generateOAuth2AuthLink(callbackURL, {
    scope: ["tweet.read", "tweet.write", "users.read", "offline.access"],
  });

  codeVerifierSaved = codeVerifier;
  storedState = state;

  res.redirect(url);
});

app.get("/callback", async function (req, res) {
  const { state, code } = req.query;

  if (state !== storedState) {
    return res.status(400).send("Stored tokens do not match!");
  }

  const { refreshToken } = await twitterClient.loginWithOAuth2({
    code: code as string,
    codeVerifier: codeVerifierSaved,
    redirectUri: callbackURL,
  });

  writeTwitterRefreshToken(refreshToken as string);
});

app.listen(3000, () => {
  console.log(`You can open the login page`);
});
