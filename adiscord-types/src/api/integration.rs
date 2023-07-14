use crate::Snowflake;
use super::{application::Application, user::User};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[repr(u8)]
#[allow(non_camel_case_types)]
#[derive(Deserialize_repr, Serialize_repr, Debug)]
pub enum ExpireBehavior {
    RemoveRole = 0,
    Kick,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Account {
    /// id of the account
    id: Snowflake,

    /// name of the account
    name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Integration {
    /// integration id
    pub id: Snowflake,

    /// integration name
    pub name: String,

    /// integration type (twitch, youtube, discord, or guild_subscription)
    pub r#type: String,

    /// is this integration enabled
    pub enabled: bool,

    /// is this integration syncing
    pub syncing: Option<bool>,

    /// id that this integration uses for "subscribers"
    pub role_id: Option<Snowflake>,

    /// whether emoticons should be synced for this integration (twitch only currently)
    pub enable_emoticons: Option<bool>,

    /// the behavior of expiring subscribers
    pub expire_behavior: Option<ExpireBehavior>,

    /// the grace period (in days) before expiring subscribers
    pub expire_grace_period: Option<u8>,

    /// user for this integration
    pub user: Option<User>,

    /// integration account information
    pub account: Account,

    /// when this integration was last synced
    pub synced_at: Option<String>,

    /// how many subscribers this integration has
    pub subscriber_count: Option<u16>,

    /// has this integration been revoked
    pub revoked: Option<bool>,

    /// The bot/OAuth2 application for discord integrations
    pub application: Option<Application>,
    // todo: scopes, https://discord.com/developers/docs/topics/oauth2#shared-resources-oauth2-scopes
}
