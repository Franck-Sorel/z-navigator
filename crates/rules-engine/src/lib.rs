//! Rules engine — eligibility, limits, timing, cutoffs, and country rules.
//!
//! These rules decide whether an edge is *available to a user*, independent of
//! cost. They are deliberately separate from the route engine's scoring.

use chrono::{DateTime, Utc};
use money_road_domain::{Edge, UserIntent};

/// Why a candidate intentionally fails, so the product can say *why* — a useful
/// "no route" is better than a confident false route.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rejection {
    FundingMethodUnsupported,
    PayoutMethodUnsupported,
    AmountBelowMinimum,
    AmountAboveMaximum,
    DeadlineIncompatible,
    EdgeUnsupported,
}

#[derive(Debug, Clone, Default)]
pub struct Evaluation {
    pub rejected: Vec<Rejection>,
    /// UTC-by which the sender would need to start, given a deadline and window.
    pub required_funding_start: Option<DateTime<Utc>>,
    /// How much slack remains before the deadline (if the edge completes in
    /// `duration_max`).
    pub deadline_buffer: Option<chrono::Duration>,
}

/// Evaluate a single edge against a user intent.
pub fn evaluate_edge(intent: &UserIntent, edge: &Edge) -> Evaluation {
    let mut result = Evaluation::default();

    if edge.status != money_road_domain::EdgeStatus::Supported {
        result.rejected.push(Rejection::EdgeUnsupported);
    }

    let amount = intent.amount;
    if let Some(min) = edge.min_amount {
        if amount < min {
            result.rejected.push(Rejection::AmountBelowMinimum);
        }
    }
    if let Some(max) = edge.max_amount {
        if amount > max {
            result.rejected.push(Rejection::AmountAboveMaximum);
        }
    }

    // Deadline reasoning, backward from the deadline.
    if let Some(deadline) = intent.deadline {
        if let Some(dmax) = edge.duration_max {
            let needed_start = deadline - chrono::Duration::minutes(dmax as i64);
            result.required_funding_start = Some(needed_start);
            if needed_start < Utc::now() {
                result.rejected.push(Rejection::DeadlineIncompatible);
            } else {
                result.deadline_buffer = Some(deadline - Utc::now());
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use money_road_domain::{Edge, EdgeStatus, FundingMethod, PayoutMethod, Preferences, Rail, UserIntent};
    use uuid::Uuid;

    fn edge(dur_max: u32, status: EdgeStatus) -> Edge {
        Edge {
            id: Uuid::new_v4(),
            sequence: 0,
            provider_id: Uuid::new_v4(),
            rail: Rail::Bank,
            from_node: "CM".into(),
            to_node: "CN".into(),
            input_asset: "XAF".into(),
            output_asset: "CNY".into(),
            fee: Some(1.0),
            fx_rate: Some(0.01),
            min_amount: Some(0.0),
            max_amount: Some(1_000_000.0),
            duration_min: Some(1),
            duration_max: Some(dur_max),
            eligibility_rules: vec![],
            evidence_ids: vec![],
            observed_at: Utc::now(),
            expires_at: Utc::now() + chrono::Duration::days(1),
            status,
        }
    }

    fn intent(deadline: DateTime<Utc>) -> UserIntent {
        UserIntent {
            amount: 100_000.0,
            source_currency: "XAF".into(),
            destination_currency: "CNY".into(),
            deadline: Some(deadline),
            funding_method: FundingMethod::Bank,
            payout_method: PayoutMethod::Bank,
            preferences: Preferences::default(),
        }
    }

    #[test]
    fn unsupported_edge_is_rejected() {
        let eval = evaluate_edge(&intent(Utc::now() + chrono::Duration::days(1)), &edge(120, EdgeStatus::Unsupported));
        assert!(eval.rejected.contains(&Rejection::EdgeUnsupported));
    }

    #[test]
    fn tight_deadline_is_incompatible() {
        let eval = evaluate_edge(&intent(Utc::now() - chrono::Duration::hours(1)), &edge(120, EdgeStatus::Supported));
        assert!(eval.rejected.contains(&Rejection::DeadlineIncompatible));
    }

    #[test]
    fn comfortable_deadline_has_buffer() {
        let eval = evaluate_edge(&intent(Utc::now() + chrono::Duration::days(2)), &edge(120, EdgeStatus::Supported));
        assert!(eval.rejected.is_empty());
        assert!(eval.deadline_buffer.is_some());
    }
}
