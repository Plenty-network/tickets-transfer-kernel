use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct Token(pub Vec<u8>);

impl Token {
    pub fn from(v: &Vec<u8>) -> Self {
        Self(v.clone())
    }

    pub fn to_hex_string(&self) -> String {
        self.0.iter().map(|b| format!("{:02x}", b)).collect()
    }
}
