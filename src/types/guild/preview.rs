use crate::types::{emoji::Emoji, feature::Feature, sticker::Sticker};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Preview {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    pub splash: Option<String>,
    pub discovery_splash: Option<String>,
    pub emojis: Vec<Emoji>,
    pub features: Vec<Feature>,
    pub approximate_member_count: i32,
    pub approximate_presence_count: i32,
    pub description: Option<String>,
    pub stickers: Vec<Sticker>,
}
