use serde_repr::Deserialize_repr;

#[derive(Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum PremiumTier {
    None = 0,
    Tier1 = 1,
    Tier2 = 2,
    Tier3 = 3,
}
