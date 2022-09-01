use entity::entities::event;
use log::warn;
use migration::sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use serenity::builder::{CreateActionRow, CreateButton};

use serenity::{
    model::prelude::{
        component::ButtonStyle,
        interaction::{
            application_command::{ApplicationCommandInteraction, CommandDataOptionValue},
            InteractionResponseType,
        },
        ChannelType,
    },
    prelude::Context,
};

use std::{str::FromStr, sync::Arc};

use self::event_start::handle_event_start_command;

use super::Command;

mod event_start;

pub struct ApplicationCommandHandler;

impl ApplicationCommandHandler {
    pub async fn handle_command(
        context: Context,
        application_command_interaction: ApplicationCommandInteraction,
        database: Arc<DatabaseConnection>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Err(e) = application_command_interaction
            .channel_id
            .to_channel(&context)
            .await
        {
            warn!("Error getting channel: {:?}", e);
        };

        match Command::from_str(&application_command_interaction.data.name[..])? {
            Command::EventStart => {
                handle_event_start_command(application_command_interaction, context, database)
                    .await?;
            } // TODO: Add a command to delete an event on a server, and clean up
              // anything created
              // TODO: Add a command to start the event
        }
        Ok(())
    }
}
