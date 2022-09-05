use handler::Handler;
use log::LevelFilter;
use migration::sea_orm::{Database, DatabaseConnection};
use serenity::prelude::*;
use simple_logger::SimpleLogger;
use std::{
    env,
    sync::{atomic::AtomicBool, Arc},
};

mod button_handler;
mod handler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .with_module_level("serenity", LevelFilter::Error)
        .with_module_level("tracing::span", LevelFilter::Error)
        .init()
        .unwrap();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Connect to the database
    let db: DatabaseConnection = Database::connect("postgres://postgres:postgres@localhost:5432/postgres").await?;

    let db_arc: Arc<DatabaseConnection> = Arc::new(db);

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILDS
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler {
            database: db_arc.clone(),
            is_loop_running: AtomicBool::new(false),
        })
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        eprintln!("Client error: {:?}", why);
    }

    Ok(())
}
