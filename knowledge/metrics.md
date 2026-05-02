# Metrics / Monitoring

Prometheus: `localhost:9090`. Remote-write receiver enabled (needed by Tempo).
Grafana: `localhost:3000`. admin/admin. Datasources auto-provisioned (UIDs: `prometheus`, `tempo` — don't change).

## Rule: external work must be observable

Any code that fetches, polls, produces, consumes, parses external input, or talks to I/O must emit monitoring from day one:

- A success counter where useful
- Error counters by class/status when possible
- Retry counters for transient failures
- Latency histogram/timer for network and broker calls when supported
- `tracing::warn` for recoverable failures and retries
- `tracing::error` for exhausted retries or unrecoverable failures

Library crates use the `metrics` facade only. Prometheus exporter setup lives in the binary.

## Current RSS metrics

| Metric | Meaning |
|---|---|
| `rss_feed_http_status_errors_total{status}` | HTTP non-2xx responses by status |
| `rss_feed_fetch_retry_total` | Fetch retries after transient request failure |
| `rss_feed_poll_errors_total` | Poll cycle failures |
| `rss_feed_parse_errors_total` | Malformed RSS entries skipped |
