use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RoleSubscriptionData {
    pub role_subscription_listing_id: String,
    pub tier_name: String,
    pub total_months: u8,
    pub is_renewal: bool,
}
