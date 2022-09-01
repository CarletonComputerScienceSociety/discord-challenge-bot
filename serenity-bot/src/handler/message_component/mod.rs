use migration::sea_orm::DatabaseConnection;

use serenity::model::prelude::interaction::message_component::MessageComponentInteraction;

use serenity::prelude::Context;

use std::sync::Arc;

pub struct MessageComponentHandler;

impl MessageComponentHandler {
    pub async fn handle_command(
        context: Context,
        message_component_interaction: MessageComponentInteraction,
        database: Arc<DatabaseConnection>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Check if message component is a button
        let data = message_component_interaction.data;

        let custom_id: String = data.custom_id;

        // If it's a join button, check if the user is already in the event

        // If not, add them to this event

        Ok(())
    }
}
