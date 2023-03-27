use adiscord::Client;
use dotenv_codegen::dotenv;

#[tokio::main]
async fn main() {
    let client = Client::new("10", dotenv!("TOKEN"), adiscord::TokenType::Bot);
    match client
        .channel
        .get_messages("1089521338827427852", None)
        .await
    {
        Ok(messages) => println!("{:?}", messages),
        Err(error) => println!("{:?}", error),
    };

    /*match client.guild.get("1089521338286342195").await {
        Ok(guild) => println!("{:?}", guild.name),
        Err(error) => println!("{:?}", error),
    };

    match client.guild.get_preview("1089521338286342195").await {
        Ok(preview) => println!("{:?}", preview.name),
        Err(error) => println!("{:?}", error),
    };

    match client.channel.get("1089521338827427852").await {
        Ok(channel) => println!("{:?}", channel.name),
        Err(error) => println!("{:?}", error),
    };

    match client.guild.get_channels("1089521338286342195").await {
        Ok(channels) => println!("{:?}", channels),
        Err(error) => println!("{:?}", error),
    };

    match client.emoji.gets("1089521338286342195").await {
        Ok(emojis) => println!("{:?}", emojis),
        Err(error) => println!("{:?}", error),
    };

    match client.sticker.gets("1089521338286342195").await {
        Ok(stickers) => println!("{:?}", stickers),
        Err(error) => println!("{:?}", error),
    };

    match client.webhook.gets("1089521338286342195").await {
        Ok(webhooks) => println!("{:?}", webhooks),
        Err(error) => println!("{:?}", error),
    };

    match client.voice.get_regions().await {
        Ok(regions) => println!("{:?}", regions),
        Err(error) => println!("{:?}", error),
    }*/
}
