use axum::Router;
use sqlx::{sqlite::SqlitePoolOptions, Sqlite};
use std::{env, time::Duration};
mod db;
mod jwt;
mod middleware;
mod routes;

#[macro_use]
extern crate log;

#[derive(Clone)]
struct AppState {
  db_pool: sqlx::Pool<Sqlite>,
}

#[tokio::main]
async fn main() {
  // Init logger
  env_logger::init();

  // Start DB Pool
  let db_url = env::var("DATABASE_URL").expect("Database URL should be set in env file");
  let pool = SqlitePoolOptions::new()
    .max_connections(5)
    .acquire_timeout(Duration::from_secs(3))
    .connect(&db_url)
    .await
    .expect("can't connect to database");

  // Start http listener & log
  let listener =
    tokio::net::TcpListener::bind(env::var("HOST_URL").unwrap_or("0.0.0.0:3000".to_owned()))
      .await
      .unwrap();
  info!("App listening on {}", listener.local_addr().unwrap());

  let app = Router::new()
    .nest("/api", routes::app())
    .with_state(AppState { db_pool: pool });
  
  // Serve app
  axum::serve(listener, app).await.unwrap();
}
