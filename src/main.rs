use std::env;

use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};

use lazy_static::lazy_static;
use serenity::framework::standard::{
    macros::{command, group},
    CommandResult, StandardFramework,
};
use serenity::model::channel::Message;
use serenity::model::prelude::Ready;
use serenity::prelude::Mentionable;

#[group]
#[commands(ping, ip, joke)]
struct General;

lazy_static! {
    static ref REQESUT: reqwest::Client = reqwest::Client::new();
}
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if !msg.author.bot && is_thanks(&msg.content) {
            let _ = msg
                .channel_id
                .say(
                    &ctx.http,
                    format!("No, thank you {}!", msg.author.mention()),
                )
                .await;
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn is_thanks(msg: &str) -> bool {
    msg.to_lowercase().contains("thank") || msg.to_lowercase().contains("thank you")
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

#[command]
async fn ip(ctx: &Context, msg: &Message) -> CommandResult {
    let ip = REQESUT
        .get("https://api.ipify.org")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    msg.reply(ctx, ip).await?;
    Ok(())
}

#[command]
async fn joke(ctx: &Context, msg: &Message) -> CommandResult {
    let joke = REQESUT
        .get("https://icanhazdadjoke.com/")
        .header("Accept", "text/plain")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    msg.reply(ctx, joke).await?;
    Ok(())
}
