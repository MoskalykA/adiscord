use super::{channel::Channel, guild::Guild, user::User};
use crate::Snowflake;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[repr(u8)]
#[derive(Deserialize_repr, Serialize_repr, Debug)]
pub enum Type {
    /// Incoming Webhooks can post messages to channels with a generated token
    Incoming = 1,

    /// Channel Follower Webhooks are internal webhooks used with Channel Following to post new messages into channels
    Channel,

    /// Application webhooks are webhooks used with Interactions
    Application,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Webhook {
    /// the id of the webhook
    pub id: Snowflake,

    /// the type of the webhook
    pub r#type: Type,

    /// the guild id this webhook is for, if any
    pub guild_id: Option<Snowflake>,

    /// the channel id this webhook is for, if any
    pub channel_id: Option<Snowflake>,

    /// the user this webhook was created by (not returned when getting a webhook with its token)
    pub user: Option<User>,

    /// the default name of the webhook
    pub name: Option<String>,

    /// the default user avatar hash of the webhook
    pub avatar: Option<String>,

    /// the secure token of the webhook (returned for Incoming Webhooks)
    pub token: Option<String>,

    /// the bot/OAuth2 application that created this webhook
    pub application_id: Option<Snowflake>,

    /// the guild of the channel that this webhook is following (returned for Channel Follower Webhooks)
    pub source_guild: Option<Guild>,

    /// the channel that this webhook is following (returned for Channel Follower Webhooks)
    pub source_channel: Option<Channel>,

    /// the url used for executing the webhook (returned by the webhooks OAuth2 flow)
    pub url: Option<String>,
}
