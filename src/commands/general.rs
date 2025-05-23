//! Top level commands shared that are always available

use human_time::ToHumanTimeString as _;
use tracing::{info, instrument};

use crate::{Context, commands::tracing_handler_start};

/// Responds with "pong"
#[poise::command(slash_command, prefix_command, track_edits)]
#[instrument(name = "ping", skip(ctx))]
pub async fn ping(ctx: Context<'_>) -> anyhow::Result<()> {
    tracing_handler_start(&ctx).await;
    let latency = ctx.ping().await;
    if latency.is_zero() {
        info!("latency not available yet");
        ctx.say("pong!").await?;
    } else {
        let msg = format!("pong! Bot gateway heartbeat latency is {latency:?}");
        info!(msg);
        ctx.say(msg).await?;
    }
    Ok(())
}

#[poise::command(
    hide_in_help,
    slash_command,
    prefix_command,
    track_edits,
    aliases("up")
)]
#[instrument(name = "uptime", skip(ctx))]
/// Responds with how long the bot has been running for [aliases("up")]
pub async fn uptime(ctx: Context<'_>) -> anyhow::Result<()> {
    tracing_handler_start(&ctx).await;
    ctx.say(
        ctx.data()
            .inner
            .shared_config
            .start_instant
            .elapsed()
            .to_human_time_string(),
    )
    .await?;
    Ok(())
}

/// Show help menu
#[poise::command(prefix_command, track_edits, slash_command)]
#[instrument(name = "help", skip(ctx))]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> anyhow::Result<()> {
    tracing_handler_start(&ctx).await;
    let config = Default::default();
    poise::builtins::help(ctx, command.as_deref(), config).await?;
    Ok(())
}

/// Returns the version of the bot
#[poise::command(prefix_command, track_edits, slash_command)]
#[instrument(name = "version", skip(ctx))]
pub async fn version(ctx: Context<'_>) -> anyhow::Result<()> {
    tracing_handler_start(&ctx).await;
    let msg = format!("Bot version is {}", env!("CARGO_PKG_VERSION"));
    info!(msg);
    ctx.say(msg).await?;
    Ok(())
}
