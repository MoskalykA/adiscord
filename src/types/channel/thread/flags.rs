use serde_repr::Deserialize_repr;

#[derive(Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum ChannelFlags {
    HasInteracted = 1 << 0,
    AllMessages = 1 << 1,
    OnlyMentions = 1 << 2,
    NoMessages = 1 << 3,
}
