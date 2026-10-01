# Money Road — Development Guidance (AGENTS.md)

This file captures the project vision, architecture intent, and the guardrails that
must shape every line of code. Read it fully before starting any task. It is distilled
from two source documents:

- `Money_Road_Project_Vision_and_Pitfall_Handbook.docx`
- `Money_Road_First_Route_Production_Spec.docx`

Treat those documents as the authoritative source of truth. If this file and a document
conflict, the document wins and this file should be updated.

---

## 1. What Money Road is (the vision)

Money Road is a **consumer-facing financial routing intelligence platform** for
cross-border money movement. It is **not initially a remittance company**. It maps
existing payment rails and tells a person how money can travel from an origin to a
destination under real constraints: amount, currencies, deadline, cost, speed,
reliability, and eligibility.

The core insight: **the rails already exist; the consumer-facing roadmap across them is
fragmented.** The product fills that gap by sitting between consumer comparison tools
(Monito, MoneyTransfers.com) and institutional orchestration infrastructure (Nium,
Circle). The product object is a **route**, not a provider. A route is a sequence of
financial transformations modelled as a path through a directed graph.

### The product promise
> "Money Road shows you the road your money can take."

North Star request: *"I have 100,000 XAF in Cameroon. My brother needs CNY in China
tomorrow morning. Show me the safest affordable route."* — answered with a financial map
(origin → fund → convert → settle → convert → payout → destination), explaining cost,
time, deadline, risk, evidence, and clear action. **Not** a generic list of transfer
companies.

### The mindset to lock in
- Do not start by building a money mover. Start by building a **map**.
- Do not assume the fastest rail is the best rail. Calculate the **complete path**.
- Do not trust advertised fees alone. Measure **effective cost**.
- Do not promise "guaranteed arrival" until you control the transaction (use
  *estimated arrival*, *deadline confidence*, *safety buffer*).
- Do not make Web3 the product. Make it **one possible road**.
- Do not optimize for global coverage first. **Go deep on one painful corridor.**

---

## 2. First route: the laboratory

The first production route is **Cameroon → China**. This single corridor is the
"laboratory" that forces the system to confront funding, FX, settlement, payout,
eligibility, limits, timing, compliance, and stablecoin-assisted paths. **None of these
are assumed to work; each must be verified.**

The first route may be **partially manual**. The objective is learning the complete
route lifecycle, not maximum automation. The guiding sequence:

> ONE REAL ROUTE → REAL OBSERVATIONS → STABLE DOMAIN MODEL → REPEATABLE ROUTE ONBOARDING

### Non-goals (do not build yet)
- No wallet or custody.
- No token issuance.
- No guaranteed-delivery promise.
- No global rollout.
- No unverified provider availability.
- No assumption that blockchain is cheaper.
- No globe-first development (build the engine and evidence first, visualize second).

---

## 3. Canonical domain model

The conceptual system is a **directed graph**: nodes are financial states/institutions,
edges are transformations/transfers. The same model powers the routing engine and any
globe/graph UI.

| Object | Meaning | Minimum fields |
|---|---|---|
| `Corridor` | Origin/destination market pair | origin, destination, currency pair |
| `UserIntent` | What the user needs | amount, deadline, funding, payout, preferences |
| `Provider` | Payment participant | name, markets, capabilities, regulatory context |
| `Rail` | Mechanism moving value | bank, MTO, mobile money, card, stablecoin, etc. |
| `Edge` | One executable movement/transformation | from, to, provider, rail, cost, time, constraints |
| `Quote` | Observed/retrieved price | amount, fees, FX, timestamp, expiry |
| `Eligibility` | Conditions to use an edge | country, identity, account, purpose, limits |
| `Evidence` | Support for a route claim | source, timestamp, scope, confidence |
| `Route` | Ordered set of edges | edges, cost, recipient amount, ETA |
| `Observation` | What happened after execution | actual cost, actual time, success/failure |
| `FailureMode` | Known way a route fails | trigger, mitigation, uncertainty |

**A provider is not a route.** Assemble edges into complete paths; reject paths with
unsupported critical edges.

---

## 4. Evidence model (the moat)

The long-term moat is **not the map** (easy to imitate) — it is a trustworthy dataset of
what actually happens on each corridor. Route facts are **time-dependent claims with a
source and validity window**, never timeless configuration.

Evidence **states**:

| State | Meaning | Production behavior |
|---|---|---|
| `Verified` | Current authoritative source or controlled test | May support production eligibility |
| `Observed` | Confirmed by actual execution/observation | Strong evidence; retain conditions |
| `Estimated` | Calculated/inferred | Show uncertainty |
| `Stale` | Outside freshness window | Do not silently use for routing |
| `Unverified` | Claim not independently confirmed | Research only |
| `Unsupported` | No evidence of current executability | Exclude from production |

**Rules:**
- Every production recommendation must be explainable: *"We recommend this path because
  these edges are currently supported by these pieces of evidence."*
- **Never fill a missing edge with a plausible assumption.**
- **Stale evidence must never silently drive production routing.**
- A route engine that hides uncertainty is dangerous. One that exposes it can be trusted.

Data fields to carry: `value`, `source` (API / official docs / observation / manual),
`observed_at`, `expires_at`, `confidence`, `jurisdiction`, `conditions`.

---

## 5. Routing engine

The engine optimizes a **constrained path**, not merely the lowest fee.

```
Feasible(route) =
    supported_origin AND supported_destination
    AND funding_method_supported AND receiving_method_supported
    AND amount_within_limits
    AND regulatory_constraints_satisfied
    AND route_available_at_required_time
```

Effective cost (conceptual; the formula evolves):

```
effective_cost = explicit_fees
               + FX_spread
               + network_fees
               + intermediary_fees
               + expected_loss_from_failure
               + other mandatory costs
```

Do **not** hard-code a universal "best" route. Route score must incorporate explicit
user preferences (cheapest / fastest / simplest / balanced). A student with a tuition
deadline may rationally prefer a more expensive route with a large time buffer.
**Preserve ranking dimensions; never hide quality in one mysterious score.**

| Dimension | Example measure |
|---|---|
| Cost | sender spend / recipient amount / effective cost |
| Speed | expected window + deadline buffer |
| Reliability | observed success + evidence quality |
| Accessibility | eligibility, funding and payout requirements |
| Risk | regulatory/compliance uncertainty + operational exposure |
| Complexity | steps and required user actions |
| Freshness | age/confidence of evidence |

---

## 6. Time and deadline model

Never reduce route time to a single optimistic ETA. Represent a **window** plus
operational constraints:

- Funding time, provider processing, settlement, payout
- Business vs calendar days, cutoffs and time zones
- Compliance-review uncertainty
- Recipient-side availability, weekends/holidays
- Required user actions

For deadlines, calculate a **safety buffer**. Do **not** call a route "guaranteed" until
there is actual execution control plus an enforceable service commitment. Use language
like *estimated arrival*, *deadline confidence*, *buffer*.

---

## 7. Architecture intent

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

- **Start with PostgreSQL.** A graph database is not required for the first
  implementation. The conceptual graph is the domain model; physical storage evolves
  when route scale/query patterns justify it.
- **Provider adapters are separate from the core routing engine.** The engine consumes
  normalized quotes, constraints, and evidence — never provider-specific API details.
- Later: event store / observations, background quote refresh, provider webhooks,
  reliability analytics, B2B API.

Data flow: `USER INTENT → NORMALIZATION → GRAPH QUERY → FEASIBILITY → COST/TIME MODEL →
ROUTE RANKING → EXPLANATION → PROVIDER HANDOFF → OBSERVATION`.

---

## 8. The critical boundary: intelligence vs money movement

The safest first architecture is **informational/referral**. Money Road does not
receive, custody, convert, or transmit customer funds; it recommends a route and hands
the user to a regulated provider.

```
PHASE 1:  User → Money Road (recommendation) → provider → transaction
LATER:    User → Money Road → licensed execution partner → transaction
MUCH LATER: User → Money Road → Money Road-controlled execution → recipient
```

Regulatory analysis must be done by qualified counsel **per jurisdiction** before any
execution feature. Do not assume "we only orchestrate" means "no license needed"
(e.g., Canada MSB rules; BEAC/CEMAC payment-services regulation for Cameroon).

---

## 9. Research discipline

- **Primary source first**: provider/regulator documentation or controlled observation.
- Date every important claim.
- Separate *supported* from *possible*.
- Separate *quoted* from *estimated*.
- Separate *technical capability* from *legal availability*.
- Separate *provider claims* from *our observations*.
- When a route cannot be verified, **say so explicitly**.
- Prefer official APIs, authorized partnerships, published data, and manual verification
  over scraping (scraping is brittle and may violate provider terms).

Specific hard rules:
- **China / Alipay**: never encode "stablecoin → Alipay" as fact just because a
  technical bridge exists. Verify origin-country eligibility, exact recipient payout,
  account/wallet requirements, limits, current availability, provider authorization.
- **Stablecoins**: a rail, not the product identity. Evaluate the complete chain only
  after every edge is included (local funding → compliant on-ramp → stablecoin
  settlement → compliant off-ramp → local payout).

---

## 10. Pitfalls & guardrails (do not repeat these)

1. Building the wallet first → start informational/referral; prove demand first.
2. "Nobody does this" → comparison tools and orchestration already exist; define the
   whitespace precisely (underserved corridors + end-to-end roadmap + deadline reasoning
   + explainability).
3. Treating a provider list as routing → model routes as graph paths.
4. Comparing only advertised fees → calculate effective delivered cost.
5. Using stale quotes → store source, timestamp, freshness, confidence.
6. Assuming Web3 is always cheaper → compare complete paths, not individual rails.
7. Promising a guarantee too early → use confidence/buffer language.
8. Hiding commercial relationships → separate ranking from monetization; disclose.
9. Going global immediately → go deep on one corridor first.
10. Scraping everything → prefer APIs/partnerships/manual verification.
11. Building the globe before the engine → domain model + evidence first.
12. Confusing ETA with certainty → represent distributions/buffers/failure probability.
13. Ignoring the last mile → payout eligibility is a hard constraint.
14. Ignoring funding constraints → model first and last mile equally.
15. Becoming a crypto product → stablecoins are one possible rail.
16. Automating before learning payments → manually research/execute simulated routes
    before automating.

---

## 11. Production gates (before a route is "live")

1. **Route completeness** — at least one end-to-end path has no unknown critical edge.
2. **Evidence** — critical claims have current supporting evidence.
3. **Cost** — sender cost and recipient amount calculable or clearly uncertain.
4. **Time** — timing and operational constraints represented.
5. **Eligibility** — material eligibility and limits known.
6. **Legal boundary** — activity/handoff reviewed for applicable jurisdiction.
7. **Failure handling** — unsupported routes fail visibly.
8. **Observability** — search, selection, handoff, and outcome events recorded.
9. **Freshness** — stale evidence cannot silently drive production.
10. **Explanation** — a human can inspect why a route was recommended.

### Failure states
A useful **"no route"** result is better than a confident false route. Handle explicitly:
no route found; critical edge not verified; user not eligible; amount over limit;
deadline incompatible; pricing/evidence stale; routes cannot be compared; provider
availability unconfirmed.

---

## 12. Instrumentation (events to emit)

`route_search_started`, `route_search_completed`, `route_count_returned`,
`route_rejected`, `route_selected`, `provider_handoff`, `user_reported_success`,
`user_reported_failure`, `evidence_expired`, `quote_changed`.

---

## 13. Monetization discipline

Affiliate/referral monetization is plausible (Wise, Western Union, MoneyTransfers.com),
but it creates a **conflict-of-interest risk**: the route that pays the most commission
may not be best for the customer. **Commercial compensation must never silently alter
the route score.** If commercial relationships exist, disclose them and keep ranking
logic separate from monetization.

---

## 14. Working conventions for this repo

- Language/stack intent: **Rust** API backend; **PostgreSQL** storage; web/mobile UI.
  (Confirm actual tooling before assuming; this repo currently contains only spec docs.)
- `AGENTS.md` is the agent-facing guide. Keep it in sync with the two source documents if
  the project drifts.
- Domain/evidence/route concepts from Section 3–5 must remain first-class in code —
  never collapse a route into a provider name or a quote into a constant.
- Before changing behavior, ground every claim in a real file:line reference, driven by
  the current code, not assumptions.

---

## 15. Deliberately postponed (do not scope these in early work)

Global route discovery, complex ML prediction, automatic multi-provider execution,
balances/wallets, custody, token issuance, large provider marketplace, complex globe
animation, universal scraping, guarantees/insurance, enterprise orchestration before
consumer route learning.

---

## 16. Definition of done (first route)

A real user can enter a concrete Cameroon → China request; Money Road returns a complete
route **or explains why it cannot**. The route is represented as explicit edges; critical
claims are evidence-backed and freshness-aware; displayed cost and timing are explainable;
material eligibility and limits are checked; every major user step is visible; unsupported
Alipay/stablecoin/bank/provider behavior is never silently assumed; external regulated
execution stays with the appropriate provider; outcome data is captured; and the team can
state what must be generalized before adding another corridor.
