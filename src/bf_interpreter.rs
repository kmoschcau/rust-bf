pub struct BfInterpreter {
    pub registers: Vec<u8>,
    pub pointer: usize,
    pub loop_markers: Vec<usize>,
    pub code_index: usize,
}

impl BfInterpreter {
    pub fn new() -> Self {
        BfInterpreter {
            registers: vec!(0),
            pointer: 0,
            loop_markers: vec!(),
            code_index: 0,
        }
    }

    pub fn run(&mut self, code: &str) -> Result<(), String> {
        self.code_index = 0;
        let code_vec: Vec<char> = code.chars().collect();
        while self.code_index < code_vec.len() {
            match code_vec.get(self.code_index) {
                Some('>') => self.inc_pointer(),
                Some('<') => self.dec_pointer(),
                Some('+') => self.inc_register(),
                Some('-') => self.dec_register(),
                Some('.') => self.print_char(),
                Some(',') => self.read_char(),
                Some('[') => self.start_loop(),
                Some(']') => self.end_loop(),
                Some(&character) => return Err(format!("Unexpected code glyph: {}", &character)),
                None => return Err("Unexpected end of source code!".to_string()),
            }
            self.code_index += 1;
        }
        Ok(())
    }

    pub fn reset(&mut self) {
        self.registers = vec!(0);
        self.pointer = 0;
        self.loop_markers = vec!();
        self.code_index = 0;
    }

    fn inc_pointer(&mut self) {
        self.pointer += 1;
        if self.registers.get(self.pointer).is_none() {
            self.registers.push(0);
        }
    }

    fn dec_pointer(&mut self) {
        if !self.pointer == 0 {
            self.pointer -= 1;
        }
    }

    fn inc_register(&mut self) {
        self.registers[self.pointer] += 1;
    }

    fn dec_register(&mut self) {
        if self.registers[self.pointer] == 0 {
            return;
        }
        self.registers[self.pointer] -= 1;
    }

    fn print_char(&mut self) {
        print!("{}", self.registers[self.pointer] as char);
    }

    fn read_char(&mut self) {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).expect("Could not read line from stdin");
        match buffer.chars().next() {
            Some(character) => {
                if character.len_utf8() > 1 {
                    panic!("Only single byte characters are allowed!");
                } else {
                    self.registers[self.pointer] = character as u8;
                }
            },
            None => panic!("No character read!")
        }
    }

    fn start_loop(&mut self) {
        self.loop_markers.push(self.code_index);
    }

    fn end_loop(&mut self) {
        if self.loop_markers.is_empty() {
            return;
        }

        if self.registers[self.pointer] == 0 {
            self.loop_markers.pop();
        } else {
            self.code_index = *self.loop_markers.last().unwrap();
        }
    }
}
