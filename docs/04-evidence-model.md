# 04 — Evidence Model

> The long-term moat is **not the map** (that is easy to imitate). It is a trustworthy
> dataset of what *actually* happens on each corridor.

## Core belief

Route facts are **time-dependent claims with a source and a validity window** — never
timeless configuration. A fee, limit, payout method, or processing time is a claim that
becomes stale.

## Data fields carried per evidence item

```
value         # Current fee, rate, limit, ETA, ...
source        # API / official docs / observation / manual
observed_at   # When the value was obtained
expires_at    # When the value should be considered stale
confidence    # Confidence in the data
jurisdiction  # Country / regulatory context
conditions    # Amount, payment method, recipient method, account status
```

## Evidence states

| State | Meaning | Production behavior |
|---|---|---|
| `Verified` | Current authoritative source or controlled test | May support production eligibility |
| `Observed` | Confirmed by actual execution/observation | Strong evidence; retain conditions |
| `Estimated` | Calculated/inferred | Show uncertainty |
| `Stale` | Previously valid but outside freshness window | Do not silently use for routing |
| `Unverified` | Claim not independently confirmed | Research only |
| `Unsupported` | No evidence of current executability | Exclude from production |

## Three evidence classes

1. **Official / provider data** — published by the provider or returned by an authorized
   API.
2. **Observed data** — measured through real or controlled transactions.
3. **Estimated data** — calculated from incomplete evidence. Must be labeled as estimated.

## Rules

- Every production recommendation must be explainable:
  > "We recommend this path because these edges are currently supported by these pieces
  > of evidence."
- **Never fill a missing edge with a plausible assumption.**
- **Stale evidence must never silently drive production routing.**
- A route engine that hides uncertainty is dangerous. One that **exposes** uncertainty can
  become trusted.

## Explainability

Route facts should render as: *"We recommend this path because edge A is supported by
[source, observed 2026-09-20, expires 2026-09-27, 95% confidence] and edge B is supported
by ..."*
