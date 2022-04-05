use std::collections::HashSet;
use std::env;

use maplit::hashset;
use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};

use lazy_static::lazy_static;
use regex::Regex;
use serenity::framework::standard::macros::help;
use serenity::framework::standard::{help_commands, Args, CommandGroup, HelpOptions};
use serenity::framework::standard::{macros::group, CommandResult, StandardFramework};
use serenity::model::channel::Message;
use serenity::model::id::UserId;
use serenity::model::prelude::Ready;

lazy_static! {
    static ref REQESUT: reqwest::Client = reqwest::Client::new();
    static ref REGEX_DICE: Regex = Regex::new(r"^([1-9][0-9]?|100)[Dd]([1-9]\d*)$").unwrap();
    static ref HARDLY: Regex = Regex::new(r"(\w{2,}(?:[aeiou]r|re))(?:\W|$)").unwrap();
    static ref BLACK_LIST: HashSet<&'static str> = hashset![
        "their", "another", "other", "tenor", "more", "there", "before", "never", "your", "after",
        "over", "youre", "here", "hear", "floor", "bear", "fair", "fare", "faire", "fairer",
        "four", "fore",
    ];
}

mod commands;
mod handlers;
mod models;

use commands::*;
use handlers::*;

#[group]
#[commands(
    ping,
    ip,
    joke,
    yomama,
    trivia,
    excuse,
    chuck_norris,
    trump,
    count,
    flip,
    roll,
    haphazardly
)]
struct General;

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

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if !msg.author.bot {
            if let Some(thanks) = is_thanks(&msg) {
                let _ = msg.channel_id.say(&ctx.http, thanks).await;
            }
            if let Some(hardly) = hardly(&msg.content) {
                let _ = msg.channel_id.say(&ctx.http, hardly).await;
            }
            if let Some(shirley) = shirley(&msg.content) {
                let _ = msg.channel_id.say(&ctx.http, shirley).await;
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
