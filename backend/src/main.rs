use axum::response::Redirect;
use axum::{
    debug_handler,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Form, Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;
use tower_http::cors::CorsLayer;

// エラーハンドリングの参考: https://github.com/tokio-rs/axum/blob/main/examples/anyhow-error-response/src/main.rs

#[tokio::main]
async fn main() {
    let _ = dotenv::dotenv();
    let url = std::env::var("DATABASE_URL").unwrap();
    let pool = SqlitePool::connect(&url).await.unwrap();

    let app = Router::new()
        .route("/", get(list))
        .route("/create", post(create))
        .route("/delete/:id", get(delete))
        .route("/update", post(update))
        .with_state(pool)
        .layer(CorsLayer::very_permissive());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize, Deserialize)]
struct Todo {
    id: i64,
    description: String,
    done: bool,
}

#[derive(Serialize, Deserialize)]
struct NewTodo {
    description: String,
}

#[debug_handler]
async fn list(State(pool): State<SqlitePool>) -> Result<Json<Vec<Todo>>, AppError> {
    let todos = sqlx::query_as!(Todo, "SELECT id, description, done FROM todos ORDER BY id")
        .fetch_all(&pool)
        .await?;
    Ok(Json(todos))
}

#[debug_handler]
async fn create(
    State(pool): State<SqlitePool>,
    Form(todo): Form<NewTodo>,
) -> Result<Redirect, AppError> {
    sqlx::query!(
        "INSERT INTO todos (description) VALUES (?)",
        todo.description
    )
    .execute(&pool)
    .await?;
    Ok(Redirect::to("http://localhost:5173"))
}

#[debug_handler]
async fn delete(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> Result<Redirect, AppError> {
    sqlx::query!("DELETE FROM todos WHERE id = ?", id)
        .execute(&pool)
        .await?;
    Ok(Redirect::to("http://localhost:5173"))
}

#[debug_handler]
async fn update(
    State(pool): State<SqlitePool>,
    Form(todo): Form<Todo>,
) -> Result<Redirect, AppError> {
    sqlx::query!(
        "UPDATE todos SET description = ?, done = ? WHERE id = ?",
        todo.description,
        todo.done,
        todo.id
    )
    .execute(&pool)
    .await?;
    Ok(Redirect::to("http://localhost:5173"))
}

struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
