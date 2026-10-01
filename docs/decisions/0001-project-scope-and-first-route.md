# ADR 0001 — Project scope and first route

- **Status**: Accepted
- **Date**: 2026-10-01
- **Source**: `Money_Road_Project_Vision_and_Pitfall_Handbook.docx`,
  `Money_Road_First_Route_Production_Spec.docx`

## Context

Money Road is a consumer-facing financial routing intelligence platform. We need to lock
in what the initial product is and is not, and choose a first corridor to act as the
"laboratory."

## Decision

1. **Informational / referral first.** Money Road does not receive, custody, convert, or
   transmit customer funds in phase 1. It recommends a route and hands the user to a
   regulated provider.
2. **First route = Cameroon → China** (single corridor, "go deep not global").
3. **No wallet / custody / token issuance / guaranteed delivery / globe-first
   development.**
4. **Domain model is a directed graph**; routes are ordered edge sets, not provider names.
5. **Evidence is the moat** — time-dependent, source-tagged, freshness-aware.
6. **First route may be partially manual** to learn the lifecycle before automating.

## Consequences

- Regulatory analysis is required per jurisdiction before any execution feature
  (do not assume orchestration = unlicensed).
- Core investments go into the domain/evidence model and routing engine before UI polish.
- Every future corridor should become *verified data + configuration*, not a new
  application.
