#![expect(unused_variables)]

use crate::secrets::KeyName;
use poise::serenity_prelude::*;
use std::time::Instant;

#[derive(Debug)]
pub struct StartupConfig {
    pub test_guild_id: Option<GuildId>,
    pub bot_startup_channel: Option<ChannelId>,
}

#[derive(Debug)]
/// Shares immutable data across various places in the application by each just having a pointer to a leaked instance of this struct
pub struct SharedConfig {
    pub start_instant: Instant,
    pub auth_role_id: RoleId,
    pub channel_unranked: ChannelId,
    // pub db_pool: sqlx::PgPool,
}

impl StartupConfig {
    pub fn new() -> Self {
        Self {
            test_guild_id: KeyName::TestGuildId.get_non_secret_parse_opt(),
            bot_startup_channel: KeyName::StartupMsgChannel.get_non_secret_parse_opt(),
        }
    }

    pub fn is_production(&self) -> bool {
        self.test_guild_id.is_none()
    }
}

impl Default for StartupConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl SharedConfig {
    pub fn try_new() -> anyhow::Result<&'static Self> {
        let auth_role_id = KeyName::AuthRoleId.get_non_secret_parse()?;
        let channel_unranked = KeyName::CohortChannel.get_non_secret_parse()?;
        let result = Box::new(Self {
            start_instant: Instant::now(),
            auth_role_id,
            channel_unranked,
        });
        Ok(Box::leak(result))
    }

    /// Doesn't actually perform the save but spawns a task to do it in the background    
    pub fn save_kv<T: serde::Serialize>(&self, key: &str, value: &T) -> anyhow::Result<()> {
        Ok(())
        // TODO: Connect to DB
        // let pool = self.db_pool.clone();
        // let key = key.to_string();
        // let value = serde_json::to_string(value).context("failed to convert to json")?;
        // tokio::spawn(async move {
        //     let query = sqlx::query!(
        //         "\
        //         INSERT INTO kv_store (id, content)
        //         VALUES ($1, $2)
        //         ON CONFLICT(id)
        //         DO UPDATE SET
        //         content = EXCLUDED.content;",
        //         key,
        //         value
        //     );
        //     match query.execute(&pool).await {
        //         Ok(query_result) => {
        //             if query_result.rows_affected() == 1 {
        //                 info!("Save completed for key: {key}");
        //             } else {
        //                 error!(
        //                     ?key,
        //                     "Expected 1 row to be affected by save but got: {}",
        //                     query_result.rows_affected()
        //                 )
        //             }
        //         }
        //         Err(err_msg) => error!(
        //             ?err_msg,
        //             "Failed to save content for key: {key} to kv store"
        //         ),
        //     }
        // });
        // Ok(())
    }

    pub async fn load_or_default_kv<T: serde::de::DeserializeOwned + Default>(
        &self,
        key: &str,
    ) -> T {
        T::default()
        // TODO: Connect to DB
        /*
        let record_opt = match sqlx::query!("SELECT content FROM kv_store where id = $1", key)
            .fetch_optional(&self.db_pool)
            .await
        {
            Ok(content) => content,
            Err(err_msg) => {
                error!(?err_msg, "Failed to get content for key: {key}");
                None
            }
        };
        let record = match record_opt {
            Some(record) => record,
            None => {
                info!("No content found in DB for key: {key}");
                return T::default();
            }
        };
        match serde_json::from_str(&record.content) {
            Ok(x) => x,
            Err(err_msg) => {
                error!(?err_msg, ?record.content, "Failed to convert content extracted from the database");
                T::default()
            }
        }
         */
    }
}
