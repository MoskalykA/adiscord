use serde_repr::Deserialize_repr;

#[allow(non_camel_case_types)]
#[derive(Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum DefaultMessageNotifications {
    ALL_MESSAGES = 0,
    ONLY_MENTIONS = 1,
}
