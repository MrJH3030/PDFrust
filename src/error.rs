use derive_more::{Display, From};
use inquire::InquireError;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    //#[display(...)]
    Custom(String),

    InvalidRangeError {
        start: u32,
        end: u32,
    },

    FailedToParseRange {
        range_string: String,
    },

    #[from]
    Io(std::io::Error),

    #[from]
    ParseInt(std::num::ParseIntError),

    #[from]
    InquireError(InquireError),

    #[from]
    LoPdf(lopdf::Error)


}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
