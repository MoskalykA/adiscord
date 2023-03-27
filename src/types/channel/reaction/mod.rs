pub mod default;

use crate::types::emoji::Emoji;
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Reaction {
    pub count: u16,
    pub me: bool,
    pub emoji: Emoji,
}
