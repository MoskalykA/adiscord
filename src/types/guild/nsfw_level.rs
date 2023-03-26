use serde_repr::Deserialize_repr;

#[derive(Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum NSFWLevel {
    Default = 0,
    Explicit = 1,
    Safe = 2,
    AgeRestricted = 3,
}
