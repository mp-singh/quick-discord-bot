use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{
        command::CommandOptionType,
        interaction::{
            self,
            application_command::{ApplicationCommandInteraction, CommandDataOption},
            InteractionResponseType,
        },
        RoleId,
    },
    prelude::Context,
};
use std::str::FromStr;

use crate::{
    clients::digital_ocean::{
        models::droplet::{Droplet, DropletCreate},
        DigitalOcean,
    },
    commands::slash::SubCommand,
    utils::interactions::Interaction,
};

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
                        .description("Data center you wish to host in ")
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
                        .description("Droplet size")
                        .kind(CommandOptionType::String)
                        .add_string_choice("Basic: s-1vcpu-1gb-intel", "s-1vcpu-1gb-intel")
                        .add_string_choice("Basic: s-4vcpu-8gb-intel", "s-4vcpu-8gb-intel")
                        .required(true)
                })
                .create_sub_option(|option| {
                    option
                        .name("os")
                        .description("operating system")
                        .kind(CommandOptionType::String)
                        .add_string_choice("Ubuntu 20.04", "ubuntu-20-04-x64")
                        .add_string_choice("Ubuntu 18.04", "ubuntu-18-04-x64")
                        .required(true)
                })
        })
        .create_option(|opt| {
            opt.name("unprovision")
                .description("Delete the KF2 server running in the cloud")
                .kind(CommandOptionType::SubCommand)
                .create_sub_option(|option| {
                    option
                        .name("tag")
                        .description("Droplet tag")
                        .kind(CommandOptionType::String)
                        .required(true)
                })
        })
        .create_option(|opt| {
            opt.name("list")
                .description("List all droplets running in the cloud")
                .kind(CommandOptionType::SubCommand)
        })
}

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction, do_client: &DigitalOcean) {
    // if !super::requires_role(
    //     RoleId(1035710963070013540),
    //     &command.member.as_ref().unwrap().roles,
    //     ctx,
    //     command,
    // )
    // .await
    // {
    //     return;
    // }

    let sub_command = command.data.options.first().unwrap();
    if sub_command.kind == CommandOptionType::SubCommand {
        match SubCommand::from_str(sub_command.name.as_str()).unwrap() {
            SubCommand::Provision => {
                provison_new(ctx, command, &sub_command.options, do_client).await
            }
            SubCommand::UnProvision => un_provision(ctx, command, &sub_command.options).await,
            SubCommand::List => list_all(ctx, command, &sub_command.options, do_client).await,
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

    let mut interaction = Interaction::new(ctx, command);
    interaction.reply("Provison Request Sent!").await;

    // do_client.create_droplet(new);
}

async fn un_provision(
    ctx: &Context,
    command: &ApplicationCommandInteraction,
    options: &[CommandDataOption],
) {
    let mut interaction = Interaction::new(ctx, command);
    interaction.reply("Unprovisioning KF2 Server").await;
}

async fn list_all(
    ctx: &Context,
    command: &ApplicationCommandInteraction,
    options: &[CommandDataOption],
    do_client: &DigitalOcean,
) {
    let mut interaction = Interaction::new(ctx, command);
    match do_client.list_droplets_by_tag_name("kf2").await {
        Ok(droplets) => {
            let mut msg = String::new();
            for droplet in droplets {
                msg.push_str(&format!("{}: {}", droplet.id, droplet.name));
            }
            interaction.reply(&msg).await;
        }
        Err(_) => interaction.reply("Unable to list KF2 Servers").await,
    };
}
