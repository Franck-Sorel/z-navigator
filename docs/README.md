# Money Road — Documentation

> **"Money Road shows you the road your money can take."**

A consumer-facing **financial routing intelligence platform** for cross-border money
movement. This directory is the single source of truth for the product vision,
architecture, domain model, and operating guidance.

These documents are distilled from two authoritative source documents:

- `Money_Road_Project_Vision_and_Pitfall_Handbook.docx`
- `Money_Road_First_Route_Production_Spec.docx`

The top-level `AGENTS.md` summarizes the same guidance for coding agents; these docs
carry the detail.

## Index

| Doc | Contents |
|---|---|
| [01-vision](01-vision.md) | The problem, the product insight, the mental model |
| [02-architecture](02-architecture.md) | Technical architecture intent, data flow, components |
| [03-domain-model](03-domain-model.md) | Canonical objects and the directed-graph core |
| [04-evidence-model](04-evidence-model.md) | The data moat: states, freshness, provenance |
| [05-routing-engine](05-routing-engine.md) | Feasibility, effective cost, ranking dimensions |
| [06-time-and-deadline](06-time-and-deadline.md) | Windows, buffers, "estimated" vs "guaranteed" |
| [07-first-route](07-first-route-cameroon-china.md) | The Cameroon → China laboratory |
| [08-research-and-verification](08-research-and-verification.md) | Research discipline and route classes |
| [09-monetization](09-monetization.md) | Revenue paths and conflict-of-interest policy |
| [10-pitfalls-and-guardrails](10-pitfalls-and-guardrails.md) | The 16 documented pitfalls |
| [11-production-gates](11-production-gates-and-failure-states.md) | Launch gates and failure states |
| [12-instrumentation](12-instrumentation-and-observability.md) | Events, observability, ops runbook |
| [decisions](decisions/README.md) | Architecture Decision Records (ADRs) |

## Reading order for newcomers

1. **Vision** — why this product exists and what it is not.
2. **First route** — what the laboratory must prove.
3. **Domain & evidence model** — the vocabulary and the moat.
4. **Routing engine** — how routes are evaluated.
5. **Architecture** — how the system is physically organized.
6. **Production gates** — what "done" and "live" mean.
