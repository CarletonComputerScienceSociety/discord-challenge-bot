use db_entity::entities::event;
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

use std::sync::Arc;

use crate::handler::interaction::InteractionCustomId;

pub struct ApplicationCommandHandler;

pub async fn handle_event_create_command(
    application_command_interaction: ApplicationCommandInteraction,
    context: Context,
    database: Arc<DatabaseConnection>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating event");
    // Get the name parameter
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

    // Create a category for the event
    let category = guild
        .create_channel(&context.http, |c| {
            c.name(event_name).kind(ChannelType::Category)
        })
        .await?;

    // Create the main channel for people to join the event in
    let channel = guild
        .create_channel(&context.http, |c| {
            c.name("rules")
                .kind(ChannelType::Text)
                // Make sure this channel is inside the category we
                // just created
                .category(category.id)
        })
        .await?;

    // Create the event for the database
    let event = event::ActiveModel {
        discord_server_id: Set(application_command_interaction.guild_id.unwrap().0 as i64),
        discord_category_id: Set(category.id.0 as i64),
        discord_main_channel_id: Set(channel.id.0 as i64),
        name: Set(event_name.to_string()),
        ..Default::default()
    };

    // Insert the event into the database
    let event: event::Model = event.insert(database.as_ref()).await?;

    // Create a join button for the event
    channel
        .send_message(&context.http, |m| {
            m.content("Join the event").components(|c| {
                c.add_action_row(
                    CreateActionRow::default()
                        .add_button(
                            CreateButton::default()
                                .custom_id(
                                    serde_json::to_string(&InteractionCustomId::StartEvent {
                                        event_id: event.id,
                                    })
                                    .unwrap(),
                                )
                                .label("Join")
                                .style(ButtonStyle::Primary)
                                .to_owned(),
                        )
                        .to_owned(),
                )
            })
        })
        .await?;

    // Respond to the interaction
    application_command_interaction
        .create_interaction_response(&context.http, |r| {
            r.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|d| d.content("Event created!").ephemeral(true))
        })
        .await?;

    Ok(())
}
