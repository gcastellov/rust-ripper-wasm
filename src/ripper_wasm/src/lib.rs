use crate::algorithms::implementations::{EncoderFactory, Algorithm};
use wasm_bindgen::prelude::*;
use std::collections::HashMap;

extern crate js_sys;
extern crate base64;
extern crate md5;
extern crate sha256;
extern crate md4;
extern crate sha1;

mod algorithms;

#[wasm_bindgen]
pub struct Ripper {
    input: String,
    dictionary: Dictionary,
    word_match: Option<String>,
    algorithm: Option<Algorithm>,
    elapsed_seconds: Option<f64>,
    dictionar_lists: HashMap<String, String>,
}

struct Dictionary {
    word_list: Vec<String>,
    index: usize
}

impl Default for Dictionary {   
    fn default() -> Self {
        Dictionary {
            word_list: Vec::default(),
            index: 0
        }
    }
}

impl Dictionary {
    fn load(&mut self, entries: String) {
        self.word_list = entries
            .split("\r\n")
            .map(|word|word.split("\n"))
            .flatten()
            .filter_map(|word|if word.is_empty() { None } else { Some(word.to_string()) })
            .collect()
    }

    fn len(&self) -> usize {
        self.word_list.len()
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

#[wasm_bindgen]
impl Ripper {
    
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Ripper { 
            input: String::default(),
            dictionary: Dictionary::default(),
            word_match: None,
            algorithm: None,
            elapsed_seconds: None,
            dictionar_lists: HashMap::default(),
        }
    }

    pub fn has_dictionary(&self, key: String) -> bool {
        self.dictionar_lists.contains_key(&key)
    }

    pub fn add_dictionary(&mut self, key: &str, value: &str) {
        self.dictionar_lists.insert(key.to_string(), value.to_string());
    }

    pub fn load_dictionaries(&mut self, keys: Vec<JsValue>) {
        let entries: String = keys
            .iter()
            .filter_map(|k|k.as_string()) 
            .filter_map(|key|self.dictionar_lists.get(&key))
            .fold(String::default(), |a,b| a + b);

        self.dictionary.load(entries);
    }

    pub fn set_input(&mut self, input: &str) {
        self.input = match self.algorithm {
            Some(Algorithm::Base64) => input.to_string(),
            _ => input.to_lowercase(),
        };
    }

    pub fn set_algorithm(&mut self, algorithm: Algorithm) {
        self.algorithm = Some(algorithm);
    }

    pub fn try_match(&mut self) -> bool {
        self.reset();

        if self.algorithm.is_none() {
            panic!("set algorithm first!");
        }

        let starting = js_sys::Date::now();
        let algorithm = self.algorithm.as_ref().unwrap();
        let encoder = algorithm.get_encoder();

        for word in &mut self.dictionary {
            let digest = encoder.encode(&word);
            if digest == self.input {
                self.word_match = Some(word.clone());
                break;
            }
        }

        let ending = js_sys::Date::now();
        let elapsed = (ending - starting) / 1_000f64;
        self.elapsed_seconds = Some(elapsed);
        self.word_match.is_some()
    }

    pub fn get_word_list_count(&self) -> usize {
        self.dictionary.len()
    }

    pub fn get_progress(&self) -> usize {
        self.dictionary.index
    }

    pub fn get_match(&self) -> String {
        self.word_match.clone().unwrap_or_default()
    }

    pub fn get_elapsed_seconds(&self) -> f64 {
        self.elapsed_seconds.unwrap_or(0.0)
    }
}

impl Ripper {
    fn reset(&mut self) {
        self.word_match = None;
        self.elapsed_seconds = None;
        self.dictionary.index = 0;
    }
}

#[cfg(test)]
mod tests {

    #![cfg(target_arch = "wasm32")]
    extern crate wasm_bindgen_test;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_test::*;
    use crate::Ripper;    
    use crate::Algorithm;

    const ENGLISH_KEY: &str = "english";
        
    fn compute(input: &str, algorithm: Algorithm) {
        const WORD_LIST: &str = "one\r\ntwo\r\nmy_word\r\nthree";
        let dictionary_lists: Vec<JsValue> = vec![ JsValue::from_str(ENGLISH_KEY) ];

        let mut cracker: Ripper = Ripper::new();
        cracker.set_algorithm(algorithm);
        cracker.add_dictionary(ENGLISH_KEY, WORD_LIST);
        cracker.load_dictionaries(dictionary_lists);
        cracker.set_input(input);
        
        assert_eq!(true, cracker.try_match());
        assert_eq!("my_word", cracker.word_match.unwrap_or_default());
    }

    #[wasm_bindgen_test]
    fn reset_clear_result() {
        let mut cracker = Ripper::new();
        cracker.word_match = Some("match".to_string());
        cracker.reset();

        assert_eq!(None, cracker.word_match);
        assert_eq!(None, cracker.elapsed_seconds);
        assert_eq!(0.0, cracker.get_elapsed_seconds());
        assert_eq!(true, cracker.get_match().is_empty());
        assert_eq!(0, cracker.get_progress());
    }

    #[wasm_bindgen_test]
    fn load_word_entries_filter_empty_lines() {
        const WORD_LIST: &str = "\r\n";
        let dictionary_lists: Vec<JsValue> = vec![ JsValue::from_str(ENGLISH_KEY) ];

        let mut cracker: Ripper = Ripper::new();
        cracker.add_dictionary(ENGLISH_KEY, WORD_LIST);
        cracker.load_dictionaries(dictionary_lists);
        assert_eq!(0, cracker.get_word_list_count());
    }

    #[wasm_bindgen_test]
    fn load_word_entries_using_new_line_style() {
        const WORD_LIST: &str = "one\ntwo\nthree";
        let dictionary_lists: Vec<JsValue> = vec![ JsValue::from_str(ENGLISH_KEY) ];

        let mut cracker: Ripper = Ripper::new();
        cracker.add_dictionary(ENGLISH_KEY, WORD_LIST);
        cracker.load_dictionaries(dictionary_lists);
        assert_eq!(3, cracker.get_word_list_count());
    }

    #[wasm_bindgen_test]
    fn load_word_entries_combining_new_line_style() {
        const WORD_LIST: &str = "one\r\ntwo\nthree\r\nfour";
        let dictionary_lists: Vec<JsValue> = vec![ JsValue::from_str(ENGLISH_KEY) ];

        let mut cracker: Ripper = Ripper::new();
        cracker.add_dictionary(ENGLISH_KEY, WORD_LIST);
        cracker.load_dictionaries(dictionary_lists);
        assert_eq!(4, cracker.get_word_list_count());
    }
    
    #[wasm_bindgen_test]
    fn load_word_entries_reset_values() {
        const WORD_LIST_ONE: &str = "one\r\ntwo\r\nthree";
        const WORD_LIST_TWO: &str = "one\r\ntwo\r\nthree\r\nfour";
        let dictionary_lists_one: Vec<JsValue> = vec![ JsValue::from_str(ENGLISH_KEY) ];
        let dictionary_lists_two: Vec<JsValue> = vec![ JsValue::from_str("french") ];

        let mut cracker: Ripper = Ripper::new();
        cracker.add_dictionary(ENGLISH_KEY, WORD_LIST_ONE);
        cracker.load_dictionaries(dictionary_lists_one);
        assert_eq!(3, cracker.get_word_list_count());

        cracker.add_dictionary("french", WORD_LIST_TWO);
        cracker.load_dictionaries(dictionary_lists_two);
        assert_eq!(4, cracker.get_word_list_count());
    }

    #[wasm_bindgen_test]
    fn compute_md5() {
        compute("E4EAC943E400CD75335CE2A751E794F4", Algorithm::Md5);
    }

    #[wasm_bindgen_test]
    fn compute_base64() {
        compute("bXlfd29yZA==", Algorithm::Base64);
    }

    #[wasm_bindgen_test]
    fn compute_sha256() {
        compute("7C375E543FB8235B84054D89818C8D30B1C55CD06A65236C56EFE6223D43C06E", Algorithm::Sha256);
    }

    #[wasm_bindgen_test]
    fn compuet_md4() {
        compute("3B9AFF425FA5F33A77B0DC569AB4FE60", Algorithm::Md4)
    }

    #[wasm_bindgen_test]
    fn compuet_sha1() {
        compute("3E047347D97C14169F3EA769B1F28CAF9D6A8E5D", Algorithm::Sha1);
    }
}