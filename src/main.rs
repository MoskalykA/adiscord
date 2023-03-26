use adiscord::Client;
use dotenv_codegen::dotenv;

#[tokio::main]
async fn main() {
    let client = Client::new("10", dotenv!("TOKEN"), adiscord::TokenType::Bot);
    match client.guilds.get("1089521338286342195").await {
        Ok(guild) => println!("{:?}", guild.name),
        Err(error) => println!("{:?}", error),
    };

    match client.guilds.get_preview("1089521338286342195").await {
        Ok(guild) => println!("{:?}", guild),
        Err(error) => println!("{:?}", error),
    };

    match client.guilds.get_channel("1089521338827427852").await {
        Ok(guild) => println!("{:?}", guild),
        Err(error) => println!("{:?}", error),
    };
}
