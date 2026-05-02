use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("entry missing guid")]
    MissingGuid,

    #[error("entry '{guid}' missing title")]
    MissingTitle { guid: String },

    #[error("entry '{guid}' missing link")]
    MissingLink { guid: String },

    #[error("entry '{guid}' missing pub_date")]
    MissingPubDate { guid: String },

    #[error("failed to parse feed: {0}")]
    Feed(#[from] feed_rs::parser::ParseFeedError),
}

#[derive(Debug, Error)]
pub enum FetchError {
    #[error("failed to build http client: {0}")]
    Client(#[source] reqwest::Error),

    #[error("http request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("http request to '{url}' failed with status {status}")]
    Status {
        url: String,
        status: reqwest::StatusCode,
    },

    #[error("rss parse failed: {0}")]
    Parse(#[from] ParseError),
}
