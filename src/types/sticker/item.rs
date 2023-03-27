use super::format::StickerFormatType;
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct StickerItem {
    pub id: String,
    pub name: String,
    pub format_type: StickerFormatType,
}
