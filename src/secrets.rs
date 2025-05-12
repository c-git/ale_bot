//! This modules handles access to all the secret information

use anyhow::Context;
use secrecy::SecretString;
use std::str::FromStr;
use tracing::warn;

pub enum KeyName {
    DiscordToken,

    /// Used for testing to register the commands directly for the guild
    TestGuildId,

    /// The RoleId of the role that can run privileged commands
    AuthRoleId,

    /// The channel to be used for cohort messages
    CohortChannel,

    /// The chanel to use for the startup message
    StartupMsgChannel,
}

impl AsRef<str> for KeyName {
    fn as_ref(&self) -> &str {
        match self {
            KeyName::DiscordToken => "TOKEN",
            KeyName::TestGuildId => "TEST_GUILD_ID",
            KeyName::AuthRoleId => "AUTH_ROLE_ID",
            KeyName::CohortChannel => "COHORT_CHANNEL",
            KeyName::StartupMsgChannel => "STARTUP_MSG_CHANNEL",
        }
    }
}

impl KeyName {
    pub fn get_secret_string(&self) -> anyhow::Result<SecretString> {
        Ok(SecretString::new(
            get_env_var(self.as_ref())
                .with_context(|| format!("failed to get environment variable: {}", self.as_ref()))?
                .into(),
        ))
    }

    pub fn get_non_secret_string(&self) -> anyhow::Result<String> {
        get_env_var(self.as_ref())
    }

    pub fn get_non_secret_parse<F, E>(&self) -> anyhow::Result<F>
    where
        E: 'static + Send + Sync + std::error::Error,
        F: FromStr<Err = E>,
    {
        self.get_non_secret_string()?
            .parse()
            .with_context(|| format!("failed to parse environment variable: {}", self.as_ref()))
    }

    pub fn get_non_secret_parse_opt<F, E>(&self) -> Option<F>
    where
        E: 'static + Send + Sync + std::error::Error,
        F: FromStr<Err = E>,
    {
        match self.get_non_secret_parse() {
            Ok(x) => Some(x),
            Err(_) => {
                warn!(
                    "failed to optionally load {}. Defaulting to use None instead",
                    self.as_ref(),
                );
                None
            }
        }
    }
}

fn get_env_var(key: &str) -> anyhow::Result<String> {
    std::env::var(key).with_context(|| format!("failed to get environment variable: {key}"))
}
