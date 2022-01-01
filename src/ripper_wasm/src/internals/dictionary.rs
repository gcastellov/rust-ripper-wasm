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

#[derive(Default)]
pub struct DictionaryList {
    word_list: Vec<String>,
    index: usize,
}

pub trait Dictionary: Iterator {
    fn len(&self) -> usize;
    fn start(&mut self);
    fn get_index(&self) -> usize;
    fn get_chunk(&mut self, size: usize) -> Option<&[String]>;
    fn forward(&mut self, size: usize);
    fn get_last(&self) -> Option<String>;
}

impl DictionaryList {
    pub fn new(entries: &[String]) -> Self {
        DictionaryList {
            word_list: Vec::from(entries),
            index: 0,
        }
    }
}

impl Dictionary for DictionaryList {
    fn len(&self) -> usize {
        self.word_list.len()
    }

    fn start(&mut self) {
        self.index = 0;
    }

    fn get_index(&self) -> usize {
        self.index
    }

    fn get_chunk(&mut self, size: usize) -> Option<&[String]> {
        let chunk = if self.index + size < self.word_list.len() {
            self.word_list.get(self.index..self.index + size)
        } else {
            self.word_list.get(self.index..)
        };

        if chunk.unwrap().is_empty() {
            None
        } else {
            chunk
        }
    }

    fn forward(&mut self, size: usize) {
        for _ in 0..size {
            self.next();
        }
    }

    fn get_last(&self) -> Option<String> {
        if self.index == 0 {
            None
        } else if let Some(word) = self.word_list.get(self.index - 1) {
            Some(word.to_owned())
        } else {
            None
        }
    }
}

impl Iterator for DictionaryList {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        match self.word_list.get(self.index) {
            Some(word) => {
                self.index += 1;
                Some(word.clone())
            }
            _ => None,
        }
    }
}

impl Iterator for DictionaryMaker {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        self.counter += 1;
        let mut current = self.word.pop().unwrap();
        self.last_word = Some(self.word.clone());

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
            word: vec![0],
            counter: 0,
            owned: 0,
            last_word: None,
            buffer: Vec::default()
        }
    }

    fn translate_word(&self, word: Vec<usize>) -> String {
        word.iter().map(|c| self.chars[*c]).collect()
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
            let word = self.next().unwrap();
            self.buffer.push(word);
        }

        Some(self.buffer.as_slice())
    }

    fn forward(&mut self, size: usize) {
        for _ in 0..size {
            self.next();
        }
    }

    fn get_last(&self) -> Option<String> {
        let last = self.last_word.to_owned().unwrap();
        Some(self.translate_word(last))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterate_when_empty_returns_none() {
        let mut dictionary = DictionaryList::default();
        assert!(dictionary.next().is_none());
    }

    #[test]
    fn iterate_when_not_empty_return_some() {
        let mut dictionary = DictionaryList::new(vec![String::from("my_word")].as_slice());
        let word = dictionary.next();
        assert!(word.is_some());
        assert!(dictionary.next().is_none());
    }

    #[test]
    fn get_chunk() {
        const CHUNK_SIZE: usize = 50;
        const CONTENT_LENGTH: usize = 1000;

        let words: Vec<String> = (0..1000).map(|num| num.to_string()).collect();

        let mut dictionary = DictionaryList::new(words.as_slice());
        let mut content: Vec<String> = Vec::default();
        let mut rounds: usize = 0;

        while let Some(chunk) = dictionary.get_chunk(CHUNK_SIZE) {
            let mut chunk_vector: Vec<String> = chunk.iter().map(|word| word.clone()).collect();
            content.append(&mut chunk_vector);
            dictionary.forward(CHUNK_SIZE);
            rounds += 1;
        }

        assert_eq!(20, rounds);
        assert_eq!(CONTENT_LENGTH, dictionary.get_index());
        assert_eq!(words, content);
    }

    #[test]
    fn get_last() {
        let mut dictionary = DictionaryList::new(vec![String::from("one"), String::from("two")].as_slice());

        assert!(dictionary.get_last().is_none());

        dictionary.next();
        assert_eq!(dictionary.get_last().unwrap(), "one");

        dictionary.next();
        assert_eq!(dictionary.get_last().unwrap(), "two");
    }
}