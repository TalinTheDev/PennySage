use axum::Router;
use env_logger::Env;
use log::{error, info};
use sqlx::{sqlite::SqlitePoolOptions, Connection, Executor, Sqlite};
use std::fs;
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

  let rt = env::var("RUNTIME");
  if let Ok(runtime) = rt {
    if runtime == "docker" {
      info!("Running in docker container...")
    } else {
      match load_local_env() {
        Some(e) => {
          return e;
        }
        None => {}
      }
    }
  } else {
    match load_local_env() {
      Some(e) => {
        return e;
      }
      None => {}
    }
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
    .acquire_timeout(Duration::from_secs(5))
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

  backup_database(db_url).await;
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

fn load_local_env() -> Option<ExitCode> {
  info!("Running outside of docker container...");
  // Load env file
  if let Err(e) = dotenvy::dotenv() {
    error!("Error reading env variables ({})", e);
    return Some(ExitCode::FAILURE);
  }
  None
}

pub async fn backup_database(db_url: String) {
  match fs::create_dir_all("./backups") {
    Ok(_) => {
      info!("Backup directory created successfully.");
    }
    Err(e) => {
      error!(
        "Error creating backup directory ({}). Could not backup database",
        e
      );
      return;
    }
  }

  delete_old_backups();
  let connection = sqlx::SqliteConnection::connect(&db_url).await;

  if let Ok(mut connection) = connection {
    match connection
      .execute(
        format!(
          "VACUUM INTO './backups/db-backup_{}.db'",
          chrono::Local::now().to_rfc3339()
        )
        .as_str(),
      )
      .await
    {
      Ok(_) => {
        info!("Database backup created successfully.");
      }
      Err(e) => {
        error!("Error creating database backup ({})", e);
      }
    }
  } else {
    error!(
      "Error connecting to database for backup ({})",
      connection.unwrap_err()
    );
  }
}

fn delete_old_backups() {
  let backup_dir = "./backups";
  let max_backups = 5;

  let entries = fs::read_dir(backup_dir);
  if let Ok(entries) = entries {
    let mut backups: Vec<_> = entries.filter_map(|entry| entry.ok()).collect();

    // Sort the backups by modified time. Exit early if error in reading modified time to prevent backup files from being lost
    backups.sort_by_key(|entry| {
      match entry.metadata() {
        Ok(metadata) => match metadata.modified() {
          Ok(modified_time) => modified_time,
          Err(_) => {
            error!("Error getting modified time for backup file");
            return;
          }
        },
        Err(_) => {
          error!("Error getting metadata for backup file");
          return;
        }
      };
    });

    // Delete old backups if there are more than max_backups
    if backups.len() > max_backups {
      for entry in &backups[..(backups.len() - max_backups)] {
        if let Err(e) = fs::remove_file(entry.path()) {
          error!("Error deleting old backup file ({})", e);
        }
      }
    }
  } else {
    error!("Error reading backup directory.");
  }
}
