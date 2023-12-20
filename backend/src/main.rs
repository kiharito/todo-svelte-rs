use axum::{routing::get, Router};
use axum_error::Result;
use sqlx::sqlite::SqlitePool;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenv::dotenv();
    let url = std::env::var("DATABASE_URL")?;
    let pool = SqlitePool::connect(&url).await?;

    let app = Router::new()
        .route("/", get(index))
        .with_state(pool)
        .layer(CorsLayer::very_permissive());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    Ok(axum::serve(listener, app).await?)
}

async fn index() -> String {
    format!("Hello, world!")
}
