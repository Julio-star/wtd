#[derive(Clone)]
pub struct IndexEntity {
    pub start: usize,
    pub end: usize,
}

pub struct Word {
    pub text: String,
    pub indexes: IndexEntity,
}

impl Word {
    pub fn add_letter_to_word(& mut self, new_letter: char) {
        let text_length: usize = self.text.len();
        
        self.text.insert(text_length, new_letter);
    }
}