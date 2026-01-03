
#![allow(dead_code)]
#![allow(unused)]

use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};

use iced::{Element};
use iced::widget::{row, Column, text};
use thiserror::Error;

pub struct FileTree {
    root: Option<FsDir>,
    focus_id: Option<String>
}

#[derive(Debug, Clone)]
pub enum FileTreeMessage {
    OpenDir(PathBuf),
    SetFocus(String),
    ToggleExpandDir(String),
    OpenFile(String),
}

// TODO handle invalid strings gracefully
fn pathbuf_to_string(buf: &PathBuf) -> String {
    buf.clone().into_os_string().into_string().unwrap_or_else(|_|
        panic!("Invalid path value: {:?}", buf)
    )
}


impl FileTree {
    fn new() -> Self {
        FileTree {
            root: None,
            focus_id: None,
        }
    }

    pub fn update(&mut self, message: FileTreeMessage) {
        unimplemented!()
    }

    // Impliment the view as a collection of nested Columns
    pub fn view(&self) -> Element<'_, FileTreeMessage> {
        // If the file dir isn't open then we just render a placeholder message for now
        // TODO, put a button here to load a filesystem
        if self.root.is_none() {
            return row!(text("No Directory loaded")).into();
        }

        else {
            let root = self.root.as_ref().unwrap();
            let filetree = self.render_level(root);
            filetree.into()
        }
    }

    // TODO consider using keyed columns and the from_vecs method
    fn render_level(&self, fs_dir: &FsDir) -> Column<'_, FileTreeMessage> {
        let mut col = Column::<'_, FileTreeMessage>::new();
        // Loop over the directories 
        for dir in fs_dir.dirs.iter() {
            col = col.push(text(pathbuf_to_string(&dir.path)));
            if fs_dir.expanded {
                col = col.push(self.render_level(dir));
            }
        }
        // Loop over the files
        for file in fs_dir.files.iter() {
            col = col.push(text(pathbuf_to_string(file)));
        }
        col
    }

}

/////////// Logic ///////////////////

#[derive(Debug, PartialEq, Clone)]
pub struct FsDir {
    pub id: String,
    pub path: PathBuf,
    pub files: Vec<PathBuf>,
    pub dirs: Vec<FsDir>,
    pub expanded: bool,
}

impl FsDir {
    fn init(path: &PathBuf, parent_id: &str, id_count: usize) -> Self {
        let id = format!("{}_{}", parent_id, id_count);
        let dir_name = path.file_stem().unwrap();
        FsDir {
            id, 
            path: PathBuf::from(dir_name),
            files: vec!(),
            dirs: vec!(),
            expanded: false,
        }
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum FileSystemError {
    #[error("Could not read directory at: '{path:?}'")]
    CouldNotReadDir { path: OsString },
    #[error("Could not read file")]
    CouldNotReadFile,
    #[error("Not Memristor Directory")]
    NotMemristerDirectory,
}

fn validate_memristor_dir_structure(root_dir: &Path) -> Result<(), FileSystemError> {
    let contents = fs::read_dir(root_dir).map_err(|_| FileSystemError::CouldNotReadDir {
        path: root_dir.into(),
    })?;

    let mut has_typst_dir = false;
    let mut has_pdf_dir = false;

    for entry in contents {
        let entry = entry.map_err(|_| FileSystemError::CouldNotReadFile)?;
        let is_dir = entry.file_type().unwrap().is_dir();

        if is_dir && entry.file_name() == "pdf" {
            has_pdf_dir = true;
        } else if is_dir && entry.file_name() == "typst" {
            has_typst_dir = true;
        }
    }

    if !(has_pdf_dir && has_typst_dir) {
        Err(FileSystemError::NotMemristerDirectory)
    } else {
        Ok(())
    }
}

// TODO Consider just using unique integers for the ids 
// for now keep this as-is since it might be useful to 
// retain path information in the ids 
// TODO consider how I can make the id of the top level dir tidier
fn read_directory(dir: &PathBuf, parent_id: &str, id_count: usize) -> Result<FsDir, FileSystemError> {
    let mut fs_dir = FsDir::init(dir, parent_id, id_count);

    let contents =
        fs::read_dir(dir).map_err(|_| FileSystemError::CouldNotReadDir { path: dir.into() })?;

    let mut child_dir_count = 0;
    for entry in contents {
        let entry = entry.map_err(|_| FileSystemError::CouldNotReadFile)?;
        let is_dir = entry.file_type().unwrap().is_dir();

        if is_dir {
            fs_dir.dirs.push(read_directory(&entry.path(), &fs_dir.id, child_dir_count)?);
            child_dir_count += 1;
        } else {
            fs_dir.files.push(entry.path());
        }
    }
    Ok(fs_dir)
}

pub fn read_filesystem(root_dir: &Path) -> Result<FsDir, FileSystemError> {
    validate_memristor_dir_structure(root_dir)?;
    let typst_path = root_dir.join("typst");
    read_directory(&typst_path, "", 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_fs(subpath: &str) -> PathBuf {
        PathBuf::from("./test/test_fs").join(subpath)
    }

    #[test]
    fn empty_dir() {
        let test_fs = make_test_fs("empty");
        let result = read_filesystem(&test_fs);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), FileSystemError::NotMemristerDirectory);
    }

    #[test]
    fn populated_dir() {
        // TODO figure out how to make this not sensitive to ordering
        let expected = FsDir {
            id: "_0".into(),
            path: PathBuf::from("typst"),
            files: vec![PathBuf::from(
                "./test/test_fs/populated/typst/top_level.typ",
            )],
            expanded: false,
            dirs: vec![
                FsDir {
                    id: "_0_0".into(),
                    path: PathBuf::from("dir2"),
                    files: vec![PathBuf::from(
                        "./test/test_fs/populated/typst/dir2/.gitkeep",
                    )],
                    dirs: vec![],
                    expanded: false,
                },
                FsDir {
                    id: "_0_1".into(),
                    path: PathBuf::from("dir1"),
                    files: vec![PathBuf::from(
                        "./test/test_fs/populated/typst/dir1/in_dir1.typ",
                    )],
                    dirs: vec![],
                    expanded: false,
                },
            ],
        };

        let test_fs = make_test_fs("populated");
        let result = read_filesystem(&test_fs);
        assert!(result.is_ok());
        assert_eq!(expected, result.unwrap());
    }
}
