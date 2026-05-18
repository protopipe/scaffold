# Project Guidelines

## Noesis Alignment

Use Noesis as a reference and constraint system. Do not treat it as a backlog, implementation plan, or source of project decisions.

Project decisions belong in ADRs under `doc/adr`.

## Engineering Defaults

- Prefer open source, inspectable tools.
- Keep tools small, composable, and replaceable.
- Make technology decisions explicit before they become defaults.
- Reuse the same environment contract locally and in CI.
- Keep GitHub Actions thin; put runtime orchestration in Compose profiles.

## Repository Boundaries

- `services/` contains service implementation code.
- `tests/bdd/` contains blackbox behavior tests.
- `tests/load/` contains synthetic load feedback.
- `doc/` contains long-lived knowledge.
- `agents/` contains agent-neutral guidance.

