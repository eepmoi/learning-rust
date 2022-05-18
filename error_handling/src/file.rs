use std::fs::File;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FileError {
    #[error(transparent)]
    StdIoError(#[from] std::io::Error),
}

pub fn open_file() -> Result<File, FileError> {
    let f = File::open("hello.txt")?;
    return Ok(f);
    // panic!();
}
