pub mod channel;
pub mod emoji;
pub mod guild;
pub mod sticker;
pub mod types;
pub mod voice;
pub mod webhook;

use channel::Channel;
use emoji::Emoji;
use guild::Guild;
use serde_derive::Deserialize;
use sticker::Sticker;
use voice::Voice;
use webhook::Webhook;

const BASE_URL: &str = "https://discord.com/api/v";

pub struct Client {
    pub emoji: Emoji,
    pub guild: Guild,
    pub sticker: Sticker,
    pub channel: Channel,
    pub webhook: Webhook,
    pub voice: Voice,
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
            sticker: Sticker::new(url.clone(), client.clone(), token.clone()),
            webhook: Webhook::new(url.clone(), client.clone(), token.clone()),
            voice: Voice::new(url.clone(), client.clone(), token.clone()),
            channel: Channel::new(url, client, token),
        }
    }
}
