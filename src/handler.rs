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
                Commands::Ping => commands::slash::ping::run(&ctx, &command).await,
                Commands::KF2 => {
                    commands::slash::events::kf2::run(&ctx, &command, &do_client).await
                }
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
            if let Some(hardly) = commands::witty::hardly(&msg) {
                let _ = msg.channel_id.say(&ctx.http, hardly).await;
            }
            if let Some(im_resp) = commands::witty::im_response(&msg) {
                let _ = msg.channel_id.say(&ctx.http, im_resp).await;
            }
        }
    }
}
