# elon-doge-bot

![Screenshot of a tweet from Elon Musk showing an image from the Lion King
where Elon's face has been photoshopped over the face of Mufasa presenting
a shiba una doge which has been photoshopped over Simba](tweet.png)

**Tired of losing out on the Doge pump? Run the elon doge bot!**
Every time Elon tweets about Dogecoin, buy some! This bot monitors @elonmusk
and places a market buy order on Binance.us if his tweet mentions $DOGE.

## Usage

To run the bot, you'll need a Twitter bearer token and a Binance.us API key
and secret.

You'll also need [https://www.rust-lang.org/tools/install](Rust installed
on your machine).

Copy `Config.toml.example` and rename it `Config.toml`. Put your credentials in
the file and set `usdt_order_size`.

In the terminal, navigate to the repository and execute `cargo run`.
