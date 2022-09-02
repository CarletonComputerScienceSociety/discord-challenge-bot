use log::{error, info, trace};
use migration::sea_orm::DatabaseConnection;
use serenity::{
    async_trait,
    model::{
        application::interaction::Interaction, gateway::Ready, id::GuildId,
        prelude::command::CommandOptionType,
    },
    prelude::*,
};
use std::sync::{atomic::AtomicBool, Arc};
use strum_macros::{Display, EnumString, IntoStaticStr};
mod application_command;
mod interaction;
mod message_component;

pub struct Handler {
    pub database: Arc<DatabaseConnection>,
    pub is_loop_running: AtomicBool,
}

pub const VELOREN_SERVER_ID: u64 = 345993194322001923;

#[derive(IntoStaticStr, EnumString, Display)]
pub enum Command {
    #[strum(serialize = "event-start")]
    EventStart,
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, context: Context, interaction: Interaction) {
        if let Err(e) = self.interaction_create(context, interaction).await {
            error!("Error handling interaction: {:?}", e);
        }
    }

    async fn ready(&self, context: Context, ready: Ready) {
        let name = ready.user.name;
        info!("{} is connected!", name);

        if let Err(e) = GuildId(VELOREN_SERVER_ID)
            .set_application_commands(&context.http, |commands| {
                // Start event command
                commands.create_application_command(|command| {
                    command
                        .name(Command::EventStart)
                        .description("Start a challenge event")
                        .create_option(|option| {
                            // Option to get name of event
                            option
                                .name("event-start")
                                .description("Start a challenge event")
                                .kind(CommandOptionType::String)
                                .required(true)
                        })
                })
            })
            .await
        {
            error!("Error setting application commands: {:?}", e);
        }
    }

    // This mostly came from the Serenity docs
    // https://github.com/serenity-rs/serenity/blob/current/examples/e13_parallel_loops/src/main.rs
    async fn cache_ready(&self, _context: Context, _guilds: Vec<GuildId>) {
        trace!("Cache built successfully!");
    }
}
