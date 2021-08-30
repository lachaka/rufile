pub enum InputMode {
    Editing,
    Normal,
    Error,
}

pub struct Input {
    pub input: String,
    pub input_mode: InputMode,
  //  src: OsString,
  //  dst: OsString,s
}

impl Default for Input {
    fn default() -> Input {
        Input {
            input: String::new(),
            input_mode: InputMode::Normal,
          //  src: "".to_string().to
        }
    }
}

impl Input {
    pub fn execute(&mut self) {
        self.validate_input();
        self.input.drain(..);
    }

    fn validate_input(&mut self) {
        if !self.input.starts_with(':') {
            self.input_mode = InputMode::Error;
        } else {
            self.input_mode = InputMode::Normal;
        }
    }
}