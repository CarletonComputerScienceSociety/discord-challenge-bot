use crate::handler::interaction::InteractionCustomId;
use db_entity::entities::{event, event::Entity as EventEntity};
use log::warn;
use migration::{sea_orm::*, Condition};
use serenity::{
    model::{
        application::interaction::InteractionResponseType,
        prelude::interaction::application_command::{
            ApplicationCommandInteraction, CommandDataOptionValue,
        },
    },
    prelude::*,
};
use std::sync::Arc;

pub struct ApplicationCommandHandler;

pub async fn handle_event_start_command(
    application_command_interaction: ApplicationCommandInteraction,
    context: Context,
    database: Arc<DatabaseConnection>,
) -> Result<(), Box<dyn std::error::Error>> {
    let command_options = &application_command_interaction.data.options;

    let team_count = match command_options
        .get(0)
        .expect("No member count provided")
        .resolved
        .as_ref()
        .expect("No member count provided")
    {
        CommandDataOptionValue::Integer(i) => i,
        _ => {
            warn!("Invalid event name");
            return Ok(());
        }
    };

    let event_name = match command_options
        .get(1)
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

    // // Make sure the event name exists in this server
    // if EventEntity::find()
    //     .filter(
    //         Condition::all()
    //             .add(event::Column::Name.contains(&event_name))
    //             .add(
    //                 event::Column::DiscordId
    //                     .contains(&self.message_component_interaction.user.id.0.to_string()),
    //             ),
    //     )
    //     .all(self.database.as_ref())
    //     .await?
    //     .len()
    //     > 0
    // {
    //     // Notify the user that they're already in the event
    //     self.message_component_interaction
    //         .create_interaction_response(&self.context.http, |r| {
    //             r.kind(InteractionResponseType::ChannelMessageWithSource)
    //                 .interaction_response_data(|d| {
    //                     d.content("You already joined this event!").ephemeral(true)
    //                 })
    //         })
    //         .await?;

    //     return Ok(());
    // }

    // Get all the participants
    // Randomize them into teams

    // Respond to the interaction
    application_command_interaction
        .create_interaction_response(&context.http, |r| {
            r.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|d| d.content("Event created!").ephemeral(true))
        })
        .await?;

    Ok(())
}
