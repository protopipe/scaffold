# ADR 0005: Use WireMock for HTTP Dependency Tests

## Status

Accepted

## Context

Blackbox tests need deterministic behavior for external HTTP dependencies.

Calling real external services would make the scaffold flaky, slow, and context-dependent.

## Decision

Use WireMock as a containerized HTTP test double.

Cucumber steps configure WireMock mappings through its Admin API before calling the service under test.

## Consequences

Tests can simulate upstream behavior without network dependencies.

The service must receive external endpoints through configuration instead of hardcoding URLs.

