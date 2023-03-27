pub mod activity;
pub mod component;
pub mod flags;
pub mod interaction;
pub mod reference;
pub mod role_subscription_data;
pub mod r#type;

use self::{
    activity::MessageActivity, component::MessageComponent, flags::MessageFlags,
    interaction::MessageInteraction, r#type::MessageType, reference::MessageReference,
    role_subscription_data::RoleSubscriptionData,
};
use super::{attachment::Attachment, embed::Embed, mention::Mention, reaction::Reaction, Channel};
use crate::types::{application::Application, role::Role, sticker::item::StickerItem, user::User};
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Message {
    pub id: String,
    pub channel_id: String,
    pub author: User,
    pub content: String,
    pub timestamp: String,
    pub edited_timestamp: Option<String>,
    pub tts: bool,
    pub mention_everyone: bool,
    pub mentions: Vec<User>,
    pub mention_roles: Vec<Role>,
    pub mention_channels: Option<Vec<Mention>>,
    pub attachments: Vec<Attachment>,
    pub embeds: Vec<Embed>,
    pub reactions: Option<Vec<Reaction>>,
    //nonce?	integer or string	used for validating a message was sent
    pub pinned: bool,
    pub webhook_id: Option<String>,
    pub r#type: MessageType,
    pub activity: Option<MessageActivity>,
    pub application: Option<Application>,
    pub application_id: Option<String>,
    pub message_reference: Option<MessageReference>,
    pub flags: Option<MessageFlags>,
    pub referenced_message: Option<Box<Message>>,
    pub interaction: Option<MessageInteraction>,
    pub thread: Option<Channel>,
    pub components: Option<Vec<MessageComponent>>,
    pub sticker_items: Option<Vec<StickerItem>>,
    //stickers: Option<Vec<Sticker>>, (deprecated)
    pub position: Option<u32>,
    pub role_subscription_data: Option<RoleSubscriptionData>,
}
