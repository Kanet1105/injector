use std::time::Duration;

use rss_feed::FeedFetcher;
use rss_feed::error::FetchError;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

async fn spawn_response(status: u16, body: &'static str, delay: Duration) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        let mut buffer = [0_u8; 1024];
        let _ = stream.read(&mut buffer).await.unwrap();

        if !delay.is_zero() {
            tokio::time::sleep(delay).await;
        }

        let response = format!(
            "HTTP/1.1 {status} OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        );
        stream.write_all(response.as_bytes()).await.unwrap();
    });

    format!("http://{addr}/feed.xml")
}

#[tokio::test]
async fn fetch_bytes_should_return_response_body_on_success() {
    let url = spawn_response(200, "<rss></rss>", Duration::ZERO).await;
    let fetcher = FeedFetcher::new().unwrap();

    let body = fetcher.fetch_bytes(&url).await.unwrap();

    assert_eq!(body, b"<rss></rss>");
}

#[tokio::test]
async fn fetch_bytes_should_return_status_error_on_non_success() {
    let url = spawn_response(503, "unavailable", Duration::ZERO).await;
    let fetcher = FeedFetcher::new().unwrap();

    let error = fetcher.fetch_bytes(&url).await.unwrap_err();

    assert!(matches!(error, FetchError::Status { status, .. } if status.as_u16() == 503));
}

#[tokio::test]
async fn fetch_bytes_should_return_error_when_request_times_out() {
    let url = spawn_response(200, "too late", Duration::from_millis(200)).await;
    let client = reqwest::Client::builder()
        .timeout(Duration::from_millis(50))
        .build()
        .unwrap();
    let fetcher = FeedFetcher::with_client(client);

    let error = fetcher.fetch_bytes(&url).await.unwrap_err();

    assert!(matches!(error, FetchError::Http(err) if err.is_timeout()));
}
