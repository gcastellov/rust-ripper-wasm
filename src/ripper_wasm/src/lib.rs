use wasm_bindgen::prelude::*;
extern crate js_sys;
extern crate base64;
extern crate md5;
extern crate sha256;
extern crate md4;
extern crate sha1;

use md4::{Md4,Digest};

#[wasm_bindgen]
pub enum Algorithm {
    Md5 = 1,
    Base64 = 2,
    Sha256 = 3,
    Md4 = 4,
    Sha1 = 5,
}

#[wasm_bindgen]
pub struct Ripper {
    input: String,
    word_list: Vec<String>,
    word_list_progress: usize,
    word_match: Option<String>,
    algorithm: Option<Algorithm>,
    elapsed_seconds: Option<f64>,
}

struct Md5Wrapper {}
struct Base64Wrapper {}
struct Sha256Wrapper {}
struct Md4Wrapper {}
struct Sha1Wrapper {}

#[wasm_bindgen]
impl Ripper {
    
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Ripper { 
            input: String::new(),
            word_list: Vec::default(),
            word_list_progress: 0,
            word_match: None,
            algorithm: None,
            elapsed_seconds: None
        }
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

    pub fn load_word_entries(&mut self, entries: String) {        
        self.word_list = entries
            .split("\r\n")
            .map(|word|word.split("\n"))
            .flatten()
            .filter_map(|word|if word.is_empty() { None } else { Some(word.to_string()) })
            .collect();
    }

    pub fn try_match(&mut self) -> bool {
        self.reset();

        if self.algorithm.is_none() {
            panic!("set algorithm first!");
        }

        let starting = js_sys::Date::now();
        let algorithm = self.algorithm.as_ref().unwrap();
        let encoder = algorithm.get_encoder();

        for word in self.word_list.iter() {
            self.word_list_progress += 1;
            let digest = encoder.encode(word);
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
        self.word_list.len()
    }

    pub fn get_progress(&self) -> usize {
        self.word_list_progress
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
        self.word_list_progress = 0;
    }    
}

trait EncoderFactory {
    fn get_encoder(&self) -> &dyn Encoder;
}

trait Encoder {
    fn encode(&self, input: &String) -> String;
}

impl Encoder for Md5Wrapper {    
    fn encode(&self, input: &String) -> String { 
        let digest = md5::compute(input);
        format!("{:x}", digest)
    }
}

impl Encoder for Base64Wrapper {
    fn encode(&self, input: &String) -> String { 
        base64::encode(input)
    }
}

impl Encoder for Sha256Wrapper {
    fn encode(&self, input: &String) -> String { 
        sha256::digest(input)
    }
}

impl Encoder for Md4Wrapper {
    fn encode(&self, input: &String) -> String { 
        let mut hasher = Md4::new();
        hasher.update(input);
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}

impl Encoder for Sha1Wrapper {
    fn encode(&self, input: &String) -> String { 
        let mut hasher = sha1::Sha1::new();
        let bytes: &[u8] = input.as_bytes();
        hasher.update(bytes);
        hasher.digest().to_string()
    }
}

impl EncoderFactory for Algorithm {    
    fn get_encoder(&self) -> &dyn Encoder { 
        match self {
            Algorithm::Md5 => &Md5Wrapper { },
            Algorithm::Sha256 => &Sha256Wrapper { },
            Algorithm::Base64 => &Base64Wrapper { },
            Algorithm::Md4 => &Md4Wrapper { },
            Algorithm::Sha1 => &Sha1Wrapper { }
        }
    }
}

#[cfg(test)]
mod tests {

    #![cfg(target_arch = "wasm32")]
    extern crate wasm_bindgen_test;
    use wasm_bindgen_test::*;
    use crate::Ripper;    
    use crate::Algorithm;
        
    fn compute(input: &str, algorithm: Algorithm) {
        const WORD_LIST: &str = "one\r\ntwo\r\nmy_word\r\nthree";
        let mut cracker: Ripper = Ripper::new();
        cracker.set_algorithm(algorithm);
        cracker.load_word_entries(WORD_LIST.to_string());
        cracker.set_input(input);
        
        assert_eq!(true, cracker.try_match());
        assert_eq!("my_word", cracker.word_match.unwrap_or_default());
    }

    #[wasm_bindgen_test]
    fn reset_clear_result() {
        let mut cracker = Ripper::new();
        cracker.word_match = Some("match".to_string());
        cracker.word_list_progress = 500;
        cracker.reset();

        assert_eq!(None, cracker.word_match);
        assert_eq!(None, cracker.elapsed_seconds);
        assert_eq!(0.0, cracker.get_elapsed_seconds());
        assert_eq!(true, cracker.get_match().is_empty());
        assert_eq!(0, cracker.word_list_progress);
        assert_eq!(0, cracker.get_progress());
    }

    #[wasm_bindgen_test]
    fn load_word_entries_filter_empty_lines() {
        const WORD_LIST: &str = "\r\n";

        let mut cracker: Ripper = Ripper::new();
        cracker.load_word_entries(WORD_LIST.to_string());
        assert_eq!(0, cracker.get_word_list_count());
    }

    #[wasm_bindgen_test]
    fn load_word_entries_using_new_line_style() {
        const WORD_LIST: &str = "one\ntwo\nthree";

        let mut cracker: Ripper = Ripper::new();
        cracker.load_word_entries(WORD_LIST.to_string());
        assert_eq!(3, cracker.get_word_list_count());
    }

    #[wasm_bindgen_test]
    fn load_word_entries_combining_new_line_style() {
        const WORD_LIST: &str = "one\r\ntwo\nthree\r\nfour";

        let mut cracker: Ripper = Ripper::new();
        cracker.load_word_entries(WORD_LIST.to_string());
        assert_eq!(4, cracker.get_word_list_count());
    }
    
    #[wasm_bindgen_test]
    fn load_word_entries_reset_values() {
        const WORD_LIST_ONE: &str = "one\r\ntwo\r\nthree";
        const WORD_LIST_TWO: &str = "one\r\ntwo\r\nthree\r\nfour";

        let mut cracker: Ripper = Ripper::new();
        cracker.load_word_entries(WORD_LIST_ONE.to_string());
        assert_eq!(3, cracker.get_word_list_count());

        cracker.load_word_entries(WORD_LIST_TWO.to_string());
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