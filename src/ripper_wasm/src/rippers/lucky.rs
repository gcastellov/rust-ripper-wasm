use crate::internals::{algorithms::{HashAlgorithm, HashEncoder, HashEncoderFactory}, wrapper::Inner, dictionarylist::{DictionaryList}, management::{DictionaryManager}};
use crate::rippers::CHUNK_SIZE;
use wasm_bindgen::prelude::*;

#[derive(Clone)]
struct AlgorithmList {
    index: usize,
    algorithms: Vec<HashAlgorithm>,
}

impl Default for AlgorithmList {
    fn default() -> Self {
        AlgorithmList {
            index: 0,
            algorithms: HashAlgorithm::iterator().cloned().collect(),
        }
    }
}

impl AlgorithmList {
    fn len(&self) -> usize {
        self.algorithms.len()
    }
}

impl Iterator for AlgorithmList {
    type Item = HashAlgorithm;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(algorithm) = self.algorithms.get(self.index) {
            self.index += 1;
            Some(*algorithm)
        } else {
            None
        }
    }
}

#[wasm_bindgen]
#[derive(Default)]
pub struct LuckyRipper {
    inner: Inner,
    algorithm_list: AlgorithmList,
}

#[wasm_bindgen]
impl LuckyRipper {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        LuckyRipper {
            inner: Inner::new(Box::new(DictionaryList::default())),
            algorithm_list: AlgorithmList::default()
        }
    }

    pub fn get_last_word(&self) -> String {
        (self
            .inner
            .dictionary
            .get_last()
            .unwrap_or(String::default()))
        .to_owned()
    }

    pub fn set_input(&mut self, input: &str) {
        self.inner.input = input.to_string();
    }

    pub fn get_progress(&mut self) -> usize {
        self.inner.dictionary.get_index() * self.algorithm_list.len()
    }

    pub fn get_match(&self) -> String {
        self.inner.get_match()
    }

    pub fn get_elapsed_seconds(&self) -> f64 {
        self.inner.get_elapsed_seconds()
    }

    pub fn start_matching(&mut self) {
        self.algorithm_list = AlgorithmList::default();
        self.inner.reset();
        self.inner.start_ticking();
    }

    pub fn check(&mut self, milliseconds: f64) -> bool {
        let encoders: Vec<Box<dyn HashEncoder>> = self.algorithm_list.to_owned()
            .filter_map(|a|a.get_encoder())
            .map(|(_, encoder)|encoder)
            .collect();

        LuckyRipper::core_check(milliseconds, &mut self.inner, &encoders)
    }

    pub fn is_checking(&self) -> bool {
        self.inner.word_match.is_none() && !self.inner.dictionary.has_ended()
    }

    pub fn set_dictionary(&mut self, dictionary_manager: &mut DictionaryManager) {
        self.inner.dictionary = dictionary_manager.make();
    }
}

impl LuckyRipper {
    pub fn core_check(
        milliseconds: f64,
        mut inner: &mut Inner,
        encoders: &Vec<Box<dyn HashEncoder>>,
    ) -> bool {
        let starting = js_sys::Date::now();

        while let Some(chunk) = inner.dictionary.get_chunk(CHUNK_SIZE) {
            let mut index = 0;
            let mut current: Option<&String> = chunk.get(index);

            while inner.word_match.is_none() && current.is_some() {
                let current_word = current.unwrap();

                for i in 0..encoders.len() {
                    let encoder = &encoders[i];
                    let digest = encoder.encode(current_word);
                    if digest == inner.input {
                        inner.word_match = Some(current_word.clone());
                        break;
                    }
                }

                index += 1;
                current = chunk.get(index);
            }

            if inner.word_match.is_some() || js_sys::Date::now() - starting > milliseconds {
                break;
            }
        }

        inner.word_match.is_some()
    }    
}