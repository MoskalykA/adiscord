pub mod channel;
pub mod emoji;
pub mod guild;
pub mod sticker;
pub mod types;
pub mod voice;
pub mod webhook;

macro_rules! generate_struct {
    ($struct_name:ident) => {
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

#[cfg(feature = "gateway")]
use tungstenite::connect;

#[cfg(feature = "gateway")]
use url::Url;

#[cfg(feature = "gateway")]
pub async fn test() {
    use std::time::Duration;

    use crate::types::gateway::Gateway;
    use tokio::time::interval;
    use tungstenite::Message;

    let (mut socket, response) =
        connect(Url::parse("wss://gateway.discord.gg/?v=10&encoding=json").unwrap())
            .expect("Can't connect");

    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());
    println!("Response contains the following headers:");

    loop {
        let msg = socket.read_message().expect("Error reading message");
        let gateway = serde_json::from_str::<Gateway>(&msg.to_string()).unwrap();
        match gateway.op {
            types::gateway::opcode::GatewayOpcode::Dispatch => {
                println!("Dispatch")
            }
            types::gateway::opcode::GatewayOpcode::Heartbeat => {
                println!("Heartbeat");
            }
            types::gateway::opcode::GatewayOpcode::Identify => {
                println!("Identify")
            }
            types::gateway::opcode::GatewayOpcode::PresenceUpdate => {
                println!("Presence Update")
            }
            types::gateway::opcode::GatewayOpcode::VoiceStateUpdate => {
                println!("Voice State Update")
            }
            types::gateway::opcode::GatewayOpcode::Resume => {
                println!("Resume")
            }
            types::gateway::opcode::GatewayOpcode::Reconnect => {
                println!("Reconnect")
            }
            types::gateway::opcode::GatewayOpcode::RequestGuildMembers => {
                println!("Request Guild Members")
            }
            types::gateway::opcode::GatewayOpcode::InvalidSession => {
                println!("Invalid Session")
            }
            types::gateway::opcode::GatewayOpcode::Hello => {
                println!("Hello");

                tokio::spawn(async move {
                    let mut interval = interval(Duration::from_secs(10));
                    loop {
                        interval.tick().await;

                        socket
                            .write_message(Message::Text(
                                r#"{
                                    "op": 1,
                                    "d": 251
                                }"#
                                .into(),
                            ))
                            .unwrap();
                    }
                });

                println!("Sent");
            }
            types::gateway::opcode::GatewayOpcode::HeartbeatAck => {
                println!("Heartbeat Ack")
            }
        };

        println!("Received");
    }
}
