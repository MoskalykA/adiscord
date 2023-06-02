use serde_repr::{Deserialize_repr, Serialize_repr};

#[allow(non_camel_case_types)]
#[derive(Deserialize_repr, Serialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum MemberFlags {
    None,

    /// Member has left and rejoined the guild
    DID_REJOIN = 1 << 0,

    /// Member has completed onboarding
    COMPLETED_ONBOARDING = 1 << 1,

    /// Member is exempt from guild verification requirements
    BYPASSES_VERIFICATION = 1 << 2,

    /// Member has started onboarding
    STARTED_ONBOARDING = 1 << 3,
}
