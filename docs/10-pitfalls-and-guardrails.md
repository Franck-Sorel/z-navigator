# 10 — Pitfalls & Guardrails

The 16 documented pitfalls. Each pairs a **mistake** with the **correct** approach.

| # | Pitfall | Do instead |
|---|---|---|
| 1 | Building the wallet first | Start informational/referral; prove demand first |
| 2 | "Nobody does this" | Define the whitespace: underserved corridors + roadmap + deadline reasoning + explainability |
| 3 | Treating a provider list as routing | Model routes as paths through a graph |
| 4 | Comparing only advertised fees | Calculate effective delivered cost |
| 5 | Using stale quotes | Store source, timestamp, freshness, confidence |
| 6 | Assuming Web3 is always cheaper | Compare complete paths, not individual rails |
| 7 | Promising a guarantee too early | Use confidence/buffer language until execution + liability are controlled |
| 8 | Hiding commercial relationships | Separate ranking from monetization; disclose |
| 9 | Going global immediately | Start with one corridor and go deep |
| 10 | Scraping everything | Prefer APIs, partnerships, published data, manual verification |
| 11 | Building the globe before the engine | Build domain model + evidence first; visualize second |
| 12 | Confusing ETA with certainty | Represent distributions, buffers, failure probability |
| 13 | Ignoring the last mile | Treat payout eligibility as a hard route constraint |
| 14 | Ignoring funding constraints | Model first mile and last mile equally |
| 15 | Becoming a crypto product | Present stablecoins as one possible rail |
| 16 | Automating before learning payments | Manually research/execute simulated routes first |

## The mental model

Approach the project as a **systems problem**, not a fintech UI project:

```
USER INTENT → CONSTRAINTS → PAYMENT GRAPH → FEASIBILITY
→ ROUTE COST + TIME + RELIABILITY → RANKING → EXPLAINABLE ROADMAP
→ PROVIDER HANDOFF → OBSERVATION → (feeds back into route data)
```

The **feedback loop** is the long-term asset: every verified quote, completed route,
failure, delay, and correction improves the graph.
