use std::fmt;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ShortError {
    InvalidString,
    InvalidNumber,
}

impl fmt::Display for ShortError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ShortError::InvalidString => write!(f, "Not a valid string to decode"),
            ShortError::InvalidNumber => write!(f, "Number is unable to be converted to a usize"),
        }
    }
}
