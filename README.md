# elon-doge-bot

![Screenshot of a tweet from Elon Musk showing an image from the Lion King
where Elon's face has been photoshopped over the face of Mufasa presenting
a shiba una doge which has been photoshopped over Simba](tweet.png)

**Tired of losing out on the Doge pump? Run the elon doge bot!**
Every time Elon tweets about Dogecoin, buy some! This bot monitors
[@elonmusk](https://twitter.com/elonmusk) and places a market buy order on
Binance.us if one of his new tweet mentions $DOGE.

## Usage

To run the bot, you'll need a Twitter bearer token and a Binance.us API key
and secret.

You'll also need [Rust installed on your machine](https://www.rust-lang.org/tools/install).

Copy `Config.toml.example` and rename it `Config.toml`. Put your credentials in
the file and set `order_size_doge` to the amount of $DOGE you want to buy on each
trade.

In the terminal, navigate to the repository and execute `cargo run`.

### Using Internation Binance

The bot is set up to trade on Binance.us. To use the international
Binance, in `Cargo.toml` change the line

```
binance = { git = "https://github.com/rhinodavid/binance-rs.git" }
```

to

```
binance = { git = "https://github.com/wisespace-io/binance-rs.git" }
```
