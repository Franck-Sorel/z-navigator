//! Money Road API — serves the UI and orchestrates the routing pipeline.
//!
//! Pipeline (informational/referral only in phase 1):
//! `USER INTENT → NORMALIZATION → GRAPH QUERY → FEASIBILITY → COST/TIME MODEL
//!  → ROUTE RANKING → EXPLANATION → PROVIDER HANDOFF → OBSERVATION`
//!
//! Money Road does not receive, custody, convert, or transmit customer funds. It
//! recommends a route and hands the user to a regulated provider.

mod db;
mod routes;

use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "money_road_api=info,tower_http=info".into()),
        )
        .init();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://moneyroad:moneyroad@localhost:5432/moneyroad".into());

    let pool = db::connect(&database_url).await?;

    let app = routes::router(pool);
    let addr: SocketAddr = std::env::var("BIND_ADDR")
        .unwrap_or_else(|_| "0.0.0.0:8080".into())
        .parse()?;

    tracing::info!("Money Road API listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
