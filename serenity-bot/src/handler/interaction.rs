use entity::entities::event;
use migration::sea_orm::ActiveModelTrait;
use migration::sea_orm::Set;
use serenity::model::{application::interaction::Interaction, prelude::ChannelType};
use serenity::prelude::*;
use std::str::FromStr;
use tracing::warn;

use super::{Command, Handler};

impl Handler {
    pub async fn interaction_create2(
        &self,
        context: Context,
        interaction: Interaction,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Get the slash command, or return if it's not a slash command.
        let slash_command = if let Some(slash_command) = interaction.application_command() {
            slash_command
        } else {
            return Ok(());
        };

        if let Err(e) = slash_command.channel_id.to_channel(&context).await {
            warn!("Error getting channel: {:?}", e);
        };

        match Command::from_str(&slash_command.data.name[..])? {
            Command::EventStart => {
                // Get the guild
                let guild = context
                    .http
                    .get_guild(slash_command.guild_id.unwrap().0)
                    .await?;

                // Create a category for the event
                let category = guild
                    .create_channel(&context.http, |c| {
                        c.name("event").kind(ChannelType::Category)
                    })
                    .await?;

                // Add the event to the database
                let event = event::ActiveModel {
                    discord_server_id: Set(slash_command.guild_id.unwrap().0.to_string()),
                    discord_category_id: Set(category.id.0.to_string()),
                    ..Default::default()
                };

                // Save the event to the database
                event.insert(&self.database).await?;

                // Create a text channel for the event
                let _channel = guild
                    .create_channel(&context.http, |c| {
                        c.name("event")
                            .kind(ChannelType::Text)
                            // Make sure this channel is inside the category we
                            // just created
                            .category(category.id)
                    })
                    .await?;

                // Todo
                // - Store the event name
                // - Store the main channel
                // - Store each participant that wants to join

                // Add a button to join the event
            }
        }

        Ok(())
    }
}
