pub mod flags;

use crate::types::user::User;
use serde::Deserialize;
use self::flags::ThreadMemberFlags;

#[derive(Deserialize, Debug)]
pub struct ThreadMember {
    pub id: Option<String>,
    pub user_id: Option<String>,
    pub join_timestamp: String,
    pub flags: ThreadMemberFlags,
    pub member: Option<User>,
}
