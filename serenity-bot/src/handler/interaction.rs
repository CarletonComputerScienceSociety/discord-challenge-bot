






use serenity::model::application::interaction::Interaction;






use serenity::prelude::*;

use tracing::warn;

use super::Handler;

const REVIEW_STRING: &str = "review";
const APPROVE_STRING: &str = "approve";
const VERSION_STRING: &str = "labbot-version";

impl Handler {
    pub async fn interaction_create2(
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
                let _merge_request_number = slash_command
                    .data
                    .options
                    .get(0)
                    .expect("Expected int option")
                    .resolved
                    .as_ref()
                    .expect("Expected int object");
            }
            APPROVE_STRING => {
                let _merge_request_number = slash_command
                    .data
                    .options
                    .get(0)
                    .expect("Expected int option")
                    .resolved
                    .as_ref()
                    .expect("Expected int object");
            }
            _ => {
                warn!("should not happen");
                return Ok(());
            }
        }

        Ok(())
    }
}
