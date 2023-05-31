pub mod channel;
pub mod emoji;
pub mod guild;
pub mod sticker;
pub mod types;
pub mod voice;
pub mod webhook;

#[cfg(feature = "gateway")]
pub mod gateway;

macro_rules! generate_struct {
    ($struct_name:ident) => {
        #[derive(Clone)]
        pub struct $struct_name {
            pub url: String,
            pub client: reqwest::Client,
            pub token: String,
        }

        impl $struct_name {
            pub fn new(url: String, client: reqwest::Client, token: String) -> Self {
                Self { url, client, token }
            }
        }
    };
}

generate_struct!(Channel);
generate_struct!(Emoji);
generate_struct!(Guild);
generate_struct!(Sticker);
generate_struct!(Voice);
generate_struct!(Webhook);

use serde::Deserialize;

#[cfg(feature = "gateway")]
use adiscord_intents::Intent;

#[cfg(feature = "gateway")]
use std::collections::HashMap;

#[cfg(feature = "gateway")]
use crate::gateway::Callback;

const BASE_URL: &str = "https://discord.com/api/v";

pub struct Client {
    pub token: String,

    #[cfg(feature = "gateway")]
    pub intents: Vec<Intent>,

    #[cfg(feature = "gateway")]
    pub callbacks: HashMap<String, Callback>,

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
            token: token.clone(),

            #[cfg(feature = "gateway")]
            intents: Vec::new(),

            #[cfg(feature = "gateway")]
            callbacks: HashMap::new(),

            emoji: Emoji::new(url.clone(), client.clone(), token.clone()),
            guild: Guild::new(url.clone(), client.clone(), token.clone()),
            sticker: Sticker::new(url.clone(), client.clone(), token.clone()),
            webhook: Webhook::new(url.clone(), client.clone(), token.clone()),
            voice: Voice::new(url.clone(), client.clone(), token.clone()),
            channel: Channel::new(url, client, token),
        }
    }
}
