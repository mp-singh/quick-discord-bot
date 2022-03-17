use std::collections::HashSet;
use std::env;

use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serenity::framework::standard::macros::help;
use serenity::framework::standard::{help_commands, Args, CommandGroup, HelpOptions};
use serenity::framework::standard::{
    macros::{command, group},
    CommandResult, StandardFramework,
};
use serenity::model::channel::Message;
use serenity::model::id::UserId;
use serenity::model::prelude::Ready;
use serenity::prelude::Mentionable;

lazy_static! {
    static ref REQESUT: reqwest::Client = reqwest::Client::new();
}
#[group]
#[commands(ping, ip, joke, yomama)]
struct General;

struct Handler;

#[help]
async fn my_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}

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
        .configure(|c| c.prefix("~"))
        .help(&MY_HELP) // set the bot's prefix to "~"
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

#[derive(Serialize, Deserialize)]
pub struct Joke {
    pub joke: String,
}

#[command]
async fn yomama(ctx: &Context, msg: &Message) -> CommandResult {
    let yomama = REQESUT
        .get("https://api.yomomma.info/")
        .header("Accept", "text/plain")
        .send()
        .await
        .unwrap()
        .json::<Joke>()
        .await
        .unwrap();
    msg.reply(ctx, yomama.joke).await?;
    Ok(())
}
