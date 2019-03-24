use std::fmt::{self, Display, Formatter};
use std::error;

#[derive(Debug)]
pub enum Error  {
    CardLength(usize),
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::CardLength(_) => "invalid number of cards in starting hand",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::CardLength(_) => None,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match *self {
            Error::CardLength(n) => write!(f, "invalid number of cards ({}) in starting hand", n),
        }
    }
}
