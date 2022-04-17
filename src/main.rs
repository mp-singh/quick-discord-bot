use std::collections::{HashMap, HashSet};
use std::env;

use maplit::{hashmap, hashset};
use reqwest::redirect;
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
    static ref REQESUT: reqwest::Client = reqwest::Client::builder()
        .redirect(redirect::Policy::none())
        .build()
        .unwrap();
    static ref REGEX_DICE: Regex = Regex::new(r"^([1-9][0-9]?|100)[Dd]([1-9]\d*)$").unwrap();
    static ref HARDLY: Regex = Regex::new(r"(\w{2,}(?:[aeiou]r|re))(?:\W|$)").unwrap();
    static ref TRANSFORMATION_TYPES: HashMap<&'static str, Vec<f32>> = hashmap! {
        "laplacian" => vec![0.0, 1.0, 0.0, 1.0, -4.0, 1.0, 0.0, 1.0, 0.0],
        "sobel" => vec![-1.0, 0.0, 1.0, -2.0, 0.0, 2.0, -1.0, 0.0, 1.0],
        "prewitt" => vec![-1.0, 0.0, 1.0, -1.0, 0.0, 1.0, -1.0, 0.0, 1.0],
        "roberts" => vec![1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0],
        "scharr" => vec![-3.0, 0.0, 3.0, -10.0, 0.0, 10.0, -3.0, 0.0, 3.0],
        "laplacian_of_gaussian" => vec![0.0, 0.0, 1.0, 2.0, 1.0, 0.0, 0.0, 0.0, 0.0],
        "gaussian" => vec![1.0, 2.0, 1.0, 2.0, 4.0, 2.0, 1.0, 2.0, 1.0],
        "unsharp_mask" => vec![0.0, -1.0, 0.0, -1.0, 5.0, -1.0, 0.0, -1.0, 0.0],
    };
    static ref BLACK_LIST: HashSet<&'static str> = hashset![
        "another",
        "other",
        "tenor",
        "before",
        "never",
        "over",
        "youre",
        "fairer",
        "after",
        "everywhere",
        "ever",
        "hardware",
        "software",
        "anywhere",
        "super",
        "order"
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
    haphazardly,
    pirate,
    cv,
    lucky,
    face,
    now
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
            if let Some(thanks) = thanks(&msg) {
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
