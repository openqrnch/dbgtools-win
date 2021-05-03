//! Error types and error conversions.

use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
  MiniDump(String),
  IO(String),
  BadFormat(String)
}

impl std::error::Error for Error {}

impl From<io::Error> for Error {
  fn from(err: io::Error) -> Self {
    Error::IO(err.to_string())
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match &*self {
      Error::MiniDump(s) => {
        write!(f, "Unable to create minidump; {}", s)
      }
      Error::IO(s) => {
        write!(f, "I/O error; {}", s)
      }
      Error::BadFormat(s) => {
        write!(f, "Bad format error; {}", s)
      }
    }
  }
}

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
