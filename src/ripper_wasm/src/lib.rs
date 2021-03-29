use crate::algorithms::implementations::*;
use crate::internals::core::*;

extern crate js_sys;
extern crate base64;
extern crate md5;
extern crate sha256;
extern crate md4;
extern crate sha1;
extern crate ripemd320;

mod algorithms;
mod internals;
mod rippers;

#[cfg(test)]
mod tests {

    #![cfg(target_arch = "wasm32")]
    extern crate wasm_bindgen_test;
    use crate::rippers::hashing::HashRipper;
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
                const MILLIS: f64 = 500f64;
                let dictionary_lists: Vec<JsValue> = vec![ JsValue::from_str(ENGLISH_KEY) ];

                let mut cracker: HashRipper = HashRipper::new();
                cracker.set_algorithm(algorithm);
                cracker.add_dictionary(ENGLISH_KEY, WORD_LIST);
                cracker.load_dictionaries(dictionary_lists);
                cracker.set_input(input);
                cracker.start_matching();
                
                assert!(cracker.check(MILLIS));
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

    #[wasm_bindgen_test]
    fn chunky_check() {
        let dictionary_lists: Vec<JsValue> = vec![ JsValue::from_str(ENGLISH_KEY) ];
        let words: String = (0..99999)
            .map(|num|num.to_string() + "\n")
            .collect();
        
        let mut cracker: HashRipper = HashRipper::new();
        cracker.set_algorithm(HashAlgorithm::Md5);
        cracker.add_dictionary(ENGLISH_KEY, words.as_str());
        cracker.load_dictionaries(dictionary_lists);
        cracker.set_input("e57023ed682d83a41d25acb650c877da");
        cracker.start_matching();
        
        while cracker.get_progress() < cracker.get_word_list_count() {
            if cracker.check(500f64) {
                break;
            }
        }
       
        assert_eq!("99998", cracker.get_match());
        assert_ne!(0.0, cracker.get_elapsed_seconds());
    }
}