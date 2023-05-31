use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Deserialize_repr, Serialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum VideoQuality {
    /// Discord chooses the quality for optimal performance
    AUTO = 1,

    /// 720p
    FULL,
}
