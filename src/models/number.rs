use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]

pub struct Bin {
    pub length: Option<u8>,
    pub luhn: Option<bool>
}