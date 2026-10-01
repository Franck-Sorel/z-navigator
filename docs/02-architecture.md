# 02 — Architecture

## High-level diagram

```
                    MONEY ROAD — Web / Mobile UI
                              |
                              v
                          Rust API
                              |
          +-------------+-------------+-------------+
          |             |             |             |
          v             v             v             v
    Route Engine    Evidence DB   Provider Adapters   Rules Engine
          |             |             |             |
          +-------------+-------------+-------------+
                              |
                         PostgreSQL
```

## Core principle

> The routing engine depends on **Money Road concepts**, not provider APIs. Provider
> integrations are **adapters** that normalize data into Money Road's own models.

The engine consumes normalized quotes, constraints, and evidence — never provider-specific
API details. This keeps the core stable while corridors and providers change.

## Components

| Component | Responsibility |
|---|---|
| **Route Engine** | Candidate generation, feasibility filtering, scoring, ranking |
| **Evidence DB** | Current values and provenance (freshness, source, confidence) |
| **Provider Adapters** | Normalized interfaces to external provider data |
| **Rules Engine** | Eligibility, limits, timing, cutoff, country rules |
| **API (Rust)** | Serves the UI; orchestrates the pipeline |
| **PostgreSQL** | Primary storage (see below) |

## Data flow

```
USER INTENT → NORMALIZATION → GRAPH QUERY → FEASIBILITY → COST/TIME MODEL
→ ROUTE RANKING → EXPLANATION → PROVIDER HANDOFF → OBSERVATION
```

The **observation** step closes the loop and feeds back into route data: every verified
quote, completed route, failure, delay, and correction improves the graph (subject to
privacy, authorization, and applicable law).

## Storage decision

- **Start with PostgreSQL.** A graph database is *not required* for the first
  implementation.
- The conceptual graph is the **domain model**; the physical storage technology can evolve
  when route scale or query patterns justify it.

## Later additions (not yet built)

Event store / observations · background quote refresh · provider webhooks · reliability
analytics · B2B API.

## Repo layout (intent)

```
crates/
  domain/           # Canonical domain model (Corridor, UserIntent, Route, ...)
  evidence/         # Evidence types, states, freshness, provenance
  route-engine/     # Candidate generation, feasibility, cost/time, ranking
  rules-engine/     # Eligibility, limits, timing, cutoffs, country rules
  provider-adapters/# Provider integration traits + adapters
  api/              # Rust HTTP API
migrations/         # PostgreSQL schema migrations
docker-compose.yml  # Local PostgreSQL
docs/               # This documentation
```
