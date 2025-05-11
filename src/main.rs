#![warn(clippy::str_to_string)]

mod commands;

use commands::commands_list;
use poise::serenity_prelude as serenity;
use std::{collections::HashMap, env::var, sync::Mutex};
use tracing::info;
use tracing_subscriber::{
    EnvFilter,
    fmt::{self, format::FmtSpan},
    prelude::*,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer().with_span_events(FmtSpan::ACTIVE))
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("zbus=warn,serenity=warn,info")),
        )
        .init();

    info!("Bot version is {}", env!("CARGO_PKG_VERSION"));

    // FrameworkOptions contains all of poise's configuration option in one struct
    // Every option can be omitted to use its default value
    let options = poise::FrameworkOptions {
        commands: commands_list(),
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .setup(move |ctx, ready, framework| {
            Box::pin(async move {
                let connect_msg = format!(
                    "{} is connected! Version: {}",
                    ready.user.name,
                    env!("CARGO_PKG_VERSION")
                );
                info!("{connect_msg}");
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(commands::Data {
                    votes: Mutex::new(HashMap::new()),
                })
            })
        })
        .options(options)
        .build();

    let token = var("TOKEN").expect("Missing `TOKEN` env var, see README for more information.");
    let intents = serenity::GatewayIntents::non_privileged();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client
        .expect("failed to create client")
        .start()
        .await
        .expect("failed to start client")
}
