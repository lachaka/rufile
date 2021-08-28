use std::fmt::{self, Display, Formatter};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FileType {
    SOCK,  // socket
    LNK,   // symbolic link
    REG,   // regular file
    BLK,   // block device
    DIR,   // directory
    CHR,   // character device
    FIFO,  // FIFO
    UNDEFINED,
}

impl FileType {
    pub fn new(file: u32) -> FileType {
        let mode_t: u32 = 0o0170000;
        
        FileType::from(file & mode_t)
    }
}

impl From<u32> for FileType {
    fn from(file_type: u32) -> FileType {
        match file_type {
            0o0140000 => FileType::SOCK,
            0o0120000 => FileType::LNK ,
            0o0100000 => FileType::REG ,
            0o0060000 => FileType::BLK ,
            0o0040000 => FileType::DIR ,
            0o0020000 => FileType::CHR ,
            0o0010000 => FileType::FIFO,
            _ => FileType::UNDEFINED,
        }
    }
}

impl From<FileType> for char {
    fn from(file_type: FileType) -> char {
        match file_type {
            FileType::SOCK =>'s',
            FileType::LNK => 'l',
            FileType::REG => '-',
            FileType::BLK => 'b',
            FileType::DIR => 'd',
            FileType::CHR => 'c',
            FileType::FIFO =>'p',
            FileType::UNDEFINED => '\0',
        }
    }
}

impl Display for FileType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}