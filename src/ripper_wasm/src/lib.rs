use crate::algorithms::implementations::*;
use crate::internals::core::*;
use wasm_bindgen::prelude::*;

extern crate js_sys;
extern crate base64;
extern crate md5;
extern crate sha256;
extern crate md4;
extern crate sha1;

mod algorithms;
mod internals;

#[wasm_bindgen]
pub struct HashRipper {
    inner: Inner,
    algorithm: Option<HashAlgorithm>,
}

#[wasm_bindgen]
pub struct SymetricRipper {
    inner: Inner,
    algorithm: Option<SymetricAlgorithm>,
    key_dictionary: Dictionary,
}

#[wasm_bindgen]
impl SymetricRipper {
    
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        SymetricRipper {
            inner: Inner::default(),
            key_dictionary: Dictionary::default(),
            algorithm: None,
        }
    }

    pub fn set_input(&mut self, input: &str) {
        self.inner.input = input.to_string();
    }

    pub fn set_algorithm(&mut self, algorithm: SymetricAlgorithm) {
        self.algorithm = Some(algorithm);
    }

    pub fn has_dictionary(&self, key: String) -> bool {
        self.inner.has_dictionary(key)
    }

    pub fn add_dictionary(&mut self, key: &str, value: &str) {
        self.inner.add_dictionary(key, value)
    }

    pub fn load_dictionaries(&mut self, keys: Vec<JsValue>) {        
        let keys_as_string = keys
            .iter()
            .filter_map(|k|k.as_string())
            .collect();

        self.inner.load_dictionaries(keys_as_string);
    }

    pub fn try_match(&mut self) -> bool {
        self.inner.reset();

        if self.algorithm.is_none() {
            panic!("set algorithm first!");
        }

        let starting = js_sys::Date::now();
        let algorithm = self.algorithm.as_ref().unwrap();
        let encoder = algorithm.get_encoder().unwrap();

        while let Some(key) = self.key_dictionary.next() {
            self.inner.dictionary.start();
            for word in &mut self.inner.dictionary {
                let digest = encoder.encode(&key, &word);
                if digest == self.inner.input {
                    self.inner.word_match = Some(word.clone());
                    break;
                }
            }                        
            if self.inner.word_match.is_some() {
                break;
            }
        }
        
        let ending = js_sys::Date::now();
        let elapsed = (ending - starting) / 1_000f64;
        self.inner.elapsed_seconds = Some(elapsed);
        self.inner.word_match.is_some()
    }

    pub fn get_word_list_count(&self) -> usize {
        self.inner.get_word_list_count()
    }

    pub fn get_progress(&self) -> usize {
        let mut rounds: usize = 0;
        let current_index = self.key_dictionary.get_index();
        if current_index > 0 {
            rounds = current_index - 1;
        }

        rounds * self.inner.dictionary.len() + current_index
    }

    pub fn get_match(&self) -> String {
        self.inner.get_match()
    }

    pub fn get_elapsed_seconds(&self) -> f64 {
        self.inner.get_elapsed_seconds()
    }
}

#[wasm_bindgen]
impl HashRipper {
    
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        HashRipper { 
            inner: Inner::default(),
            algorithm: None,
        }
    }

    pub fn has_dictionary(&self, key: String) -> bool {
        self.inner.has_dictionary(key)
    }

    pub fn add_dictionary(&mut self, key: &str, value: &str) {
        self.inner.add_dictionary(key, value)
    }

    pub fn load_dictionaries(&mut self, keys: Vec<JsValue>) {        
        let keys_as_string = keys
            .iter()
            .filter_map(|k|k.as_string())
            .collect();

        self.inner.load_dictionaries(keys_as_string);
    }

    pub fn set_input(&mut self, input: &str) {
        self.inner.input = match self.algorithm {
            Some(HashAlgorithm::Base64) => input.to_string(),
            _ => input.to_lowercase(),
        };
    }

    pub fn set_algorithm(&mut self, algorithm: HashAlgorithm) {
        self.algorithm = Some(algorithm);
    }

    pub fn try_match(&mut self) -> bool {
        self.inner.reset();

        if self.algorithm.is_none() {
            panic!("set algorithm first!");
        }

        let starting = js_sys::Date::now();
        let algorithm = self.algorithm.as_ref().unwrap();
        let encoder = algorithm.get_encoder().unwrap();

        for word in &mut self.inner.dictionary {
            let digest = encoder.encode(&word);
            if digest == self.inner.input {
                self.inner.word_match = Some(word.clone());
                break;
            }
        }

        let ending = js_sys::Date::now();
        let elapsed = (ending - starting) / 1_000f64;
        self.inner.elapsed_seconds = Some(elapsed);
        self.inner.word_match.is_some()
    }

    pub fn get_word_list_count(&self) -> usize {
        self.inner.get_word_list_count()
    }

    pub fn get_progress(&self) -> usize {
        self.inner.dictionary.get_index()
    }

    pub fn get_match(&self) -> String {
        self.inner.get_match()
    }

    pub fn get_elapsed_seconds(&self) -> f64 {
        self.inner.get_elapsed_seconds()
    }
}

impl Clone for HashRipper {    
    fn clone(&self) -> Self { 
        HashRipper {
            algorithm: self.algorithm.clone(),
            inner: Inner {
                input: self.inner.input.clone(),
                dictionary: self.inner.dictionary.clone(),
                dictionary_cache: self.inner.dictionary_cache.clone(),
                dictionary_selection: self.inner.dictionary_selection.clone(),
                elapsed_seconds: None,
                word_match: None,
            }
        }
    }
}

#[cfg(test)]
mod tests {

    #![cfg(target_arch = "wasm32")]
    extern crate wasm_bindgen_test;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_test::*;
    use super::*;

    const ENGLISH_KEY: &str = "english";
        
    macro_rules! try_match_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[wasm_bindgen_test]
            fn $name() {
                let (input, algorithm) = $value;
                const WORD_LIST: &str = "one\r\ntwo\r\nmy_word\r\nthree";
                let dictionary_lists: Vec<JsValue> = vec![ JsValue::from_str(ENGLISH_KEY) ];

                let mut cracker: HashRipper = HashRipper::new();
                cracker.set_algorithm(algorithm);
                cracker.add_dictionary(ENGLISH_KEY, WORD_LIST);
                cracker.load_dictionaries(dictionary_lists);
                cracker.set_input(input);
                
                assert_eq!(true, cracker.try_match());
                assert_eq!("my_word", cracker.get_match());
            }
        )*
        }
    }

    try_match_tests! {
        match_md4: ("3B9AFF425FA5F33A77B0DC569AB4FE60", HashAlgorithm::Md4),
        match_md5: ("E4EAC943E400CD75335CE2A751E794F4", HashAlgorithm::Md5),
        match_base64: ("bXlfd29yZA==", HashAlgorithm::Base64),
        match_sha256: ("7C375E543FB8235B84054D89818C8D30B1C55CD06A65236C56EFE6223D43C06E", HashAlgorithm::Sha256),
        match_sha1: ("3E047347D97C14169F3EA769B1F28CAF9D6A8E5D", HashAlgorithm::Sha1),
    }
}