use super::{role::Role, user::User};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Emoji {
    /// emoji id
    pub id: Option<String>,

    /// emoji name
    pub name: Option<String>,

    /// roles allowed to use this emoji
    pub roles: Option<Vec<Role>>,

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
