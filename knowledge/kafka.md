# Kafka / Redpanda

Redpanda is a Kafka-compatible broker. Drop-in replacement; use standard `rdkafka` crate.

## Ports (local docker-compose)

| Port | Protocol |
|------|----------|
| 9092 | Kafka API (producers/consumers) |
| 9644 | Admin API + Prometheus metrics |
| 8081 | Schema Registry |
| 8082 | HTTP Proxy |
| 8080 | Redpanda Console UI |

## Rust crate: `rdkafka`

- Wraps `librdkafka` (C). Requires cmake or `cmake-build` feature.
- `FutureProducer` — async produce. Send with `Duration` timeout.
- `StreamConsumer` — async stream via `.recv().await`.
- `ClientConfig` — key/value config strings, same as Kafka.

## Key config strings

```
bootstrap.servers   = "localhost:9092"
group.id            = "<consumer-group>"
enable.auto.commit  = "false"          # prefer manual commit
auto.offset.reset   = "earliest"
message.timeout.ms  = "5000"
```

## Workspace crates

- `crates/redpanda` — `Producer` + `Consumer` wrappers
- `crates/config` — `RedpandaSettings` struct (brokers, topic, group_id)

## Env vars (INJECTOR__ prefix)

```
INJECTOR__REDPANDA__BROKERS=localhost:9092
INJECTOR__REDPANDA__TOPIC=injector.events
INJECTOR__REDPANDA__GROUP_ID=injector-consumer
```

## Schema Registry

REST API on `:8081`. Stores Avro/Protobuf schemas keyed by subject.
Not yet integrated — future: use `schema_registry_converter` crate.

## Protobuf pipeline

`.proto` files → `crates/proto/proto/`. Compiled by `prost-build` in `build.rs`.
Generated code included via `include!(concat!(env!("OUT_DIR"), "/....rs"))`.
