use std::fmt::Display;

use serenity::{
    model::prelude::{
        interaction::{
            application_command::ApplicationCommandInteraction, InteractionResponseType,
        },
        Message,
    },
    prelude::Context,
};

pub mod constants;

pub struct Interaction<'a> {
    message: Option<Message>,
    interaction: &'a ApplicationCommandInteraction,
    ctx: &'a Context,
    initial_response: bool,
    ephemeral: bool,
}

impl<'a> Interaction<'a> {
    pub const fn new(
        ctx: &'a Context,
        interaction: &'a ApplicationCommandInteraction,
        ephemeral: bool,
    ) -> Self {
        Self {
            message: None,
            interaction,
            ctx,
            initial_response: false,
            ephemeral,
        }
    }

    async fn initial(&mut self) {
        if !self.initial_response {
            self.interaction
                .create_interaction_response(self.ctx, |r| {
                    r.kind(InteractionResponseType::DeferredChannelMessageWithSource)
                        .interaction_response_data(|d| d.ephemeral(self.ephemeral))
                })
                .await
                .unwrap();
            self.initial_response = true;
        }
    }

    pub async fn reply(&mut self, content: impl Display + Send) {
        self.initial().await;
        if let Some(message) = self.message.as_ref() {
            self.interaction
                .edit_followup_message(self.ctx, message.id, |m| {
                    m.content(content).components(|c| c)
                })
                .await
                .unwrap();
        } else {
            self.message = Some(
                self.interaction
                    .create_followup_message(self.ctx, |m| m.content(content).components(|c| c))
                    .await
                    .unwrap(),
            );
        }
    }
}
