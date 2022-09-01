use entity::entities::event;
use migration::sea_orm::ActiveModelTrait;
use migration::sea_orm::Set;
use serenity::builder::CreateActionRow;
use serenity::builder::CreateButton;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::component::ButtonStyle;
use serenity::model::prelude::interaction::application_command::CommandDataOptionValue;
use serenity::model::{application::interaction::Interaction, prelude::ChannelType};
use serenity::prelude::*;
use std::str::FromStr;
use tracing::warn;

use super::application_command::ApplicationCommandHandler;
use super::message_component::MessageComponentHandler;
use super::{Command, Handler};

pub trait InteractionHandler {
    fn handle_command(
        context: Context,
        command: Command,
        args: Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>>;
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
                MessageComponentHandler::handle_command(
                    context,
                    message_component,
                    self.database.clone(),
                )
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
