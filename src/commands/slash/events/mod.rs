use serenity::{
    model::prelude::{interaction::application_command::ApplicationCommandInteraction, RoleId},
    prelude::Context,
};

use crate::utils::interactions::Interaction;

pub mod kf2;

pub async fn requires_role(
    user_role: RoleId,
    all_roles: &[RoleId],
    ctx: &Context,
    command: &ApplicationCommandInteraction,
) -> bool {
    let found = all_roles.iter().any(|role| *role == user_role);
    let mut interaction = Interaction::new(ctx, command, true);

    if !found {
        interaction
            .reply("You do not have permission to use this command.")
            .await;
    }
    found
}
