use std::str::FromStr;

use serenity::{
    async_trait,
    model::prelude::{
        command::Command,
        interaction::Interaction::{self, ApplicationCommand},
        Message, Ready,
    },
    prelude::{Context, EventHandler},
};

use crate::{
    clients::digital_ocean::DigitalOceanClientBuiler,
    commands::{
        self,
        slash::{self, Commands},
        witty::{shirley, thanks},
    },
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let do_client = DigitalOceanClientBuiler::new().build();
        if let ApplicationCommand(command) = interaction {
            match Commands::from_str(command.data.name.as_str()).unwrap() {
                Commands::Ping => commands::slash::general::run_ping(&ctx, &command).await,
                Commands::Ip => commands::slash::general::run_ip(&ctx, &command).await,
                Commands::Now => commands::slash::general::run_now(&ctx, &command).await,
                Commands::Joke => commands::slash::general::run_joke(&ctx, &command).await,
                Commands::Yomama => commands::slash::general::run_yomama(&ctx, &command).await,
                Commands::Excuse => commands::slash::general::run_excuse(&ctx, &command).await,
                Commands::Trump => commands::slash::general::run_trump(&ctx, &command).await,
                Commands::ChuckNorris => {
                    commands::slash::general::run_chuck_norris(&ctx, &command).await
                }
                Commands::KF2 => {
                    commands::slash::events::kf2::run(&ctx, &command, &do_client).await
                }
                Commands::Nasa => commands::slash::general::run_nasa(&ctx, &command).await,
                Commands::ChatGPT => commands::slash::chatgpt::run(&ctx, &command).await,
            };
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        if let Err(e) = Command::set_global_application_commands(&ctx.http, |commands| {
            commands
                .create_application_command(|command| slash::events::kf2::register(command))
                .create_application_command(|command| slash::chatgpt::register(command))
                .create_application_command(|command| slash::general::register_ping(command))
                .create_application_command(|command| slash::general::register_ip(command))
                .create_application_command(|command| slash::general::register_now(command))
                .create_application_command(|command| slash::general::register_joke(command))
                .create_application_command(|command| {
                    slash::general::register_chuck_norris(command)
                })
                .create_application_command(|command| slash::general::register_yomama(command))
                .create_application_command(|command| slash::general::register_excuse(command))
                .create_application_command(|command| slash::general::register_trump(command))
                .create_application_command(|command| slash::general::register_nasa(command))
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
            if let Some(hardly) = commands::witty::hardly(&msg) {
                let _ = msg.channel_id.say(&ctx.http, hardly).await;
            }
            if let Some(im_resp) = commands::witty::im_response(&msg) {
                let _ = msg.channel_id.say(&ctx.http, im_resp).await;
            }
        }
    }
}
