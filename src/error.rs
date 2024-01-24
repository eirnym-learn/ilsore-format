pub(crate) static mut VERBOSE_ERRORS: bool = true;

#[derive(Debug)]
pub(crate) enum Error {
    Io(std::io::Error),
    Git(git2::Error),
    Message(String),
}

pub(crate) type Result<T, E = Error> = std::result::Result<T, E>;

pub(crate) fn error_control<T, E: std::fmt::Debug>(result: Result<T, E>) -> Result<Option<T>> {
    if result.is_ok() {
        return Ok(result.ok());
    }

    let err: E = result.err().unwrap();
    unsafe {
        if VERBOSE_ERRORS == true {
            println!("{:?}", err);
        }
    }

    return Ok(None);
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<git2::Error> for Error {
    fn from(err: git2::Error) -> Self {
        Self::Git(err)
    }
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Self::Message(s)
    }
}

impl From<&'_ str> for Error {
    fn from(s: &str) -> Self {
        Self::Message(s.to_string())
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(err) => Some(err),
            Error::Git(err) => Some(err),
            Error::Message(_) => None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(err) => err.fmt(f),
            Error::Git(err) => err.fmt(f),
            Error::Message(err) => err.fmt(f),
        }
    }
}
