use serde_repr::Deserialize_repr;

#[allow(non_camel_case_types)]
#[derive(Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum SortOrder {
    LATEST_ACTIVITY = 0,
    CREATION_DATE = 1,
}
