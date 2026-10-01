# Money Road

> **"Money Road shows you the road your money can take."**

A consumer-facing **financial routing intelligence platform** for cross-border money
movement. Money Road maps existing payment rails and tells a person how money can travel
from an origin to a destination under real constraints: amount, currencies, deadline,
cost, speed, reliability, and eligibility.

**This is one possible road — not a money mover, not a wallet.** Money Road recommends a
route and hands the user to a regulated provider (Phase 1: informational/referral).

## North Star

> *"I have 100,000 XAF in Cameroon. My brother needs CNY in China tomorrow morning. Show
> me the safest affordable route."*

## First route (the laboratory)

**Cameroon → China.** The first route may be partially manual. The objective is learning
the complete route lifecycle before optimizing for scale:

> ONE REAL ROUTE → REAL OBSERVATIONS → STABLE DOMAIN MODEL → REPEATABLE ROUTE ONBOARDING

## Repo layout

```
crates/
  domain/            # Canonical domain model (Corridor, Route, Edge, ...)
  evidence/          # Evidence states, freshness, provenance
  route-engine/      # Candidate generation, feasibility, cost/time, ranking
  rules-engine/      # Eligibility, limits, timing, cutoffs, country rules
  provider-adapters/ # Provider integration traits + adapters
  api/               # Rust HTTP API
migrations/          # PostgreSQL schema migrations
docs/                # Full documentation set
```

## Quick start

Requirements: Rust (stable), Docker, PostgreSQL 16 (via docker-compose).

```bash
# 1. Start PostgreSQL
docker compose up -d

# 2. Run migrations (see API crate docs)

# 3. Run the API
cargo run -p money-road-api
```

## Documentation

Begin at [docs/README.md](docs/README.md). The top-level [AGENTS.md](AGENTS.md) is the
agent-facing development guide; detailed documents live under `docs/`.

## Status

Early exploration. Vision and first-route production spec are captured in the docs; the
initial Rust workspace and schema are being scaffolded. This repository contains the
project's strategy and architecture, **not** legal advice. Regulatory questions must be
reviewed with qualified counsel per jurisdiction before any execution feature.

## License

[MIT](LICENSE)
