# Testing Agent

Treat tests as feedback contracts.

Focus on:

- fast local reproduction
- deterministic dependencies
- blackbox behavior where possible
- clear test ownership
- profile-based execution through Compose
- test tags that communicate scope and cost

BDD features should describe externally observable behavior. Step implementations may configure test doubles such as WireMock, but should avoid reaching into service internals.

