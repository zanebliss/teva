use std::fmt::Display;
use std::io;

#[derive(Debug)]
pub enum RevwError {
    NotEnoughCommits,
    MissingConfigKey,
    InvalidPath,
    UnhandledIO(io::Error),
    Git(git2::Error),
    Deserialization(toml::de::Error),
    CtrlCError(ctrlc::Error),
}

impl Display for RevwError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RevwError::NotEnoughCommits => write!(f, "Not enough commits"),
            RevwError::Git(e) => write!(f, "Git error: {}", e),
            RevwError::UnhandledIO(e) => write!(f, "Unhandled IO error: {}", e),
            RevwError::Deserialization(e) => write!(f, "Config error: {}", e),
            RevwError::MissingConfigKey => write!(f, "Missing config key"),
            RevwError::InvalidPath => write!(f, "Config value referenced invalid path"),
            RevwError::CtrlCError(e) => write!(f, "Ctrlc error: {}", e),
        }
    }
}

impl From<git2::Error> for RevwError {
    fn from(e: git2::Error) -> Self {
        RevwError::Git(e)
    }
}

impl From<io::Error> for RevwError {
    fn from(e: io::Error) -> Self {
        RevwError::UnhandledIO(e)
    }
}

impl From<toml::de::Error> for RevwError {
    fn from(e: toml::de::Error) -> Self {
        RevwError::Deserialization(e)
    }
}

impl From<ctrlc::Error> for RevwError {
    fn from(e: ctrlc::Error) -> Self {
        RevwError::CtrlCError(e)
    }
}

pub type RevwResult<T> = Result<T, RevwError>;
