use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use super::dictionary::{Dictionary, DictionaryList};

#[wasm_bindgen]
pub struct DictionaryManager {
    dictionary_cache: HashMap<String, Vec<String>>,
    dictionary_selection: Vec<String>,
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
    pub fn make(&self) -> Box<dyn Dictionary<Item=String>> {
        let entries: Vec<String> = self
            .dictionary_selection
            .iter()
            .filter_map(|key|self.dictionary_cache.get(key))
            .flat_map(|word| word.to_owned())
            .collect();

        Box::new(DictionaryList::new(&entries))
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

#[cfg(test)]
mod tests {
    use super::*;

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
}