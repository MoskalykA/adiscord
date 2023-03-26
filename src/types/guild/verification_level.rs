use serde_repr::Deserialize_repr;

#[allow(non_camel_case_types)]
#[derive(Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum VerificationLevel {
    NONE = 0,
    LOW = 1,
    MEDIUM = 2,
    HIGH = 3,
    VERY_HIGH = 4,
}
