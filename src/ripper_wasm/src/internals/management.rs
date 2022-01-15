use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use super::{dictionarylist::{DictionaryList}, dictionarymaker::DictionaryMaker};

#[wasm_bindgen]
pub struct DictionaryManager {
    dictionary_cache: HashMap<String, Vec<String>>,
    dictionary_selection: Vec<String>,
    dictionary_type: DictionaryType,
}

pub trait Dictionary: Iterator {
    fn len(&self) -> usize;
    fn start(&mut self);
    fn get_index(&self) -> usize;
    fn get_chunk(&mut self, size: usize) -> Option<&[String]>;
    fn get_last(&self) -> Option<String>;
    fn has_ended(&self) -> bool;
}

enum DictionaryType {
    List = 0,
    Maker = 1,
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

    pub fn set_type(&mut self, dictionary_type: u8) {
        self.dictionary_type = match dictionary_type {
            1 => DictionaryType::Maker,
            _ => DictionaryType::List
        };
    }

}

impl DictionaryManager {
    pub fn make(&self) -> Box<dyn Dictionary<Item=String>> {
        match self.dictionary_type {
            DictionaryType::List => {
                let entries: Vec<String> = self
                .dictionary_selection
                .iter()
                .filter_map(|key|self.dictionary_cache.get(key))
                .flat_map(|word| word.to_owned())
                .collect();
    
                Box::new(DictionaryList::new(&entries))
            },
            DictionaryType::Maker => Box::new(DictionaryMaker::new(16, &(32..128).filter_map(char::from_u32).collect()))
        }
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
            dictionary_type: DictionaryType::List,
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