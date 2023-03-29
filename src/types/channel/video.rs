use serde_repr::Deserialize_repr;

#[derive(Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum VideoQuality {
    /// Discord chooses the quality for optimal performance
    AUTO = 1,

    /// 720p
    FULL = 2,
}
