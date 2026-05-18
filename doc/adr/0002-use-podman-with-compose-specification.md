# ADR 0002: Use Podman with Compose Specification

## Status

Accepted

## Context

The Protopipe technology radar prefers open, composable infrastructure and avoids standalone Docker and `docker-compose`.

The scaffold still needs a practical local and CI environment contract. GitHub Actions and local `act`-style workflows often assume a Docker-compatible container interface. That makes pure `nerdctl` usage harder in some developer and CI contexts.

## Decision

Use the Compose Specification as the environment contract and Podman as the default execution engine.

The project should use `podman-compose` commands in CI to avoid GitHub runner environments selecting a Docker Compose plugin as the external `podman compose` provider.

CI installs `podman-compose==1.5.0` through `pip --user` instead of Ubuntu `apt`, because older distro packages such as `podman-compose 1.0.6` do not support the `--profile` option required by this scaffold.

Compose files must avoid Docker-specific assumptions where practical.

## Consequences

Local and CI environments can reuse the same profile definitions.

This is a pragmatic decision, not a technology-radar override. If future constraints favor `nerdctl`, the Compose contract should make the runtime substitution smaller than a CI rewrite.

Using `podman-compose` directly is slightly less elegant than `podman compose`, but it makes the selected provider explicit and avoids accidental Docker socket coupling.

Pinning `podman-compose` in CI adds a small maintenance obligation, but it prevents silent behavior changes from runner image package versions.
