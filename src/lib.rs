pub mod channel;
pub mod emoji;
pub mod guild;
pub mod types;

use channel::Channel;
use emoji::Emoji;
use guild::Guild;
use serde_derive::Deserialize;

const BASE_URL: &str = "https://discord.com/api/v";

pub struct Client {
    pub emoji: Emoji,
    pub guild: Guild,
    pub channel: Channel,
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
    /// # Examples
    ///
    /// ```
    /// use adiscord::Client;
    /// use dotenv_codegen::dotenv;
    /// 
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::new("10", dotenv!("TOKEN"), adiscord::TokenType::Bot);
    /// }    
    /// ```
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
            emoji: Emoji::new(url.clone(), client.clone(), token.clone()),
            guild: Guild::new(url.clone(), client.clone(), token.clone()),
            channel: Channel::new(url, client, token),
        }
    }
}
