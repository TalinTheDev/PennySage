/// Application state for the backend server
///
/// Currently, it only contains the database connection pool
#[derive(Clone)]
pub struct AppState {
    pub db_pool: sqlx::Pool<sqlx::Sqlite>,
}
