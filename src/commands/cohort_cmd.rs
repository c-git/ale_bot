//! Groups the commands related to the unranked challenge

use poise::serenity_prelude::{CacheHttp, ChannelId};
use tracing::instrument;

use self::interested_list::register;
use crate::{
    Context, Data,
    commands::{call_to_parent_command, is_auth, tracing_handler_start},
};

mod interested_list;

#[poise::command(
    prefix_command,
    slash_command,
    track_edits,
    aliases("ur"),
    subcommand_required,
    subcommands("register", "start_event")
)]
#[instrument(name = "unranked", skip(ctx))]
/// Commands related to the Unranked Challenge [aliases("ur")]
pub async fn unranked(ctx: Context<'_>) -> anyhow::Result<()> {
    call_to_parent_command(ctx).await
}

#[poise::command(hide_in_help, prefix_command, guild_only = true, check = "is_auth")]
#[instrument(name = "unranked-start_event", skip(ctx))]
/// Resets ideas and scores for the start of the new event and sets the message with the leading idea
pub async fn start_event(ctx: Context<'_>) -> anyhow::Result<()> {
    tracing_handler_start(&ctx).await;
    ctx.reply("Request started").await?;
    do_start_event(ctx, ctx.channel_id(), ctx.data()).await?;
    Ok(())
}

#[instrument(skip(_cache_http, _data))]
pub async fn do_start_event(
    _cache_http: impl CacheHttp,
    channel_id: ChannelId,
    _data: &Data,
) -> anyhow::Result<()> {
    todo!()
}
