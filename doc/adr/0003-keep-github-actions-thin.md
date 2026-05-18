# ADR 0003: Keep GitHub Actions Thin

## Status

Accepted

## Context

CI definitions often duplicate local scripts and drift away from developer workflows.

The scaffold aims for shift-left environment reuse: the same commands should be useful locally and in CI.

## Decision

Keep GitHub Actions as thin entrypoints that invoke Compose profiles.

Workflow YAML may install or prepare the container runtime, but test orchestration belongs in `compose.yaml`.

## Consequences

CI logic stays small and reviewable.

Failures are easier to reproduce locally because the environment contract is not hidden inside GitHub Actions.

