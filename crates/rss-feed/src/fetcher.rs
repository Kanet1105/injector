use std::future::Future;
use std::pin::Pin;
use std::time::{Duration, Instant};

use reqwest::Client;

use crate::error::FetchError;

const REQUEST_TIMEOUT: Duration = Duration::from_secs(15);

/// Fetches raw RSS feed bytes.
///
/// Trait boundary lets `Poller` tests use a fake fetcher instead of a real socket.
pub trait FeedClient {
    fn fetch_bytes<'a>(
        &'a self,
        url: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<u8>, FetchError>> + Send + 'a>>;
}

/// Thin HTTP client for retrieving RSS XML.
#[derive(Debug, Clone)]
pub struct FeedFetcher {
    client: Client,
}

impl FeedFetcher {
    pub fn new() -> Result<Self, FetchError> {
        let client = Client::builder()
            .timeout(REQUEST_TIMEOUT)
            .build()
            .map_err(FetchError::Client)?;

        Ok(Self { client })
    }

    pub fn with_client(client: Client) -> Self {
        Self { client }
    }

    pub async fn fetch_bytes(&self, url: &str) -> Result<Vec<u8>, FetchError> {
        let started_at = Instant::now();
        let result = self.fetch_bytes_inner(url).await;

        metrics::histogram!("rss_feed_fetch_duration_seconds")
            .record(started_at.elapsed().as_secs_f64());

        result
    }

    async fn fetch_bytes_inner(&self, url: &str) -> Result<Vec<u8>, FetchError> {
        let response = self.client.get(url).send().await?;
        let status = response.status();

        if !status.is_success() {
            metrics::counter!("rss_feed_http_status_errors_total", "status" => status.as_u16().to_string()).increment(1);
            return Err(FetchError::Status {
                url: url.to_owned(),
                status,
            });
        }

        Ok(response.bytes().await?.to_vec())
    }
}

impl FeedClient for FeedFetcher {
    fn fetch_bytes<'a>(
        &'a self,
        url: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<u8>, FetchError>> + Send + 'a>> {
        Box::pin(async move { FeedFetcher::fetch_bytes(self, url).await })
    }
}
