# binlist_rs

![License](https://img.shields.io/crates/l/binlist_rs)
![Crates.io](https://img.shields.io/crates/v/binlist_rs)
![Build](https://img.shields.io/github/actions/workflow/status/Junaid433/binlist_rs/ci.yml?branch=master)

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

    let mut lookup = BinLookup::new();
    let result = lookup.lookup(bin, None).await;

    if let Ok(info) = result {
        println!("Scheme: {}", info.scheme.unwrap_or("Unknown".into()));
        if let Some(country) = info.country {
            println!("Country: {}", country.name.unwrap_or("Unknown".into()));
        }
    } else {
        println!("Failed to look up BIN.");
    }
}
```

## 📚 API Reference

* `BinLookup::new()`: Initializes the client.
* `lookup(bin: &str, proxy_url: Option<&str>)`: Performs the API request.
* Returns a `Result<CardInfo, LookupError>`.

### Available Fields in Response:

* `scheme`, `brand`, `type`, `prepaid`
* `country`: name, emoji, currency, coordinates
* `bank`: name

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

MIT © 2025 Junaid Rahman ([https://github.com/junaid433](https://github.com/junaid433))

## 🧠 Acknowledgments

Powered by the awesome Binlist API ([https://binlist.net](https://binlist.net)).