//! Groups the functionality related to accountability cohorts

use crate::{config::SharedConfig, model::cohort::interested_list::InterestedList};
use std::sync::{Arc, Mutex};

pub mod interested_list;

pub struct Cohort {
    scores: Arc<Mutex<InterestedList>>,
    shared_config: &'static SharedConfig,
}

impl Cohort {
    pub async fn new(shared_config: &'static SharedConfig) -> Self {
        let scores = Arc::new(Mutex::new(InterestedList::new(shared_config).await));
        Self {
            scores,
            shared_config,
        }
    }

    fn save<T: serde::Serialize>(&self, key: &str, value: &T) -> anyhow::Result<()> {
        self.shared_config.save_kv(key, value)
    }
}
