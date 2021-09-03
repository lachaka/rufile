use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::{env, fs, io};
use std::os::unix::fs::PermissionsExt;

pub struct OperationExecutor {
    last_operation: char,
    last_op_file_path: PathBuf,
}

impl Default for OperationExecutor {
    fn default() -> OperationExecutor {
        OperationExecutor {
            last_operation: '\0',
            last_op_file_path: PathBuf::new(),
        }
    }
}

impl OperationExecutor {
    fn save_file_path(&mut self, file_name: &str) -> io::Result<()> {
        self.last_op_file_path.push(env::current_dir()?);
        self.last_op_file_path.push(file_name);

        Ok(())
    }

    fn is_file(&self) -> bool {
        self.last_op_file_path.is_file()
    }

    fn copy_dir(&self, _dir_name: &OsStr) -> io::Result<()> {
        todo!()
    }

    fn copy_file(&self, file_name: &OsStr) -> io::Result<()> {
        let output = Path::new(file_name);
        fs::copy(&self.last_op_file_path, output)?;

        Ok(())
    }

    pub fn copy(&mut self, args: Vec<&str>) -> io::Result<()> {
        let file_name = args[0];

        self.save_file_path(file_name)?;
        self.last_operation = 'c';

        Ok(())
    }

    pub fn cut(&mut self, args: Vec<&str>) -> io::Result<()> {
        let file_name = args[0];

        self.save_file_path(file_name)?;
        self.last_operation = 'm';

        Ok(())
    }

    pub fn paste(&mut self, _args: Vec<&str>) -> io::Result<()> {
        let file = self.last_op_file_path.file_name().unwrap();

        if self.last_operation == 'c' {
            if self.is_file() {
                self.copy_file(&file)?;
            } else { // copy directory
                self.copy_dir(&file)?;
            }
        } else if self.last_operation == 'm' { // cut file
            if self.is_file() {
                self.copy_file(&file)?;
                fs::remove_file(&self.last_op_file_path)?;
            }  else { // move directory
                self.copy_dir(&file)?;
                fs::remove_dir_all(&self.last_op_file_path)?;
            }
        }
        
        self.last_operation = 'p';

        Ok(())
    }

    pub fn delete(&mut self, args: Vec<&str>) -> io::Result<()> {
        let file_name = args[0];

        self.save_file_path(&file_name)?;

        if self.is_file() {
            fs::remove_file(&file_name)?;
        } else { // directory
            fs::remove_dir_all(&file_name)?;
        }
          
        self.last_operation = 'd';

        Ok(())
    }

    pub fn rename(&mut self, args: Vec<&str>) -> io::Result<()> {
        let file_name = args[0];
        let new_name = args[1];

        fs::rename(file_name, new_name)?;

        self.last_operation = 'r';

        Ok(())
    }

    pub fn create(&mut self, args: Vec<&str>) -> io::Result<()> {
        let file_name = args[1];

        if args[0] == "d" { // directory
            fs::create_dir(file_name)?;
        } else if args[0] == "f" { // file
            fs::File::create(file_name)?;
        }

        self.last_operation = 'n';

        Ok(())
    }

    pub fn edit(&mut self, args: Vec<&str>) -> io::Result<()> {
        let file_name = args[0];
        let modes = u32::from_str_radix(args[1], 8).unwrap();
        fs::set_permissions(file_name, fs::Permissions::from_mode(modes))?;

        self.last_operation = 'e';

        Ok(())
    }
}