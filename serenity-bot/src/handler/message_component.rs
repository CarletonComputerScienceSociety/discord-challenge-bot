use migration::sea_orm::DatabaseConnection;

use serenity::model::prelude::interaction::message_component::MessageComponentInteraction;

use serenity::prelude::Context;

use std::sync::Arc;

pub struct MessageComponentHandler;

impl MessageComponentHandler {
    pub async fn handle_command(
        _context: Context,
        _application_command_interaction: MessageComponentInteraction,
        _database: Arc<DatabaseConnection>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
