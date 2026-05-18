# Protopipe Scaffold

Polyglot project scaffold aligned with Protopipe Noesis guidelines.

The scaffold keeps the local and CI execution model intentionally close:

- Compose Specification is the shared environment contract.
- Podman is the default local and CI execution engine.
- GitHub Actions only invokes the same Compose profiles used locally.
- Documentation, ADRs, tests, and agent guidance live in versioned project files.

## Quick Start

```bash
podman-compose -f compose.yaml --profile dev up --build
```

Run blackbox BDD tests:

```bash
podman-compose -f compose.yaml --profile test up --build --abort-on-container-exit --exit-code-from test-bdd
```

Run a small k6 smoke load test:

```bash
podman-compose -f compose.yaml --profile load up --build --abort-on-container-exit --exit-code-from test-load
```

## Structure

```text
.github/workflows/      Thin GitHub Actions entrypoints
agents/                 Agent-neutral project guidance
deployments/helm/       Future Kubernetes deployment packaging
doc/adr/                Architecture Decision Records
doc/arc42/              Architecture documentation
doc/plantuml/in/        PlantUML sources
doc/plantuml/out/       Generated diagram output
services/               Polyglot services
skills/                 Reusable guidance and workflow snippets
tests/bdd/              Cucumber blackbox tests
tests/load/             k6 load tests
```

## Technology Positioning

The scaffold starts with Rust as an example service technology, WireMock for deterministic HTTP dependency simulation, Cucumber for blackbox acceptance tests, k6 for load feedback, MkDocs for documentation, and PlantUML for diagrams.

All project-specific technology decisions are captured as ADRs in `doc/adr`.
