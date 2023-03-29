use super::flags::ChannelFlags;
use crate::types::user::User;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ThreadMember {
    pub id: Option<String>,
    pub user_id: Option<String>,
    pub join_timestamp: String,
    pub flags: ChannelFlags,
    pub member: Option<User>,
}
