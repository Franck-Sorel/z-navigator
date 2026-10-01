# 05 — Routing Engine

The engine optimizes a **constrained path**, not merely the lowest fee.

## Feasibility

```
Feasible(route) =
    supported_origin            AND
    supported_destination       AND
    funding_method_supported    AND
    receiving_method_supported  AND
    amount_within_limits        AND
    regulatory_constraints_satisfied AND
    route_available_at_required_time
```

## Effective cost

Compare complete **delivered** cost, not advertised transfer fees (conceptual formula,
evolution encouraged):

```
effective_cost = explicit_fees
               + FX_spread
               + network_fees
               + intermediary_fees
               + expected_loss_from_failure
               + other mandatory costs
```

### Cost metrics

| Metric | Definition |
|---|---|
| Sender spend | Total amount leaving sender control, incl. known fees |
| Recipient receives | Expected destination amount after known conversions/fees |
| Explicit fees | Displayed or retrieved provider/network fees |
| FX cost | Difference between effective conversion and reference rate |
| Unknown cost | Material cost that cannot currently be verified |
| Cost confidence | Confidence in the displayed cost |

## Ranking: preserve dimensions

Do **not** hide route quality in one mysterious score. Rank on visible dimensions and let
the user state a preference (cheapest / fastest / simplest / balanced).

| Dimension | Example measure |
|---|---|
| Cost | sender spend / recipient amount / effective cost |
| Speed | expected window + deadline buffer |
| Reliability | observed success + evidence quality |
| Accessibility | eligibility, funding and payout requirements |
| Risk | regulatory/compliance uncertainty + operational exposure |
| Complexity | steps and required user actions |
| Freshness | age/confidence of evidence |

`Route score = f(preferences × ranked dimensions)`, with the dimensions always visible.

## A worked intuition

A student with a tuition deadline may rationally prefer a **more expensive** route that
has a **large time buffer**. Cost is only one dimension; the engine must not silently
trade away deadline safety for a lower fee.
