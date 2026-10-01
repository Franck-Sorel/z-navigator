//! HTTP routes for the Money Road API.
//!
//! Phase 1 is informational/referral: these endpoints return *evidence-backed
//! route recommendations*, they never execute a transfer.

use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::json;
use sqlx::PgPool;

use money_road_domain::{FundingMethod, PayoutMethod, UserIntent};

pub fn router(pool: PgPool) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/api/v1/intent", axum::routing::post(intent))
        .with_state(AppState { pool })
}

#[derive(Clone)]
struct AppState {
    pool: PgPool,
}

async fn health(State(state): State<AppState>) -> Json<serde_json::Value> {
    let db_ok = sqlx::query("SELECT 1").execute(&state.pool).await.is_ok();
    Json(json!({
        "status": if db_ok { "ok" } else { "degraded" },
        "database": db_ok,
        "phase": "informational/referral",
    }))
}

/// Request shape mirrored from the `UserIntent` domain object.
#[derive(Deserialize)]
struct IntentRequest {
    amount: f64,
    source_currency: String,
    destination_currency: String,
    funding_method: FundingMethod,
    payout_method: PayoutMethod,
}

async fn intent(State(_state): State<AppState>, Json(req): Json<IntentRequest>) -> Json<serde_json::Value> {
    let _intent = UserIntent {
        amount: req.amount,
        source_currency: req.source_currency,
        destination_currency: req.destination_currency,
        deadline: None,
        funding_method: req.funding_method,
        payout_method: req.payout_method,
        preferences: Default::default(),
    };

    // Placeholder that is honest about its state: no quotes/edges are assumed.
    // A useful "no route" beats a confident false route.
    Json(json!({
        "status": "accepted",
        "message": "Intent captured. Route search requires verified edges and evidence, which are not yet wired.",
        "routes": [],
    }))
}
