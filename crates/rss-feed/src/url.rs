/// Build a Google News RSS search URL for `query`.
///
/// The query is URL-encoded. Locale is fixed to `en-US` / `US`.
pub fn google_news_rss_url(query: &str) -> String {
    let encoded = urlencoding::encode(query);
    format!("https://news.google.com/rss/search?q={encoded}&hl=en-US&gl=US&ceid=US:en")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn url_should_encode_simple_query() {
        let url = google_news_rss_url("rust programming");
        assert_eq!(
            url,
            "https://news.google.com/rss/search?q=rust%20programming&hl=en-US&gl=US&ceid=US:en"
        );
    }

    #[test]
    fn url_should_encode_special_characters() {
        let url = google_news_rss_url("AI & ML");
        assert!(url.contains("AI%20%26%20ML"));
    }
}
