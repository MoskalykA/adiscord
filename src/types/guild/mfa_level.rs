use serde_repr::Deserialize_repr;

#[derive(Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum MFALevel {
    /// guild has no MFA/2FA requirement for moderation actions
    None = 0,

    /// guild has a 2FA requirement for moderation actions
    Elevated = 1,
}
