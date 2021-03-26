pub mod core {
    
    use std::collections::HashMap;

    pub struct Dictionary {
        word_list: Vec<String>,
        index: usize
    }

    pub struct Inner {
        pub input: String,
        pub dictionary: Dictionary,
        pub dictionary_cache: HashMap<String, String>,
        pub word_match: Option<String>,
        pub dictionary_selection: Vec<String>,
        pub starting_at: Option<f64>,
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

        pub fn get_chunk(&mut self, size: usize) -> Option<&[String]> {
            let chunk = if self.index + size < self.word_list.len() {
                self.word_list.get(self.index..self.index + size)
            } else {
                self.word_list.get(self.index..)
            };

            if chunk.unwrap().is_empty() { None } else  { chunk }
        }

        pub fn forward(&mut self, size: usize) {
            for _ in 0..size {
                self.next();
            }
        }

        pub fn get_last(&self) -> Option<&String> {
            if self.index == 0 {
                None
            } else {
                self.word_list.get(self.index-1)
            }
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

    impl Inner {
        pub fn reset(&mut self) {
            self.word_match = None;
            self.starting_at = None;
            self.dictionary.start();
        }
    
        pub fn get_word_list_count(&self) -> usize {
            self.dictionary.len()
        }
    
        pub fn get_match(&self) -> String {
            self.word_match.clone().unwrap_or_default()
        }
    
        pub fn get_elapsed_seconds(&self) -> f64 {
            if let Some(starting_at) = self.starting_at {
                (js_sys::Date::now() - starting_at) / 1_000f64
            } else {
                0.0f64
            }
        }
    
        pub fn load_dictionaries(&mut self, keys: Vec<String>) {
            if self.dictionary_selection != keys {
                self.dictionary_selection = keys;
                let entries: String = self.dictionary_selection
                    .iter()
                    .filter_map(|key|self.dictionary_cache.get(key))
                    .fold(String::default(), |a,b| a + b);
        
                self.dictionary.load(entries)
            }
        }

        pub fn has_dictionary(&self, key: String) -> bool {
            self.dictionary_cache.contains_key(&key)
        }
    
        pub fn add_dictionary(&mut self, key: &str, value: &str) {
            self.dictionary_cache.insert(key.to_string(), value.to_string());
        }

        pub fn start_ticking(&mut self) {
            self.starting_at = Some(js_sys::Date::now());
        }

        pub fn get_last_word(&self) -> String {
            (self.dictionary.get_last().unwrap_or(&String::default())).to_owned()
        }
    }
    
    impl Default for Inner {
        fn default() -> Self {
            Inner {
                dictionary: Dictionary::default(),
                dictionary_cache: HashMap::default(),
                dictionary_selection: Vec::default(),
                input: String::default(),
                word_match: None,
                starting_at: None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::internals::core::*;

    const ENGLISH: &str = "english";

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

    #[test]
    fn reset_clear_result() {
        let mut inner = Inner::default();
        inner.word_match = Some("match".to_string());
        inner.reset();

        assert_eq!(None, inner.word_match);
        assert_eq!(None, inner.starting_at);
        assert_eq!(0.0, inner.get_elapsed_seconds());
        assert_eq!(true, inner.get_match().is_empty());
    }

    #[test]
    fn loading_entries() {
        const SPANISH: &str = "spanish";

        let keys_one: Vec<String> = vec![ String::from(ENGLISH) ];
        let keys_two: Vec<String> = vec![ String::from(ENGLISH), String::from(SPANISH) ];

        let mut inner = Inner::default();
        inner.add_dictionary(ENGLISH, "aaaaaa\n");
        inner.add_dictionary(SPANISH, "bbbbbb\n");

        inner.load_dictionaries(keys_one.clone());
        inner.load_dictionaries(keys_one.clone());
        assert_eq!(1, inner.get_word_list_count());
        
        inner.load_dictionaries(keys_two);
        assert_eq!(2, inner.get_word_list_count());

        inner.load_dictionaries(keys_one);
        assert_eq!(1, inner.get_word_list_count());
    }

    #[test]
    fn get_chunk() {
        const CHUNK_SIZE: usize = 50;

        let mut inner = Inner::default();
        let words: String = (0..100)
            .map(|num|num.to_string() + "\n")
            .collect();

        inner.add_dictionary(ENGLISH, words.as_str());
        inner.load_dictionaries(vec![ ENGLISH.to_string() ]);
        let mut rounds: usize = 0;
        
        while let Some(_) = inner.dictionary.get_chunk(CHUNK_SIZE) {
            inner.dictionary.forward(CHUNK_SIZE);
            rounds += 1;
        }

        assert_eq!(2, rounds);
        assert_ne!(0, inner.dictionary.get_index());
    }

    #[test]
    fn get_last() {
        let mut dictionary = Dictionary::default();
        dictionary.load(String::from("one\ntwo\n"));

        assert!(dictionary.get_last().is_none());

        dictionary.next();
        assert_eq!(dictionary.get_last().unwrap(), "one");

        dictionary.next();
        assert_eq!(dictionary.get_last().unwrap(), "two");
    }

    #[test]
    fn get_last_word_when_empty() {
        let inner = Inner::default();
        let actual = inner.get_last_word();

        assert_eq!(actual, String::default());
    }

    #[test]
    fn get_last_word_when_not_empty() {
        let mut inner = Inner::default();
        let words: String = (0..10)
            .map(|num|num.to_string() + "\n")
            .collect();

        inner.add_dictionary(ENGLISH, words.as_str());
        inner.load_dictionaries(vec![ ENGLISH.to_string() ]);
        
        assert_eq!(inner.get_last_word(), String::default());

        inner.dictionary.next();
        assert_eq!(inner.get_last_word(), "0");
    }
}
