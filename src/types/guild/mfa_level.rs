use serde_repr::Deserialize_repr;

#[derive(Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum MFALevel {
    None = 0,
    Elevated = 1,
}
