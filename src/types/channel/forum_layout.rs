use serde_repr::Deserialize_repr;

#[allow(non_camel_case_types)]
#[derive(Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum ForumLayout {
    /// No default has been set for forum channel
    NOT_SET = 0,

    /// Display posts as a list
    LIST_VIEW = 1,

    /// Display posts as a collection of tiles
    GALLERY_VIEW = 2,
}
