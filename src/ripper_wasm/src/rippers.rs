const CHUNK_SIZE: usize = 500;

pub mod lucky {
    
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
}

pub mod hashing {

    use crate::rippers::CHUNK_SIZE;
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
        pub fn new() -> Self {
            HashRipper {
                inner: Inner::default(),
                algorithm: None,
                encoder: None,
            }
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

        pub fn get_last_word(&self) -> String {
            self.inner.get_last_word()
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

        pub fn is_checking(&self) -> bool {
            self.inner.word_match.is_none() && self.get_progress() < self.get_word_list_count()
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
}

pub mod symetric {
    
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
}