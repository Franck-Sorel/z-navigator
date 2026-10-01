# 12 — Instrumentation & Observability

## Events to emit

| Event | Purpose |
|---|---|
| `route_search_started` | Demand |
| `route_search_completed` | Search success |
| `route_count_returned` | Coverage |
| `route_rejected` | Missing edges/constraints |
| `route_selected` | Preference |
| `provider_handoff` | Conversion |
| `user_reported_success` | Outcome |
| `user_reported_failure` | Failure learning |
| `evidence_expired` | Data freshness |
| `quote_changed` | Pricing volatility |

Emit these across search, selection, handoff, and outcome so the feedback loop (and the
production gates G8/G9) can be measured.

## Ops runbook (first route)

1. Create corridor inventory.
2. Create provider/rail inventory.
3. Collect primary-source evidence.
4. Normalize candidate edges.
5. Mark unsupported claims as unverified.
6. Assemble complete routes.
7. Validate eligibility and limits.
8. Calculate end-to-end cost/time.
9. Review legal boundary.
10. Run controlled observations where appropriate.
11. Launch narrowly.
12. Monitor failures and stale evidence.
13. Turn repeated manual work into product requirements.

## Production deliverables

- Verified corridor map
- Normalized route dataset
- Evidence register
- At least one complete production route
- Rejected/unsupported route register
- Explainable route UI
- Provider handoff process
- Failure/incident log
- Observation dataset
- Architecture-change log
- Repeatable next-corridor onboarding procedure

## Route-two architecture extraction

After route one reaches production, run a formal extraction review (don't clone the
implementation):

| Question | Learning |
|---|---|
| Which edge types repeat? | Core domain abstractions |
| Which provider fields repeat? | Adapter interface |
| Which eligibility rules repeat? | Rules engine |
| Which evidence types repeat? | Evidence schema |
| Which timing constraints repeat? | Temporal model |
| Which failures repeat? | Failure taxonomy |
| Which data changes frequently? | Refresh/quote architecture |
| Which work remains manual? | Operations/automation boundary |
| Which differences are country-specific? | Corridor configuration |
| Which UI elements remain stable? | Product primitives |

## Deliberately postponed

Global route discovery · complex ML prediction · automatic multi-provider execution ·
balances/wallets · custody · token issuance · large provider marketplace · complex globe
animation · universal scraping · guarantees/insurance · enterprise orchestration before
consumer route learning.
