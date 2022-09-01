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
                // Get the event name from the command
                let event_name = match slash_command
                    .data
                    .options
                    .get(0)
                    .expect("No event name provided")
                    .resolved
                    .as_ref()
                    .expect("No event name provided")
                {
                    CommandDataOptionValue::String(s) => s,
                    _ => {
                        warn!("Invalid event name");
                        return Ok(());
                    }
                };

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

                // Create a text channel for the event
                let channel = guild
                    .create_channel(&context.http, |c| {
                        c.name("event")
                            .kind(ChannelType::Text)
                            // Make sure this channel is inside the category we
                            // just created
                            .category(category.id)
                    })
                    .await?;

                // Add the event to the database
                let event = event::ActiveModel {
                    discord_server_id: Set(slash_command.guild_id.unwrap().0.to_string()),
                    discord_category_id: Set(category.id.0.to_string()),
                    discord_main_channel_id: Set(channel.id.0.to_string()),
                    name: Set(event_name.to_string()),
                    ..Default::default()
                };

                // Save the event to the database
                event.insert(self.database.as_ref()).await?;

                // TODO:
                // - Store each participant that wants to join

                // Add a button to join the event
                let join_button = channel
                    .send_message(&context.http, |m| {
                        m.content("Join the event").components(|c| {
                            c.add_action_row(
                                CreateActionRow::default()
                                    .add_button(
                                        CreateButton::default()
                                            .custom_id(uuid::Uuid::new_v4().simple().to_string())
                                            .label("Join")
                                            .style(ButtonStyle::Primary)
                                            .to_owned(),
                                    )
                                    .to_owned(),
                            )
                        })
                    })
                    .await?;
            }
        }

        Ok(())
    }
}
