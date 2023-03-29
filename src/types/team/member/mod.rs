pub mod state;

use self::state::MemberState;
use crate::types::user::User;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TeamMember {
    /// the user's membership state on the team
    pub membership_state: MemberState,

    /// will always be ["*"]
    pub permissions: Vec<String>,

    /// the id of the parent team of which they are a member
    pub team_id: String,

    /// the avatar, discriminator, id, and username of the user
    pub user: User,
}
