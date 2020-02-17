#![deny(rust_2018_idioms)]

use std::{error, fmt};

pub mod encoders;
pub mod line_parser;
pub mod storage;
pub mod time;

pub mod delorean {
    include!(concat!(env!("OUT_DIR"), "/delorean.rs"));
}

// TODO: audit all errors and their handling in main

#[derive(Debug, Clone, PartialEq)]
pub struct Error {
    pub description: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}
