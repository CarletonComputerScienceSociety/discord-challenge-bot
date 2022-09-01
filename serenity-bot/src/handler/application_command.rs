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
use serenity::model::prelude::ChannelType;
use serenity::prelude::Context;

use std::str::FromStr;
use std::sync::Arc;

use super::Command;

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
                // Get the event name from the command
                let event_name = match application_command_interaction
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
                    .get_guild(application_command_interaction.guild_id.unwrap().0)
                    .await?;

                // Create a category for the event
                let category = guild
                    .create_channel(&context.http, |c| {
                        c.name(event_name).kind(ChannelType::Category)
                    })
                    .await?;

                // Create a text channel for the event
                let channel = guild
                    .create_channel(&context.http, |c| {
                        c.name("rules")
                            .kind(ChannelType::Text)
                            // Make sure this channel is inside the category we
                            // just created
                            .category(category.id)
                    })
                    .await?;

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

                // Add the event to the database
                let event = event::ActiveModel {
                    discord_server_id: Set(application_command_interaction
                        .guild_id
                        .unwrap()
                        .0
                        .to_string()),
                    discord_category_id: Set(category.id.0.to_string()),
                    discord_main_channel_id: Set(channel.id.0.to_string()),
                    discord_event_join_button_id: Set(join_button.id.0.to_string()),
                    name: Set(event_name.to_string()),
                    ..Default::default()
                };

                // Save the event to the database
                event.insert(database.as_ref()).await?;

                // Respond that the event has been created
                application_command_interaction
                    .create_interaction_response(&context.http, |r| {
                        r.interaction_response_data(|d| d.title("Event created!").ephemeral(true))
                    })
                    .await?;
            }
        }
        Ok(())
    }
}
