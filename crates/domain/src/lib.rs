//! Canonical domain model for Money Road.
//!
//! The conceptual system is a **directed graph**: nodes are financial
//! states/institutions, edges are transformations/transfers. The same model powers
//! the routing engine and any globe/graph UI.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// ISO 4217 currency code (e.g. `XAF`, `CNY`).
pub type Currency = String;

/// Two-letter country code (e.g. `CM`, `CN`).
pub type Country = String;

/// Origin/destination market pair.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Corridor {
    pub origin: Country,
    pub destination: Country,
    pub currency_pair: (Currency, Currency),
}

/// What the user needs.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserIntent {
    pub amount: f64,
    pub source_currency: Currency,
    pub destination_currency: Currency,
    /// Optional soft deadline (UTC). Treated as a first-class constraint.
    pub deadline: Option<DateTime<Utc>>,
    pub funding_method: FundingMethod,
    pub payout_method: PayoutMethod,
    pub preferences: Preferences,
}

/// Preference axis the user optimizes for. Dimensions are always kept visible.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Preferences {
    pub cheapest: bool,
    pub fastest: bool,
    pub simplest: bool,
    pub balanced: bool,
}

/// How value enters the system.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum FundingMethod {
    Bank,
    MobileMoney,
    Card,
    Cash,
    Crypto,
}

/// How value leaves the system at the destination.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PayoutMethod {
    Bank,
    MobileWallet,
    Cash,
    Stablecoin,
}

/// Mechanism moving value.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Rail {
    Bank,
    Mto,
    MobileMoney,
    Card,
    Stablecoin,
    Other,
}

/// Payment participant.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Provider {
    pub id: Uuid,
    pub name: String,
    pub markets: Vec<Country>,
    pub capabilities: Vec<String>,
    /// Free-form regulatory/jurisdiction context (informational only).
    pub regulatory_context: Option<String>,
}

/// One executable movement/transformation in the graph.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Edge {
    pub id: Uuid,
    /// Order within a route.
    pub sequence: u32,
    pub provider_id: Uuid,
    pub rail: Rail,
    pub from_node: String,
    pub to_node: String,
    pub input_asset: Currency,
    pub output_asset: Currency,
    pub fee: Option<f64>,
    pub fx_rate: Option<f64>,
    pub min_amount: Option<f64>,
    pub max_amount: Option<f64>,
    pub duration_min: Option<u32>,
    pub duration_max: Option<u32>,
    pub eligibility_rules: Vec<String>,
    pub evidence_ids: Vec<Uuid>,
    pub observed_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub status: EdgeStatus,
}

/// Whether an edge is currently considered usable.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum EdgeStatus {
    Supported,
    Unsupported,
    PendingVerification,
}

/// Observed/retrieved price with timestamp and expiry.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Quote {
    pub amount: f64,
    pub fees: f64,
    pub fx_rate: f64,
    pub captured_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

/// Conditions to use an edge.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Eligibility {
    pub country: Country,
    pub identity_required: Option<String>,
    pub account_required: Option<String>,
    pub purpose_restrictions: Option<String>,
    pub min_amount: Option<f64>,
    pub max_amount: Option<f64>,
}

/// Reference to evidence supporting a route claim.
///
/// The full evidence record (value, source, observed_at, expires_at, confidence,
/// jurisdiction, conditions) lives in the `evidence` crate.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EvidenceRef {
    pub id: Uuid,
    pub claim: String,
}

/// Ordered set of edges — the product object. **A route is not a provider.**
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Route {
    pub id: Uuid,
    pub corridor: Corridor,
    pub status: RouteStatus,
    pub input_amount: f64,
    pub source_currency: Currency,
    pub destination_currency: Currency,
    pub expected_sender_spend: f64,
    pub expected_recipient_amount: f64,
    pub expected_duration_min: Option<u32>,
    pub expected_duration_max: Option<u32>,
    pub deadline_buffer: Option<std::time::Duration>,
    pub confidence: f64,
    pub edges: Vec<Edge>,
    pub evidence_ids: Vec<Uuid>,
    pub failure_mode_ids: Vec<Uuid>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RouteStatus {
    Draft,
    Feasible,
    Rejected,
    InProduction,
}

/// What happened after execution.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Observation {
    pub route_id: Uuid,
    pub actual_cost: f64,
    pub actual_time: std::time::Duration,
    pub succeeded: bool,
    pub recorded_at: DateTime<Utc>,
    pub notes: Option<String>,
}

/// Known way a route fails.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FailureMode {
    pub id: Uuid,
    pub trigger: String,
    pub mitigation: Option<String>,
    /// Whether the likelihood/impact is quantified.
    pub uncertainty: bool,
}
