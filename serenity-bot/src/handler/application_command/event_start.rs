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

use super::Command;

pub struct ApplicationCommandHandler;

#[derive(Serialize, Deseri)]
struct EventJoin {
    pub event_id: u64,
    pub user_id: u64,
}

pub async fn handle_event_start_command(
    application_command_interaction: ApplicationCommandInteraction,
    context: Context,
    database: Arc<DatabaseConnection>,
) -> Result<(), Box<dyn std::error::Error>> {
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

    let guild = context
        .http
        .get_guild(application_command_interaction.guild_id.unwrap().0)
        .await?;

    let category = guild
        .create_channel(&context.http, |c| {
            c.name(event_name).kind(ChannelType::Category)
        })
        .await?;

    let channel = guild
        .create_channel(&context.http, |c| {
            c.name("rules")
                .kind(ChannelType::Text)
                // Make sure this channel is inside the category we
                // just created
                .category(category.id)
        })
        .await?;

    // Create a join button for the event
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

    // Create the event for the database
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

    // Insert the event into the database
    event.insert(database.as_ref()).await?;

    // Respond to the interaction
    application_command_interaction
        .create_interaction_response(&context.http, |r| {
            r.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|d| d.content("Event created!").ephemeral(true))
        })
        .await?;

    Ok(())
}
