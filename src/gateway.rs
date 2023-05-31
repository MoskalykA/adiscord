use crate::gateway::GatewayOpcode::Identify;
use crate::types::channel::message::Message;
use crate::types::gateway::identify::connection::GatewayIdentifyConnection;
use crate::types::gateway::identify::GatewayIdentify;
use crate::types::gateway::opcode::GatewayOpcode;
use crate::types::gateway::Gateway;
use crate::Client;
use async_trait::async_trait;
use ezsockets::ClientConfig;
use serde_json::to_string;
use serde_json::to_value;
use serde_json::Value;
use std::collections::HashMap;
use std::io::BufRead;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::interval;
use tokio::time::sleep;
use url::Url;

const GATEWAY_URL: &str = "wss://gateway.discord.gg/?v=10&encoding=json";
const HEARTBEAT_MULTIPLIER: f64 = 0.1;

pub type Callback = Arc<dyn Fn(Value) + Send + Sync>;

#[derive(Debug)]
enum Call {
    NewLine(String),
}

struct GatewayClient {
    token: String,
    handle: ezsockets::Client<Self>,
    callbacks: HashMap<String, Callback>,
    identify: bool,
    heartbeat_count: u8,
}

impl GatewayClient {
    fn new(
        token: String,
        handle: ezsockets::Client<Self>,
        callbacks: HashMap<String, Callback>,
        identify: bool,
        heartbeat_count: u8,
    ) -> Self {
        Self {
            token,
            handle,
            callbacks,
            identify,
            heartbeat_count,
        }
    }
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
                let heartbeat_interval = gateway
                    .d
                    .expect("Heartbeat interval is required")
                    .get("heartbeat_interval")
                    .expect("Heartbeat interval is required")
                    .as_f64()
                    .expect("Heartbeat interval is required");

                let handle = std::sync::Arc::new(self.handle.clone());
                let duration = Duration::from_millis(heartbeat_interval as u64);
                tokio::spawn(async move {
                    sleep(Duration::from_millis(
                        (heartbeat_interval * HEARTBEAT_MULTIPLIER) as u64,
                    ))
                    .await;

                    handle.text(
                        r#"{
                        "op": 1,
                        "d": null
                    }"#
                        .into(),
                    );

                    let mut interval = interval(duration);
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
                println!("Heartbeat Ack");

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
                            intents: 513,
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

impl Client {
    pub fn on_message(&mut self, callback: fn(Message)) {
        self.callbacks.insert(
            "MESSAGE_CREATE".to_owned(),
            Arc::new(move |value| {
                let message: Message = serde_json::from_value(value).unwrap();
                callback(message);
            }),
        );
    }

    pub async fn init(self) {
        tracing_subscriber::fmt::init();

        let url = Url::parse(GATEWAY_URL).unwrap();
        let config = ClientConfig::new(url);
        let (handle, future) = ezsockets::connect(
            |handle| GatewayClient::new(self.token, handle, self.callbacks, false, 0),
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
}
