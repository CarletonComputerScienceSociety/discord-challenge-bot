use std::env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use chrono::offset::Utc;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::{Activity, Ready};
use serenity::model::id::{ChannelId, GuildId};
use serenity::model::prelude::Interaction;
use serenity::model::prelude::application_command::ApplicationCommandInteractionDataOptionValue;
use serenity::prelude::*;

impl Handler {
    async fn interaction_create(
        &self,
        context: Context,
        interaction: Interaction,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Get the slash command, or return if it's not a slash command.
        let slash_command = if let Some(slash_command) = interaction.application_command() {
            slash_command
        } else {
            return Ok(());
        };

        if let Err(e) = slash_command.channel_id.to_channel(&context).await {
            warn!("Error getting channel: {:?}", e);
        };

        match &slash_command.data.name[..] {
            REVIEW_STRING => {
                let merge_request_number = slash_command
                    .data
                    .options
                    .get(0)
                    .expect("Expected int option")
                    .resolved
                    .as_ref()
                    .expect("Expected int object");

                if let CommandDataOptionValue::Integer(number) =
                    merge_request_number
                {
                    self.open_discord_mr_thread(context, slash_command.clone(), *number as i64)
                        .await?
                } else {
                    warn!("Merge request isn't a number");
                    return Ok(());
                }
            }
            APPROVE_STRING => {
                let merge_request_number = slash_command
                    .data
                    .options
                    .get(0)
                    .expect("Expected int option")
                    .resolved
                    .as_ref()
                    .expect("Expected int object");

                if let CommandDataOptionValue::Integer(number) =
                    merge_request_number
                {
                    self.approve_mr(context, slash_command.clone(), *number as i64)
                        .await?
                } else {
                    warn!("Merge request isn't a number");
                    return Ok(());
                }
            }
            VERSION_STRING => {
                slash_command
                    .create_interaction_response(&context.http, |response| {
                        response
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|message| {
                                message.content(
                                    MessageBuilder::new()
                                        // This will show an error with
                                        // rust-analyzer, but it compiles just fine
                                        // https://github.com/rust-analyzer/rust-analyzer/issues/6835
                                        //
                                        // Example output: `git:efe04ac-modified`
                                        .push(git_version!(prefix = "git:", fallback = "unknown"))
                                        .build(),
                                )
                            })
                    })
                    .await?;
            }
            _ => {
                warn!("should not happen");
                return Ok(());
            }
        }

        Ok(())
    }
}