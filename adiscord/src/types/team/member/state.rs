use serde_repr::{Deserialize_repr, Serialize_repr};

#[allow(non_camel_case_types)]
#[derive(Deserialize_repr, Serialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum MemberState {
    INVITED = 1,
    ACCEPTED,
}
