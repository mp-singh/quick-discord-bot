use std::env;

use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};

use serenity::framework::standard::{
    macros::{command, group},
    CommandResult, StandardFramework,
};
use serenity::model::channel::Message;
use serenity::model::prelude::Ready;
use serenity::prelude::Mentionable;

#[group]
#[commands(ping, ip)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if !msg.author.bot {
            let user = msg.author;
            match msg.content.to_lowercase().contains("thank you")
                || msg.content.to_lowercase().contains("thanks")
            {
                true => {
                    let _ = msg
                        .channel_id
                        .say(&ctx.http, format!("No, thank you {}!", user.mention()))
                        .await;
                }
                false => (),
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    let token = env::var("DISCORD_TOKEN").expect("token");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong").await?;
    Ok(())
}

// // send my ip address
#[command]
async fn ip(ctx: &Context, msg: &Message) -> CommandResult {
    let ip = reqwest::get("https://api.ipify.org")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    msg.reply(ctx, ip).await?;
    Ok(())
}
