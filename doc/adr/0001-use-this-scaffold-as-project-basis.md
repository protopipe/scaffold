# ADR 0001: Use This Scaffold as Project Basis

## Status

Accepted

## Context

The repository starts as a fresh project and needs a reusable baseline for polyglot services, documentation, tests, CI, and agent guidance.

Noesis is the canonical reasoning space. Project decisions must live outside Noesis and remain explainable against it.

## Decision

Use this scaffold as the project basis.

The scaffold provides:

- a documentation structure with ADR, arc42, and PlantUML areas
- a `services/` boundary for polyglot services
- project-level blackbox test areas
- Compose profiles as reusable local and CI execution contracts
- agent-neutral guidance in `agents/`

## Consequences

Projects can start with visible defaults and change them through ADRs.

The scaffold must avoid becoming a hidden platform. Reuse must remain explicit, inspectable, and replaceable.

