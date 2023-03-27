use serde_repr::Deserialize_repr;

#[allow(non_camel_case_types)]
#[derive(Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum MemberFlags {
    None = 0,
    DID_REJOIN = 1 << 0,
    COMPLETED_ONBOARDING = 1 << 1,
    BYPASSES_VERIFICATION = 1 << 2,
    STARTED_ONBOARDING = 1 << 3,
}
