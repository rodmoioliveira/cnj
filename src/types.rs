pub type Result<T> = std::result::Result<T, MyError>;

use std::fmt::Display;

#[derive(Debug)]
pub enum MyError {
    IoError(std::io::Error),
    SerdeError(serde_json::Error),
    ClapError(clap::error::Error),
    CsvError(csv::Error),
}

// https://www.lpalmieri.com/posts/error-handling-rust/
// https://fettblog.eu/rust-enums-wrapping-errors/
// https://www.sheshbabu.com/posts/rust-error-handling/
impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyError::IoError(io_error) => write!(f, "{io_error}"),
            MyError::SerdeError(serde_error) => write!(f, "{serde_error}"),
            MyError::ClapError(clap_error) => write!(f, "{clap_error}"),
            MyError::CsvError(csv_error) => write!(f, "{csv_error}"),
        }
    }
}

impl std::error::Error for MyError {}

impl From<std::io::Error> for MyError {
    fn from(err: std::io::Error) -> Self {
        MyError::IoError(err)
    }
}

impl From<serde_json::Error> for MyError {
    fn from(err: serde_json::Error) -> Self {
        MyError::SerdeError(err)
    }
}

impl From<clap::Error> for MyError {
    fn from(err: clap::Error) -> Self {
        MyError::ClapError(err)
    }
}

impl From<csv::Error> for MyError {
    fn from(err: csv::Error) -> Self {
        MyError::CsvError(err)
    }
}
