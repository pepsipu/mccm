use bollard::errors::Error as BollardError;

#[derive(Debug)]
pub enum ServerStateError {
    Docker(BollardError),
    Io(std::io::Error),
}

impl std::fmt::Display for ServerStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Docker(err) => write!(f, "docker error: {err}"),
            Self::Io(err) => write!(f, "io error: {err}"),
        }
    }
}

impl std::error::Error for ServerStateError {}

impl From<BollardError> for ServerStateError {
    fn from(value: BollardError) -> Self {
        Self::Docker(value)
    }
}

impl From<std::io::Error> for ServerStateError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}
