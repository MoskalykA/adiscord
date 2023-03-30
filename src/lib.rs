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

#[cfg(feature = "gateway")]
generate_struct!(Gateway);

use serde::Deserialize;

const BASE_URL: &str = "https://discord.com/api/v";

#[derive(Clone)]
pub struct Client {
    pub emoji: Emoji,
    pub guild: Guild,
    pub sticker: Sticker,
    pub channel: Channel,
    pub webhook: Webhook,
    pub voice: Voice,

    #[cfg(feature = "gateway")]
    pub gateway: Gateway,
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
            channel: Channel::new(url.clone(), client.clone(), token.clone()),

            #[cfg(feature = "gateway")]
            gateway: Gateway::new(url, client, token),
        }
    }
}

#[cfg(feature = "gateway")]
pub async fn test(client: &Client) {
    use crate::types::gateway::{
        identify::{connection::GatewayIdentifyConnection, GatewayIdentify},
        Gateway,
    };
    use rand::Rng;
    use std::time::Duration;
    use tokio::time::interval;
    use tungstenite::connect;
    use url::Url;

    let (socket, _) = connect(Url::parse("wss://gateway.discord.gg/?v=10&encoding=json").unwrap())
        .expect("Can't connect");

    let mut heartbeat_count = 0;
    let gateway_arc = std::sync::Arc::new(client.gateway.clone());
    let socket_arc = std::sync::Arc::new(std::sync::Mutex::new(socket));

    loop {
        let msg = socket_arc
            .lock()
            .unwrap()
            .read_message()
            .expect("Error reading message");

        let gateway = serde_json::from_str::<Gateway>(&msg.to_string()).unwrap();
        match gateway.op {
            types::gateway::opcode::GatewayOpcode::Dispatch => {
                println!("Dispatch");
                println!("{:?}", gateway.d);
            }
            types::gateway::opcode::GatewayOpcode::Heartbeat => {
                println!("Heartbeat");
            }
            types::gateway::opcode::GatewayOpcode::Identify => {
                println!("Identify");
            }
            types::gateway::opcode::GatewayOpcode::PresenceUpdate => {
                println!("Presence Update");
            }
            types::gateway::opcode::GatewayOpcode::VoiceStateUpdate => {
                println!("Voice State Update");
            }
            types::gateway::opcode::GatewayOpcode::Resume => {
                println!("Resume");
            }
            types::gateway::opcode::GatewayOpcode::Reconnect => {
                println!("Reconnect");
            }
            types::gateway::opcode::GatewayOpcode::RequestGuildMembers => {
                println!("Request Guild Members");
            }
            types::gateway::opcode::GatewayOpcode::InvalidSession => {
                println!("Invalid Session");
            }
            types::gateway::opcode::GatewayOpcode::Hello => {
                let heartbeat_interval = gateway
                    .d
                    .expect("Heartbeat interval is required")
                    .get("heartbeat_interval")
                    .expect("Heartbeat interval is required")
                    .as_f64()
                    .expect("Heartbeat interval is required");

                let mut rng = rand::thread_rng();
                let jitter: f64 = rng.gen();
                tokio::time::sleep(Duration::from_millis((heartbeat_interval * jitter) as u64))
                    .await;

                client.gateway.send_heartbeat(&socket_arc).await;

                let duration = Duration::from_millis(heartbeat_interval as u64);
                let gateway_arc_clone = std::sync::Arc::clone(&gateway_arc);
                let socket_arc_clone = std::sync::Arc::clone(&socket_arc);

                tokio::spawn(async move {
                    let mut interval = interval(duration);
                    loop {
                        interval.tick().await;

                        gateway_arc_clone.send_heartbeat(&socket_arc_clone).await;
                    }
                });
            }
            types::gateway::opcode::GatewayOpcode::HeartbeatAck => {
                println!("Heartbeat Ack");

                heartbeat_count += 1;
                if heartbeat_count == 2 {
                    let socket_arc_clone = std::sync::Arc::clone(&socket_arc);
                    client
                        .gateway
                        .identify(
                            socket_arc_clone,
                            GatewayIdentify {
                                token: client.gateway.token.clone(),
                                properties: GatewayIdentifyConnection {
                                    os: "windows".to_owned(),
                                    browser: "adiscord".to_owned(),
                                    device: "adiscord".to_owned(),
                                },
                                compress: None,
                                large_threshold: None,
                                shard: None,
                                presence: None,
                                intents: 0,
                            },
                        )
                        .await;
                }
            }
        };
    }
}
