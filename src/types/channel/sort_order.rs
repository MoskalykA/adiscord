use serde_repr::Deserialize_repr;

#[allow(non_camel_case_types)]
#[derive(Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum SortOrder {
    /// Sort forum posts by activity
    LATEST_ACTIVITY,

    /// Sort forum posts by creation time (from most recent to oldest)
    CREATION_DATE,
}
