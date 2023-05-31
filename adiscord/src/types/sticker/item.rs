use super::format::StickerFormatType;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct StickerItem {
    /// id of the sticker
    pub id: String,

    /// name of the sticker
    pub name: String,

    /// type of sticker format
    pub format_type: StickerFormatType,
}
