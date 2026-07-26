use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("ALPM error: {0}")]
    Alpm(String),

    #[error("Cache error: {0}")]
    Cache(String),

    #[error("Config error: {0}")]
    Config(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("TOML parse error: {0}")]
    Toml(String),

    #[error("Package not found: {0}")]
    PackageNotFound(String),

    #[error("Transaction failed: {0}")]
    TransactionFailed(String),

    #[error("Terminal error: {0}")]
    Terminal(String),

    #[error("BlackArch repository not configured")]
    BlackArchMissing,

    #[error("Privilege escalation required for: {0}")]
    PrivilegeRequired(String),

    #[error("Cache corrupted, rebuilding")]
    CacheCorrupted,

    #[error("Feature unavailable: {0}")]
    Unavailable(String),
}

pub type Result<T> = std::result::Result<T, AppError>;
