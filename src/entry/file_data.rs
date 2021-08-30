use std::io::{BufReader, Error, prelude::*};
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
    file_type: FileType,
    permissions: FilePermissions,
    mod_time: SystemTime,
    file_size: u64,
}

impl FileData {
    pub fn new(entry: DirEntry) -> std::io::Result<FileData> {
        let metadata = entry.metadata()?;
        let file_type = FileType::new(metadata.st_mode());
        let permissions = FilePermissions::new(metadata.st_mode());
        
        Ok(FileData {
            name: entry.file_name().into_string().unwrap(),
            file_type,
            permissions,
            mod_time: metadata.modified()?,
            file_size: metadata.len(),
        })
    }

    pub fn preview(&self) -> Result<String, Error> {
        let file = File::open(&self.name)?;
        let lines = BufReader::new(&file)
            .lines()
            .take(10);
            
        let mut head: Vec<String> = Vec::with_capacity(10);
        
        for line in lines {
            if let Ok(line) = line {
                head.push(line);
            } else {
                return Ok("".to_string());
            }
        }
            
        Ok(head.join("\n"))
    }

    pub fn get_mime_type(&self) -> Result<String, FileMagicError> {
        let file = &self.name;
        let magic = magic!().expect("error");
  
        magic.file(file)
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
