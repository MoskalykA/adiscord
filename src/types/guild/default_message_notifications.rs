use serde_repr::Deserialize_repr;

#[allow(non_camel_case_types)]
#[derive(Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum DefaultMessageNotifications {
    /// members will receive notifications for all messages by default
    ALL_MESSAGES = 0,

    /// members will receive notifications only for messages that @mention them by default
    ONLY_MENTIONS = 1,
}
