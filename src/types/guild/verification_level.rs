use serde_repr::Deserialize_repr;

#[allow(non_camel_case_types)]
#[derive(Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum VerificationLevel {
    /// unrestricted
    NONE,

    /// must have verified email on account
    LOW,

    /// must be registered on Discord for longer than 5 minutes
    MEDIUM,

    /// must be a member of the server for longer than 10 minutes
    HIGH,

    /// must have a verified phone number
    VERY_HIGH,
}
