use serde_repr::Deserialize_repr;

#[derive(Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum PremiumTier {
    /// guild has not unlocked any Server Boost perks
    None,

    /// guild has unlocked Server Boost level 1 perks
    Tier1,

    /// guild has unlocked Server Boost level 2 perks
    Tier2,

    /// guild has unlocked Server Boost level 3 perks
    Tier3,
}
