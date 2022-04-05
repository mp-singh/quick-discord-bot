use std::collections::HashSet;
use std::env;

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
use serenity::prelude::Mentionable;

lazy_static! {
    static ref REQESUT: reqwest::Client = reqwest::Client::new();
    static ref REGEX_DICE: Regex = Regex::new(r"^([1-9][0-9]?|100)[Dd]([1-9]\d*)$").unwrap();
    static ref HARDLY: Regex = Regex::new(r"(\w{2,}(?:[aeiou]r|re))(?:\W|$)").unwrap();
    static ref BLACK_LIST: HashSet<&'static str> = {
        let mut set = HashSet::new();
        set.insert("their");
        set.insert("another");
        set.insert("tenor");
        set.insert("more");
        set.insert("there");
        set.insert("before");
        set.insert("never");
        set.insert("your");
        set.insert("after");
        set.insert("over");
        set.insert("you're");
        set.insert("youre");
        set.insert("here");
        set.insert("floor");
        set
    };
}

mod commands;
mod models;

use commands::*;

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
            if is_thanks(&msg.content) {
                let _ = msg
                    .channel_id
                    .say(
                        &ctx.http,
                        format!("No, thank you {}!", msg.author.mention()),
                    )
                    .await;
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

fn hardly(msg: &str) -> Option<String> {
    for cap in HARDLY.captures_iter(msg) {
        let word = cap.get(1).unwrap().as_str();
        if !BLACK_LIST.contains(word) {
            return Some(format!(
                "{}{}? I hardly know her!",
                word[0..1].to_uppercase(),
                &word[1..]
            ));
        }
    }
    None
}

fn shirley(msg: &str) -> Option<String> {
    match msg.to_lowercase().contains("surely") {
        true => Some("Don't call me Shirley!".to_string()),
        false => None,
    }
}

fn is_thanks(msg: &str) -> bool {
    msg.to_lowercase().contains("thank") || msg.to_lowercase().contains("thx")
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
