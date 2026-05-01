# injector

[![CI](https://github.com/Kanet1105/injector/actions/workflows/ci.yml/badge.svg)](https://github.com/Kanet1105/injector/actions/workflows/ci.yml)

Data ingestion pipeline — pulls Google News RSS feeds and produces to Redpanda.

## Stack

| Component | Role |
|---|---|
| Redpanda | Kafka-compatible message broker |
| Redis | Dedup / seen-guid tracking |
| Prometheus | Metrics |
| Grafana | Dashboards |
| Tempo | Distributed tracing |

## Workspace

```
crates/
  config/      Shared configuration types
  redpanda/    Producer / consumer (samsa — pure Rust Kafka)
  rss-feed/    Google News RSS fetcher
proto/
  news_item.proto
```

## Dev

```bash
# start infrastructure
docker compose up -d

# fmt + lint + test + build
just ci
```

## Docs

- [`CONTRIBUTING.md`](CONTRIBUTING.md) — commit conventions
- [`knowledge/decisions.md`](knowledge/decisions.md) — architecture decisions
