#![allow(dead_code, unused)]

// This module handles interacting with the typst compiler - 
// for now we're just doing that through the CLI as a subprocess 
// since typst's Rust interface isn;t stable.

use std::io::{self, Write};
use std::path::PathBuf;
use std::fs::{self, OpenOptions};
use std::process::{Command, Stdio};
use std::time::{SystemTime, Instant, Duration};

use tempdir::TempDir;

use crate::error::{TypstError, FileSystemError};

const SECONDS_BETWEEN_RENDER: u64 = 5;

pub struct TypstContext {
    preview_path: PathBuf,
    temp_dir: TempDir,
    next_render: Instant,
}

pub enum RenderResult {
    Debounce,
    Success(Vec<PathBuf>),
    Error(TypstError),
}

impl TypstContext { 
    // TODO This should check that typst is installed 
    pub fn new() -> Result<Self, TypstError> {
        let temp_dir = TempDir::new("memristor").map_err(|_| {
            TypstError::TempDirError { message: "Couldn't create temporary directory".into() }
        })?;
        Ok(TypstContext {
            preview_path: temp_dir.path().join("preview{0p}.svg"),
            temp_dir,
            next_render: Instant::now(),
        })
    }

    // This is going to block the UI - we need to have it happen on another thread
    pub fn render(&mut self, content: &str, open_file: &PathBuf) -> RenderResult {
        // Debounce render messages
        let now = Instant::now();
        if now < self.next_render {
            return RenderResult::Debounce;
        }

        let compile_result = self.compile(content, open_file);

        // Update the debounce time
        let next_render = now.checked_add(Duration::from_secs(SECONDS_BETWEEN_RENDER)).unwrap();
        self.next_render = next_render;

        if compile_result.is_err() {
            return RenderResult::Error(compile_result.unwrap_err());
        }

        // Get the preview files
        match self.get_preview_files() {
            Err(err) => RenderResult::Error(TypstError::FilesystemError(err.kind())),
            Ok(files) => RenderResult::Success(files),
        }
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
        // This might not be the best way to do this given the system 
        // clock isn't montonic 
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
