mod events;
mod models;
mod commands;

use serenity::prelude::*;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = std::env::var("DISCORD_TOKEN").expect("TOKEN must be set.");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .event_handler(events::Handler {prefix: "!"})
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
