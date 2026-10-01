//! Route engine — constrained path optimization, not merely lowest fee.
//!
//! The engine computes an *effective delivered cost* and preserves ranking
//! dimensions so user preferences never hide route quality in one mysterious
//! score.

#![allow(dead_code)] // placeholder until scoring is fully wired

use money_road_domain::Edge;

/// A single edge's contribution to cost, decomposed so it is explainable.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CostBreakdown {
    pub explicit_fees: f64,
    pub fx_spread: f64,
    pub network_fees: f64,
    pub intermediary_fees: f64,
    pub expected_loss_from_failure: f64,
    pub other_mandatory_costs: f64,
}

impl CostBreakdown {
    /// `effective_cost = explicit_fees + FX_spread + network_fees +
    /// intermediary_fees + expected_loss_from_failure + other_mandatory_costs`
    pub fn effective_cost(&self) -> f64 {
        self.explicit_fees
            + self.fx_spread
            + self.network_fees
            + self.intermediary_fees
            + self.expected_loss_from_failure
            + self.other_mandatory_costs
    }
}

/// Ranking dimensions — kept separate and visible, never collapsed into one score.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct RouteDimensions {
    /// Sender spend / recipient amount / effective cost.
    pub cost: f64,
    /// Expected window + deadline buffer.
    pub speed: f64,
    /// Observed success + evidence quality.
    pub reliability: f64,
    /// Eligibility, funding and payout requirements (converted to a normalized score).
    pub accessibility: f64,
    /// Regulatory/compliance + operational exposure (inverted: lower is better).
    pub risk: f64,
    /// Steps and required user actions.
    pub complexity: f64,
    /// Age/confidence of evidence.
    pub freshness: f64,
}

/// Placeholder extraction of cost components from an edge. In a real
/// implementation these come from verified quotes and evidence, not assumptions.
pub fn cost_from_edge(edge: &Edge) -> CostBreakdown {
    CostBreakdown {
        explicit_fees: edge.fee.unwrap_or(0.0),
        // FX spread is difference between effective and reference rate;
        // placeholder uses the edge fx_rate only.
        fx_spread: 0.0,
        ..Default::default()
    }
}
