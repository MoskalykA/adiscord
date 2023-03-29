use serde_repr::Deserialize_repr;

#[allow(non_camel_case_types)]
#[derive(Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum ExplicitContentFilter {
    /// media content will not be scanned
    DISABLED,

    /// media content sent by members without roles will be scanned
    MEMBERS_WITHOUT_ROLES,

    /// media content sent by all members will be scanned
    ALL_MEMBERS,
}
