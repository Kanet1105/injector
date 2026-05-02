use chrono::{DateTime, Utc};

/// A single parsed article from a Google News RSS feed.
///
/// `query` is injected by the poller — it is not present in the feed itself.
/// `description` is raw HTML as returned by Google News — strip tags downstream if needed.
/// `source_url` is not available from feed-rs RSS2 parsing (`<source url="...">` attribute
/// is dropped). `source_name` is also not populated — feed-rs silently ignores the RSS2
/// `<source>` element entirely. Both fields stay empty until addressed with a custom
/// XML pre-pass or a different parser. Tracked for follow-up.
#[derive(Debug, Clone, PartialEq)]
pub struct FeedItem {
    /// Stable unique article id — used as Redpanda message key (see ADR-001).
    pub guid: String,
    pub title: String,
    pub link: String,
    /// Raw HTML snippet. Not sanitised at this layer.
    pub description: String,
    pub pub_date: DateTime<Utc>,
    pub source_name: String,
    /// Not populated by the RSS parser — see struct doc above.
    pub source_url: String,
    /// The Google News search query that produced this item.
    pub query: String,
}
