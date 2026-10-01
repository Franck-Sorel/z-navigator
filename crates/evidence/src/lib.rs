//! Evidence model — the long-term moat.
//!
//! Route facts are **time-dependent claims with a source and validity window**,
//! never timeless configuration. `Stale` evidence must never silently drive
//! production routing.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// How a value was obtained.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Source {
    Api,
    OfficialDocs,
    Observation,
    Manual,
    Estimated,
}

/// Production-behavior state of an evidence item.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum EvidenceState {
    /// Current authoritative source or controlled test.
    Verified,
    /// Confirmed by actual execution/observation.
    Observed,
    /// Calculated/inferred — show uncertainty.
    Estimated,
    /// Outside the freshness window.
    Stale,
    /// Not independently confirmed — research only.
    Unverified,
    /// No evidence of current executability — exclude from production.
    Unsupported,
}

impl EvidenceState {
    /// Whether this state may support production routing.
    pub fn supports_production(&self) -> bool {
        matches!(self, EvidenceState::Verified | EvidenceState::Observed)
    }
}

/// A single evidence item.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Evidence {
    pub id: Uuid,
    pub claim: String,
    pub value: serde_json::Value,
    pub source: Source,
    pub state: EvidenceState,
    pub observed_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub confidence: f64,
    pub jurisdiction: Option<String>,
    pub conditions: Option<String>,
}

impl Evidence {
    /// Derive the effective state from freshness plus recorded state.
    /// A previously `Verified`/`Observed` claim that has expired becomes `Stale`.
    pub fn effective_state(&self, now: DateTime<Utc>) -> EvidenceState {
        if now > self.expires_at {
            match self.state {
                EvidenceState::Verified | EvidenceState::Observed => EvidenceState::Stale,
                other => other,
            }
        } else {
            self.state
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn ev(expires: DateTime<Utc>, state: EvidenceState) -> Evidence {
        Evidence {
            id: Uuid::new_v4(),
            claim: "test".into(),
            value: serde_json::json!(1),
            source: Source::Manual,
            state,
            observed_at: Utc::now(),
            expires_at: expires,
            confidence: 0.9,
            jurisdiction: None,
            conditions: None,
        }
    }

    #[test]
    fn verified_expires_to_stale() {
        let past = Utc.timestamp_opt(1_000_000, 0).unwrap();
        let now = Utc.timestamp_opt(5_000_000, 0).unwrap();
        assert_eq!(ev(past, EvidenceState::Verified).effective_state(now), EvidenceState::Stale);
    }

    #[test]
    fn fresh_verified_supports_production() {
        let future = Utc.timestamp_opt(9_000_000, 0).unwrap();
        let now = Utc.timestamp_opt(5_000_000, 0).unwrap();
        let e = ev(future, EvidenceState::Verified);
        assert_eq!(e.effective_state(now), EvidenceState::Verified);
        assert!(e.effective_state(now).supports_production());
    }
}
