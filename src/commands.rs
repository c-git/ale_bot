use tracing::{info, instrument};

// Types used by all command functions
type Context<'a> = poise::Context<'a, Data, anyhow::Error>;

// Custom user data passed to all command functions
#[derive(Debug, Default)]
pub struct Data {}

pub fn commands_list() -> Vec<poise::Command<Data, anyhow::Error>> {
    vec![help(), ping(), version()]
}

/// Show this help menu
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> anyhow::Result<()> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom: "POC Bot for ALE",
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

/// Responds with "pong"
#[poise::command(slash_command, prefix_command, track_edits)]
#[instrument(name = "ping", skip(ctx))]
pub async fn ping(ctx: Context<'_>) -> anyhow::Result<()> {
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

/// Returns the version of the bot
#[poise::command(prefix_command, track_edits, slash_command)]
#[instrument(name = "version", skip(ctx))]
pub async fn version(ctx: Context<'_>) -> anyhow::Result<()> {
    let msg = format!("Bot version is {}", env!("CARGO_PKG_VERSION"));
    info!(msg);
    ctx.say(msg).await?;
    Ok(())
}
