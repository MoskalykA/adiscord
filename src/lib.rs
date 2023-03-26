use guilds::Guilds;
use serde_derive::Deserialize;

pub mod guilds;
pub mod types;

const BASE_URL: &str = "https://discord.com/api/v";

pub struct Client {
    pub guilds: Guilds,
}

#[derive(Deserialize, Debug)]
pub struct Error {
    pub code: u16,
    pub message: String,
}

#[derive(PartialEq)]
pub enum TokenType {
    Bot,
    Bearer,
}

impl Client {
    pub fn new(version: &str, token: &str, token_type: TokenType) -> Self {
        let url = format!("{BASE_URL}{version}");
        let client = reqwest::Client::new();
        let token = format!(
            "{} {token}",
            if token_type == TokenType::Bot {
                "Bot"
            } else {
                "Bearer"
            }
        );

        Self {
            guilds: Guilds::new(url, client, token),
        }
    }
}
