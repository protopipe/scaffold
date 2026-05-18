# ADR 0006: Use Cucumber for Blackbox Acceptance Tests

## Status

Accepted

## Context

The scaffold needs acceptance tests that describe externally observable behavior without coupling to a service implementation language.

## Decision

Place Cucumber features and steps under `tests/bdd`.

Features use tags such as `@rust-example` and `@blackbox` to describe scope. Steps call the running service and WireMock over HTTP.

## Consequences

BDD tests remain project-level validation instead of service internals.

Tags can later select subsets by service, capability, or test cost.

