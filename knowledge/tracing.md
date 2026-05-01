Grafana Tempo. Jaeger-compatible tracing backend.

Endpoints:
- OTLP gRPC: `localhost:4317`
- OTLP HTTP: `localhost:4318`
- Jaeger HTTP: `localhost:14268/api/traces`
- Jaeger gRPC: `localhost:14250`
- Zipkin: `localhost:9411`
- Jaeger UDP: `localhost:6831`

View traces: Grafana → Explore → Tempo datasource.
Span metrics pushed to Prometheus via remote-write.
