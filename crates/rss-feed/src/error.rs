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
