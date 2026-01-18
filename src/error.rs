#![allow(dead_code, unused)]

use std::ffi::OsString;
use std::io;

use thiserror::Error;


#[derive(Error, Debug, PartialEq)]
pub enum FileSystemError {
    #[error("Could not read directory at: '{path:?}'")]
    ReadDirError { path: OsString },

    #[error("Could not read file")]
    ReadFileError { path: OsString },

    #[error("Could not create file")]
    CreateFileError { path: OsString },

    #[error("Could not create dir")]
    CreateDirError { path: OsString },

    #[error("Not Memristor Directory")]
    NotMemristerDirectory,

}

#[derive(Error, Debug, PartialEq, Clone)]
pub enum TypstError {
    #[error("Could not find typst on path")]
    TypstNotInstalled,

    // This will need to handle structured data - so we can put more 
    // information in the editor
    #[error("Compilation failed")]
    CompilationError { message: String },

    #[error("Temporary Directory operation failed")]
    TempDirError { message: String },

    #[error("Filesystem Error")]
    FilesystemError(io::ErrorKind),
}
