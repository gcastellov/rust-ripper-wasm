use crate::rippers::CHUNK_SIZE;
use crate::SymetricEncoder;
use crate::SymetricEncoderFactory;
use crate::Dictionary;
use crate::SymetricAlgorithm;
use crate::Inner;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct SymetricRipper {
    inner: Inner,
    algorithm: Option<SymetricAlgorithm>,
    key_dictionary: Dictionary,
    encoder: Option<Box<dyn SymetricEncoder>>,
}

#[wasm_bindgen]
impl SymetricRipper {

    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        SymetricRipper {
            inner: Inner::default(),
            key_dictionary: Dictionary::default(),
            algorithm: None,
            encoder: None,
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

    pub fn start_matching(&mut self) {
        self.inner.reset();

        if self.algorithm.is_none() {
            panic!("set algorithm first!");
        }

        self.inner.start_ticking();
        let algorithm = self.algorithm.as_ref().unwrap();
        self.encoder = algorithm.get_encoder();
    }

    pub fn check(&mut self, milliseconds: f64) -> bool {
        let starting = js_sys::Date::now();
        let encoder = self.encoder.as_ref().unwrap();
        let mut current_key: String = String::default();
        
        if self.inner.dictionary.get_index() == 0 {
            if let Some(key) = self.key_dictionary.next() {
                current_key = key.clone();
            } else {
                return false;
            }
        }

        while let Some(chunk) = self.inner.dictionary.get_chunk(CHUNK_SIZE) {
            
            let mut index = 0;
            let mut current: Option<&String> = chunk.get(index);
            
            while self.inner.word_match.is_none() && current.is_some() {
                let current_word = current.unwrap();
                let digest = encoder.encode(current_word, &current_key);
                if digest == self.inner.input {
                    self.inner.word_match = Some(current_word.clone());
                }
                
                index += 1;
                current = chunk.get(index);
            }
            
            self.inner.dictionary.forward(CHUNK_SIZE);
            if self.inner.word_match.is_some() || js_sys::Date::now() - starting > milliseconds {
                break;
            }
        }

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

    pub fn get_last_word(&self) -> String {
        self.inner.get_last_word()
    }
}