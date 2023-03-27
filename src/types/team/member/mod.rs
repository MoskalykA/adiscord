pub mod state;

use self::state::MemberState;
use crate::types::user::User;
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TeamMember {
    pub membership_state: MemberState,
    pub permissions: Vec<String>,
    pub team_id: String,
    pub user: User,
}
