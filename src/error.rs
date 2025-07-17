use derive_more::From;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    InvalidRangeError { start: u32, end: u32},

    FailedToParseRange {range_string : String},

    #[from]
    Io(std::io::Error),

    #[from]
    ParseInt(std::num::ParseIntError),

}


impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}