// This module handles interacting with the typst compiler - 
// for now we're just doing that through the CLI as a subprocess 
// since typst's Rust interface isn;t stable.

use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;
use std::fs::OpenOptions;

use tempdir::TempDir;

pub struct TypstContext {
    temp_dir: TempDir,
}

impl TypstContext { 
    // This should check that typst is installed 
    pub fn new() -> Self {
        TypstContext {
            temp_dir: TempDir::new("memristor").expect("Couldn't create temp dir"),
        }
    }

    // Can write file with stdin like so:
    // echo "Hello World" | typst c - output.pdf
    // Next question - how do imports work in this context

    // TODO delete any old preview files
    // TODO figure out how typst handles input from stdin
    pub fn compile(&self, content: &str) -> io::Error<Vec<PathBuf>> {
        // Write the content to a file 
        let temp_path = self.temp_dir.path().join("content.typ");
        let mut file = OpenOptions::new().write(true).create(true).open(&temp_path).expect("Could not create content file");
        file.write(content.as_bytes()).expect("Could not write to file");

        // Compile the file
        Command::new("typst")
            .arg("-c").arg(&temp_path)
            .args(["format", "svg"])
            .output()
            .unwrap();


        // Return the list of Preview files
        todo! ()
    }
}
