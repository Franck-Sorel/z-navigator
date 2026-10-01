//! Provider adapters — the *only* place that knows provider-specific API details.
//!
//! The routing engine consumes normalized quotes, constraints, and evidence via
//! these traits. It never depends on a provider's API shape. This is the
//! "adapter, not the domain model" principle.

use async_trait::async_trait;
use money_road_domain::{Corridor, Quote, UserIntent};
use money_road_evidence::{Evidence, Source};

/// A normalized pull of current quotes for a corridor.
#[derive(Debug, Clone)]
pub struct StreamQuote {
    pub corridor: Corridor,
    pub quotes: Vec<Quote>,
    /// Supporting evidence with provenance for each quote.
    pub evidence: Vec<Evidence>,
}

/// Adapter contract every provider integration implements.
#[async_trait]
pub trait ProviderAdapter: Send + Sync {
    /// Human-readable provider name (informational).
    fn provider_name(&self) -> &'static str;

    /// Whether this adapter can currently service the corridor.
    async fn supports(&self, corridor: &Corridor) -> bool;

    /// Fetch current quotes for an intent's corridor, with evidence provenance.
    async fn quotes(&self, intent: &UserIntent) -> Result<StreamQuote, AdapterError>;
}

/// Errors surfaced to the engine; never provider-specific details.
#[derive(Debug, thiserror::Error)]
pub enum AdapterError {
    #[error("provider unavailable: {0}")]
    Unavailable(String),
    #[error("provider rejected request: {0}")]
    Rejected(String),
    #[error("provider returned no usable data: {0}")]
    Empty(String),
}

/// Convenience: build `Evidence` stamped as a provider-API source.
pub fn provider_evidence(claim: String, value: serde_json::Value) -> Evidence {
    Evidence {
        id: uuid::Uuid::new_v4(),
        claim,
        value,
        source: Source::Api,
        state: money_road_evidence::EvidenceState::Verified,
        observed_at: chrono::Utc::now(),
        expires_at: chrono::Utc::now() + chrono::Duration::hours(6),
        confidence: 0.95,
        jurisdiction: None,
        conditions: None,
    }
}
