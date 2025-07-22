use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Country {
    pub numeric: Option<String>,
    pub alpha2: Option<String>,
    pub name: Option<String>,
    pub emoji: Option<String>,
    pub currency: Option<String>,
    pub latitude: Option<i32>,
    pub longitude: Option<i32>
}