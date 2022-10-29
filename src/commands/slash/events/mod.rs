pub mod event;
use serenity::{
    model::prelude::{
        interaction::{
            application_command::ApplicationCommandInteraction, InteractionResponseType,
        },
        RoleId,
    },
    prelude::Context,
};

pub async fn requires_role(
    user_role: RoleId,
    all_roles: &[RoleId],
    ctx: &Context,
    command: &ApplicationCommandInteraction,
) -> bool {
    let found = all_roles.iter().any(|role| *role == user_role);
    if !found {
        command
            .create_interaction_response(&ctx, |r| {
                r.kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|m| {
                        m.content("You do not have permission to use this command")
                            .ephemeral(true)
                    })
            })
            .await
            .unwrap();
    }
    found
}
