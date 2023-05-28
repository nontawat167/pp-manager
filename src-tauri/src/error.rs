pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    SqlError(rusqlite::Error),
    IO(std::io::Error),
}

impl From<std::io::Error> for Error {
    fn from(val: std::io::Error) -> Self {
        Error::IO(val)
    }
}

impl From<rusqlite::Error> for Error {
    fn from(val: rusqlite::Error) -> Self {
        Error::SqlError(val)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> core::result::Result<(), std::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
