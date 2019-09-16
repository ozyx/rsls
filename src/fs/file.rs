use std::fs::{self};
use std::path::PathBuf;

pub struct File {
    /// The filename
    pub name: String,
    /// The file extension
    pub ext: Option<String>,
    /// The file path
    pub path: PathBuf,
    /// The file's metadata
    pub metadata: fs::Metadata,
}

impl File {
    /// Whether this file is a directory on the filesystem.
    pub fn is_directory(&self) -> bool {
        self.metadata.is_dir()
    }

    /// Whether this file is a regular file on the filesystem
    pub fn is_file(&self) -> bool {
        self.metadata.is_file()
    }

    /// Whether this file is a hidden file or directory
    pub fn is_hidden(&self) -> bool {
        return self.name.starts_with(".");
    }
}
