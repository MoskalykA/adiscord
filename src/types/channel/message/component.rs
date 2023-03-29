use serde_repr::Deserialize_repr;

#[derive(Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum MessageComponent {
    /// Container for other components
    ActionRow = 1,

    /// Button object
    Button,

    /// Select menu for picking from defined text options
    StringSelect,

    /// Text input object
    TextInput,

    /// Select menu for users
    UserSelect,

    /// Select menu for roles
    RoleSelect,

    /// Select menu for mentionables (users and roles)
    MentionableSelect,

    /// Select menu for channels
    ChannelSelect,
}
