use crate::api::sticker::Sticker;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Update {
    /// ID of the guild
    pub guild_id: String,

    /// Array of stickers
    pub stickers: Vec<Sticker>,
}
