use crate::gateway::GatewayOpcode::Identify;
use crate::types::channel::message::Message;
use crate::types::channel::Channel;
use crate::types::gateway::identify::connection::GatewayIdentifyConnection;
use crate::types::gateway::identify::GatewayIdentify;
use crate::types::gateway::opcode::GatewayOpcode;
use crate::types::gateway::Gateway;
use crate::Client;
use adiscord_intents::generate_intent_number;
use adiscord_intents::Intent;
use async_trait::async_trait;
use ezsockets::ClientConfig;
use serde::Deserialize;
use serde_json::to_string;
use serde_json::to_value;
use serde_json::Value;
use std::collections::HashMap;
use std::io::BufRead;
use std::sync::Arc;
use std::time::Duration;
use tokio::time;
use url::Url;

const GATEWAY_URL: &str = "wss://gateway.discord.gg/?v=10&encoding=json";

/// This type is used in the gateway system for callbacks.
pub type Callback = Arc<dyn Fn(Value) + Send + Sync>;

#[derive(Debug)]
enum Call {
    NewLine(String),
}

struct GatewayClient {
    token: String,
    handle: ezsockets::Client<Self>,
    intents: u16,
    callbacks: HashMap<String, Callback>,
    identify: bool,
    heartbeat_ack: bool,
    heartbeat_count: u8,
}

impl GatewayClient {
    fn new(
        token: String,
        handle: ezsockets::Client<Self>,
        intents: u16,
        callbacks: HashMap<String, Callback>,
        identify: bool,
        heartbeat_ack: bool,
        heartbeat_count: u8,
    ) -> Self {
        Self {
            token,
            handle,
            intents,
            callbacks,
            identify,
            heartbeat_ack,
            heartbeat_count,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Heartbeat {
    #[serde(rename = "heartbeat_interval")]
    pub interval: f32,
}

#[async_trait]
impl ezsockets::ClientExt for GatewayClient {
    type Call = Call;

    async fn on_text(&mut self, text: String) -> Result<(), ezsockets::Error> {
        let gateway = serde_json::from_str::<Gateway>(&text).unwrap();
        match gateway.op {
            GatewayOpcode::Dispatch => {
                let callback = self.callbacks.get(&gateway.t.unwrap());
                if let Some(callback) = callback {
                    callback(gateway.d.unwrap());
                }
            }
            GatewayOpcode::Heartbeat => {
                println!("Heartbeat");
            }
            GatewayOpcode::Identify => {
                println!("Identify");
            }
            GatewayOpcode::PresenceUpdate => {
                println!("Presence Update");
            }
            GatewayOpcode::VoiceStateUpdate => {
                println!("Voice State Update");
            }
            GatewayOpcode::Resume => {
                println!("Resume");
            }
            GatewayOpcode::Reconnect => {
                println!("Reconnect");
            }
            GatewayOpcode::RequestGuildMembers => {
                println!("Request Guild Members");
            }
            GatewayOpcode::InvalidSession => {
                println!("Invalid Session");
            }
            GatewayOpcode::Hello => {
                let data = gateway.d.expect("Data is required");
                let heartbeat: Heartbeat = serde_json::from_value(data).unwrap();
                let interval = heartbeat.interval as u64;

                let handle = std::sync::Arc::new(self.handle.clone());
                let duration = Duration::from_millis(interval);
                tokio::spawn(async move {
                    time::sleep(Duration::from_millis(interval / 4)).await;

                    handle.text(
                        r#"{
                        "op": 1,
                        "d": null
                    }"#
                        .into(),
                    );

                    let mut interval = time::interval(duration);
                    loop {
                        interval.tick().await;

                        handle.text(
                            r#"{
                            "op": 1,
                            "d": null
                        }"#
                            .into(),
                        );
                    }
                });
            }
            GatewayOpcode::HeartbeatAck => {
                if self.heartbeat_ack {
                    println!("Heartbeat Ack");
                }

                if !self.identify {
                    self.heartbeat_count += 1;

                    if self.heartbeat_count == 2 {
                        let data = GatewayIdentify {
                            token: self.token.clone(),
                            properties: GatewayIdentifyConnection {
                                os: "windows".to_owned(),
                                browser: "adiscord".to_owned(),
                                device: "adiscord".to_owned(),
                            },
                            compress: None,
                            large_threshold: None,
                            shard: None,
                            presence: None,
                            intents: self.intents,
                        };

                        let identify = Gateway {
                            op: Identify,
                            d: Some(to_value(data).unwrap()),
                            s: None,
                            t: None,
                        };

                        self.handle.text(to_string(&identify).unwrap());
                        self.identify = true;
                    }
                }
            }
        };

        Ok(())
    }

    async fn on_binary(&mut self, _: Vec<u8>) -> Result<(), ezsockets::Error> {
        Ok(())
    }

    async fn on_call(&mut self, _: Self::Call) -> Result<(), ezsockets::Error> {
        Ok(())
    }
}

macro_rules! generate_event {
    ($function_name:ident, $event_name:literal, $type:ty) => {
        pub fn $function_name(&mut self, callback: fn($type)) {
            self.callbacks.insert(
                $event_name.to_owned(),
                Arc::new(move |value| {
                    let data: $type = serde_json::from_value(value).unwrap();
                    callback(data);
                }),
            );
        }
    };
}

impl Client {
    /// # Add an intent
    /// 
    /// This will add an enum to a list of enums and later transform this set into a number, allowing you to define which events are visible and which are not.
    /// 
    /// ## Examples
    ///
    /// ```
    /// use adiscord::Client;
    /// use dotenv_codegen::dotenv;
    /// 
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut client = Client::new("10", dotenv!("TOKEN"), adiscord::TokenType::Bot);
    ///     client.add_intent(Intent::MessageContent);
    /// }
    /// ```
    pub fn add_intent(&mut self, intent: Intent) {
        self.intents.push(intent);
    }

    /// # Add intents
    /// 
    /// This does the same thing as the `add_intent` function, but this function takes a list of enums and then adds them.
    /// 
    /// ## Examples
    ///
    /// ```
    /// use adiscord::Client;
    /// use dotenv_codegen::dotenv;
    /// 
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut client = Client::new("10", dotenv!("TOKEN"), adiscord::TokenType::Bot);
    ///     client.add_intents(vec![ Intent::MessageContent ]);
    /// }
    /// ```
    pub fn add_intents(&mut self, intents: Vec<Intent>) {
        for intent in intents {
            self.intents.push(intent);
        }
    }

    generate_event!(on_message, "MESSAGE_CREATE", Message);
    generate_event!(on_message_update, "MESSAGE_UPDATE", Message);

    generate_event!(on_channel_create, "CHANNEL_CREATE", Channel);
    generate_event!(on_channel_update, "CHANNEL_UPDATE", Channel);
    generate_event!(on_channel_delete, "CHANNEL_DELETE", Channel);

    generate_event!(on_thread_create, "THREAD_CREATE", Channel);
    generate_event!(on_thread_update, "THREAD_UPDATE", Channel);
    generate_event!(on_thread_delete, "THREAD_DELETE", Channel);

    /// # Create a connection to Discord
    /// 
    /// This will simply create the connection to Discord.
    /// 
    /// ## Examples
    ///
    /// ```
    /// use adiscord::Client;
    /// use dotenv_codegen::dotenv;
    /// 
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::new("10", dotenv!("TOKEN"), adiscord::TokenType::Bot);
    ///     client.init().await;
    /// }
    /// ```
    pub async fn init(self) {
        tracing_subscriber::fmt::init();

        let url = Url::parse(GATEWAY_URL).unwrap();
        let config = ClientConfig::new(url);
        let (handle, future) = ezsockets::connect(
            |handle| {
                GatewayClient::new(
                    self.token,
                    handle,
                    generate_intent_number(self.intents),
                    self.callbacks,
                    false,
                    self.heartbeat_ack,
                    0,
                )
            },
            config,
        )
        .await;

        tokio::spawn(async move {
            let stdin = std::io::stdin();
            let lines = stdin.lock().lines();
            for line in lines {
                let line = line.unwrap();
                handle.call(Call::NewLine(line));
            }
        });

        future.await.unwrap();
    }

    /// # "Heartbeat Ack" message or not
    /// 
    /// This will allow you to choose whether or not to receive the little "Heartbeat Ack" message when Discord responds to a heartbeat.
    /// 
    /// ## Examples
    ///
    /// ```
    /// use adiscord::Client;
    /// use dotenv_codegen::dotenv;
    /// 
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut client = Client::new("10", dotenv!("TOKEN"), adiscord::TokenType::Bot);
    ///     client.set_heartbeat_ack(true);
    /// }
    /// ```
    pub fn set_heartbeat_ack(&mut self, state: bool) {
        self.heartbeat_ack = state;
    }
}
