//! This is the library for the application. The majority of the logic can be found here
//! It is split into two main parts. The parts that receive commands from discord [`commands`] and
//! the part that handles the actual logic of what to do in the [`model`]

#![warn(unused_crate_dependencies)]

use anyhow::Context as _;
use poise::serenity_prelude as serenity;
use secrecy::ExposeSecret;
use secrets::KeyName;
use tracing::{info, instrument, warn};
use tracing_subscriber::{
    EnvFilter,
    fmt::{self, format::FmtSpan},
    prelude::*,
};

pub use self::{
    commands::commands_list,
    config::{SharedConfig, StartupConfig},
    model::Data,
};

mod commands;
mod config;
mod model;
mod secrets;

/// Type used by poise framework as the context when commands are triggered
type Context<'a> = poise::Context<'a, Data, anyhow::Error>;

trait RemoveElement<T: PartialEq> {
    /// Returns true iff the element was found and removed
    fn remove_element(&mut self, element: &T) -> bool;
}

impl<T: PartialEq> RemoveElement<T> for Vec<T> {
    fn remove_element(&mut self, element: &T) -> bool {
        let index = self
            .iter()
            .enumerate()
            .find_map(|(i, x)| if x == element { Some(i) } else { None });
        if let Some(i) = index {
            self.remove(i);
            true
        } else {
            false
        }
    }
}

trait AuthorPreferredDisplay {
    async fn author_preferred_display(&self) -> String;
}

impl AuthorPreferredDisplay for Context<'_> {
    async fn author_preferred_display(&self) -> String {
        match self.author_member().await {
            Some(member) => member.display_name().to_string(),
            None => self.author().name.clone(),
        }
    }
}

trait Resettable: Default {
    fn reset(&mut self) {
        *self = Default::default();
    }
}

/// Removes identified problems with inputs
/// Not trying to remove all markdown just the parts that are
/// likely to cause issues. More will be added as needed
#[must_use]
#[instrument]
fn sanitize_markdown(s: String) -> String {
    const PATTERNS: [&str; 4] = ["**", "__", "```", "\n"];
    let mut result = s;
    for pattern in PATTERNS.iter() {
        result = result.replace(pattern, "");
    }
    info!(result);
    result
}

pub async fn start_bot() {
    tracing_subscriber::registry()
        .with(fmt::layer().with_span_events(FmtSpan::NEW | FmtSpan::CLOSE))
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("zbus=warn,serenity=warn,info")),
        )
        .init();

    info!("Bot version is {}", env!("CARGO_PKG_VERSION"));

    #[cfg(debug_assertions)]
    dotenv::dotenv().ok(); // Load environment variables

    // Load startup configuration
    let startup_config = StartupConfig::new();
    info!(?startup_config);

    let shared_config = SharedConfig::try_new().expect("failed to created shared_config");

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
                let data = Data::new(shared_config, ctx.clone()).await;
                info!("END OF SETUP CLOSURE");
                Ok(data)
            })
        })
        .options(options)
        .build();

    let token = KeyName::DiscordToken
        .get_secret_string()
        .expect("failed to get discord token");
    let intents = serenity::GatewayIntents::non_privileged();

    let client = serenity::ClientBuilder::new(token.expose_secret(), intents)
        .framework(framework)
        .await;

    client
        .expect("failed to create client")
        .start()
        .await
        .expect("failed to start client")
}
