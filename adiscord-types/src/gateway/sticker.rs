use crate::{api::sticker::Sticker, Snowflake};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Update {
    /// ID of the guild
    pub guild_id: Snowflake,

    /// Array of stickers
    pub stickers: Vec<Sticker>,
}
