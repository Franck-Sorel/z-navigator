# 07 — First Route: Cameroon → China

## Strategic decision

The first route is the **laboratory** that turns the concept into a real system. The
objective is **not maximum automation** — it is to take one corridor from user intent to
an evidence-backed, explainable route recommendation and learn what to standardize.

> The first route may be **partially manual**. Optimize for learning the complete route
> lifecycle before optimizing for scale.

## Why this corridor

Cameroon → China forces the system to confront **funding, FX, settlement, payout,
eligibility, limits, timing, compliance, and stablecoin-assisted paths**. None are assumed
to work; each must be **verified**.

## What route one must prove

1. A real user can describe a transfer using a small set of inputs.
2. Money Road can model the problem as funding / conversion / settlement / payout edges.
3. Every critical edge has evidence with source, timestamp, scope, and confidence.
4. At least one complete route can be explained from sender to recipient.
5. Routes can be compared on delivered amount, total cost, time, constraints, and
   evidence quality.
6. The system can say **"no route"** or **"not verified"** instead of inventing an answer.
7. Provider handoff works without Money Road taking custody.
8. Route facts can be revalidated as fees, limits, availability, and eligibility change.

## Non-goals (do not build yet)

No wallet or custody · no token issuance · no guaranteed-delivery promise · no global
rollout · no unverified provider availability · no assumption that blockchain is cheaper
· no globe-first development.

## Candidate route classes (research categories, not availability claims)

| Class | Investigate | Purpose |
|---|---|---|
| Bank / SWIFT | Correspondent path, fees, value date, recipient requirements | Fiat baseline |
| Money transfer provider | Origin eligibility, China payout, fees, limits, timing | Consumer baseline |
| Institutional payout API | Partner availability, onboarding, supported payout | Orchestration rail |
| Stablecoin-assisted | On-ramp, blockchain transfer, compliant off-ramp, CNY payout | Tests Web3 as one edge |
| Hybrid | Fiat funding + stablecoin settlement + local payout where lawful | Tests end-to-end optimization |

## The China / Alipay rule

Never encode **"stablecoin → Alipay"** as fact merely because a technical bridge exists.
Verify for the exact corridor:

- Origin-country eligibility
- Exact recipient payout
- Recipient account/wallet requirements
- Limits
- Current availability
- Provider authorization
- Fresh evidence

## Stablecoin checklist (complete chain)

Local funding → compliant conversion/on-ramp → stablecoin settlement → compliant off-ramp
→ local payout. Account for: on-ramp fee/spread · network fee · liquidity/conversion
spread · off-ramp fee · payout fee · compliance friction · confirmation time · settlement
time · limits · failure/reversal behavior.
