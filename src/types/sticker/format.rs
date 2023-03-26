use serde_repr::Deserialize_repr;

#[derive(Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum StickerFormatType {
    PNG = 1,
    APNG = 2,
    Lottie = 3,
    GIF = 4,
}
