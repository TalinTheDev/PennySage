use derive_more::From;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    // IO errors
    #[from]
    IoError(std::io::Error),

    // SQLX errors
    #[from]
    DatabaseError(sqlx::Error),
    #[from]
    DatabaseMigrateError(sqlx::migrate::MigrateError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
