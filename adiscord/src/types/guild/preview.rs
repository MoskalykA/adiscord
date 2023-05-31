use crate::types::{emoji::Emoji, feature::Feature, sticker::Sticker};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Preview {
    /// guild id
    pub id: String,

    /// guild name (2-100 characters, excluding trailing and leading whitespace)
    pub name: String,

    /// icon hash
    pub icon: Option<String>,

    /// splash hash
    pub splash: Option<String>,

    /// discovery splash hash; only present for guilds with the "DISCOVERABLE" feature
    pub discovery_splash: Option<String>,

    /// custom guild emojis
    pub emojis: Vec<Emoji>,

    /// enabled guild features
    pub features: Vec<Feature>,

    /// approximate number of members in this guild
    pub approximate_member_count: i32,

    /// approximate number of online members in this guild
    pub approximate_presence_count: i32,

    /// the description for the guild
    pub description: Option<String>,

    /// custom guild stickers
    pub stickers: Vec<Sticker>,
}
