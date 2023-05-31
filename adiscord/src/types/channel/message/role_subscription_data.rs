use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct RoleSubscriptionData {
    /// the id of the sku and listing that the user is subscribed to
    pub role_subscription_listing_id: String,

    /// the name of the tier that the user is subscribed to
    pub tier_name: String,

    /// the cumulative number of months that the user has been subscribed for
    pub total_months: u8,

    /// whether this notification is for a renewal rather than a new purchase
    pub is_renewal: bool,
}
