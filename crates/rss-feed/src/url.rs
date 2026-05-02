/// Google News locale query parameters.
///
/// Defaults match the previous hard-coded URL: `hl=en-US`, `gl=US`, `ceid=US:en`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GoogleNewsLocale {
    /// Host language, e.g. `en-US`, `fr-FR`.
    pub hl: String,
    /// Geographic location, e.g. `US`, `FR`.
    pub gl: String,
    /// Country:language edition, e.g. `US:en`, `FR:fr`.
    pub ceid: String,
}

impl Default for GoogleNewsLocale {
    fn default() -> Self {
        Self {
            hl: "en-US".to_owned(),
            gl: "US".to_owned(),
            ceid: "US:en".to_owned(),
        }
    }
}

/// Build a Google News RSS search URL for `query` and `locale`.
pub fn google_news_rss_url(query: &str, locale: &GoogleNewsLocale) -> String {
    let encoded_query = urlencoding::encode(query);
    let encoded_hl = urlencoding::encode(&locale.hl);
    let encoded_gl = urlencoding::encode(&locale.gl);
    let encoded_ceid = urlencoding::encode(&locale.ceid);

    format!(
        "https://news.google.com/rss/search?q={encoded_query}&hl={encoded_hl}&gl={encoded_gl}&ceid={encoded_ceid}"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn url_should_encode_simple_query_with_default_locale() {
        let url = google_news_rss_url("rust programming", &GoogleNewsLocale::default());
        assert_eq!(
            url,
            "https://news.google.com/rss/search?q=rust%20programming&hl=en-US&gl=US&ceid=US%3Aen"
        );
    }

    #[test]
    fn url_should_encode_special_characters() {
        let url = google_news_rss_url("AI & ML", &GoogleNewsLocale::default());
        assert!(url.contains("AI%20%26%20ML"));
    }

    #[test]
    fn url_should_use_configured_locale() {
        let locale = GoogleNewsLocale {
            hl: "fr-FR".to_owned(),
            gl: "FR".to_owned(),
            ceid: "FR:fr".to_owned(),
        };

        let url = google_news_rss_url("intelligence artificielle", &locale);

        assert_eq!(
            url,
            "https://news.google.com/rss/search?q=intelligence%20artificielle&hl=fr-FR&gl=FR&ceid=FR%3Afr"
        );
    }
}
