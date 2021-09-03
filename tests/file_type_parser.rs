use rufile::entry::type_parser::FileType;

#[test]
fn test_file_type() {
    assert_eq!(FileType::SOCK, FileType::new(49152));
    assert_eq!(FileType::LNK, FileType::new(40960));
    assert_eq!(FileType::REG, FileType::new(32768));
    assert_eq!(FileType::BLK, FileType::new(24576));
    assert_eq!(FileType::DIR, FileType::new(16384));
    assert_eq!(FileType::CHR, FileType::new(8192));
    assert_eq!(FileType::FIFO, FileType::new(4096));
    assert_eq!(FileType::UNDEFINED, FileType::new(1));
}

#[test]
fn test_file_type_str() {
    assert_ne!('c', FileType::new(49152).into());

    assert_eq!('s', FileType::new(49152).into());
    assert_eq!('l', FileType::new(40960).into());
    assert_eq!('-', FileType::new(32768).into());
    assert_eq!('b', FileType::new(24576).into());
    assert_eq!('d', FileType::new(16384).into());
    assert_eq!('c', FileType::new(8192).into());
    assert_eq!('p', FileType::new(4096).into());

    assert_eq!('\0', FileType::new(5).into());
}