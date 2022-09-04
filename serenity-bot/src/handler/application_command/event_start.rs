use crate::handler::interaction::InteractionCustomId;
use db_entity::entities::{
    event, event::Entity as EventEntity, participant, participant::Entity as ParticipantEntity,
    team, team::Entity as TeamEntity,
};
use log::warn;
use migration::{sea_orm::*, Condition};
use rand::seq::SliceRandom;
use rnglib::{Language, RNG};
use serenity::{
    model::{
        application::interaction::InteractionResponseType,
        prelude::interaction::application_command::{
            ApplicationCommandInteraction, CommandDataOptionValue,
        },
    },
    prelude::*,
};
use std::sync::Arc;

pub struct ApplicationCommandHandler;

pub async fn handle_event_start_command(
    application_command_interaction: ApplicationCommandInteraction,
    context: Context,
    database: Arc<DatabaseConnection>,
) -> Result<(), Box<dyn std::error::Error>> {
    let command_options = &application_command_interaction.data.options;

    let team_count = match command_options
        .get(0)
        .expect("No member count provided")
        .resolved
        .as_ref()
        .expect("No member count provided")
    {
        CommandDataOptionValue::Integer(i) => i,
        _ => {
            warn!("Invalid event name");
            return Ok(());
        }
    };

    let event_name = match command_options
        .get(1)
        .expect("No event name provided")
        .resolved
        .as_ref()
        .expect("No event name provided")
    {
        CommandDataOptionValue::String(s) => s,
        _ => {
            warn!("Invalid event name");
            return Ok(());
        }
    };

    // TODO: Make sure that the user has perms to do this
    
    // Get all the events and print them
    let events = EventEntity::find()
        .all(database.as_ref())
        .await?;

    for event in events {
        println!("{:?}", event);
    }

    // Print the query
    let query = EventEntity::find()
        .filter(
            Condition::all()
                .add(event::Column::Name.contains(&event_name))
                // .add(
                //     event::Column::DiscordServerId.contains(
                //         &application_command_interaction
                //             .guild_id
                //             .unwrap()
                //             .0
                //             .to_string(),
                //     ),
                // ),
        )
        .build(DbBackend::Sqlite).to_string();

    println!("{}", query);

    // Make sure the event name exists in this server
    let events = EventEntity::find()
        .filter(
            Condition::all()
                .add(event::Column::Name.contains(&event_name))
                .add(
                    event::Column::DiscordServerId.contains(
                        &application_command_interaction
                            .guild_id
                            .unwrap()
                            .0
                            .to_string(),
                    ),
                ),
        )
        .all(database.as_ref())
        .await?;

    match events.len() {
        // TODO: Make sure only one event with a name can be created
        0 => {
            // Notify the user that there is no event by this name in this server
            application_command_interaction
                .create_interaction_response(context.http, |r| {
                    r.kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|d| {
                            d.content("That event doesn't exist on this server!")
                                .ephemeral(true)
                        })
                })
                .await?;
        }
        1 => {
            let event = events.get(0).unwrap();

            // Get all the participants
            let mut participants = ParticipantEntity::find()
                .filter(
                    Condition::all()
                        .add(event::Column::Name.contains(&event_name))
                        .add(
                            event::Column::DiscordServerId.contains(
                                &application_command_interaction
                                    .guild_id
                                    .unwrap()
                                    .0
                                    .to_string(),
                            ),
                        ),
                )
                .all(database.as_ref())
                .await?;

            let number_of_participants = participants.len();
            let number_of_teams = *team_count as usize;

            let number_of_participants_per_team = number_of_participants / number_of_teams;

            // Randomize them into teams with random names
            participants.shuffle(&mut rand::thread_rng());

            // Seed the team name generator
            let rng = RNG::new(&Language::Fantasy).unwrap();

            let mut teams = Vec::new();

            for team_number in 0..number_of_teams {
                let mut team_participants = Vec::new();

                for _ in 0..number_of_participants_per_team {
                    team_participants.push(participants.remove(0));
                }

                let team_name = format!("Team {}", rng.generate_name());

                // Create roles for each team
                let team_role = application_command_interaction
                    .guild_id
                    .unwrap()
                    .create_role(&context.http, |r| r.name(&team_name))
                    .await?;

                // Create channels for each team
                let team_channel = application_command_interaction
                    .guild_id
                    .unwrap()
                    .create_channel(&context.http, |c| {
                        c.name(&team_name)
                            .kind(serenity::model::channel::ChannelType::Text)
                            .category(event.discord_category_id.parse::<u64>().unwrap())
                    })
                    .await?;

                // Create the event for the database
                teams.push(team::ActiveModel {
                    discord_channel_id: Set(team_channel.id.0.to_string()),
                    event_id: Set(event.id),
                    team_channel_id: Set(team_channel.id.0.to_string()),
                    team_role_id: Set(team_role.id.0.to_string()),
                    ..Default::default()
                });

                // TODO: Add each participant to this team
                for participant in team_participants {
                    let mut participant: participant::ActiveModel = participant.into();
                    participant.team_id = Set(Some(team_number.to_string()));
                    participant.update(database.as_ref()).await?;
                }
            }

            // Add the teams to the database
            TeamEntity::insert_many(teams)
                .exec(database.as_ref())
                .await?;

            // Respond to the interaction
            application_command_interaction
                .create_interaction_response(&context.http, |r| {
                    r.kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|d| d.content("Event started!").ephemeral(true))
                })
                .await?;
        }
        _ => unreachable!(),
    }

    Ok(())
}
