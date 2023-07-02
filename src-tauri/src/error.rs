pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
}

impl From<std::io::Error> for Error {
    fn from(val: std::io::Error) -> Self {
        Error::IO(val)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> core::result::Result<(), std::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
