use crate::models::BinInformation;
use crate::errors::{LookupError, RateLimitExceeded};
use reqwest::{Client, Proxy};

pub struct BinLookup {
    pub info: Option<BinInformation>
}

impl BinLookup {
    pub fn new() -> Self {
        Self {info: None}
    }
    pub async fn lookup(&mut self, bin: &str, proxy_url: Option<&str>) -> Result<BinInformation, LookupError> {
        let client = if let Some(proxy_str) = proxy_url {
            let proxy = Proxy::all(proxy_str)?;
            Client::builder().proxy(proxy).build()?
        } else {
            Client::new()
        };
        let url = format!("https://lookup.binlist.net/{}", bin);
        let response = client.get(&url).send().await?;
        match response.status().as_u16() {
            200 => {
                let data = response.json::<BinInformation>().await?;
                self.info = Some(data.clone());
                Ok(data)
            }
            429 => Err(RateLimitExceeded::new(429).into()),
            status => Err(LookupError::BINLookupError(bin.to_string(), status))
        }
    }
}

