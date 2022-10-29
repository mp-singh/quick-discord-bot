use openapi::apis::configuration::Configuration;
use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{
        command::CommandOptionType,
        interaction::{
            application_command::{ApplicationCommandInteraction, CommandDataOption},
            InteractionResponseType,
        },
        Role, RoleId,
    },
    prelude::Context,
};

use crate::lazy_statics::DO_CONFIGURATION;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("kf2")
        .description("All info regarding KF2 server")
        .create_option(|opt| {
            opt.name("provision")
                .description("Provsion a kf2 server to run in the cloud")
                .kind(CommandOptionType::SubCommand)
        })
        .create_option(|opt| {
            opt.name("unprovision")
                .description("Delete the KF2 server running in the cloud")
                .kind(CommandOptionType::SubCommand)
        })
        .create_option(|opt| {
            opt.name("list")
                .description("List all droplets running in the cloud")
                .kind(CommandOptionType::SubCommand)
        })
}

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) {
    if !super::requires_role(
        RoleId(1035710963070013540),
        &command.member.as_ref().unwrap().roles,
        ctx,
        command,
    )
    .await
    {
        return;
    }

    let sub_command = command.data.options.first().unwrap();
    if sub_command.kind == CommandOptionType::SubCommand {
        match sub_command.name.as_str() {
            "provision" => provison_new(ctx, command, &sub_command.options).await,
            "unprovision" => un_provision(ctx, command, &sub_command.options).await,
            "list" => list_all(ctx, command, &sub_command.options).await,
            _ => unreachable!(),
        }
    }
}

async fn provison_new(
    ctx: &Context,
    command: &ApplicationCommandInteraction,
    options: &[CommandDataOption],
) {
    //TODO:: create droplet with tag kf2
    // let droplets_create_request = DropletsCreateRequest {

    // }

    // let respone = match openapi::apis::droplets_api::droplets_create(&configuration, droplets_create_request).await {
    //     Ok(a) => {
    //         a.droplet.networks.v4.unwrap()
    //     },
    //     Err(e) => {
    //         if let Err(e) = command
    //     .create_interaction_response(&ctx.http, |resp| {
    //         resp.kind(InteractionResponseType::ChannelMessageWithSource)
    //             .interaction_response_data(|msg| msg.content("Unable to Provison new KF2 Server"))
    //     })
    //     .await
    // {
    //     println!("unable to Provison new KF2 Server: {}", e)
    // }
    //     }
    // };

    if let Err(e) = command
        .create_interaction_response(&ctx.http, |resp| {
            resp.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|msg| msg.content("Provison Request Sent!"))
        })
        .await
    {
        println!("unable to Provison new KF2 Server: {}", e)
    }
}

async fn un_provision(
    ctx: &Context,
    command: &ApplicationCommandInteraction,
    options: &[CommandDataOption],
) {
    //TODO: Still need to set this up eh!!
    if let Err(e) = command
        .create_interaction_response(&ctx.http, |resp| {
            resp.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|msg| msg.content("Unprovison Request Sent!"))
        })
        .await
    {
        println!("unable to UnProvison KF2 server: {}", e)
    }
}

async fn list_all(
    ctx: &Context,
    command: &ApplicationCommandInteraction,
    options: &[CommandDataOption],
) {
    let droplets = match openapi::apis::droplets_api::droplets_list(
        DO_CONFIGURATION,
        None,
        None,
        Some("kf2"),
        Some("kf2"),
    )
    .await
    {
        Ok(r) => {
            if let Some(droplets) = r.droplets {
                droplets
            } else {
                vec![]
            }
        }
        Err(e) => {
            if let Err(e) = command
                .create_interaction_response(&ctx.http, |resp| {
                    resp.kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|msg| {
                            msg.content("Unable to list all KF2 servers")
                        })
                })
                .await
            {
                println!("unable to list all KF2 servers: {}", e)
            }
        }
    };

    if let Err(e) = command
        .create_interaction_response(&ctx.http, |resp| {
            resp.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|msg| msg.content("Unprovison Request Sent!"))
        })
        .await
    {
        println!("unable to UnProvison KF2 server: {}", e)
    }
}
