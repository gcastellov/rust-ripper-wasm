use crate::rippers::CHUNK_SIZE;
use crate::DictionaryManager;
use crate::HashEncoderFactory;
use crate::HashEncoder;
use crate::Inner;
use crate::HashAlgorithm;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct HashRipper {
    inner: Inner,
    algorithm: Option<HashAlgorithm>,
    encoder: Option<Box<dyn HashEncoder>>,
}

#[wasm_bindgen]
impl HashRipper {
    
    #[wasm_bindgen(constructor)]
    pub fn new(dictionary_manager: &mut DictionaryManager) -> Self {
        HashRipper {
            inner: Inner::new(dictionary_manager.make()),
            algorithm: None,
            encoder: None,
        }
    }

    pub fn get_last_word(&self) -> String {
        (self.inner.dictionary.get_last().unwrap_or(&String::default())).to_owned()
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
        let encoder = self.encoder.as_ref().unwrap();
        HashRipper::core_check(milliseconds, &mut self.inner, encoder)
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

    pub fn is_checking(&self) -> bool {
        self.inner.word_match.is_none() && self.get_progress() < self.inner.dictionary.len()
    }

    pub fn set_dictionary(&mut self, dictionary_manager: &mut DictionaryManager) {
        self.inner = Inner::new(dictionary_manager.make());
    }
}

impl HashRipper {

    pub fn core_check(milliseconds: f64, mut inner: &mut Inner, encoder: &Box<dyn HashEncoder>) -> bool {
        let starting = js_sys::Date::now();

        while let Some(chunk) = inner.dictionary.get_chunk(CHUNK_SIZE) {
            let mut index = 0;
            let mut current: Option<&String> = chunk.get(index);
            
            while inner.word_match.is_none() && current.is_some() {
                let current_word = current.unwrap();
                let digest = encoder.encode(current_word);
                if digest == inner.input {
                    inner.word_match = Some(current_word.clone());
                }
                
                index += 1;
                current = chunk.get(index);
            }
            
            inner.dictionary.forward(CHUNK_SIZE);
            
            if inner.word_match.is_some() || js_sys::Date::now() - starting > milliseconds {
                break;
            }
        }

        inner.word_match.is_some()
    }
}