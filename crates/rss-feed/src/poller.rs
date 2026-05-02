use std::future::Future;
use std::time::Duration;

use backon::{ExponentialBuilder, Retryable};
use injector_config::PollerConfig;
use tracing::warn;

use crate::error::{FetchError, ParseError};
use crate::feed_item::FeedItem;
use crate::fetcher::FeedFetcher;
use crate::parser::parse;
use crate::url::{GoogleNewsLocale, google_news_rss_url};

/// One Google News RSS poller for one query.
pub struct Poller {
    config: PollerConfig,
    fetcher: FeedFetcher,
}

impl Poller {
    pub fn new(config: PollerConfig) -> Self {
        Self {
            config,
            fetcher: FeedFetcher::new(),
        }
    }

    /// Fetch and parse one RSS feed snapshot.
    pub async fn poll_once(&self) -> Result<Vec<FeedItem>, FetchError> {
        let locale = GoogleNewsLocale {
            hl: self.config.hl.clone(),
            gl: self.config.gl.clone(),
            ceid: self.config.ceid.clone(),
        };
        let url = google_news_rss_url(&self.config.query, &locale);
        let bytes = self.fetch_with_backoff(&url).await?;
        let parsed = parse(&bytes, &self.config.query)?;

        collect_items(parsed)
    }

    /// Run forever, calling `on_items` after each successful poll.
    pub async fn run<F, Fut>(&self, mut on_items: F)
    where
        F: FnMut(Vec<FeedItem>) -> Fut,
        Fut: Future<Output = ()>,
    {
        let mut interval = tokio::time::interval(Duration::from_secs(self.config.interval_secs));

        loop {
            interval.tick().await;

            match self.poll_once().await {
                Ok(items) => on_items(items).await,
                Err(error) => {
                    metrics::counter!("rss_feed_poll_errors_total").increment(1);
                    warn!(query = %self.config.query, %error, "rss feed poll failed");
                }
            }
        }
    }

    async fn fetch_with_backoff(&self, url: &str) -> Result<Vec<u8>, FetchError> {
        let backoff = ExponentialBuilder::default()
            .with_max_times(self.config.max_retries as usize)
            .with_min_delay(Duration::from_secs(1))
            .with_max_delay(Duration::from_secs(30))
            .with_jitter();

        (|| async { self.fetcher.fetch_bytes(url).await })
            .retry(backoff)
            .notify(|error, delay| {
                metrics::counter!("rss_feed_fetch_retry_total").increment(1);
                warn!(%error, ?delay, "rss feed fetch failed; retrying");
            })
            .await
    }
}

fn collect_items(results: Vec<Result<FeedItem, ParseError>>) -> Result<Vec<FeedItem>, FetchError> {
    let mut items = Vec::with_capacity(results.len());

    for result in results {
        match result {
            Ok(item) => items.push(item),
            Err(error) => {
                metrics::counter!("rss_feed_parse_errors_total").increment(1);
                warn!(%error, "rss feed item parse failed");
                return Err(FetchError::Parse(error));
            }
        }
    }

    Ok(items)
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use super::*;

    fn item(guid: &str) -> FeedItem {
        FeedItem {
            guid: guid.to_owned(),
            title: "title".to_owned(),
            link: "https://example.com".to_owned(),
            description: "description".to_owned(),
            pub_date: Utc::now(),
            source_name: String::new(),
            source_url: String::new(),
            query: "query".to_owned(),
        }
    }

    #[test]
    fn collect_items_should_return_all_valid_items() {
        let items = collect_items(vec![Ok(item("1")), Ok(item("2"))]).unwrap();

        assert_eq!(items.len(), 2);
        assert_eq!(items[1].guid, "2");
    }

    #[test]
    fn collect_items_should_propagate_first_parse_error() {
        let err = collect_items(vec![
            Ok(item("1")),
            Err(ParseError::MissingTitle {
                guid: "bad".to_owned(),
            }),
            Ok(item("2")),
        ])
        .unwrap_err();

        assert!(matches!(
            err,
            FetchError::Parse(ParseError::MissingTitle { .. })
        ));
    }
}
