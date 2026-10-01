# 03 — Domain Model

## The core abstraction

The conceptual system is a **directed graph** where:

- **Nodes** = financial states / institutions (origin, a bank, a network, a wallet,
  destination).
- **Edges** = transformations / transfers (fund, convert, settle, pay out).

The same model powers the routing engine and any globe/graph interface.

## Canonical objects

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

## The route is first-class

A route is modelled as an **ordered set of edges**, not as a provider name:

```
Route {
  id,
  corridor,
  status,
  input_amount,
  source_currency,
  destination_currency,
  expected_sender_spend,
  expected_recipient_amount,
  expected_duration_min/max,
  deadline_buffer,
  confidence,
  edge_ids[],
  evidence_ids[],
  failure_mode_ids[]
}
```

### Edge record

```
Edge {
  id,
  sequence,
  provider_id,
  rail_id,
  from_node / to_node,
  input_asset / output_asset,
  fee,
  fx_rate,
  min_amount / max_amount,
  duration_min / duration_max,
  eligibility_rules[],
  evidence_ids[],
  observed_at,
  expires_at,
  status
}
```

## Route-edge contract

Every edge must be able to answer, with evidence:

1. What transformation or movement occurs?
2. Who operates it?
3. Where does value start and end?
4. What currencies/assets are involved?
5. What funding and payout methods are supported?
6. What are fees, FX costs, and other known costs?
7. What are min/max amounts?
8. What eligibility / KYC / purpose requirements apply?
9. What are cutoff times and business-day constraints?
10. What is the expected processing-time range?
11. What can cause failure?
12. What evidence supports each claim, and when does it expire?

## Guardrails

- **A provider is not a route.** Assemble edges into complete paths; reject paths with
  unsupported critical edges.
- Keep `Corridor`, `Route`, `Quote`, `Evidence`, and `Edge` as real first-class types in
  code — never collapse them into a generic map or a hard-coded constant.
