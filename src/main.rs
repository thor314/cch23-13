#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(clippy::clone_on_copy)]

mod c13;
mod error;
#[cfg(test)] mod tests;
mod utils;

use axum::{
  http::StatusCode,
  response::IntoResponse,
  routing::{get, post},
  Router,
};
use error::MyError;
use tracing::info;

async fn hello_world() -> &'static str { "Hello, world!" }

async fn error_handler() -> impl IntoResponse {
  (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
}

#[shuttle_runtime::main]
async fn main(
  #[shuttle_shared_db::Postgres] pool: sqlx::PgPool,
  #[shuttle_secrets::Secrets] secret_store: shuttle_secrets::SecretStore,
) -> shuttle_axum::ShuttleAxum {
  utils::setup(&secret_store).unwrap();
  info!("hello thor");

  sqlx::migrate!().run(&pool).await.unwrap();

  let router = Router::new()
    .route("/", get(hello_world))
    .nest("/13", c13::router(pool))
    .route("/-1/error", get(error_handler))
    .route("/-1/health", get(|| async { StatusCode::OK }));

  Ok(router.into())
}
