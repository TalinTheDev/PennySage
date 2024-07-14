use axum::Router;
use sqlx::{sqlite::SqlitePoolOptions, Sqlite};
use std::{env, time::Duration};
use tower_http::cors::CorsLayer;
mod db;
mod jwt;
mod middleware;
mod routes;

#[derive(Clone)]
struct AppState {
  db_pool: sqlx::Pool<Sqlite>,
}

#[tokio::main]
async fn main() {
  // Load env file
  dotenvy::dotenv().expect("env file should be present");

  // Start DB Pool
  let db_url = env::var("DATABASE_URL").expect("Databse URL should be set in env file");
  let pool = SqlitePoolOptions::new()
    .max_connections(5)
    .acquire_timeout(Duration::from_secs(3))
    .connect(&db_url)
    .await
    .expect("can't connect to database");

  // Start http listener & log
  let listener =
    tokio::net::TcpListener::bind(env::var("HOST_URL").unwrap_or("127.0.0.1:3000".to_owned()))
      .await
      .unwrap();
  println!("listening on {}", listener.local_addr().unwrap());
  let app = Router::new()
    .nest("/api", routes::app())
    .layer(CorsLayer::very_permissive()) // TODO: Change this for prod
    .with_state(AppState { db_pool: pool });
  // Serve app
  axum::serve(listener, app).await.unwrap();
}
