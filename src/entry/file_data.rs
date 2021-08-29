use std::io::{Error, prelude::*, BufReader};
use std::fs::{DirEntry, File};
use std::os::linux::fs::MetadataExt;
use std::time::SystemTime;
use std::fmt::{self, Display, Formatter};
use chrono::{DateTime, Local};
use filemagic::{FileMagicError, magic};
use humansize::{FileSize, file_size_opts};

use super::permissions::FilePermissions;
use super::type_parser::FileType;

#[derive(Debug)]
pub struct FileData {
    pub name: String,
    pub file_type: FileType,
    pub permissions: FilePermissions,
    mod_time: SystemTime,
    file_size: u64,
}

impl FileData {
    pub fn new(entry: DirEntry) -> std::io::Result<FileData> {
        let metadata = entry.metadata()?;
        let file_type = FileType::new(metadata.st_mode());
        let permissions = FilePermissions::new(metadata.st_mode());
        let mod_time = metadata.modified()?;
        let file_size = metadata.len();

        Ok(FileData {
            name: entry.file_name().into_string().unwrap(),
            file_type,
            permissions,
            mod_time,
            file_size,
        })
    }

    pub fn preview(&self) -> Result<String, Error> {
        let file = File::open(&self.name)?;

        if self.is_dir() {
            return Ok(" ".to_string());
        }
        let head: String = BufReader::new(file)
            .lines()
            .take(10)
            .map(|l| l.unwrap() + "\n")
            .collect();

        Ok(head)
    }

    pub fn get_mime_type(&self) -> Result<String, FileMagicError> {
        let magic = magic!().expect("error");
  
        magic.file(&self.name)
    }

    pub fn info(&self) -> String {
        let mut mime_type = String::new();

        if let Ok(mtype) = self.get_mime_type() {
            mime_type = mtype;
        }
        let mod_time: DateTime<Local> = self.mod_time.into();

        format!("{}{}\n{}\n{}\n{}", self.file_type, 
            self.permissions,
            self.file_size.file_size(file_size_opts::DECIMAL).unwrap(),
            mod_time.format("%b %e %T"),
            mime_type)
    }

    pub fn is_dir(&self) -> bool {
        self.file_type == FileType::DIR
    }

    pub fn is_file(&self) -> bool {
        self.file_type == FileType::REG
    }
}

impl Display for FileData {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}\n{}", self.name,self.info())    
    }
}
