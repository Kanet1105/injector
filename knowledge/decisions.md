# Architecture Decisions

## ADR-001 — NewsItem dedup strategy

**Date:** 2026-05-01
**Status:** Decided

### Context
Google News RSS feeds re-serve the same articles across polls. We need to avoid processing duplicates downstream without bloating the proto schema.

### Decision
Use `guid` as the Redpanda **message key**.

- Topic configured as **compacted**: broker retains only the latest message per key, dedup happens at storage level for free.
- Consumers that need strict exactly-once tracking use **Redis** (TTL-based seen-guid set).
- No hash field on the proto — `guid` from RSS is stable and unique per article.

### Consequences
- Compacted topic required for the news topic. Set `cleanup.policy=compact` when creating it.
- If a feed ever has missing/unstable guids, revisit and add a deterministic hash field (e.g. `sha256(link)`).

### Rejected alternatives
- Hash field on proto — extra compute, not needed when guid is reliable.
- Consumer-only dedup — works but wasteful (processes then discards duplicates).

---

## ADR-002 — FeedItem transform location

**Date:** 2026-05-01
**Status:** Decided

### Context
`rss-feed` parses RSS XML into a `FeedItem` struct. The pipeline needs proto `NewsItem`. Where does the `FeedItem → NewsItem` conversion live?

### Decision
**Option B — binary/service crate wires the conversion.**

`rss-feed` owns `FeedItem` and stays pure: fetch + parse only, no proto dependency.
The binary crate that assembles the pipeline does `NewsItem::from(item)` (or equivalent).

### Consequences
- `rss-feed` has zero coupling to proto or wire format. Easy to test in isolation.
- Conversion logic lives in the binary for now — acceptable at this scale.
- When a second source type appears (Atom feed, webhook, etc.), extract a `transform` crate that depends on both `rss-feed` and `proto` and owns all `From` impls.

### Rejected alternatives
- `From` in `rss-feed` — couples fetcher to proto, recompiles on wire format changes.
- Dedicated `transform` crate now — overkill for one source type.
