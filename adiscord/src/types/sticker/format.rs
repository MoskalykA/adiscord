use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Deserialize_repr, Serialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum StickerFormatType {
    PNG = 1,
    APNG,
    Lottie,
    GIF,
}
