pub mod r#type;

use self::r#type::WebhookType;
use super::{channel::Channel, guild::Guild, user::User};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Webhook {
    /// the id of the webhook
    pub id: String,

    /// the type of the webhook
    pub r#type: WebhookType,

    /// the guild id this webhook is for, if any
    pub guild_id: Option<String>,

    /// the channel id this webhook is for, if any
    pub channel_id: Option<String>,

    /// the user this webhook was created by (not returned when getting a webhook with its token)
    pub user: Option<User>,

    /// the default name of the webhook
    pub name: Option<String>,

    /// the default user avatar hash of the webhook
    pub avatar: Option<String>,

    /// the secure token of the webhook (returned for Incoming Webhooks)
    pub token: Option<String>,

    /// the bot/OAuth2 application that created this webhook
    pub application_id: Option<String>,

    /// the guild of the channel that this webhook is following (returned for Channel Follower Webhooks)
    pub source_guild: Option<Guild>,

    /// the channel that this webhook is following (returned for Channel Follower Webhooks)
    pub source_channel: Option<Channel>,

    /// the url used for executing the webhook (returned by the webhooks OAuth2 flow)
    pub url: Option<String>,
}
