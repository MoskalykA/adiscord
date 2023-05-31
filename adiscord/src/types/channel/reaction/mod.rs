pub mod default;

use crate::types::emoji::Emoji;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Reaction {
    /// times this emoji has been used to react
    pub count: u16,

    /// whether the current user reacted using this emoji
    pub me: bool,

    /// emoji information
    pub emoji: Emoji,
}
