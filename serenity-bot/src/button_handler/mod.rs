use migration::sea_orm::DatabaseConnection;
use serenity::{
    async_trait,
    model::prelude::interaction::Interaction,
    prelude::{Context, EventHandler},
};
use std::sync::Arc;

pub struct CustomHandler {
    pub database: Arc<DatabaseConnection>,
}

#[async_trait]
impl EventHandler for CustomHandler {
    async fn interaction_create(&self, _ctx: Context, interaction: Interaction) {
        // Print out the interaction id

        if let Interaction::MessageComponent(message) = interaction {
            let component = message.message.id;
            println!("{}", component);
        }
    }
}
