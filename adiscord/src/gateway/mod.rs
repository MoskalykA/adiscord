pub mod types;

use self::types::Callback;
use crate::Client;
use adiscord_intents::{generate_intent_number, Intent};
use adiscord_types::api::audit::LogEntry;
use adiscord_types::api::auto_moderation;
use adiscord_types::api::guild::Guild;
use adiscord_types::api::voice::VoiceState;
use adiscord_types::api::{channel::Channel, message::Message};
use adiscord_types::gateway::guild::{BanAdd, BanRemove, Scheduled, Unavailable};
use adiscord_types::gateway::voice::ServerUpdate;
use adiscord_types::gateway::{
    channel, guild, identify, opcode::Opcode, ready::Ready, role, thread, webhook, Gateway,
};
use adiscord_types::gateway::{
    emoji, integration, invite, member, message, presence, reaction, scheduled, sticker, typing,
};
use async_trait::async_trait;
use ezsockets::ClientConfig;
use serde::Deserialize;
use serde_json::to_string;
use serde_json::to_value;
use std::collections::HashMap;
use std::io::BufRead;
use std::sync::Arc;
use std::time::Duration;
use tokio::time;
use url::Url;

const GATEWAY_URL: &str = "wss://gateway.discord.gg/?v=10&encoding=json";

#[derive(Debug)]
enum Call {
    NewLine(String),
}

struct GatewayClient {
    token: String,
    handle: ezsockets::Client<Self>,
    intents: u32,
    callbacks: HashMap<String, Callback>,
    heartbeat_ack: bool,
    heartbeat_count: u32,
    heartbeat_ack_count: bool,
}

impl GatewayClient {
    fn new(
        token: String,
        handle: ezsockets::Client<Self>,
        intents: u32,
        callbacks: HashMap<String, Callback>,
        heartbeat_ack: bool,
        heartbeat_count: u32,
        heartbeat_ack_count: bool,
    ) -> Self {
        Self {
            token,
            handle,
            intents,
            callbacks,
            heartbeat_ack,
            heartbeat_count,
            heartbeat_ack_count,
        }
    }
}

#[derive(Deserialize)]
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
            Opcode::Dispatch => {
                let callback = self.callbacks.get(&gateway.t.unwrap());
                if let Some(callback) = callback {
                    callback(gateway.d.unwrap());
                }
            }
            Opcode::Heartbeat => {
                println!("Heartbeat");
            }
            Opcode::Identify => {
                println!("Identify");
            }
            Opcode::PresenceUpdate => {
                println!("Presence Update");
            }
            Opcode::VoiceStateUpdate => {
                println!("Voice State Update");
            }
            Opcode::Resume => {
                println!("Resume");
            }
            Opcode::Reconnect => {
                println!("Reconnect");
            }
            Opcode::RequestGuildMembers => {
                println!("Request Guild Members");
            }
            Opcode::InvalidSession => {
                println!("Invalid Session");
            }
            Opcode::Hello => {
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
            Opcode::HeartbeatAck => {
                self.heartbeat_count += 1;

                if self.heartbeat_ack {
                    if self.heartbeat_ack_count {
                        println!("Heartbeat Ack -> {}", self.heartbeat_count);
                    } else {
                        println!("Heartbeat Ack");
                    }
                }

                if self.heartbeat_count == 2 {
                    let data = identify::Identify {
                        token: self.token.clone(),
                        properties: identify::IdentifyConnection {
                            os: os_info::get().os_type().to_string().to_lowercase(),
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
                        op: Opcode::Identify,
                        d: Some(to_value(data).unwrap()),
                        s: None,
                        t: None,
                    };

                    self.handle.text(to_string(&identify).unwrap());
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
            self.gateway.callbacks.insert(
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
    /// use dotenvy_macro::dotenv;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut client = Client::new(dotenv!("TOKEN"));
    ///     client.add_intent(Intent::MessageContent);
    /// }
    /// ```
    pub fn add_intent(&mut self, intent: Intent) {
        self.gateway.intents.push(intent);
    }

    /// # Add intents
    ///
    /// This does the same thing as the `add_intent` function, but this function takes a list of enums and then adds them.
    ///
    /// ## Examples
    ///
    /// ```
    /// use adiscord::Client;
    /// use dotenvy_macro::dotenv;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut client = Client::new(dotenv!("TOKEN"));
    ///     client.add_intents(vec![ Intent::MessageContent ]);
    /// }
    /// ```
    pub fn add_intents(&mut self, intents: Vec<Intent>) {
        for intent in intents {
            self.gateway.intents.push(intent);
        }
    }

    /// # Add all intents
    ///
    /// This function will simply add all the available intents.
    ///
    /// ## Examples
    ///
    /// ```
    /// use adiscord::Client;
    /// use dotenvy_macro::dotenv;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut client = Client::new(dotenv!("TOKEN"));
    ///     client.all_intents();
    /// }
    /// ```
    pub fn all_intents(&mut self) {
        let all = vec![
            Intent::Guilds,
            Intent::GuildMembers,
            Intent::GuildModeration,
            Intent::GuildEmojisAndStickers,
            Intent::GuildIntegrations,
            Intent::GuildWebhooks,
            Intent::GuildInvites,
            Intent::GuildVoiceStates,
            Intent::GuildPresences,
            Intent::GuildMessages,
            Intent::GuildMessageReactions,
            Intent::GuildMessageTyping,
            Intent::DirectMessages,
            Intent::DirectMessageReactions,
            Intent::DirectMessageTyping,
            Intent::MessageContent,
            Intent::GuildScheduledEvents,
            Intent::AutoModerationConfiguration,
            Intent::AutoModerationExecution,
        ];

        for intent in all {
            self.gateway.intents.push(intent);
        }
    }

    generate_event!(on_ready, "READY", Ready);

    generate_event!(
        on_auto_moderation_rule_create,
        "AUTO_MODERATION_RULE_CREATE",
        auto_moderation::Rule
    );

    generate_event!(
        on_auto_moderation_rule_update,
        "AUTO_MODERATION_RULE_UPDATE",
        auto_moderation::Rule
    );

    generate_event!(
        on_auto_moderation_rule_delete,
        "AUTO_MODERATION_RULE_DELETE",
        auto_moderation::Rule
    );

    generate_event!(
        on_auto_moderation_action_execution,
        "AUTO_MODERATION_ACTION_EXECUTION",
        auto_moderation::Execution
    );

    generate_event!(on_channel_create, "CHANNEL_CREATE", Channel);

    generate_event!(on_channel_update, "CHANNEL_UPDATE", Channel);

    generate_event!(on_channel_delete, "CHANNEL_DELETE", Channel);

    generate_event!(
        on_channel_pins_update,
        "CHANNEL_PINS_UPDATE",
        channel::PinsUpdate
    );

    generate_event!(on_thread_create, "THREAD_CREATE", Channel);

    generate_event!(on_thread_update, "THREAD_UPDATE", Channel);

    generate_event!(on_thread_delete, "THREAD_DELETE", thread::Delete);

    generate_event!(on_thread_list_sync, "THREAD_LIST_SYNC", thread::ListSync);

    generate_event!(
        on_thread_member_update,
        "THREAD_MEMBER_UPDATE",
        thread::MemberUpdate
    );

    generate_event!(
        on_thread_members_update,
        "THREAD_MEMBERS_UPDATE",
        thread::MembersUpdate
    );

    generate_event!(on_guild_create, "GUILD_CREATE", guild::Create);

    generate_event!(on_guild_update, "GUILD_UPDATE", Guild);

    generate_event!(on_guild_delete, "GUILD_DELETE", Unavailable);

    generate_event!(
        on_guild_audit_log_entry_create,
        "GUILD_AUDIT_LOG_ENTRY_CREATE",
        LogEntry
    );
    generate_event!(on_guild_ban_add, "GUILD_BAN_ADD", BanAdd);

    generate_event!(on_guild_ban_remove, "GUILD_BAN_REMOVE", BanRemove);

    generate_event!(on_emojis_update, "GUILD_EMOJIS_UPDATE", emoji::Update);

    generate_event!(on_stickers_update, "GUILD_STICKERS_UPDATE", sticker::Update);

    generate_event!(
        on_integrations_update,
        "GUILD_INTEGRATIONS_UPDATE",
        integration::IntegrationsUpdate
    );

    generate_event!(om_member_add, "GUILD_MEMBER_ADD", member::Add);

    generate_event!(on_member_remove, "GUILD_MEMBER_REMOVE", member::Remove);

    generate_event!(on_member_update, "GUILD_MEMBER_UPDATE", member::Update);

    generate_event!(on_role_create, "GUILD_ROLE_CREATE", role::Create);

    generate_event!(on_role_update, "GUILD_ROLE_UPDATE", role::Update);

    generate_event!(on_role_delete, "GUILD_ROLE_DELETE", role::Delete);

    generate_event!(
        on_scheduled_event_create,
        "GUILD_SCHEDULED_EVENT_CREATE",
        Scheduled
    );

    generate_event!(
        on_scheduled_event_update,
        "GUILD_SCHEDULED_EVENT_UPDATE",
        Scheduled
    );

    generate_event!(
        on_scheduled_event_delete,
        "GUILD_SCHEDULED_EVENT_DELETE",
        Scheduled
    );

    generate_event!(
        on_scheduled_event_user_add,
        "GUILD_SCHEDULED_EVENT_USER_ADD",
        scheduled::UserAdd
    );

    generate_event!(
        on_scheduled_event_user_remove,
        "GUILD_SCHEDULED_EVENT_USER_REMOVE",
        scheduled::UserRemove
    );

    generate_event!(
        on_integration_create,
        "INTEGRATION_CREATE",
        integration::Create
    );
    generate_event!(
        on_integration_update,
        "INTEGRATION_UPDATE",
        integration::Update
    );
    generate_event!(
        on_integration_delete,
        "INTEGRATION_DELETE",
        integration::Delete
    );

    generate_event!(on_invite_update, "INVITE_CREATE", invite::Create);

    generate_event!(on_invite_delete, "INVITE_DELETE", invite::Delete);

    generate_event!(on_message, "MESSAGE_CREATE", message::Create);

    generate_event!(on_message_update, "MESSAGE_UPDATE", Message);

    generate_event!(on_message_delete, "MESSAGE_DELETE", message::Delete);

    generate_event!(
        on_message_delete_bulk,
        "MESSAGE_DELETE_BULK",
        message::DeleteBulk
    );

    generate_event!(on_reaction_add, "MESSAGE_REACTION_ADD", reaction::Add);

    generate_event!(
        on_reaction_remove,
        "MESSAGE_REACTION_REMOVE",
        reaction::Remove
    );
    generate_event!(
        on_reaction_remove_all,
        "MESSAGE_REACTION_REMOVE_ALL",
        reaction::RemoveAll
    );
    generate_event!(
        on_reaction_remove_emoji,
        "MESSAGE_REACTION_REMOVE_EMOJI",
        reaction::RemoveEmoji
    );

    generate_event!(on_presence_update, "PRESENCE_UPDATE", presence::Update);

    generate_event!(on_typing_start, "TYPING_START", typing::Start);

    generate_event!(on_voice_state_update, "VOICE_STATE_UPDATE", VoiceState);

    generate_event!(on_voice_server_update, "VOICE_SERVER_UPDATE", ServerUpdate);

    generate_event!(on_webhooks_update, "WEBHOOKS_UPDATE", webhook::Update);

    /// # Create a connection to Discord
    ///
    /// This will simply create the connection to Discord.
    ///
    /// ## Examples
    ///
    /// ```
    /// use adiscord::Client;
    /// use dotenvy_macro::dotenv;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::new(dotenv!("TOKEN"));
    ///     client.init().await;
    /// }
    /// ```
    pub async fn init(self) {
        let url = Url::parse(GATEWAY_URL).unwrap();
        let config = ClientConfig::new(url);
        let (handle, future) = ezsockets::connect(
            |handle| {
                GatewayClient::new(
                    self.token,
                    handle,
                    generate_intent_number(self.gateway.intents),
                    self.gateway.callbacks,
                    self.gateway.heartbeat_ack,
                    0,
                    self.gateway.heartbeat_ack_count,
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
    /// use dotenvy_macro::dotenv;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut client = Client::new(dotenv!("TOKEN"));
    ///     client.set_heartbeat_ack(true);
    /// }
    /// ```
    pub fn set_heartbeat_ack(&mut self, state: bool) {
        self.gateway.heartbeat_ack = state;
    }

    /// # "Heartbeat counter
    ///
    /// This function counts the number of heartbeats.
    ///
    /// ## Examples
    ///
    /// ```
    /// use adiscord::Client;
    /// use dotenvy_macro::dotenv;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut client = Client::new(dotenv!("TOKEN"));
    ///     client.set_heartbeat_ack_count(true);
    /// }
    /// ```
    pub fn set_heartbeat_ack_count(&mut self, state: bool) {
        self.gateway.heartbeat_ack_count = state;
    }
}
