use super::management::Dictionary;

#[derive(Default)]
pub struct DictionaryList {
    word_list: Vec<String>,
    index: usize,
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

        self.index = if self.index + size < self.word_list.len() { 
            self.index + size 
        } else {
            self.word_list.len()
        };

        if chunk.unwrap().is_empty() {
            None
        } else {
            chunk
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

    fn has_ended(&self) -> bool {
        self.index >= self.len()
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
    fn get_chunk_when_iterating_until_the_end_returns_complete_list() {
        const CHUNK_SIZE: usize = 50;
        const CONTENT_LENGTH: usize = 1000;

        let words: Vec<String> = (0..1000).map(|num| num.to_string()).collect();

        let mut dictionary = DictionaryList::new(words.as_slice());
        let mut content: Vec<String> = Vec::default();
        let mut rounds: usize = 0;

        while let Some(chunk) = dictionary.get_chunk(CHUNK_SIZE) {
            let mut chunk_vector: Vec<String> = chunk.iter().map(|word| word.clone()).collect();
            content.append(&mut chunk_vector);
            rounds += 1;
        }

        assert_eq!(20, rounds);
        assert_eq!(CONTENT_LENGTH, dictionary.get_index());
        assert_eq!(words, content);
    }

    #[test]
    fn getting_last_when_iterating_returns_previous_word() {
        let mut dictionary = DictionaryList::new(vec![String::from("one"), String::from("two")].as_slice());

        assert!(dictionary.get_last().is_none());

        dictionary.next();
        assert_eq!(dictionary.get_last().unwrap(), "one");

        dictionary.next();
        assert_eq!(dictionary.get_last().unwrap(), "two");
    }
}