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
        if self.index < self.algorithms.len() {
            let result = &self.algorithms[self.index];
            self.index += 1;
            Some(result.clone())
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
}

#[wasm_bindgen]
impl LuckyRipper {
    
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        LuckyRipper {
            inner: Inner::default(),
            algorithm_list: AlgorithmList::default(),
            encoder: None,
            input: String::default(),
        }
    }

    pub fn set_input(&mut self, input: &str) {
        self.input = input.to_string();
    }

    pub fn get_dictionary_cache_keys(&self) -> Vec<JsValue> {
        self.inner.dictionary_cache.keys().map(|k|JsValue::from(k)).collect()
    }

    pub fn get_dictionary_value(&self, key: String) -> JsValue {
        JsValue::from(self.inner.dictionary_cache.get(&key).unwrap())
    }

    pub fn get_dictionary_selection(&self) -> Vec<JsValue> {
        self.inner.dictionary_selection.iter().map(|selection|JsValue::from(selection)).collect()
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

    pub fn get_word_list_count(&self) -> usize {
        self.inner.get_word_list_count()
    }

    pub fn get_progress(&mut self) -> usize {
        let all_algorithms = HashAlgorithm::iterator().len();
        let checked_algorithms = all_algorithms - self.algorithm_list.len();
        checked_algorithms * self.get_word_list_count() + self.inner.dictionary.get_index()
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

    pub fn start_matching(&mut self) {
        self.encoder = None;
        self.algorithm_list = AlgorithmList::default();
        self.inner.reset();
        self.inner.start_ticking();
    }

    pub fn check(&mut self, milliseconds: f64) -> bool {
        if self.inner.word_match.is_none() && self.inner.dictionary.get_index() == 0 {
            if let Some(algorithm) = self.algorithm_list.pop() {
                self.encoder = algorithm.get_encoder();
                let original_input = self.input.clone();
                self.inner.input = match algorithm {
                    HashAlgorithm::Base64 => original_input,
                    _ => original_input.to_lowercase(),
                };
            } else {
                panic!("All algorithms have been used!")
            }
        }

        let encoder = self.encoder.as_ref().unwrap();
        let result = HashRipper::core_check(milliseconds, &mut self.inner, encoder);

        if self.inner.dictionary.get_index() == self.inner.get_word_list_count() && self.inner.word_match.is_none() {
            self.inner.dictionary.start();
        }

        result
    }

    pub fn is_checking(&self) -> bool {
        self.inner.word_match.is_none() && !self.algorithm_list.is_empty()
    }
}