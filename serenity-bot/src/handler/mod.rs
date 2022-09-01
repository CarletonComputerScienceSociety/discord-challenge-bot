use std::env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use chrono::offset::Utc;
use serenity::async_trait;
use serenity::model::application::interaction::Interaction;
use serenity::model::channel::Message;
use serenity::model::gateway::{Activity, Ready};
use serenity::model::id::{ChannelId, GuildId};
use serenity::model::prelude::application_command::{
    ApplicationCommandInteractionDataOptionValue, ApplicationCommandOptionType,
};
use serenity::model::prelude::command::CommandOptionType;
use serenity::prelude::*;
use tracing::{error, info, trace};

mod interaction;

pub struct Handler {
    pub is_loop_running: AtomicBool,
}

pub const VELOREN_SERVER_ID: u64 = 345993194322001923;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, context: Context, interaction: Interaction) {
        if let Err(e) = self.interaction_create2(context, interaction).await {
            error!(?e, "Error while processing message");
        }
    }

    async fn ready(&self, context: Context, ready: Ready) {
        let name = ready.user.name;
        info!(?name, "is connected!");

        // Create the review command for the Veloren server
        if let Err(e) = GuildId(VELOREN_SERVER_ID)
            .create_application_command(&context.http, |command| {
                command
                    .name("review")
                    .description("Review an MR")
                    .create_option(|option| {
                        option
                            .name("id")
                            .description("The MR to review")
                            .kind(CommandOptionType::Integer)
                            .required(true)
                    })
            })
            .await
        {
            error!(?e, "Error while creating the review command");
        }

        if let Err(e) = GuildId(VELOREN_SERVER_ID)
            .set_application_commands(&context.http, |commands| {
                commands
                    // Command to create a thread in a channel and ping @Code
                    // Reviewers. It requires the @Contributor role, and will
                    // get the name of the MR from the Gitlab API.
                    .create_application_command(|command| {
                        command
                            .name("review")
                            .description("Create a thread in this channel and ping @Code Reviewers. Requires the @Contributor role.")
                            .create_option(|option| {
                                option
                                    .name("id")
                                    .description("The MR number to review")
                                    .kind(CommandOptionType::Integer)
                                    .required(true)
                            })
                    })
                    // Approve (or revoke) command to get the bot to comment on
                    // an MR. Requires the @Contributor role.
                    .create_application_command(|command| {
                        command
                            .name("approve")
                            .description("Add an approval comment to an MR. Requires the @Contributor role.")
                            .create_option(|option| {
                                option
                                    .name("id")
                                    .description("The MR number to approve")
                                    .kind(CommandOptionType::Integer)
                                    .required(true)
                            })
                    })
                    // Get the current Git version that labbot is running on
                    .create_application_command(|command| {
                        command
                            .name("labbot-version")
                            .description("Check the Git commit that labbot is on")
                    })
            })
            .await
        {
            error!(?e, "Error while creating the review command");
        }
    }

    // This mostly came from the Serenity docs
    // https://github.com/serenity-rs/serenity/blob/current/examples/e13_parallel_loops/src/main.rs
    async fn cache_ready(&self, context: Context, _guilds: Vec<GuildId>) {
        trace!("Cache built successfully!");
    }
}
