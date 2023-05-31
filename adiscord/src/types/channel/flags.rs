use serde_repr::{Deserialize_repr, Serialize_repr};

#[allow(non_camel_case_types)]
#[derive(Deserialize_repr, Serialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum Flags {
    None,

    /// this thread is pinned to the top of its parent GUILD_FORUM channel
    PINNED = 1 << 1,

    /// whether a tag is required to be specified when creating a thread in a GUILD_FORUM channel. Tags are specified in the applied_tags field.
    REQUIRE_TAG = 1 << 4,
}
