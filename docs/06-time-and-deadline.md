# 06 — Time & Deadline Model

Never reduce a route to a single optimistic ETA. Represent a **window** plus operational
constraints, and reason **backward from the deadline**.

## What a route window includes

- Funding time
- Provider processing
- Settlement
- Payout
- Business vs calendar days
- Cutoffs and time zones
- Compliance-review uncertainty
- Recipient-side availability, weekends/holidays
- Required user actions

## Deadline-first UX

Treat the deadline as a **first-class constraint**:

```
Deadline:                      Friday 09:00
Recipient required:            CNY in destination account
Estimated route completion:    02:10 (window)
Required funding start:        before 08:00 Thursday
Safety buffer:                 6h 50m
Status:                        SAFE / TIGHT / UNSAFE
```

## Language discipline

- Do **not** call a route "guaranteed" until Money Road controls the transaction **and**
  has an enforceable service commitment from the execution provider.
- Before that point use: *estimated arrival*, *deadline confidence*, *safety buffer*.

## Reliability

A route can fail at any hop: funding, KYC, liquidity, blockchain confirmation, FX
conversion, beneficiary verification, provider outage, payout rejection, recipient-side
restrictions.

```
P(route succeeds before deadline) =
    P(step 1) × P(step 2 | step 1) × ... × P(final payout | previous)   # in time
```

Initially Money Road provides a **deadline-confidence score** and a **safety buffer**,
backed by evidence. A contractual guarantee is a later product requiring execution
control, backup routes, provider SLAs, operational staff, and clear liability.
