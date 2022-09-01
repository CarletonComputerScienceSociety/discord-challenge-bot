use log::warn;
use serde::{Deserialize, Serialize};
use serenity::{model::application::interaction::Interaction, prelude::*};

use super::{
    application_command::ApplicationCommandHandler, message_component::MessageComponentHandler,
    Command, Handler,
};

pub trait InteractionHandler {
    fn handle_command(
        context: Context,
        command: Command,
        args: Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

#[derive(Serialize, Deserialize)]
pub enum InteractionCustomId {
    StartEvent { event_id: u64 },
}

impl Handler {
    /// Handle any incoming interactions. This can either be an application
    /// command, or a message component being interacted with.
    pub async fn interaction_create(
        &self,
        context: Context,
        interaction: Interaction,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match interaction {
            Interaction::ApplicationCommand(application_command) => {
                ApplicationCommandHandler::handle_command(
                    context,
                    application_command,
                    self.database.clone(),
                )
                .await?;
            }
            Interaction::MessageComponent(message_component) => {
                MessageComponentHandler {
                    context,
                    message_component_interaction: message_component,
                    database: self.database.clone(),
                }
                .handle_command()
                .await?
            }
            _ => {
                warn!("Unhandled interaction type");
                return Ok(());
            }
        }

        Ok(())
    }
}
