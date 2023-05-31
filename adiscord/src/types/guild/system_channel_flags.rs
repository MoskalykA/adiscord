use serde_repr::Deserialize_repr;

#[derive(Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum SystemChannelFlags {
    None,

    /// Suppress member join notifications
    SuppressJoinNotifications = 1 << 0,

    /// Suppress server boost notifications
    SuppressPremiumSubscriptions = 1 << 1,

    /// Suppress server setup tips
    SuppressGuildReminderNotifications = 1 << 2,

    /// Hide member join sticker reply buttons
    SuppressJoinNotificationReplies = 1 << 3,

    /// Suppress role subscription purchase and renewal notifications
    SuppressRoleSubscriptionPurchaseNotifications = 1 << 4,

    /// Hide role subscription sticker reply buttons
    SuppressRoleSubscriptionPurchaseNotificationReplies = 1 << 5,
}
