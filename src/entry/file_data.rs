use std::fs::{DirEntry, File};
use std::io::{Error, prelude::*, BufReader};
use std::os::linux::fs::MetadataExt;

use super::permissions::FilePermissions;
use super::type_parser::FileType;

pub enum EntryError {
    ErrorGettingMetadata,
}

#[derive(Debug)]
pub struct FileData {
    pub name: String,
    pub file_type: FileType,
    pub permissions: FilePermissions,
}

impl FileData {
    pub fn new(entry: DirEntry) -> Result<FileData, EntryError> {
        if let Ok(metadata) = entry.metadata() {
            let file_type = FileType::new(metadata.st_mode());
            let permissions = FilePermissions::new(metadata.st_mode());
            
            return Ok(FileData {
                name: entry.file_name().into_string().unwrap(),
                file_type,
                permissions
            });
        }

        Err(EntryError::ErrorGettingMetadata)
    }

    pub fn preview(&self) -> Result<String, Error> {
        let file = File::open(&self.name)?;

        let head: String = BufReader::new(file)
            .lines()
            .take(10)
            .map(|l| l.unwrap() + "\n")
            .collect();
        
        Ok(head)
    }

    pub fn info(&self) -> String {
        format!(" {}{}", self.file_type, self.permissions)
    } 

    pub fn is_dir(&self) -> bool {
        self.file_type == FileType::DIR
    }

    pub fn is_file(&self) -> bool {
        self.file_type == FileType::REG
    }
}

