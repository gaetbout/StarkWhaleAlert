import { Client } from "twitter-api-sdk";
import "dotenv/config";

const client = new Client(process.env.BEARER_TOKEN!);

console.log(process.env.BEARER_TOKEN);

async function main() {
  const tweet = await client.tweets.findTweetById("20");
  if (tweet.data) {
      console.log(tweet.data.text);
  } else {
    console.log("Nothing");
  }
}

main();