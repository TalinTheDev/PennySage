use axum::Router;
use env_logger::Env;
use log::{error, info};
use sqlx::{sqlite::SqlitePoolOptions, Sqlite};
use std::{env, process::ExitCode, time::Duration};
use tokio::net::TcpListener;
use tokio::signal;
use tower_http::timeout::TimeoutLayer;
mod db;
mod jwt;
mod middleware;
mod routes;

#[derive(Clone)]
struct AppState {
  db_pool: sqlx::Pool<Sqlite>,
}

#[tokio::main]
async fn main() -> ExitCode {
  // Init logger
  let env = Env::default().filter_or("API_LOG_LEVEL", "INFO");
  env_logger::init_from_env(env);

  match env::var("RUNTIME") {
    Ok(runtime) => {
      if runtime != "docker" {
        // Load env file
        if let Err(e) = dotenvy::dotenv() {
          error!("Error reading env variables ({})", e);
          return ExitCode::FAILURE;
        }
      }
    }
    Err(_) => {}
  }

  // Start DB Pool
  let db_url = match env::var("DATABASE_URL") {
    Ok(db_url) => db_url,
    Err(e) => {
      error!("Couldn't read DATABASE_URL. Error: {}", e);
      return ExitCode::FAILURE;
    }
  };
  let pool = match SqlitePoolOptions::new()
    .max_connections(5)
    .acquire_timeout(Duration::from_secs(3))
    .connect(&db_url)
    .await
  {
    Ok(pool) => pool,
    Err(e) => {
      error!("Couldn't connect to database. Error: {}", e);
      return ExitCode::FAILURE;
    }
  };

  // Start http listener & log
  let host_url = env::var("HOST_URL").unwrap_or("0.0.0.0:3000".to_owned());
  let Ok(listener) = TcpListener::bind(host_url.clone()).await else {
    error!("Error starting server listener at {}", host_url);
    return ExitCode::FAILURE;
  };
  println!("{esc}c", esc = 27 as char);
  info!("App listening on {}", listener.local_addr().unwrap());

  let app = Router::new()
    .nest("/api", routes::app())
    .layer(TimeoutLayer::new(Duration::from_secs(10)))
    .with_state(AppState { db_pool: pool });

  // Serve app
  let Ok(_) = axum::serve(listener, app)
    .with_graceful_shutdown(shutdown_signal())
    .await
  else {
    error!("Error serving app.");
    return ExitCode::FAILURE;
  };

  ExitCode::SUCCESS
}

async fn shutdown_signal() {
  let ctrl_c = async {
    signal::ctrl_c()
      .await
      .expect("failed to install Ctrl+C handler");
  };

  #[cfg(unix)]
  let terminate = async {
    signal::unix::signal(signal::unix::SignalKind::terminate())
      .expect("failed to install signal handler")
      .recv()
      .await;
  };

  #[cfg(not(unix))]
  let terminate = std::future::pending::<()>();

  tokio::select! {
      _ = ctrl_c => {},
      _ = terminate => {},
  }
}
