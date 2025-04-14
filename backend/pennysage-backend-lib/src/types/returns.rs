use derive_more::From;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    // IO errors
    /// IO error when interacting with the filesystem
    #[from]
    IoError(std::io::Error),

    // Misc errors
    /// Error when attempting to read a env variable
    #[from]
    EnvError(std::env::VarError),

    /// Error when parsing integer from string
    #[from]
    IntParseError(std::num::ParseIntError),

    /// Error when encoding JWT tokens
    #[from]
    JWTEncodeError(jsonwebtoken::errors::Error),

    // SQLX errors
    /// SQLX error when interacting with the database
    #[from]
    DatabaseError(sqlx::Error),
    /// SQLX error when attempting to run database migrations
    #[from]
    DatabaseMigrateError(sqlx::migrate::MigrateError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
