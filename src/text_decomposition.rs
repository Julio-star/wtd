use crate::text_decomposition::decomposition_error::{LEN_DIFF, NO_DIFF, WtdError};
use crate::text_decomposition::result::IndexEntity;
use crate::text_decomposition::result::Word;

mod decomposition_error;
mod result;

#[derive(Clone)]
pub struct ArgsEntity {
    pub original_text: String,
    pub modified_text: String,
}

impl ArgsEntity {
    fn get_words_from_string(&self, text: &str) -> Result<Vec<Word>, WtdError> {
        let mut words: Vec<Word> = vec![];

        let mut current_word: Word = Word {
            text: String::from(""),
            indexes: IndexEntity { start: 0, end: 0 },
        };

        for i in 0..text.len() {
            let curr_char: char = text.chars().nth(i).unwrap();

            if curr_char == ' ' {
                // Then word is terminate so we added it
                current_word.indexes.end = i as usize;
                words.insert(words.len(), current_word);
                break;
            } else {
                current_word.add_letter_to_word(curr_char);
                current_word.indexes.start = i as usize;
            }
        }

        return Result::Ok(words);
    }

    fn get_indexes_diff_occurences(&self) -> Result<Vec<IndexEntity>, WtdError> {
        if self.original_text.len() != self.modified_text.len() {
            // For now we make an error if length differe between both txt provided
            return Result::Err(LEN_DIFF);
        }

        let original_words: Vec<Word> = self.get_words_from_string(&self.original_text)?;
        let modified_words: Vec<Word> = self.get_words_from_string(&self.modified_text)?;

        let mut result: Vec<IndexEntity> = vec![];

        for i in 0..original_words.iter().len() {
            if original_words[i].text != modified_words[i].text {
                result.insert(result.len(), original_words[i].indexes.clone());
            }
        }

        if result.is_empty() {
            return Result::Err(NO_DIFF);
        }

        return Ok(result);
    }

    pub fn get_text_diff(self) -> TextDiff {
        let indexes = match self.get_indexes_diff_occurences() {
            Ok(value) => value,
            Err(error) => panic!("{}", error),
        };

        return TextDiff {
            args: self,
            indexes,
        };
    }
}

pub struct TextDiff {
    args: ArgsEntity,
    indexes: Vec<IndexEntity>,
}

impl TextDiff {

    fn highlith_text_diff() {
        
    }

    pub fn printer(self) {
        let list_of_indexes: Vec<IndexEntity> = self.indexes;
        let text: String = self.args.original_text;

        const HIGHLITED_COLOR: i32 = 33;

        let mut text_to_be_colored: String = String::from("");

        for i in list_of_indexes{
            print!("\x1b[{}m{}\x1b[m", HIGHLITED_COLOR, &text[i.start..i.end]);
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn case1() {
        todo!("Faire les TU");
        //assert_eq!(1, 1);
    }
}
