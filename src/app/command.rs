#[derive(Default)]
pub struct CommandBar {
    pub text_field: String,
    pub location_pointer: usize,
}

#[derive(Debug)]
pub struct Command {
    pub command: String,
    pub args: Vec<String>,
}

impl CommandBar {
    pub fn append(&mut self, character: char) {
        self.text_field.insert(self.location_pointer, character);
        self.location_pointer += 1;
    }

    pub fn backspace(&mut self) {
        if self.location_pointer == 0 {
            return;
        }
        self.text_field.remove(self.location_pointer - 1);
        self.location_pointer -= 1;
    }
    pub fn reset(&mut self) {
        self.text_field.clear();
        self.location_pointer = 0;
    }
}
