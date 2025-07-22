use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Bank {
    pub name: Option<String>
}