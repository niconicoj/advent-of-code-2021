#[derive(Debug)]
pub enum Error {
    BadFormat,
    Io,
}

impl From<std::num::ParseIntError> for Error {
    fn from(_: std::num::ParseIntError) -> Self {
        return Error::BadFormat;
    }
}

impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Self {
        return Error::Io;
    }
}
