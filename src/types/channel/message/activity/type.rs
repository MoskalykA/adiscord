use serde_repr::Deserialize_repr;

#[allow(non_camel_case_types)]
#[derive(Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum ActivityType {
    JOIN = 1,
    SPECTATE,
    LISTEN,
    JOIN_REQUEST = 5,
}
