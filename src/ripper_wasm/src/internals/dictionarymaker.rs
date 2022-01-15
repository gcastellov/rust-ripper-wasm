use super::management::Dictionary;

#[derive(Default)]
pub struct DictionaryMaker {
    pub chars: Vec<char>,
    pub word_length: usize,
    pub index: usize,
    pub owned: u8,
    pub word: Option<Vec<usize>>,
    pub last_word: Option<Vec<usize>>,
    buffer: Vec<String>
}

impl Iterator for DictionaryMaker {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {

        let mut current_word = self.word.take().unwrap_or_default();

        if self.chars.is_empty() || self.word_length == 0 {
            return None;
        } else if current_word.len() == self.word_length && current_word.iter().all(|c|*c == self.chars.len() - 1) {
            self.last_word = Some(current_word.clone());
            return None;
        } else if current_word.is_empty() {
            current_word.push(0);
            let translated_word = self.translate_word(current_word.to_owned());
            self.word = Some(current_word);
            return Some(translated_word);
        }

        self.index += 1;        
        self.last_word = Some(current_word.clone());
        let mut current = current_word.pop().unwrap();

        if current < self.chars.len() - 1 {
            current += 1;
            current_word.push(current);
        } else if current_word.is_empty() {
            current_word.push(0);
            current_word.push(0);
        } else {
            self.owned += 1;
            let mut last = *current_word.last().unwrap();
            while !current_word.is_empty() && last == self.chars.len() - 1 {
                current_word.pop();
                self.owned += 1;
                if !current_word.is_empty() {
                    last = *current_word.last().unwrap();
                }
            }

            let value = match current_word.pop() {
                Some(last) => last + 1,
                _ => 0,
            };

            current_word.push(value);

            if self.owned > 0 {
                for _ in 0..self.owned {
                    current_word.push(0);
                }
                self.owned = 0;
            }
        }

        let translated_word = self.translate_word(current_word.to_owned());
        self.word = Some(current_word);
        Some(translated_word)
    }
}

impl DictionaryMaker {

    pub fn new(
        word_lenth: usize,
        available_chars: &Vec<char>,
    ) -> DictionaryMaker {
        DictionaryMaker {
            word_length: word_lenth,
            chars: available_chars.to_owned(),
            word: None,
            index: 0,
            owned: 0,
            last_word: None,
            buffer: Vec::default()
        }
    }

    fn translate_word(&self, word: Vec<usize>) -> String {
        word.iter().map(|c|self.chars[*c]).collect()
    }
}

impl Dictionary for DictionaryMaker {
    fn len(&self) -> usize {
        self.index
    }

    fn start(&mut self) {
        self.index = 0;
        self.owned = 0;
        self.last_word = None;
        self.word = None;
        self.buffer = Vec::default();
    }

    fn get_index(&self) -> usize {
        self.index
    }

    fn get_chunk(&mut self, size: usize) -> Option<&[String]> {
        self.buffer = Vec::default();                

        for _ in 0..size {
            if let Some(word) = self.next() {
                self.buffer.push(word);
            } else {
                break;
            }
        }

        self.index += self.buffer.len();

        match self.buffer.len() {
            0 => None,
            _ => Some(self.buffer.as_slice())
        }
    }

    fn get_last(&self) -> Option<String> {
        if self.last_word.is_some() {
            Some(self.translate_word(self.last_word.to_owned().unwrap()))
        } else {
            None
        }
    }

    fn has_ended(&self) -> bool {
        self.word.is_none() && self.last_word.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterate_when_default_returns_none() {
        let mut dictionary = DictionaryMaker::default();
        assert!(dictionary.next().is_none());
    }

    #[test]
    fn getting_chunk_when_chunk_is_bigger_returns_until_the_end() {
        const CHUNK_SIZE: usize = 50;

        let mut dictionary = DictionaryMaker::new(2, &vec!['a', 'b']);
        let chunk = dictionary.get_chunk(CHUNK_SIZE);

        assert!(chunk.is_some());
        assert_eq!(vec!["a", "b", "aa", "ab", "ba", "bb"], chunk.unwrap())
    }

    #[test]
    fn getting_last_when_iterating_returns_previous_word() {
        let mut dictionary = DictionaryMaker::new(2, &vec!['a', 'b']);

        assert!(dictionary.get_last().is_none());

        dictionary.next();
        assert!(dictionary.get_last().is_none());

        dictionary.next();
        assert_eq!(dictionary.get_last().unwrap(), "a");

        dictionary.next();
        assert_eq!(dictionary.get_last().unwrap(), "b");

        dictionary.next();
        assert_eq!(dictionary.get_last().unwrap(), "aa");
        
        dictionary.next();
        assert_eq!(dictionary.get_last().unwrap(), "ab");

        dictionary.next();
        assert_eq!(dictionary.get_last().unwrap(), "ba");

        dictionary.next();
        assert_eq!(dictionary.get_last().unwrap(), "bb");
    }
}