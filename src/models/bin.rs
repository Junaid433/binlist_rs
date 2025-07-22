use serde::Deserialize;
use super::{Bank, Country, Bin};

#[derive(Debug, Deserialize, Clone)]
pub struct BinInformation {
    #[serde(rename = "number")]
    pub bin_data: Option<Bin>,
    pub scheme: Option<String>,
    #[serde(rename = "type")]
    pub card_type: Option<String>,
    pub brand: Option<String>,
    pub prepaid: Option<bool>,
    pub country: Option<Country>,
    pub bank: Option<Bank>,
}