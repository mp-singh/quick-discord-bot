use std::thread;

use serenity::{
    builder::CreateApplicationCommand,
    model::{
        prelude::{
            command::CommandOptionType,
            interaction::{
                application_command::{
                    ApplicationCommandInteraction, CommandDataOption, CommandDataOptionValue,
                },
                InteractionResponseType,
            },
            Role, RoleId,
        },
        Permissions,
    },
    prelude::Context,
};
use tokio::runtime::Handle;

use crate::clients::digital_ocean::{models::droplet::DropletCreate, DigitalOcean};

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("kf2")
        .description("All info regarding KF2 server")
        .create_option(|opt| {
            opt.name("provision")
                .description("Provsion a kf2 server to run in the cloud")
                .kind(CommandOptionType::SubCommand)
                .create_sub_option(|option| {
                    option
                        .name("region")
                        .description("Data center you wish to host in (defaults to Toronto)")
                        .kind(CommandOptionType::String)
                        .add_string_choice("Toronto - tor1", "tor1")
                        .add_string_choice("New York 1 - nyc1", "nyc1")
                        .add_string_choice("New York 3 - nyc3", "nyc3")
                        .add_string_choice("San Francisco - sfo3", "sfo3")
                        .required(true)
                })
                .create_sub_option(|option| {
                    option
                        .name("size")
                        .description("Droplet size (defaults to s-1vcpu-1gb-intel)")
                        .kind(CommandOptionType::String)
                        .add_string_choice("Basic: s-1vcpu-1gb-intel", "s-1vcpu-1gb-intel")
                        .add_string_choice("Basic: s-4vcpu-8gb-intel", "s-4vcpu-8gb-intel")
                        .required(true)
                })
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

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction, do_client: &DigitalOcean) {
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
            "provision" => provison_new(ctx, command, &sub_command.options, do_client).await,
            "unprovision" => un_provision(ctx, command, &sub_command.options).await,
            // "list" => list_all(ctx, command, &sub_command.options).await,
            _ => unreachable!(),
        }
    }
}

async fn provison_new(
    ctx: &Context,
    command: &ApplicationCommandInteraction,
    options: &[CommandDataOption],
    do_client: &DigitalOcean,
) {
    let region = options
        .iter()
        .find(|opt| opt.name == "region")
        .unwrap()
        .value
        .as_ref()
        .unwrap()
        .as_str()
        .unwrap();

    let size = options
        .iter()
        .find(|opt| opt.name == "size")
        .unwrap()
        .value
        .as_ref()
        .unwrap()
        .as_str()
        .unwrap();

    //TODO:: create droplet with tag kf2

    let new = DropletCreate {
        name: "kf2.server".to_owned(),
        region: region.to_string(),
        size: size.to_string(),
        image: "ubuntu-20-04-x64".to_string(),
        tags: vec!["kf2".to_string()],
        ssh_key: None,
        backup: None,
    };

    do_client.create_droplet(new);
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

// async fn list_all(
//     ctx: &Context,
//     command: &ApplicationCommandInteraction,
//     options: &[CommandDataOption],
// ) {
//     let droplets = match openapi::apis::droplets_api::droplets_list(
//         DO_CONFIGURATION,
//         None,
//         None,
//         Some("kf2"),
//         Some("kf2"),
//     )
//     .await
//     {
//         Ok(r) => {
//             if let Some(droplets) = r.droplets {
//                 droplets
//             } else {
//                 vec![]
//             }
//         }
//         Err(e) => {
//             if let Err(e) = command
//                 .create_interaction_response(&ctx.http, |resp| {
//                     resp.kind(InteractionResponseType::ChannelMessageWithSource)
//                         .interaction_response_data(|msg| {
//                             msg.content("Unable to list all KF2 servers")
//                         })
//                 })
//                 .await
//             {
//                 println!("unable to list all KF2 servers: {}", e)
//             }
//         }
//     };

//     if let Err(e) = command
//         .create_interaction_response(&ctx.http, |resp| {
//             resp.kind(InteractionResponseType::ChannelMessageWithSource)
//                 .interaction_response_data(|msg| msg.content("Unprovison Request Sent!"))
//         })
//         .await
//     {
//         println!("unable to UnProvison KF2 server: {}", e)
//     }
// }
