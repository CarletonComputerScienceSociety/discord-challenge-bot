use std::env;
use std::sync::atomic::{AtomicBool};




use handler::Handler;




use serenity::prelude::*;
use tracing::debug;

use tracing_subscriber::EnvFilter;

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
async fn main() {
    init_tracing();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILDS
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler {
            is_loop_running: AtomicBool::new(false),
        })
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        eprintln!("Client error: {:?}", why);
    }
}
