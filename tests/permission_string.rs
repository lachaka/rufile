use rufile::entry::permissions::FilePermissions;

#[test]
fn test_permission_string() {
    assert_eq!("---------", FilePermissions::new(0o000).to_string());
    
    assert_eq!("--------x", FilePermissions::new(0o001).to_string());
    assert_eq!("-----x---", FilePermissions::new(0o010).to_string());
    assert_eq!("-----x--x", FilePermissions::new(0o011).to_string());
    assert_eq!("--x------", FilePermissions::new(0o100).to_string());
    assert_eq!("--x-----x", FilePermissions::new(0o101).to_string());
    assert_eq!("--x--x---", FilePermissions::new(0o110).to_string());
    assert_eq!("--x--x--x", FilePermissions::new(0o111).to_string());

    assert_eq!("-------w-", FilePermissions::new(0o002).to_string());
    assert_eq!("----w----", FilePermissions::new(0o020).to_string());
    assert_eq!("----w--w-", FilePermissions::new(0o022).to_string());
    assert_eq!("-w-------", FilePermissions::new(0o200).to_string());
    assert_eq!("-w-----w-", FilePermissions::new(0o202).to_string());
    assert_eq!("-w--w----", FilePermissions::new(0o220).to_string());
    assert_eq!("-w--w--w-", FilePermissions::new(0o222).to_string());

    assert_eq!("------r--", FilePermissions::new(0o004).to_string());
    assert_eq!("---r-----", FilePermissions::new(0o040).to_string());
    assert_eq!("---r--r--", FilePermissions::new(0o044).to_string());
    assert_eq!("r--------", FilePermissions::new(0o400).to_string());
    assert_eq!("r-----r--", FilePermissions::new(0o404).to_string());
    assert_eq!("r--r-----", FilePermissions::new(0o440).to_string());
    assert_eq!("r--r--r--", FilePermissions::new(0o444).to_string());

    assert_eq!("rw-rw-r--", FilePermissions::new(0o664).to_string());
    assert_eq!("rw-rw-r-x", FilePermissions::new(0o665).to_string());
    assert_eq!("rw-rw-rw-", FilePermissions::new(0o666).to_string());

    assert_eq!("rwxrwxrwx", FilePermissions::new(0o777).to_string());
}