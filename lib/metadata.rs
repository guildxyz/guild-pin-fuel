use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Action {
    Joined,
    #[serde(rename = "Admin of")]
    Admin,
    #[serde(rename = "Owner of")]
    Owner,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenUri {
    pub name: Action,
    pub description: String,
    pub image: String,
    pub attributes: [Attribute; 6],
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "trait_type", content = "value")]
#[serde(rename_all = "camelCase")]
pub enum Attribute {
    Type(Action),
    GuildId(u64),
    UserId(u64),
    Rank(u64),
    ActionDate(String), // dates are formatted as string
    MintDate(String),   // dates are formatted as string
}

impl Attribute {
    pub fn mint_date(&self) -> Option<u64> {
        if let Self::MintDate(d) = self {
            d.parse::<u64>().ok()
        } else {
            None
        }
    }
}
