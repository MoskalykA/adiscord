use serde_repr::Deserialize_repr;

#[derive(Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum SystemChannelFlags {
    None = 0,
    SuppressJoinNotifications = 1 << 0,
    SuppressPremiumSubscriptions = 1 << 1,
    SuppressGuildReminderNotifications = 1 << 2,
    SuppressJoinNotificationReplies = 1 << 3,
    SuppressRoleSubscriptionPurchaseNotifications = 1 << 4,
    SuppressRoleSubscriptionPurchaseNotificationReplies = 1 << 5,
}
