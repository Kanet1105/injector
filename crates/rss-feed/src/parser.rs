use chrono::{DateTime, Utc};
use feed_rs::model::Entry;

use crate::error::ParseError;
use crate::feed_item::FeedItem;

/// Parse raw RSS bytes into a list of results, one per entry.
///
/// Each entry is parsed independently — a bad entry does not abort the whole feed.
/// The caller decides how to handle individual `Err` items (log + metric + skip, or propagate).
pub fn parse(bytes: &[u8], query: &str) -> Result<Vec<Result<FeedItem, ParseError>>, ParseError> {
    let feed = feed_rs::parser::parse(bytes)?;

    let items = feed
        .entries
        .into_iter()
        .map(|entry| entry_to_feed_item(entry, query))
        .collect();

    Ok(items)
}

fn entry_to_feed_item(entry: Entry, query: &str) -> Result<FeedItem, ParseError> {
    let guid = entry.id;
    if guid.is_empty() {
        return Err(ParseError::MissingGuid);
    }

    let title = entry
        .title
        .ok_or_else(|| ParseError::MissingTitle { guid: guid.clone() })?
        .content;

    let link = entry
        .links
        .into_iter()
        .next()
        .ok_or_else(|| ParseError::MissingLink { guid: guid.clone() })?
        .href;

    let description = entry.summary.map(|s| s.content).unwrap_or_default();

    let pub_date: DateTime<Utc> = entry
        .published
        .ok_or_else(|| ParseError::MissingPubDate { guid: guid.clone() })?;

    // feed-rs does not parse RSS2 <source> elements — source_name is always empty.
    // See FeedItem doc comment. Tracked for follow-up.
    let source_name = entry.source.unwrap_or_default();

    Ok(FeedItem {
        guid,
        title,
        link,
        description,
        pub_date,
        source_name,
        source_url: String::new(),
        query: query.to_owned(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> &'static [u8] {
        include_bytes!("../tests/fixtures/google_news.xml")
    }

    #[test]
    fn parse_should_return_one_result_per_entry() {
        let results = parse(fixture(), "artificial intelligence").unwrap();
        assert_eq!(results.len(), 3);
    }

    #[test]
    fn parse_should_map_valid_entry_fields() {
        let results = parse(fixture(), "artificial intelligence").unwrap();
        let item = results[0].as_ref().unwrap();

        assert_eq!(item.title, "OpenAI releases GPT-5");
        assert_eq!(item.link, "https://www.theverge.com/2026/5/1/openai-gpt5");
        assert_eq!(item.query, "artificial intelligence");
        // feed-rs does not parse RSS2 <source> — source_name is empty until addressed
        assert_eq!(item.source_name, "");
        assert!(!item.guid.is_empty());
    }

    #[test]
    fn parse_should_keep_raw_html_description() {
        let results = parse(fixture(), "artificial intelligence").unwrap();
        let item = results[0].as_ref().unwrap();

        assert!(item.description.contains("OpenAI"));
    }

    #[test]
    fn parse_should_return_error_when_pub_date_missing() {
        let results = parse(fixture(), "artificial intelligence").unwrap();
        let err = results[2].as_ref().unwrap_err();

        assert!(
            matches!(err, ParseError::MissingPubDate { .. }),
            "expected MissingPubDate, got {err}"
        );
    }

    #[test]
    fn parse_should_return_error_on_invalid_xml() {
        let result = parse(b"not xml at all", "test");
        assert!(result.is_err());
    }

    #[test]
    fn parse_should_set_pub_date_as_utc() {
        let results = parse(fixture(), "artificial intelligence").unwrap();
        let item = results[0].as_ref().unwrap();

        assert_eq!(item.pub_date.timezone(), Utc);
    }
}
