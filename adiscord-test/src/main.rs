use adiscord::Client;
use adiscord_intents::Intent;
use dotenv_codegen::dotenv;

#[tokio::main]
async fn main() {
    let mut client = Client::new("10", dotenv!("TOKEN"), adiscord::TokenType::Bot);
    client.set_heartbeat_ack(true);

    client.add_intents(vec![
        Intent::MessageContent,
        Intent::Guilds,
        Intent::GuildMessages,
        Intent::GuildMembers,
    ]);

    // Ready

    client.on_ready(|ready| {
        println!("{:?}", ready);
    });

    // Message

    client.on_message(|message| {
        println!("Create message -> {}", message.content);
    });

    client.on_message_update(|message| {
        println!("Update message -> {}", message.content);
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

    client.init().await;
}
