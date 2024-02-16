use axum::{
  debug_handler,
  extract::State,
  http::StatusCode,
  response::IntoResponse,
  routing::{get, post},
  Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool, Pool, Postgres};
use tracing::info;

pub fn router(pool: Pool<Postgres>) -> Router {
  let state = SqlState { pool };
  Router::new()
        .route("/sql", get(shuttle_database_get))
        // .route("/sql", get(shuttle_database_get))
        .with_state(state)
}

/// Add a Postgres database with the Shuttle Shared Database plugin, and add the pool to your
/// application state. Add a GET endpoint /13/sql that executes the SQL query SELECT 20231213 and
/// responds with the query result (an i32 turned into a string).
/// ```sh
/// curl http://localhost:8000/13/sql
///
/// 20231213
/// ```
// we'll need to store some state
#[derive(Clone)]
struct SqlState {
  pub pool: PgPool,
}

// examples:
// https://github.com/shuttle-hq/shuttle-examples/blob/main/axum/postgres/src/main.rs
//
// sqlx cli:
// https://lib.rs/crates/sqlx-cli
async fn shuttle_database_get(State(state): State<SqlState>) -> Result<String, StatusCode> {
  // query makes an SQL query
  // query_as is query w type-checking; makes a query that is mapped to a concrete type using FromRow
  let santa: Santa = sqlx::query_as::<Postgres, Santa>("SELECT 20231213 id")
    .fetch_one(&state.pool)
    .await
    .map_err(|e| {
      tracing::error!("error while fetching from database {e}");
      StatusCode::INTERNAL_SERVER_ERROR
    })?;
  info!("row in sql {santa:?}");
  Ok(santa.id.to_string())
}

// see:
// https://github.com/shuttle-hq/shuttle-examples/blob/main/axum/postgres/src/main.rs
#[derive(Serialize, FromRow, Debug)]
struct Santa {
  pub id: i32,
}
// pub note: String,

// -> impl IntoResponse {
//   let row = sqlx::query_as::<_, Todo>("SELECT 20231213").fetch_one(&state.pool).await.unwrap();
//   // let row = sqlx::query_as::<_, Todo>("SELECT 20231213
//   // number").fetch_one(&state.pool).await.unwrap();
//   row.id.to_string()
//   // "".to_string()
// }

// #[derive(Deserialize)]
// struct TodoNew {
//   pub note: String,
// }
