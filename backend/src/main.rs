use axum::extract::State;
use axum::{routing::get, Json, Router, debug_handler};
use axum_error::Result;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenv::dotenv();
    let url = std::env::var("DATABASE_URL")?;
    let pool = SqlitePool::connect(&url).await?;

    let app = Router::new()
        .route("/", get(list))
        .with_state(pool)
        .layer(CorsLayer::very_permissive());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    Ok(axum::serve(listener, app).await?)
}

#[derive(Serialize, Deserialize)]
struct Todo {
    id: i64,
    description: String,
    done: bool,
}

#[debug_handler]
async fn list(State(pool): State<SqlitePool>) -> Result<Json<Vec<Todo>>> {
    let todos = sqlx::query_as!(Todo, "SELECT id, description, done FROM todos ORDER BY id")
        .fetch_all(&pool)
        .await?;
    Ok(Json(todos))
}
