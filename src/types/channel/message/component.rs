use serde_repr::Deserialize_repr;

#[allow(non_camel_case_types)]
#[derive(Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum MessageComponent {
    ActionRow = 1,
    Button,
    StringSelect,
    TextInput,
    UserSelect,
    RoleSelect,
    MentionableSelect,
    ChannelSelect,
}
