mod commands;
mod startup;

use anyhow::Context as _;
use commands::{Data, commands_list};
use poise::serenity_prelude as serenity;
use startup::StartupConfig;
use tracing::{info, warn};
use tracing_subscriber::{
    EnvFilter,
    fmt::{self, format::FmtSpan},
    prelude::*,
};

pub async fn start_bot() {
    tracing_subscriber::registry()
        .with(fmt::layer().with_span_events(FmtSpan::NEW | FmtSpan::CLOSE))
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("zbus=warn,serenity=warn,info")),
        )
        .init();

    info!("Bot version is {}", env!("CARGO_PKG_VERSION"));

    // Load startup configuration
    let startup_config = StartupConfig::new();

    // FrameworkOptions contains all of poise's configuration option in one struct
    // Every option can be omitted to use its default value
    let options = poise::FrameworkOptions {
        commands: commands_list(),
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .setup(move |ctx, ready, framework| {
            Box::pin(async move {
                if startup_config.is_production() {
                    info!("Production run detected going to register commands globally");
                    poise::builtins::register_globally(ctx, &framework.options().commands)
                        .await
                        .context("failed to register the bot globally")?;
                } else if let Some(guild_id) = startup_config.test_guild_id {
                    info!("Development run detected going to register commands for guild: {guild_id}");
                    poise::builtins::register_in_guild(
                        ctx,
                        &framework.options().commands,
                        guild_id,
                    )
                    .await
                    .with_context(|| {
                        format!(
                            "failed to register slash commands for {:?} in guild: {guild_id}",
                            ready.user.name
                        )
                    })?;
                } else {
                    unreachable!("Assumption that providing a test guild id determines if it is a test run violated")
                }
                let connect_msg = format!("{} is connected! Version: {}", ready.user.name, env!("CARGO_PKG_VERSION"));
                info!("{connect_msg}");
                if let Some(channel) = startup_config.bot_startup_channel{
                    channel.say(ctx, connect_msg).await?;
                } else{
                    warn!("Not sending connection notification because `bot_startup_channel` not set");
                }
                info!("END OF SETUP CLOSURE");
                Ok(Data::default())
            })
        })
        .options(options)
        .build();

    let token =
        std::env::var("TOKEN").expect("Missing `TOKEN` env var, see README for more information.");
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
