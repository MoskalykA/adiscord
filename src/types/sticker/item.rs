use super::format::StickerFormatType;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct StickerItem {
    pub id: String,
    pub name: String,
    pub format_type: StickerFormatType,
}
