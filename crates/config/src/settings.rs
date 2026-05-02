use std::num::NonZeroU64;

use serde::Deserialize;

use crate::ConfigError;

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub redpanda: RedpandaConfig,
    pub pollers: Vec<PollerConfig>,
}

/// Redpanda broker connection settings.
#[derive(Debug, Clone, Deserialize)]
pub struct RedpandaConfig {
    /// Comma-separated broker list — e.g. `localhost:9092`
    pub brokers: String,
    /// Default topic to produce news items to
    pub topic: String,
    /// Consumer group id
    pub group_id: String,
}

/// Per-query RSS poller settings.
/// One poller instance is spawned per entry.
#[derive(Debug, Clone, Deserialize)]
pub struct PollerConfig {
    /// Google News search query — e.g. `"artificial intelligence"`
    pub query: String,
    /// How often to poll the feed, in seconds
    pub interval_secs: NonZeroU64,
    /// Max retry attempts on HTTP failure before giving up (exponential backoff)
    pub max_retries: u32,
    /// Google News host language — e.g. `en-US`, `fr-FR`
    #[serde(default = "default_hl")]
    pub hl: String,
    /// Google News geographic location — e.g. `US`, `FR`
    #[serde(default = "default_gl")]
    pub gl: String,
    /// Google News edition — e.g. `US:en`, `FR:fr`
    #[serde(default = "default_ceid")]
    pub ceid: String,
}

impl Settings {
    /// Load settings from `config.toml` + `INJECTOR__`-prefixed env vars.
    ///
    /// Env vars override scalar values from the file.
    /// `pollers` array must be defined in `config.toml` — env vars cannot express arrays.
    pub fn load() -> Result<Self, ConfigError> {
        let s = config::Config::builder()
            .add_source(config::File::with_name("config").required(false))
            .add_source(config::Environment::with_prefix("INJECTOR").separator("__"))
            .build()?;

        Ok(s.try_deserialize()?)
    }
}

fn default_hl() -> String {
    "en-US".to_owned()
}

fn default_gl() -> String {
    "US".to_owned()
}

fn default_ceid() -> String {
    "US:en".to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn redpanda_config_should_deserialize() {
        let raw = r#"
            [redpanda]
            brokers   = "localhost:9092"
            topic     = "injector.news"
            group_id  = "injector"

            [[pollers]]
            query         = "artificial intelligence"
            interval_secs = 60
            max_retries   = 3
        "#;

        let s: Settings = config::Config::builder()
            .add_source(config::File::from_str(raw, config::FileFormat::Toml))
            .build()
            .unwrap()
            .try_deserialize()
            .unwrap();

        assert_eq!(s.redpanda.brokers, "localhost:9092");
        assert_eq!(s.redpanda.topic, "injector.news");
        assert_eq!(s.pollers.len(), 1);
        assert_eq!(s.pollers[0].query, "artificial intelligence");
        assert_eq!(s.pollers[0].interval_secs.get(), 60);
        assert_eq!(s.pollers[0].max_retries, 3);
        assert_eq!(s.pollers[0].hl, "en-US");
        assert_eq!(s.pollers[0].gl, "US");
        assert_eq!(s.pollers[0].ceid, "US:en");
    }

    #[test]
    fn settings_should_support_multiple_pollers() {
        let raw = r#"
            [redpanda]
            brokers  = "localhost:9092"
            topic    = "injector.news"
            group_id = "injector"

            [[pollers]]
            query         = "rust programming"
            interval_secs = 30
            max_retries   = 5

            [[pollers]]
            query         = "machine learning"
            interval_secs = 120
            max_retries   = 3
        "#;

        let s: Settings = config::Config::builder()
            .add_source(config::File::from_str(raw, config::FileFormat::Toml))
            .build()
            .unwrap()
            .try_deserialize()
            .unwrap();

        assert_eq!(s.pollers.len(), 2);
        assert_eq!(s.pollers[1].query, "machine learning");
    }

    #[test]
    fn poller_config_should_reject_zero_interval() {
        let raw = r#"
            [redpanda]
            brokers  = "localhost:9092"
            topic    = "injector.news"
            group_id = "injector"

            [[pollers]]
            query         = "rust programming"
            interval_secs = 0
            max_retries   = 3
        "#;

        let result = config::Config::builder()
            .add_source(config::File::from_str(raw, config::FileFormat::Toml))
            .build()
            .unwrap()
            .try_deserialize::<Settings>();

        assert!(result.is_err());
    }

    #[test]
    fn poller_config_should_deserialize_configured_locale() {
        let raw = r#"
            [redpanda]
            brokers  = "localhost:9092"
            topic    = "injector.news"
            group_id = "injector"

            [[pollers]]
            query         = "intelligence artificielle"
            interval_secs = 60
            max_retries   = 3
            hl            = "fr-FR"
            gl            = "FR"
            ceid          = "FR:fr"
        "#;

        let s: Settings = config::Config::builder()
            .add_source(config::File::from_str(raw, config::FileFormat::Toml))
            .build()
            .unwrap()
            .try_deserialize()
            .unwrap();

        assert_eq!(s.pollers[0].hl, "fr-FR");
        assert_eq!(s.pollers[0].gl, "FR");
        assert_eq!(s.pollers[0].ceid, "FR:fr");
    }
}
