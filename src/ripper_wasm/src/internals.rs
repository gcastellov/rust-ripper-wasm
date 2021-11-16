pub mod core {
    use std::collections::HashMap;
    use wasm_bindgen::prelude::*;

    #[derive(Default)]
    pub struct Dictionary {
        word_list: Vec<String>,
        index: usize,
    }

    #[wasm_bindgen]
    pub struct DictionaryManager {
        dictionary_cache: HashMap<String, Vec<String>>,
        dictionary_selection: Vec<String>,
    }

    #[derive(Default)]
    pub struct Inner {
        pub input: String,
        pub word_match: Option<String>,
        pub starting_at: Option<f64>,
        pub dictionary: Box<Dictionary>,
    }

    #[wasm_bindgen]
    impl DictionaryManager {
        #[wasm_bindgen(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        pub fn get_word_list_count(&self) -> usize {
            self.dictionary_selection
                .iter()
                .filter_map(|key| self.dictionary_cache.get(key))
                .map(|entries| entries.len())
                .sum()
        }

        pub fn has_dictionary(&self, key: &str) -> bool {
            self.dictionary_cache.contains_key(&key.to_string())
        }

        pub fn add_dictionary(&mut self, key: &str, value: &str) {
            let entries = Self::extract(value);
            self.dictionary_cache.insert(key.to_string(), entries);
        }

        pub fn load_dictionaries(&mut self, keys: Vec<JsValue>) {
            self.dictionary_selection = keys.iter().filter_map(|k| k.as_string()).collect();
        }
    }

    impl DictionaryManager {
        pub fn make(&self) -> Box<Dictionary> {
            let entries: Vec<String> = self
                .dictionary_selection
                .iter()
                .filter_map(|key| self.dictionary_cache.get(key))
                .flat_map(|word| word.to_owned())
                .collect();

            Box::new(Dictionary::new(&entries))
        }

        fn extract(entries: &str) -> Vec<String> {
            entries
                .split("\r\n")
                .map(|word| word.split('\n'))
                .flatten()
                .filter_map(|word| {
                    if word.is_empty() {
                        None
                    } else {
                        Some(word.to_string())
                    }
                })
                .collect()
        }
    }

    impl Default for DictionaryManager {
        fn default() -> Self {
            Self {
                dictionary_cache: HashMap::default(),
                dictionary_selection: Vec::default(),
            }
        }
    }

    impl Dictionary {
        fn new(entries: &[String]) -> Self {
            Dictionary {
                word_list: Vec::from(entries),
                index: 0,
            }
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

            if chunk.unwrap().is_empty() {
                None
            } else {
                chunk
            }
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
                self.word_list.get(self.index - 1)
            }
        }
    }

    impl Iterator for Dictionary {
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

    impl Inner {
        pub fn new(dictionary: Box<Dictionary>) -> Self {
            Inner {
                input: String::default(),
                dictionary,
                word_match: None,
                starting_at: None,
            }
        }

        pub fn reset(&mut self) {
            self.word_match = None;
            self.starting_at = None;
            self.dictionary.start();
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

        pub fn start_ticking(&mut self) {
            self.starting_at = Some(js_sys::Date::now());
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn iterate_when_empty_returns_none() {
            let mut dictionary = Dictionary::default();
            assert!(dictionary.next().is_none());
        }

        #[test]
        fn iterate_when_not_empty_return_some() {
            let mut dictionary = Dictionary::new(vec![String::from("my_word")].as_slice());
            let word = dictionary.next();
            assert!(word.is_some());
            assert!(dictionary.next().is_none());
        }

        #[test]
        fn extract_word_entries_filter_empty_lines() {
            const WORD_LIST: &str = "\r\n";
            let actual = DictionaryManager::extract(WORD_LIST);
            assert_eq!(0, actual.len());
        }

        #[test]
        fn extract_word_entries_using_new_line_style() {
            const WORD_LIST: &str = "one\ntwo\nthree";
            let actual = DictionaryManager::extract(WORD_LIST);
            assert_eq!(3, actual.len());
        }

        #[test]
        fn extract_word_entries_combining_new_line_style() {
            const WORD_LIST: &str = "one\r\ntwo\nthree\r\nfour";
            let actual = DictionaryManager::extract(WORD_LIST);
            assert_eq!(4, actual.len());
        }

        #[test]
        fn get_chunk() {
            const CHUNK_SIZE: usize = 50;
            const CONTENT_LENGTH: usize = 1000;

            let words: Vec<String> = (0..1000).map(|num| num.to_string()).collect();

            let mut dictionary = Dictionary::new(words.as_slice());
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
            let mut dictionary =
                Dictionary::new(vec![String::from("one"), String::from("two")].as_slice());

            assert!(dictionary.get_last().is_none());

            dictionary.next();
            assert_eq!(dictionary.get_last().unwrap(), "one");

            dictionary.next();
            assert_eq!(dictionary.get_last().unwrap(), "two");
        }
    }
}
