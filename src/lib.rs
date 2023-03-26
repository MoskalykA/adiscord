pub mod channels;
pub mod emojis;
pub mod guilds;
pub mod types;

use channels::Channels;
use emojis::Emojis;
use guilds::Guilds;
use serde_derive::Deserialize;

const BASE_URL: &str = "https://discord.com/api/v";

pub struct Client {
    pub emojis: Emojis,
    pub guilds: Guilds,
    pub channels: Channels,
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
            emojis: Emojis::new(url.clone(), client.clone(), token.clone()),
            guilds: Guilds::new(url.clone(), client.clone(), token.clone()),
            channels: Channels::new(url, client, token),
        }
    }
}
