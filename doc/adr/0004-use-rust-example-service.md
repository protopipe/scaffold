# ADR 0004: Use Rust Example Service

## Status

Accepted

## Context

The scaffold needs a small executable example that proves service, mock, BDD, and load-test wiring.

Rust is in the Protopipe technology radar's Adopt ring.

## Decision

Provide `services/rust-example` as a minimal Actix Web service.

The service exposes `/health` and `/hello`. The `/hello` endpoint calls an HTTP upstream configured by `UPSTREAM_BASE_URL`.

## Consequences

The service demonstrates configuration through environment variables and deterministic blackbox testing through WireMock.

The example is intentionally small and should not become a reference architecture for all Rust services.

