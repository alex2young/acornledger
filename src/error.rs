use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
struct AcornError {}

impl Display for AcornError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Error for AcornError {}
