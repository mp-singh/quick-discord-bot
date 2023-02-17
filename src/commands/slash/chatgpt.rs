use crate::utils::interactions::Interaction;
use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::interaction::application_command::ApplicationCommandInteraction,
    prelude::Context,
};

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) {
    let mut interaction = Interaction::new(ctx, command, false);
    interaction.reply("This command is coming soon!!").await;
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("chatgpt")
        .description("Converse with the GPT-3 model")
        .create_option(|opt| {
            opt.name("query")
                .description("Message to send to the model(unimplemented)")
                .kind(serenity::model::prelude::command::CommandOptionType::String)
                .required(true)
        })
}
