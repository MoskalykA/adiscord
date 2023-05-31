use serde_repr::{Deserialize_repr, Serialize_repr};

#[allow(non_camel_case_types)]
#[derive(Deserialize_repr, Serialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum InteractionType {
    PING = 1,
    APPLICATION_COMMAND,
    MESSAGE_COMPONENT,
    APPLICATION_COMMAND_AUTOCOMPLETE,
    MODAL_SUBMIT,
}
