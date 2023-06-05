use adiscord::Client;
use dotenv_codegen::dotenv;

#[tokio::main]
async fn main() {
    let mut client = Client::new("10", dotenv!("TOKEN"));
    client.set_heartbeat_ack(true);
    client.set_heartbeat_ack_count(true);

    client.all_intents();

    // Ready

    client.on_ready(|ready| {
        println!("{:?}", ready);
    });

    // Auto Moderation

    client.on_auto_moderation_rule_create(|rule| {
        println!("Create rule -> {:?}", rule.name);
    });

    client.on_auto_moderation_rule_update(|rule| {
        println!("Update rule -> {:?}", rule.name);
    });

    client.on_auto_moderation_rule_delete(|rule| {
        println!("Delete rule -> {:?}", rule.name);
    });

    client.on_auto_moderation_action_execution(|execution| {
        println!("Exec rule -> {:?}", execution.content);
    });

    // Channel

    client.on_channel_create(|channel| {
        println!("Create chanel -> {:?}", channel.name);
    });

    client.on_channel_update(|channel| {
        println!("Update chanel -> {:?}", channel.name);
    });

    client.on_channel_delete(|channel| {
        println!("Delete chanel -> {:?}", channel.name);
    });

    client.on_channel_pins_update(|pins| {
        println!("Update chanel pins -> {:?}", pins.channel_id);
    });

    // Thread

    client.on_thread_create(|channel| {
        println!("Create thread -> {:?}", channel.name);
    });

    client.on_thread_update(|channel| {
        println!("Update thread -> {:?}", channel.name);
    });

    client.on_thread_delete(|channel| {
        println!("Delete thread -> {:?}", channel.id);
    });

    client.on_thread_list_sync(|list| {
        println!("List sync thread -> {:?}", list.guild_id);
    });

    client.on_thread_member_update(|member| {
        println!("Member update thread -> {:?}", member.id);
    });

    client.on_thread_members_update(|members| {
        println!("Members update thread -> {:?}", members.id);
    });

    // Guild

    client.on_guild_create(|guild| {
        println!("Create guild -> {:?}", guild.name);
    });

    client.on_guild_update(|guild| {
        println!("Update guild -> {:?}", guild.name);
    });

    client.on_guild_delete(|guild| {
        println!("Delete guild -> {:?}", guild.id);
    });

    client.on_guild_audit_log_entry_create(|log| {
        println!("Create audit log entry -> {:?}", log.id);
    });

    client.on_guild_ban_add(|ban| {
        println!("Add ban -> {:?}", ban.guild_id);
    });

    client.on_guild_ban_remove(|ban| {
        println!("Remove ban -> {:?}", ban.guild_id);
    });

    // Emojis

    client.on_emojis_update(|emojis| {
        println!("Update emojis -> {:?}", emojis.guild_id);
    });

    // Stickers

    client.on_stickers_update(|stickers| {
        println!("Update stickers -> {:?}", stickers.guild_id);
    });

    // Integrations

    client.on_integrations_update(|integrations| {
        println!("Update integrations -> {:?}", integrations.guild_id);
    });

    // Member

    client.om_member_add(|member| {
        println!("Add member -> {:?}", member.guild_id);
    });

    client.on_member_remove(|member| {
        println!("Remove member -> {:?}", member.guild_id);
    });

    client.on_member_update(|member| {
        println!("Update member -> {:?}", member.guild_id);
    });

    // Role

    client.on_role_create(|role| {
        println!("Create role -> {:?}", role.guild_id);
    });

    client.on_role_update(|role| {
        println!("Update role -> {:?}", role.guild_id);
    });

    client.on_role_delete(|role| {
        println!("Delete role -> {:?}", role.guild_id);
    });

    // Scheduled event

    client.on_scheduled_event_create(|scheduled| {
        println!("Create scheduled event -> {:?}", scheduled.name);
    });

    client.on_scheduled_event_update(|scheduled| {
        println!("Update scheduled event -> {:?}", scheduled.name);
    });

    client.on_scheduled_event_delete(|scheduled| {
        println!("Delete scheduled event -> {:?}", scheduled.name);
    });

    client.on_scheduled_event_user_add(|scheduled| {
        println!("Add scheduled event user -> {:?}", scheduled.guild_id);
    });

    client.on_scheduled_event_user_remove(|scheduled| {
        println!("Remove scheduled event user -> {:?}", scheduled.guild_id);
    });

    // Integration

    client.on_integration_create(|integration| {
        println!("Create integration -> {:?}", integration.name);
    });

    client.on_integration_update(|integration| {
        println!("Update integration -> {:?}", integration.name);
    });

    client.on_integration_delete(|integration| {
        println!("Delete integration -> {:?}", integration.id);
    });

    // Invite

    client.on_invite_update(|invite| {
        println!("Update invite -> {:?}", invite.guild_id);
    });

    client.on_invite_delete(|invite| {
        println!("Update invite -> {:?}", invite.guild_id);
    });

    // Message

    client.on_message(|message| {
        println!("Create message -> {:?}", message.content);
    });

    client.on_message_update(|message| {
        println!("Update message -> {:?}", message.content);
    });

    client.on_message_delete(|message| {
        println!("Delete message -> {:?}", message.id);
    });

    client.on_message_delete_bulk(|message| {
        println!("Delete bulk message -> {:?}", message.guild_id);
    });

    // Reaction

    client.on_reaction_add(|reaction| {
        println!("Add reaction -> {:?}", reaction.guild_id);
    });

    client.on_reaction_remove(|reaction| {
        println!("Remove reaction -> {:?}", reaction.guild_id);
    });

    client.on_reaction_remove_all(|reaction| {
        println!("Remove all reactions -> {:?}", reaction.guild_id);
    });

    client.on_reaction_remove_emoji(|reaction| {
        println!("Remove emoji reaction -> {:?}", reaction.guild_id);
    });

    // Presence

    client.on_presence_update(|presence| {
        println!("Update presence -> {:?}", presence.user.id);
    });

    // Typing

    client.on_typing_start(|typing| {
        println!("Start typing -> {:?}", typing.member);
    });

    // Voice

    client.on_voice_state_update(|voice| {
        println!("Update voice state -> {:?}", voice.member);
    });

    client.on_voice_server_update(|voice| {
        println!("Update voice server -> {:?}", voice.guild_id);
    });

    // Webhooks

    client.on_webhooks_update(|webhook| {
        println!("Update webhook -> {:?}", webhook.guild_id);
    });

    client.init().await;
}
