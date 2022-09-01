use entity::entities::event;
use log::warn;
use migration::sea_orm::ActiveModelTrait;
use migration::sea_orm::DatabaseConnection;
use migration::sea_orm::Set;
use serenity::builder::CreateActionRow;
use serenity::builder::CreateButton;

use serenity::model::prelude::component::ButtonStyle;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::interaction::application_command::CommandDataOptionValue;
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::model::prelude::ChannelType;
use serenity::prelude::Context;

use std::str::FromStr;
use std::sync::Arc;

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
            }
        }
        Ok(())
    }
}
