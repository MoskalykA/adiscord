use super::user::User;
use serde::{Deserialize, Serialize};

#[repr(u8)]
#[derive(Deserialize, Serialize, Debug)]
pub enum FormatType {
    PNG = 1,
    APNG,
    Lottie,
    GIF,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Item {
    /// id of the sticker
    pub id: String,

    /// name of the sticker
    pub name: String,

    /// type of sticker format
    pub format_type: FormatType,
}

#[repr(u8)]
#[derive(Deserialize, Serialize, Debug)]
pub enum Type {
    /// an official sticker in a pack, part of Nitro or in a removed purchasable pack
    Standard = 1,

    /// a sticker uploaded to a guild for the guild's members
    Guild,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Sticker {
    /// id of the sticker
    pub id: String,

    /// for standard stickers, id of the pack the sticker is from
    pub pack_id: Option<String>,

    /// name of the sticker
    pub name: String,

    /// description of the sticker
    pub description: String,

    /// autocomplete/suggestion tags for the sticker (max 200 characters)
    pub tags: String,

    /// Deprecated previously the sticker asset hash, now an empty string
    #[deprecated]
    pub asset: Option<String>,

    /// type of sticker
    pub r#type: Type,

    /// type of sticker format
    pub format_type: FormatType,

    /// whether this guild sticker can be used, may be false due to loss of Server Boosts
    pub available: Option<bool>,

    /// id of the guild that owns this sticker
    pub guild_id: Option<String>,

    /// the user that uploaded the guild sticker
    pub user: Option<User>,

    /// the standard sticker's sort order within its pack
    pub sort_value: Option<u32>,
}
