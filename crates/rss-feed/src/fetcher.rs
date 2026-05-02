use std::time::Duration;

use reqwest::Client;

use crate::error::FetchError;

const REQUEST_TIMEOUT: Duration = Duration::from_secs(15);

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
