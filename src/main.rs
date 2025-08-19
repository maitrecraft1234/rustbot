use poise::{framework, serenity_prelude as serenity};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.ends_with("quoi") {
            if let Err(why) = msg.channel_id.say(&ctx.http, "feur!").await {
                eprintln!("Error sending feur: {why:?}");
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let token = std::fs::read_to_string(".token").expect("Need a .token file");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder().options(poise::FrameworkOptions)

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(e) = client.start().await {
        println!("Client error: {e:?}");
    }
}
