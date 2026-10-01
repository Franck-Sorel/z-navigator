# 08 — Research & Verification

## Research discipline

- **Primary source first**: provider/regulator documentation or controlled observation.
- **Date every important claim.**
- Separate *supported* from *possible*.
- Separate *quoted* from *estimated*.
- Separate *technical capability* from *legal availability*.
- Separate *provider claims* from *our observations*.
- When a route cannot be verified, **say so explicitly**.
- Prefer **official APIs, authorized partnerships, published data, and manual
  verification** over scraping (scraping is brittle and may violate provider terms).

## First-route research procedure

### A — Map
Cameroon funding methods · conversion options · international settlement rails · China
payout options · fiat-only paths · stablecoin-assisted paths · institutional rails.

### B — Verify each edge
Origin/destination eligibility · funding/payout method · currencies · limits · fees · FX
mechanism · expected time · KYC/AML · cutoffs · failure behavior · last verification
timestamp.

### C — Assemble complete routes
A provider is not a route. Assemble edges into complete paths and **reject paths with
unsupported critical edges.**

### D — Controlled observations
Where lawful and appropriate, run small controlled transactions and record actual timing,
fees, delivered amount, user actions, and failure points.

### E — Extract architecture
Every repeated manual task is a candidate for automation. Every repeated exception becomes
a domain rule.

## Core questions to answer before execution

- Which sender countries can legally and practically fund each candidate route?
- Which destination methods are actually available to recipients in China/Canada?
- What is the real end-to-end cost for common amounts, and how much is FX?
- Which provider APIs expose live quotes and delivery estimates?
- Which providers permit affiliate/referral relationships in the target market?
- At what point does route orchestration become regulated activity?
- What stablecoin on/off-ramp combinations are legally and operationally viable?
- How should Money Road handle KYC failures or provider rejection?
- How should route reliability be measured?
- What evidence is sufficient to label a route "safe for a deadline"?
- What is the smallest corridor where repeated demand can be demonstrated?

## Important boundary

This project is a **product/engineering strategy**, not legal advice and not a substitute
for corridor-specific regulatory counsel. Regulation must be reviewed with qualified
counsel per jurisdiction (e.g., FINTRAC MSB rules; BEAC/CEMAC payment-services regulation)
before any execution features.
