use super::management::Dictionary;

#[derive(Default)]
pub struct DictionaryMaker {
    pub chars: Vec<char>,
    pub word_length: usize,
    pub counter: usize,
    pub owned: u8,
    pub word: Vec<usize>,
    pub last_word: Option<Vec<usize>>,
    buffer: Vec<String>
}

impl Iterator for DictionaryMaker {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {

        if self.chars.is_empty() || self.word_length == 0 {
            return None;
        } else if self.word.len() == self.word_length && self.word.iter().all(|c|*c == self.chars.len() - 1) {
            self.last_word = Some(self.word.clone());
            return None;
        } else if self.word.is_empty() {
            self.word.push(0);
            return Some(self.translate_word(self.word.to_owned()));
        }

        self.counter += 1;        
        self.last_word = Some(self.word.clone());
        let mut current = self.word.pop().unwrap();

        if current < self.chars.len() - 1 {
            current += 1;
            self.word.push(current);
        } else if self.word.is_empty() {
            self.word.push(0);
            self.word.push(0);
        } else {
            self.owned += 1;
            let mut last = *self.word.last().unwrap();
            while !self.word.is_empty() && last == self.chars.len() - 1 {
                self.word.pop();
                self.owned += 1;
                if !self.word.is_empty() {
                    last = *self.word.last().unwrap();
                }
            }

            let value = match self.word.pop() {
                Some(last) => last + 1,
                _ => 0,
            };

            self.word.push(value);

            if self.owned > 0 {
                for _ in 0..self.owned {
                    self.word.push(0);
                }
                self.owned = 0;
            }
        }

        Some(self.translate_word(self.word.to_owned()))
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
            word: Vec::default(),
            counter: 0,
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
        self.counter
    }

    fn start(&mut self) {
        self.counter = 0;
        self.owned = 0;
        self.last_word = None;
        self.word = Vec::default();
        self.buffer = Vec::default();
    }

    fn get_index(&self) -> usize {
        self.counter
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

        match self.buffer.len() {
            0 => None,
            _ => Some(self.buffer.as_slice())
        }
    }

    fn forward(&mut self, size: usize) {
        for _ in 0..size {
            self.next();
        }
    }

    fn get_last(&self) -> Option<String> {
        if self.last_word.is_some() {
            Some(self.translate_word(self.last_word.to_owned().unwrap()))
        } else {
            None
        }
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