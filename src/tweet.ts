import { Client } from "twitter-api-sdk";

const client = new Client(process.env.BEARER_TOKEN!);

async function doTweet() {
  // const tweet = await client.tweets.findTweetById("20");
  // if (tweet.data) {
  //   console.log(tweet.data.text);
  // } else {
  //   console.log("Nothing");
  // }
}
