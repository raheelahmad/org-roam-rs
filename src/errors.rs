#[derive(Debug)]
pub enum ExportError {
    Random(String),
}

impl std::fmt::Display for ExportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExportError::Random(reason) => write!(f, "{}", (reason)),
        }
    }
}

impl From<fs_extra::error::Error> for ExportError {
    fn from(error: fs_extra::error::Error) -> Self {
        ExportError::Random(error.to_string())
    }
}
impl From<std::io::Error> for ExportError {
    fn from(error: std::io::Error) -> Self {
        ExportError::Random(error.to_string())
    }
}
