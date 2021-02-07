use binance::account::*;
use binance::api::*;
use chrono::{DateTime, Duration, Utc};
use config::Config;
use curl::easy::{Easy, List};
use regex::Regex;
use std::collections::HashMap;
use std::str;
use std::{thread, time};

fn has_doge_string(response: &str) -> bool {
    let re = Regex::new(r"(?i)[^@]doge").unwrap();
    re.is_match(response)
}

fn place_order<F>(binance_api_key: &str, binance_secret_key: &str, order_size: F)
where
    F: Into<f64>,
{
    let account: Account = Binance::new(
        Some(binance_api_key.to_string()),
        Some(binance_secret_key.to_string()),
    );
    match account.market_buy("DOGEUSDT", order_size) {
        Ok(r) => println!("Successful order: {:?}", r),
        Err(e) => println!("Error placing order: {:?}", e),
    };
}

fn main() {
    let mut config = Config::default();
    config
        .merge(config::File::with_name("Config"))
        .unwrap()
        .merge(config::Environment::with_prefix("ELON_DOGE"))
        .unwrap();
    let config = config.try_into::<HashMap<String, String>>().unwrap();
    let binance_api_key: String = config.get("binance_api_key").unwrap().into();
    let binance_secret_key: String = config.get("binance_secret_key").unwrap().into();
    let twitter_bearer_token: String = config.get("twitter_bearer_token").unwrap().into();
    let order_size_doge: String = config.get("order_size_doge").unwrap().into();
    let order_size_doge: f32 = order_size_doge.parse::<f32>().unwrap();

    loop {
        let thirty_seconds_ago: DateTime<Utc> = Utc::now() - Duration::seconds(30);

        let mut easy = Easy::new();
        easy.url(&format!("https://api.twitter.com/2/tweets/search/recent?start_time={:?}&tweet.fields=created_at&query=(from:elonmusk)", thirty_seconds_ago)).unwrap();

        let mut list = List::new();
        let auth_header = format!("Authorization: Bearer {}", twitter_bearer_token);
        list.append(&auth_header).unwrap();
        easy.http_headers(list).unwrap();
        let binance_api_key = binance_api_key.clone();
        let binance_secret_key = binance_secret_key.clone();
        easy.write_function(move |data| {
            let response = str::from_utf8(data).unwrap().to_string();
            if has_doge_string(&response) {
                place_order(&binance_api_key, &binance_secret_key, order_size_doge);
            }
            Ok(data.len())
        })
        .unwrap();
        easy.perform().unwrap();
        let thirty_seconds = time::Duration::from_secs(30);
        thread::sleep(thirty_seconds);
    }
}

#[cfg(test)]
mod tests {
    use super::has_doge_string;

    #[test]
    fn test_regex() {
        assert_eq!(true, has_doge_string("buy Dogecoin"));
        assert_eq!(false, has_doge_string("@dogeman"))
    }
}
