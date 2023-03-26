pub mod format;
pub mod r#type;

use self::{format::StickerFormatType, r#type::StickerType};
use super::user::User;
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Sticker {
    pub id: String,
    pub pack_id: Option<String>,
    pub name: String,
    pub description: String,
    pub tags: String,
    pub asset: Option<String>,

    #[serde(rename = "type")]
    pub alias: StickerType,

    pub format_type: StickerFormatType,
    pub available: Option<bool>,
    pub guild_id: Option<String>,
    pub user: Option<User>,
    pub sort_value: Option<i128>,
}
