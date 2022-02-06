# fxoanda

This is an unofficial [Oanda](https://wwww.oanda.com/) API client. This client is still
an experimental work in progress however it is reasonably functional.

The client is generated from the Oanda V20 API definitions. The current state of the client
API is low-level but usable however I would like to see a more ergonomic layer developed on
top.

## Installation

```rust
 cargo install fxoanda
```

## Example usage

```toml
# in your Cargo.toml
reqwest = { version = "^0.11", default-features = false, features = ["blocking", "json","tokio-native-tls"] }
fxoanda = "0.1.3"

```

```rust
use fxoanda::*;

let api_key = env::var("OANDA_KEY").expect("expected OANDA_KEY environment variable to be set");
let api_host = env::var("OANDA_HOST").expect("expected OANDA_HOST environment variable to be set");

let client = fxoanda::Client {
               host: String::from(api_host),
               reqwest: reqwest::blocking::Client::new(),
               authentication: String::from(api_key),
             };
match fxoanda::GetInstrumentCandlesRequest::new()
        .with_instrument("EUR_USD".to_string())
        .with_granularity(CandlestickGranularity::H4)
        .remote(&client)
   {
       Ok(x) => println!("OK: {:#?}", x),
       Err(e) => eprintln!("ERR: {:#?}", e),
   };

```

## Warning

Forex markets are extremely risky. Automated trading is also extremely risky.
This project is extremely risky. Market conditions, news events, or software bugs
can wipe out your account in an instant.

## Disclaimer

Use this project at your own risk. The maintainers of this project make no
claims as to this product being fit for purpose. In fact, the maintainers of this
project are telling you that you shouldn't use this project.

