use serde_repr::{Deserialize_repr, Serialize_repr};

#[allow(non_camel_case_types)]
#[derive(Deserialize_repr, Serialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum ForumLayout {
    /// No default has been set for forum channel
    NOT_SET,

    /// Display posts as a list
    LIST_VIEW,

    /// Display posts as a collection of tiles
    GALLERY_VIEW,
}
