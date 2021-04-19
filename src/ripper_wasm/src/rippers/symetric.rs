use crate::rippers::CHUNK_SIZE;
use crate::DictionaryManager;
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
    pub fn new(dictionary_manager: &mut DictionaryManager) -> Self {
        SymetricRipper {
            inner: Inner::new(dictionary_manager.make()),
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
        (self.inner.dictionary.get_last().unwrap_or(&String::default())).to_owned()
    }
}