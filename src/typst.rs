#![allow(dead_code, unused)]

// This module handles interacting with the typst compiler - 
// for now we're just doing that through the CLI as a subprocess 
// since typst's Rust interface isn;t stable.

use std::io::{self, Write};
use std::path::PathBuf;
use std::fs::{self, OpenOptions};
use std::process::{Command, Stdio};
use std::time::SystemTime;

use tempdir::TempDir;

use crate::error::{TypstError, FileSystemError};

pub struct TypstContext {
    preview_path: PathBuf,
    temp_dir: TempDir,
}

impl TypstContext { 
    // This should check that typst is installed 
    pub fn new() -> Result<Self, TypstError> {
        let temp_dir = TempDir::new("memristor").map_err(|_| {
            TypstError::TempDirError { message: "Couldn't create temporary directory".into() }
        })?;
        Ok(TypstContext {
            preview_path: temp_dir.path().join("preview{0p}.svg"),
            temp_dir,
        })
    }

    fn compile(&self, content: &str, open_file: &PathBuf) -> Result<(), TypstError> {
        let content_directory = open_file.as_path().parent().unwrap();

        // Start the typst process
        let mut typst = Command::new("typst")
            .arg("compile")
            .arg("--root").arg(content_directory)
            .arg(&self.preview_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::inherit())
            .spawn()
            .unwrap();

        // Write to it's stdin
        let mut stdin = typst.stdin.take().unwrap();
        stdin.write(content.as_bytes());
        drop(stdin);

        // Wait for it to finish
        let status = typst.wait().unwrap();
        Ok(())
    }

    pub fn get_preview_files(&self) -> io::Result<Vec<PathBuf>> {
        let mut svgs = vec!();
        let dir_contents = fs::read_dir(self.temp_dir.path())?;

        // Get a list of all the svgs in the directory
        for entry in dir_contents {
            let entry = entry?;
            if entry.file_type()?.is_file() {
                let filename: PathBuf = entry.file_name().into();
                if filename.extension().is_some_and(|ext| ext == "svg") {
                svgs.push(entry.path());
                }
            }
        }

        // If we didn't find any then we're done
        if svgs.len() == 0 {
            return Ok(svgs);
        }

        // Make sure they're in numeric order 
        svgs.sort();

        // Only return files with last edited dates which are equal to or earlier 
        // than the first file 
        let start_date = fs::metadata(&svgs[0])?.modified()?;

        let new_files = svgs.into_iter().take_while(|file| {
            match fs::metadata(&file) {
                Err(_) => false,
                Ok(metadata) => {
                    match metadata.modified() {
                        Err(_) => false,
                        Ok(modified_time) => modified_time >= start_date
                    }
                }
            }
        }).collect();


        Ok(new_files)
    }
}
