use std::collections::HashSet;
use std::env;

use clients::digital_ocean::DigitalOceanClientBuiler;
use commands::witty::{shirley, thanks};
use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::{
    help_commands, macros::group, macros::help, Args, CommandGroup, CommandResult, HelpOptions,
    StandardFramework,
};
use serenity::model::{
    application::command::Command,
    channel::Message,
    id::UserId,
    prelude::{
        interaction::Interaction::{self, ApplicationCommand},
        GatewayIntents, Ready,
    },
};

mod clients;
mod commands;
mod lazy_statics;
mod models;
mod utils;

use crate::tilde::*;
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
    haphazardly,
    pirate,
    cv,
    lucky,
    face,
    now,
    movie,
    xkcd,
    nasa
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
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let do_client = DigitalOceanClientBuiler::new()
            .token("MY_AWSOME_TOKEN".to_string())
            .build();

        if let ApplicationCommand(command) = interaction {
            match command.data.name.as_str() {
                "ping" => commands::slash::ping::run(&ctx, &command).await,
                "kf2" => commands::slash::events::kf2::run(&ctx, &command, &do_client).await,
                _ => {}
            };
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        if let Err(e) = Command::set_global_application_commands(&ctx.http, |commands| {
            commands
                .create_application_command(|command| slash::ping::register(command))
                .create_application_command(|command| slash::events::kf2::register(command))
        })
        .await
        {
            println!("unable to register slash commands: {:?}", e)
        }

        println!("{} is connected!", ready.user.name);
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if !msg.author.bot {
            if let Some(thanks) = thanks(&msg) {
                let _ = msg.channel_id.say(&ctx.http, thanks).await;
            }
            if let Some(shirley) = shirley(&msg.content) {
                let _ = msg.channel_id.say(&ctx.http, shirley).await;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~"))
        .help(&MY_HELP)
        .group(&GENERAL_GROUP);

    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
