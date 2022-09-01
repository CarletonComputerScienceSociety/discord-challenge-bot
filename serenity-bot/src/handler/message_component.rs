use entity::entities::event;
use migration::sea_orm::ActiveModelTrait;
use migration::sea_orm::DatabaseConnection;
use migration::sea_orm::Set;
use serenity::builder::CreateActionRow;
use serenity::builder::CreateButton;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::component::ButtonStyle;
use serenity::model::prelude::interaction::application_command::CommandDataOptionValue;
use serenity::model::prelude::interaction::message_component::MessageComponentInteraction;
use serenity::model::prelude::interaction::{
    application_command::ApplicationCommandInteraction, Interaction,
};
use serenity::model::prelude::ChannelType;
use serenity::prelude::Context;
use serenity::prelude::*;
use std::str::FromStr;
use std::sync::Arc;
use tracing::warn;

pub struct MessageComponentHandler;

impl MessageComponentHandler {
    pub async fn handle_command(
        context: Context,
        application_command_interaction: MessageComponentInteraction,
        database: Arc<DatabaseConnection>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
