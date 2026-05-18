# ADR 0007: Use k6 for Load Feedback

## Status

Accepted

## Context

The scaffold should demonstrate synthetic validation beyond functional correctness.

Load feedback must be executable locally and in CI without adding GitHub-specific test logic.

## Decision

Use k6 scripts under `tests/load` and run them through Compose profiles.

The default scaffold test is a short smoke load test. Longer baseline, stress, or soak tests can be added as separate scripts and profiles.

## Consequences

Performance feedback becomes part of the project contract.

The default CI path should keep load tests short to avoid slow feedback loops.

