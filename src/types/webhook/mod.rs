pub mod r#type;

use self::r#type::WebhookType;
use super::{channel::Channel, guild::Guild, user::User};
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Webhook {
    pub id: String,
    pub r#type: WebhookType,
    pub guild_id: Option<String>,
    pub channel_id: Option<String>,
    pub user: Option<User>,
    pub name: Option<String>,
    pub avatar: Option<String>,
    pub token: Option<String>,
    pub application_id: Option<String>,
    pub source_guild: Option<Guild>,
    pub source_channel: Option<Channel>,
    pub url: Option<String>,
}
