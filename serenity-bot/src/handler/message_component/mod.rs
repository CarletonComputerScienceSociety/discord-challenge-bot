use super::interaction::InteractionCustomId;
use entity::entities::{participant, participant::Entity as ParticipantEntity};
use migration::{
    sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set},
    Condition,
};
use serenity::{
    model::prelude::interaction::{
        message_component::MessageComponentInteraction, InteractionResponseType,
    },
    prelude::Context,
};
use std::sync::Arc;

pub struct MessageComponentHandler {
    pub context: Context,
    pub message_component_interaction: MessageComponentInteraction,
    pub database: Arc<DatabaseConnection>,
}

impl MessageComponentHandler {
    pub async fn handle_command(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Check if message component is a button
        let data = &self.message_component_interaction.data;

        let custom_id: String = data.custom_id.clone();

        // This will parse a custom string that the message component stored.
        // This should guarentee type safety over bot restarts, since
        // everything is stored in the database.
        match serde_json::from_str::<InteractionCustomId>(&custom_id).unwrap() {
            InteractionCustomId::StartEvent { event_id } => {
                self.handle_join_button(event_id).await?
            }
        };

        // If it's a join button, check if the user is already in the event

        // If not, add them to this event

        Ok(())
    }

    /// Handle the join button for an event being pressed. If the user is
    /// already part of this event, it should return an error.
    pub async fn handle_join_button(
        &self,
        event_id: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Make sure this participany isn't already in the event
        if ParticipantEntity::find()
            .filter(
                Condition::all()
                    .add(participant::Column::EventId.contains(&event_id.to_string()))
                    .add(
                        participant::Column::DiscordId
                            .contains(&self.message_component_interaction.user.id.0.to_string()),
                    ),
            )
            .all(self.database.as_ref())
            .await?
            .len()
            > 0
        {
            // Notify the user that they're already in the event
            self.message_component_interaction
                .create_interaction_response(&self.context.http, |r| {
                    r.kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|d| {
                            d.content("You already joined this event!").ephemeral(true)
                        })
                })
                .await?;

            return Ok(());
        }

        // Create a new participant for the event
        let participant = participant::ActiveModel {
            event_id: Set(event_id.to_string()),
            discord_id: Set(self.message_component_interaction.user.id.to_string()),
            ..Default::default()
        };

        // Insert the participant into the database
        let participant: participant::Model = participant.insert(self.database.as_ref()).await?;

        // Notify the user that they have joined the event
        self.message_component_interaction
            .create_interaction_response(&self.context.http, |r| {
                r.kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|d| {
                        d.content("You have joined the event!").ephemeral(true)
                    })
            })
            .await?;

        Ok(())
    }
}
