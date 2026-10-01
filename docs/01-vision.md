# 01 — Vision

## The product, in one line

Money Road is a **consumer-facing financial routing intelligence platform** for
cross-border money movement. It is **not initially a remittance company** and it is
**not a wallet**. It maps existing payment rails and tells a person how money can travel
from an origin to a destination under real constraints: amount, currencies, deadline,
cost, speed, reliability, and eligibility.

## The core insight

The payment **rails already exist** — banks, MTOs, mobile money, cards, and blockchains.
What is fragmented is the *consumer-facing roadmap across those rails*. A person cannot
easily see the whole path their money would take, what each hop costs, how long it takes,
what is required, and what could fail.

Money Road sits in the **strategic gap** between:

- **Consumer comparison tools** (Monito, MoneyTransfers.com) — compare providers/prices.
- **Institutional orchestration infrastructure** (Nium, Circle) — move money for banks.

Money Road's differentiation is **route depth, corridor specificity, end-to-end
instructions, deadline reasoning, and transparent route evidence** — not "another
comparison website."

## The product object is a route

A **route** is not a provider. It is a sequence of financial transformations modelled as
a path through a directed graph:

```
Cameroon                     China
  | XAF                        | CNY
  v                            ^
Funding rail  →  FX / convert  →  Settlement rail  →  Off-ramp  →  Recipient
```

Representing this as a **directed graph** lets one domain model power both the routing
engine and any future globe/graph UI.

## The mindset to lock in

- **Build a map, not a money mover.**
- **Calculate the complete path**, not just the fastest rail.
- **Measure effective cost**, not advertised fees.
- **Use "estimated arrival" / "deadline confidence" / "buffer"** — never promise a
  guarantee until you control the transaction.
- **Stablecoins are one possible road**, not the product.
- **Go deep on one painful corridor** before going global.

## The real consumer problem

> I have X money. I am in country A. The recipient is in country B. They need Y currency
> by deadline D. *Which route should I use? What exactly do I have to do? What will it
> really cost? How long will each step take? What can fail? What is my safety margin?*

## North Star

> *"I have 100,000 XAF in Cameroon. My brother needs CNY in China tomorrow morning.
> Show me the safest affordable route."*

The ideal answer is a **financial map**:

```
ORIGIN → FUND → CONVERT → SETTLE → CONVERT → PAYOUT → DESTINATION
Cost: explained   Time: explained   Deadline: explained
Risk: explained   Evidence: visible   Action: clear
```

Not a generic list of money-transfer companies.

## Emphasis on evidence over assertions

The strongest verified conclusions are narrow:

- Remittance pricing varies materially by corridor and provider.
- Consumer comparison products exist.
- Institutional multi-rail payment infrastructure exists.
- Stablecoin settlement is becoming more institutionalized.
- Cross-border money movement is regulated in relevant jurisdictions.

The product opportunity must be **validated as a differentiated consumer routing and
explanation layer** in specific underserved corridors — not extrapolated from global
averages.
