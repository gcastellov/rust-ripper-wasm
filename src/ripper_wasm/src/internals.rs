pub mod core {
    pub struct Dictionary {
        word_list: Vec<String>,
        index: usize
    }

    impl Dictionary {
        pub fn load(&mut self, entries: String) {
            self.word_list = entries
                .split("\r\n")
                .map(|word|word.split("\n"))
                .flatten()
                .filter_map(|word|if word.is_empty() { None } else { Some(word.to_string()) })
                .collect()
        }

        pub fn len(&self) -> usize {
            self.word_list.len()
        }

        pub fn start(&mut self) {
            self.index = 0;
        }

        pub fn get_index(&self) -> usize {
            self.index
        }
    }

    impl Default for Dictionary {   
        fn default() -> Self {
            Dictionary {
                word_list: Vec::default(),
                index: 0
            }
        }
    }

    impl Iterator for Dictionary {
        type Item = String;
        fn next(&mut self) -> Option<Self::Item> { 
            if self.index < self.word_list.len() {
                let slice = &self.word_list[self.index..self.index+1];            
                self.index += 1;
                Some(slice[0].clone())
            } else {
                None
            }
        }
    }

    impl Clone for Dictionary {    
        fn clone(&self) -> Self { 
            Dictionary {
                index: 0,
                word_list: self.word_list.clone()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::internals::core::Dictionary;

    #[test]
    fn iterate_when_empty_returns_none() {
        let mut dictionary = Dictionary::default();
        assert_eq!(dictionary.next().is_none(), true);
    }

    #[test]
    fn iterate_when_not_empty_return_some() {
        let mut dictionary = Dictionary::default();
        dictionary.load("my_word".to_string());
        let word = dictionary.next();
        assert_eq!(word.is_some(), true);
        assert_eq!(dictionary.next().is_some(), false);
    }

    #[test]
    fn load_word_entries_filter_empty_lines() {
        const WORD_LIST: &str = "\r\n";

        let mut dictionary = Dictionary::default();
        dictionary.load(WORD_LIST.to_string());
        assert_eq!(0, dictionary.len());
    }

    #[test]
    fn load_word_entries_using_new_line_style() {
        const WORD_LIST: &str = "one\ntwo\nthree";

        let mut dictionary = Dictionary::default();
        dictionary.load(WORD_LIST.to_string());
        assert_eq!(3, dictionary.len());
    }

    #[test]
    fn load_word_entries_combining_new_line_style() {
        const WORD_LIST: &str = "one\r\ntwo\nthree\r\nfour";

        let mut dictionary = Dictionary::default();
        dictionary.load(WORD_LIST.to_string());
        assert_eq!(4, dictionary.len());
    }
    
    #[test]
    fn load_word_entries_reset_values() {
        const WORD_LIST_ONE: &str = "one\r\ntwo\r\nthree";
        const WORD_LIST_TWO: &str = "one\r\ntwo\r\nthree\r\nfour";

        let mut dictionary = Dictionary::default();
        dictionary.load(WORD_LIST_ONE.to_string());
        assert_eq!(3, dictionary.len());

        dictionary.load(WORD_LIST_TWO.to_string());
        assert_eq!(4, dictionary.len());
    }
}
