#![allow(enum_variant_names)]

/// Definition of Tantivy's error and result.

use std::io;
use std::result;
use std::path::PathBuf;
use std::error;
use std::sync::PoisonError;
use directory::error::{FileError, OpenWriteError, OpenDirectoryError};
use query;
use schema;


/// Tantivy result.
pub type Result<T> = result::Result<T, Error>;


/// Generic tantivy error.
///
/// Any specialized error return in tantivy can be converted in `tantivy::Error`.
#[derive(Debug)]
pub enum Error {
    /// Path does not exist.
    PathDoesNotExist(PathBuf),
    /// File already exists, this is a problem when we try to write into a new file.
    FileAlreadyExists(PathBuf),
    /// IO Error
    IOError(io::Error),
    /// A thread holding the locked panicked and poisoned the lock.
    Poisoned,
    /// The data within is corrupted.
    ///
    /// For instance, it contains invalid JSON.
    CorruptedFile(PathBuf, Box<error::Error + Send + Sync>),
    /// Invalid argument was passed by the user.
    InvalidArgument(String),
    /// An Error happened in one of the thread
    ErrorInThread(String),
}

impl From<io::Error> for Error {
    fn from(io_error: io::Error) -> Error {
        Error::IOError(io_error)
    }
}

impl From<query::QueryParserError> for Error {
    fn from(parsing_error: query::QueryParserError) -> Error {
        Error::InvalidArgument(format!("Query is invalid. {:?}", parsing_error))
    }
}

impl<Guard> From<PoisonError<Guard>> for Error {
    fn from(_: PoisonError<Guard>) -> Error {
        Error::Poisoned
    }
}

impl From<FileError> for Error {
    fn from(error: FileError) -> Error {
        match error {
            FileError::FileDoesNotExist(filepath) => Error::PathDoesNotExist(filepath),
            FileError::IOError(io_error) => Error::IOError(io_error),
        }
    }
}

impl From<schema::DocParsingError> for Error {
    fn from(error: schema::DocParsingError) -> Error {
        Error::InvalidArgument(format!("Failed to parse document {:?}", error))
    }
}

impl From<OpenWriteError> for Error {
    fn from(error: OpenWriteError) -> Error {
        match error {
            OpenWriteError::FileAlreadyExists(filepath) => 
                Error::FileAlreadyExists(filepath),
            OpenWriteError::IOError(io_error) => 
                Error::IOError(io_error),
        }
    }
}

impl From<OpenDirectoryError> for Error {
    fn from(error: OpenDirectoryError) -> Error {
        match error {
            OpenDirectoryError::DoesNotExist(directory_path) =>
                Error::PathDoesNotExist(directory_path),
            OpenDirectoryError::NotADirectory(directory_path) => 
                Error::InvalidArgument(format!("{:?} is not a directory", directory_path)),
        }
    }
}
