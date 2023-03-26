use serde_repr::Deserialize_repr;

#[allow(non_camel_case_types)]
#[derive(Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum ExplicitContentFilter {
    DISABLED = 0,
    MEMBERS_WITHOUT_ROLES = 1,
    ALL_MEMBERS = 2,
}
