#![allow(dead_code, unused)]

use std::fs;
use std::path::{Path, PathBuf};

use iced::{Element, Padding, Length, Color};
use iced::widget::{row, Column, text, mouse_area, container};

use crate::components;
use crate::error::FileSystemError;
use crate::styles;

pub struct FileTree {
    root: Option<FsDir>,
    focus_path: Option<String>
}

#[derive(Debug, Clone)]
pub enum Message {
    ToggleExpandDir(String),
    OpenFile(PathBuf),
    OpenDir(PathBuf),
}

// TODO handle invalid strings gracefully
fn pathbuf_to_string(buf: &PathBuf) -> String {
    buf.clone().into_os_string().into_string().unwrap_or_else(|_|
        panic!("Invalid path value: {:?}", buf)
    )
}

impl<'a> FileTree {
    pub fn new() -> Self {
        let test_dir = PathBuf::from(styles::TEST_DIR);
        let fs_dir = read_filesystem(&test_dir).unwrap();
        FileTree {
            root: None,
            focus_path: None,
        }
    }


    pub fn update(&mut self, message: Message) { 
        match message {
            Message::ToggleExpandDir(id) => {
                self.root.as_mut().map(|fs_dir| fs_dir.toggle_expanded(id));
            },
            Message::OpenFile(_) => { 
                unreachable!("Should be handled in layout");
            },
            Message::OpenDir(dir_path) => {
                let fs_dir = read_filesystem(&dir_path).unwrap();
                self.root = Some(fs_dir);
            }
        }
    }


    // Impliment the view as a collection of nested Columns
    pub fn view(&self) -> Element<'_, Message> {
        // If the file dir isn't open then we just render a placeholder message for now
        // TODO, put a button here to load a filesystem
        let content = if self.root.is_none() {
            container(
                text("No Directory loaded")
            )
        }
        else {
            let root = self.root.as_ref().unwrap();
            let filetree = self.render_level(root);
            container(filetree).align_top(Length::Fill)
        };

        row![
            content
                .padding(Padding::new(styles::SPACING_SMALL)),
            components::left_border(Color::BLACK),
        ]
        .into()
    }


    // TODO consider using keyed columns and the from_vecs method
    fn render_level(&self, fs_dir: &'a FsDir) -> Column<'a, Message> {
        let mut col = Column::<'_, Message>::new();
        // Loop over the directories 
        for dir in fs_dir.dirs.iter() {
            col = col.push(render_dir_row(dir));
            if dir.expanded {
                col = col.push(self.render_level(dir));
            }
        }
        // Loop over the files
        for (_file_count, file) in fs_dir.files.iter().enumerate() {
            col = col.push(render_file_row(file));
        }
        col
        .padding(Padding::ZERO.left(20))
    }
}


fn render_dir_row(fs_dir: &FsDir) -> Element<'_, Message> {
    let arrow = if fs_dir.expanded { '⇓' } else { '⇒' };
    mouse_area(
        row![
            text(arrow),
            text(pathbuf_to_string(&fs_dir.path)),
        ]
        .spacing(5)
        .width(Length::Fill)
        .clip(true)
    )
    .on_press(Message::ToggleExpandDir(fs_dir.id.clone()))
    .into()
}

fn render_file_row(file: &PathBuf) -> Element<'_, Message> {
    let filename = PathBuf::from(file.as_path().file_name().unwrap());
    mouse_area(
        text(pathbuf_to_string(&filename))
    )
    .on_press(Message::OpenFile(file.to_path_buf()))
    .into()
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

    fn toggle_expanded(&mut self, id: String) {
        let mut dir_stack: Vec<&mut FsDir> = vec!(self);
        while !dir_stack.is_empty() {
            let dir = dir_stack.pop().unwrap();

            if dir.id == id {
                dir.expanded = !dir.expanded;
                return;
            }
            else {
                for dir in dir.dirs.iter_mut() {
                    dir_stack.push(dir)
                }
            }
        }
        println!("Id '{}' not found when toggling expansion", id);
    }
}

fn validate_memristor_dir_structure(root_dir: &Path) -> Result<(), FileSystemError> {
    let contents = fs::read_dir(root_dir).map_err(|_| FileSystemError::ReadFileError {
        path: root_dir.into(),
    })?;

    let mut has_typst_dir = false;
    let mut has_pdf_dir = false;

    for entry in contents {
        let entry = entry.map_err(|_| FileSystemError::ReadDirError { path: root_dir.into() })?; 
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

// TODO we don;t need the id - we can just use the path 
// TODO consider how I can make the id of the top level dir tidier
// TODO it's probably faster to store the id in a Rc type
fn read_directory(dir: &PathBuf, parent_id: &str, id_count: usize) -> Result<FsDir, FileSystemError> {
    let mut fs_dir = FsDir::init(dir, parent_id, id_count);

    let contents =
        fs::read_dir(dir).map_err(|_| FileSystemError::ReadDirError { path: dir.into() })?;

    let mut child_dir_count = 0;
    for entry in contents {
        let entry = entry.map_err(|_| FileSystemError::ReadDirError { path: dir.into() })?; 
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
