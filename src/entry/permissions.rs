use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct FilePermissions {
    perms: [char; 9] 
}

impl FilePermissions {
    pub fn new(file: u32) -> FilePermissions {
        let mut perms: [char; 9] = ['-'; 9];

        let mut x: usize = 0;
        for _i in 0..3 {
            perms[x] = if (file & (0x1 << (8 - x))) >= 1 {'r'} else {'-'};
            perms[x + 1] = if (file & (0x1 << (7 - x))) >= 1 {'w'} else {'-'};
            perms[x + 2] = if (file & (0x1 << (6 - x))) >= 1 {'x'} else {'-'};
            x += 3;
        }

        FilePermissions{ perms }
    }
}

impl Display for FilePermissions {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let perms: String = self.perms.iter().collect(); 
        
        write!(f, "{}", perms)
    }
}