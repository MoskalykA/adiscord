use serde_repr::Deserialize_repr;

#[allow(non_camel_case_types)]
#[derive(Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum VideoQuality {
    /// Discord chooses the quality for optimal performance
    AUTO = 1,

    /// 720p
    FULL = 2,
}
