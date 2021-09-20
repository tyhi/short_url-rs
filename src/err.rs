use std::{error::Error, fmt};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ShortError {
    InvalidString,
}

impl Error for ShortError {}

impl fmt::Display for ShortError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ShortError::InvalidString => write!(f, "Not a valid string to decode"),
        }
    }
}
