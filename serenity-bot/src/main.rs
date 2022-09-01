use std::env;
use std::sync::atomic::AtomicBool;

use button_handler::CustomHandler;
use handler::Handler;

use migration::sea_orm::{Database, DatabaseConnection};
use serenity::prelude::*;
use tracing::debug;

use std::sync::Arc;
use tracing_subscriber::EnvFilter;

mod button_handler;
mod handler;

const LABBOT_ID: u64 = 451862707746897961;

fn init_tracing() {
    let _append_info = |mut f: EnvFilter, list: &[&str], level: &str| {
        for l in list {
            f = f.add_directive(format!("{}={}", l, level).parse().unwrap());
        }
        f
    };

    let _list = &[
        "tokio_util",
        "h2",
        "rustls",
        "serenity",
        "tungstenite",
        "async_tungstenite",
        "hyper",
        "trust_dns_resolver",
        "trust_dns_proto",
        "reqwest",
        "mio",
        "want",
        "kube",
        "tower",
    ];

    // let filter = EnvFilter::from_default_env();
    // let filter = append_info(filter.add_directive(Trace), list, "info");

    // tracing_subscriber::FmtSubscriber::builder()
    //     .with_max_level(Level::Trace)
    //     .with_env_filter(filter)
    //     .try_init()
    //     .unwrap();

    debug!("tracing initialized");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Connect to the database
    let db: DatabaseConnection = Database::connect("sqlite://test.db").await?;

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
        // .event_handler(CustomHandler {
        //     database: db_arc.clone(),
        // })
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        eprintln!("Client error: {:?}", why);
    }

    Ok(())
}
