use serde_repr::Deserialize_repr;

#[derive(Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum StickerType {
    /// an official sticker in a pack, part of Nitro or in a removed purchasable pack
    Standard = 1,

    /// a sticker uploaded to a guild for the guild's members
    Guild = 2,
}
