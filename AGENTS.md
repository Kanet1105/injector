# AGENTS.md — injector

Observability stack: Redpanda, Redis, Prometheus, Grafana, Tempo.

## Commits

All commits follow Conventional Commits — see `CONTRIBUTING.md`.

Before every commit, `just ci` must pass:
```
just ci   # fmt-check + lint + test + build
```

## Rust

Always load `knowledge/rust-guidelines.md` before writing or reviewing any Rust code.

## Coding

Always load `knowledge/coding-guidelines.md` before writing code in any language.

## Knowledge

Load only when relevant:

- Kafka/Redpanda → `knowledge/kafka.md`
- Redis → `knowledge/redis.md`
- Tracing (Tempo/Jaeger) → `knowledge/tracing.md`
- Metrics (Prometheus/Grafana) → `knowledge/metrics.md`
- Architecture decisions → `knowledge/decisions.md`
