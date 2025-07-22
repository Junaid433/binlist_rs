# binlist_rs

![License](https://img.shields.io/crates/l/binlist_rs)
![Crates.io](https://img.shields.io/crates/v/binlist_rs)
![Build](https://img.shields.io/github/actions/workflow/status/yourusername/binlist_rs/ci.yml?branch=main)

**binlist_rs** is a lightweight and async Rust wrapper for the Binlist API (https://binlist.net/). 
It allows you to perform BIN lookups with full support for proxies, custom error types, and structured JSON responses.

## ✨ Features

- 🔍 Query BINs for card metadata
- 🌍 Optional proxy support (SOCKS & HTTPS)
- 🔧 Strongly typed response models
- 🚫 Graceful handling of API rate limits
- 🧪 Designed for use in CLI tools or backend services

## 📦 Installation

Add this to your Cargo.toml:

```toml
[dependencies]
binlist_rs = "0.1.0"
````

> Replace version with the latest from [https://crates.io/crates/binlist\_rs](https://crates.io/crates/binlist_rs)

## ⚡️ Usage

```rust
use binlist_rs::BinLookup;

#[tokio::main]
async fn main() {
    let bin = "531462";
    let proxy = Some("socks5://127.0.0.1:9050");

    let mut client = BinLookup::new();
    let result = client.lookup(bin, proxy.as_deref()).await;

    match result {
        Ok(info) => {
            println!("Scheme: {:?}", info.scheme);
            println!("Country: {:?}", info.country.map(|c| c.name));
        }
        Err(err) => eprintln!("Error: {}", err),
    }
}
```

## 📚 API Reference

* `BinLookup::new()`: Initializes the client.
* `lookup(bin: &str, proxy_url: Option<&str>)`: Performs the API request.
* Returns a `Result<CardInfo, LookupError>`.

### Available Fields in Response:

* `scheme`, `brand`, `card_type`, `prepaid`
* `country`: name, emoji, currency, coordinates
* `bank`: name, phone, url

## 🛠 Error Handling

All errors implement `std::error::Error`:

* `LookupError::BINLookupError`
* `LookupError::ReqwestError`
* `LookupError::RateLimitExceeded`

## 🔐 Proxy Support

Supports:

* `http://`, `https://`
* `socks5://`, `socks5h://`

Example:

```rust
let proxy = Some("socks5://127.0.0.1:9050");
```

## 📄 License

MIT © 2025 Junaid HACKUR ([https://github.com/junaid433](https://github.com/junaid433))

## 🧠 Acknowledgments

Powered by the awesome Binlist API ([https://binlist.net](https://binlist.net)).