# 11 — Production Gates & Failure States

## Production gates

Before a route is "live," every gate below must pass.

| Gate | Required condition |
|---|---|
| G1 Route completeness | At least one end-to-end path has no unknown critical edge |
| G2 Evidence | Critical claims have current supporting evidence |
| G3 Cost | Sender cost and recipient amount are calculable or clearly uncertain |
| G4 Time | Timing and operational constraints are represented |
| G5 Eligibility | Material eligibility and limits are known |
| G6 Legal boundary | Launch activity/provider handoff reviewed for applicable jurisdiction |
| G7 Failure handling | Unsupported routes fail visibly |
| G8 Observability | Search, selection, handoff, and outcome events recorded |
| G9 Freshness | Stale evidence cannot silently drive production |
| G10 Explanation | A human can inspect why a route was recommended |

## Failure states

A useful **"no route"** result is better than a confident false route. Handle these
explicitly:

- No route found.
- Route exists but a critical edge is not verified.
- User is not eligible.
- Amount exceeds a limit.
- Deadline is incompatible.
- Pricing/evidence is stale.
- Routes exist but cannot be reliably compared.
- Provider availability cannot currently be confirmed.

## Definition of done (first route)

A real user can enter a concrete Cameroon → China request and Money Road returns a
complete route **or explains why it cannot**. Specifically the route:

- Is represented as explicit edges.
- Has critical claims that are evidence-backed and freshness-aware.
- Has explainable cost and timing.
- Has material eligibility and limits checked.
- Has every major user step visible.
- Never silently assumes unsupported Alipay/stablecoin/bank/provider behavior.
- Keeps external regulated execution with the appropriate provider.
- Captures outcome data.
- Lets the team state what must be generalized before adding another corridor.
