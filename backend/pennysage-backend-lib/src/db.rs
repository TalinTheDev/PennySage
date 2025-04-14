use chrono;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::Connection;
use sqlx::Executor;
use std::fs;
use std::time::Duration;

/// Creates and returns a new database connection pool
///
/// The pool is created with a maximum of 5 connections and a timeout of 5 seconds
///
/// Takes in a database URL as a string slice
///
/// Returns a `Result<>` containing the SqlitePool or an `sqlx::error`
pub async fn create_db_pool(db_url: &str) -> Result<sqlx::SqlitePool, sqlx::Error> {
    SqlitePoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&db_url)
        .await
}

/// Creates a backup of the database at its current state
///
/// The backup is created in the `./backups` directory which is created if it doesn't exist
///
/// Backup file is named `db-backup_<timestamp>.db`
///
/// Before creating a new backup, it deletes old backups using `delete_old_backups`
///
/// If any error occurs while creating the backup, it will return an error
pub async fn backup_database(db_url: String) -> Result<String, Box<dyn std::error::Error>> {
    match fs::create_dir_all("./backups") {
        Ok(_) => (),
        Err(e) => {
            return Err(Box::new(e));
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
                return Ok("Database backup created successfully".to_string());
            }
            Err(e) => {
                return Err(Box::new(e));
            }
        }
    } else {
        return Err(Box::new(connection.unwrap_err()));
    }
}

/// Deletes oldest database backup file if there are more than 5 old backups
///
/// If any error occurs while deleting the backup files, it will exit early and silently
///
/// Early exit prevents backup files from being lost if something goes wrong in the process
pub fn delete_old_backups() {
    let backup_dir = "./backups";
    let max_backups = 5;

    // Read the backup directory
    let entries = fs::read_dir(backup_dir);
    if let Ok(entries) = entries {
        let mut backups: Vec<_> = entries.filter_map(|entry| entry.ok()).collect();

        // Sort the backups by modified time
        backups.sort_by_key(|entry| {
            match entry.metadata() {
                Ok(metadata) => match metadata.modified() {
                    Ok(modified_time) => modified_time,
                    Err(_) => {
                        return;
                    }
                },
                Err(_) => {
                    return;
                }
            };
        });

        // Delete old backups if there are more than max_backups
        if backups.len() > max_backups {
            for entry in &backups[..(backups.len() - max_backups)] {
                if let Err(_) = fs::remove_file(entry.path()) {
                    return;
                }
            }
        }
    }
}
