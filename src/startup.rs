use std::{fmt::Debug, str::FromStr};

use poise::serenity_prelude::{ChannelId, GuildId};
use tracing::error;

#[derive(Debug)]
pub struct StartupConfig {
    pub test_guild_id: Option<GuildId>,
    pub bot_startup_channel: Option<ChannelId>,
}

impl StartupConfig {
    pub fn new() -> Self {
        Self {
            test_guild_id: parse_value_from_env("TEST_GUILD_ID"),
            bot_startup_channel: parse_value_from_env("STARTUP_MSG_CHANNEL"),
        }
    }
    pub fn is_production(&self) -> bool {
        self.test_guild_id.is_none()
    }
}

fn parse_value_from_env<R, E>(env_key: &str) -> Option<R>
where
    E: Debug,
    R: FromStr<Err = E>,
{
    std::env::var(env_key)
        .ok()
        .and_then(|x| match x.parse::<R>() {
            Ok(x) => Some(x),
            Err(err) => {
                error!(?err, "failed to parse {env_key:?}");
                None
            }
        })
}

pub fn parse_value_from_env_expect<R, E>(env_key: &str) -> R
where
    E: Debug,
    R: FromStr<Err = E>,
{
    match parse_value_from_env(env_key) {
        Some(x) => x,
        None => panic!(
            "Missing required environment variable `{env_key}`, see README for more information."
        ),
    }
}
