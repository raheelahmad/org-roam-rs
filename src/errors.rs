#[derive(Debug)]
pub enum Error {
    FileAccess(String),
    DBAccess(String),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::FileAccess(reason) => write!(f, "{}", (reason)),
            Error::DBAccess(reason) => write!(f, "{}", (reason)),
        }
    }
}

impl From<fs_extra::error::Error> for Error {
    fn from(error: fs_extra::error::Error) -> Self {
        Error::FileAccess(error.to_string())
    }
}
impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::FileAccess(error.to_string())
    }
}
impl From<rusqlite::Error> for Error {
    fn from(error: rusqlite::Error) -> Self {
        Error::DBAccess(error.to_string())
    }
}
