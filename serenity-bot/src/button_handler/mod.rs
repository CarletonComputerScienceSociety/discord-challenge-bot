use migration::sea_orm::DatabaseConnection;
use serenity::prelude::EventHandler;
use std::sync::Arc;

pub struct CustomHandler {
    pub database: Arc<DatabaseConnection>,
}

impl EventHandler for CustomHandler {
    async fn interaction_create(
        &self,
        ctx: serenity::prelude::Context,
        interaction: serenity::model::prelude::interaction::Interaction,
    ) {
    }
}
