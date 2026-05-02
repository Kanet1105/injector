use reqwest::Client;

use crate::error::FetchError;

/// Thin HTTP client for retrieving RSS XML.
#[derive(Debug, Clone)]
pub struct FeedFetcher {
    client: Client,
}

impl FeedFetcher {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn fetch_bytes(&self, url: &str) -> Result<Vec<u8>, FetchError> {
        let response = self.client.get(url).send().await?;
        let status = response.status();

        if !status.is_success() {
            return Err(FetchError::Status {
                url: url.to_owned(),
                status,
            });
        }

        Ok(response.bytes().await?.to_vec())
    }
}

impl Default for FeedFetcher {
    fn default() -> Self {
        Self::new()
    }
}
