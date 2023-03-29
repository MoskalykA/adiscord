use super::{role::Role, user::User};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Emoji {
    /// emoji id
    pub id: String,

    /// emoji name
    pub name: String,

    /// roles allowed to use this emoji
    pub roles: Vec<Role>,

    /// user that created this emoji
    pub user: Option<User>,

    /// whether this emoji must be wrapped in colons
    pub require_colons: Option<bool>,

    /// whether this emoji is managed
    pub managed: Option<bool>,

    /// whether this emoji is animated
    pub animated: Option<bool>,

    /// whether this emoji can be used, may be false due to loss of Server Boosts
    pub available: Option<bool>,
}
