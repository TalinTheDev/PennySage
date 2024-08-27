use axum::{http::HeaderValue, Router};
use http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use sqlx::{sqlite::SqlitePoolOptions, Sqlite};
use std::{env, time::Duration};
use tower_http::cors::{Any, Cors, CorsLayer};
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
  let db_url = env::var("DATABASE_URL").expect("Database URL should be set in env file");
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
  print!("{esc}c", esc = 27 as char);
  println!("App listening on {}", listener.local_addr().unwrap());
  let _origins: [String; 2] = [
    "http://192.168.1.33:5173".parse().unwrap(),
    "http://localhost:5173".parse().unwrap(),
  ]; // TODO: Change this for prod
  let app = Router::new()
    .nest("/api", routes::app())
    .layer(
      CorsLayer::very_permissive(), // CorsLayer::new()
                                    //   .allow_origin(Any)
                                    //   .allow_credentials(true)
                                    //   .allow_headers(Any)
                                    //   .allow_methods(Any),
    )
    .with_state(AppState { db_pool: pool });
  // Serve app
  axum::serve(listener, app).await.unwrap();
}
