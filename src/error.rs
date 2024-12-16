use hound;
use std::fmt;
use std::io::Error as ioError;
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    HoundError,
    IoPathDoesNotExist,
    IoError,
    IoWrongDatatyp,
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl From<ioError> for Error {
    fn from(err: ioError) -> Self {
        match err {
            _ => Self::IoError,
        }
    }
}

impl From<hound::Error> for Error {
    fn from(err: hound::Error) -> Self {
        match err {
            _ => Self::HoundError,
        }
    }
}
