use crate::DictionaryManager;
use crate::internals::core::Dictionary;
use crate::rippers::hashing::HashRipper;
use crate::HashEncoder;
use crate::HashAlgorithm;
use crate::HashEncoderFactory;
use crate::Inner;
use wasm_bindgen::prelude::*;

struct AlgorithmList {
    index: usize,
    algorithms: Vec<HashAlgorithm>,
}

impl Default for AlgorithmList {
    fn default() -> Self {
        AlgorithmList {
            index: 0,
            algorithms: HashAlgorithm::iterator().cloned().collect()
        }
    }
}

impl AlgorithmList {
    fn pop(&mut self) -> Option<HashAlgorithm> {
        self.algorithms.pop()
    }
    fn is_empty(&self) -> bool {
        self.algorithms.is_empty()
    }
    fn len(&self) -> usize {
        self.algorithms.len()
    }
}

impl Iterator for AlgorithmList {
    type Item = HashAlgorithm;
    fn next(&mut self) -> Option<Self::Item> { 
        if let Some(algorithm) = self.algorithms.get(self.index) {
            self.index += 1;
            Some(algorithm.clone())
        } else {
            None
        }
    }
}

#[wasm_bindgen]
pub struct LuckyRipper {
    inner: Inner,
    input: String,
    encoder: Option<Box<dyn HashEncoder>>,
    algorithm_list: AlgorithmList,
    has_ended: bool
}

#[wasm_bindgen]
impl LuckyRipper {
    
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        LuckyRipper {
            inner: Inner::new(Box::new(Dictionary::default())),
            algorithm_list: AlgorithmList::default(),
            encoder: None,
            input: String::default(),
            has_ended: false
        }
    }

    pub fn get_last_word(&self) -> String {
        (self.inner.dictionary.get_last().unwrap_or(&String::default())).to_owned()
    }

    pub fn set_input(&mut self, input: &str) {
        self.input = input.to_string();
    }

    pub fn get_progress(&mut self) -> usize {
        let checked_algorithms = match self.inner.dictionary.get_index() {
            0 => HashAlgorithm::iterator().len() - self.algorithm_list.len(),
            _ => HashAlgorithm::iterator().len() - self.algorithm_list.len() - 1
        };
        checked_algorithms * self.inner.dictionary.len() + self.inner.dictionary.get_index()
    }

    pub fn get_match(&self) -> String {
        self.inner.get_match()
    }

    pub fn get_elapsed_seconds(&self) -> f64 {
        self.inner.get_elapsed_seconds()
    }

    pub fn start_matching(&mut self) {
        self.encoder = None;
        self.algorithm_list = AlgorithmList::default();
        self.has_ended = false;
        self.inner.reset();
        self.inner.start_ticking();
    }

    pub fn check(&mut self, milliseconds: f64) -> bool {
        if self.inner.word_match.is_none() && self.inner.dictionary.get_index() == 0 && !self.algorithm_list.is_empty() {
            if let Some(algorithm) = self.algorithm_list.pop() {
                self.encoder = match algorithm.get_encoder() {
                    Some((_, encoder)) => Some(encoder),
                    _ => None
                };
                let original_input = self.input.clone();
                self.inner.input = match algorithm {
                    HashAlgorithm::Base64 => original_input,
                    _ => original_input.to_lowercase(),
                };
            }
        }

        let encoder = self.encoder.as_ref().unwrap();
        let result = HashRipper::core_check(milliseconds, &mut self.inner, encoder);

        if self.inner.dictionary.get_index() == self.inner.dictionary.len() && self.inner.word_match.is_none() {
            self.has_ended = self.algorithm_list.is_empty();
            if !self.has_ended {
                self.inner.dictionary.start();
            }
        }

        result
    }

    pub fn is_checking(&self) -> bool {
        self.inner.word_match.is_none() && !self.has_ended
    }

    pub fn set_dictionary(&mut self, dictionary_manager: &mut DictionaryManager) {
        self.inner.dictionary = dictionary_manager.make();
    }
}