extern crate base64;
extern crate js_sys;
extern crate md2;
extern crate md4;
extern crate md5;
extern crate ripemd128;
extern crate ripemd160;
extern crate ripemd320;
extern crate sha1;
extern crate sha256;
extern crate whirlpool;

mod rippers;
mod internals;

#[cfg(test)]
mod tests {

    #![cfg(target_arch = "wasm32")]
    extern crate wasm_bindgen_test;
    use super::*;
    use rippers::hashing::HashRipper;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_test::*;
    use internals::dictionarylist::*;
    use internals::algorithms::*;
    use internals::management::*;
    use internals::wrapper::Inner;
    use rippers::lucky::LuckyRipper;

    const ENGLISH_KEY: &str = "english";
    const FRENCH_KEY: &str = "french";

    macro_rules! try_match_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[wasm_bindgen_test]
            fn $name() {
                let (input, algorithm) = $value;
                const WORD_LIST: &str = "one\r\ntwo\r\nmy_word\r\nthree";
                const MILLIS: f64 = 500f64;
                let mut dictionary_manager: DictionaryManager = DictionaryManager::default();
                dictionary_manager.add_dictionary(ENGLISH_KEY, WORD_LIST);
                dictionary_manager.load_dictionaries(vec![ JsValue::from_str(ENGLISH_KEY) ]);

                let mut cracker: HashRipper = HashRipper::new();
                cracker.set_dictionary(&mut dictionary_manager);
                cracker.set_algorithm(algorithm);
                cracker.set_input(input);
                cracker.start_matching();

                assert!(cracker.check(MILLIS));
                assert_eq!("my_word", cracker.get_match());
            }
        )*
        }
    }

    try_match_tests! {
        match_md2: ("eb2429b9c479245b49c5d997e6cbc530", HashAlgorithm::Md2),
        match_md4: ("3B9AFF425FA5F33A77B0DC569AB4FE60", HashAlgorithm::Md4),
        match_md5: ("E4EAC943E400CD75335CE2A751E794F4", HashAlgorithm::Md5),
        match_base64: ("bXlfd29yZA==", HashAlgorithm::Base64),
        match_sha256: ("7C375E543FB8235B84054D89818C8D30B1C55CD06A65236C56EFE6223D43C06E", HashAlgorithm::Sha256),
        match_sha1: ("3E047347D97C14169F3EA769B1F28CAF9D6A8E5D", HashAlgorithm::Sha1),
        match_ripemd128: ("1a147e58b2a2c6e3dedc94ac0eeee67b", HashAlgorithm::Ripemd128),
        match_ripemd160: ("25fc9204b31e219e770f9b973b2f206b4049b732", HashAlgorithm::Ripemd160),
        match_ripemd320: ("667d71946caaadeadd667e040a3f9fdcae361e03b2dd7d7ddfebc9f0d37e58528b37f5274bc03170", HashAlgorithm::Ripemd320),
        match_whirlpool: ("8d0e01d8519e6729aeadc4a30735b972d87907b20233a7241644de2eb3120821fd8e26861e5f9f0d31ad9e5631a6b12c63ead9269ca1a15175366ea75595fd24", HashAlgorithm::Whirlpool),
        match_blake2b: ("c33f6f72d1315446d2bb87c04463c7a231bb6a14da8660ae6bf837832cc88c26c2a2eba814b1a5154a8beda5a00e615739c0f089b84484a302649869c0c3d620", HashAlgorithm::Blake2b),
        match_blake2s: ("f63362fbb63b4fa4e36956ea357c404ce4a81e988675a09bfdcb4d0fde1ea3ca", HashAlgorithm::Blake2s),
        match_tiger: ("6706d961075ae8416586f0d91f23a7f171fb671b0933d439", HashAlgorithm::Tiger),
    }

    #[wasm_bindgen_test]
    fn given_dictionary_list_and_hash_ripper_when_checking_then_gets_match() {
        let dictionary_lists: Vec<JsValue> = vec![JsValue::from_str(ENGLISH_KEY)];
        let words: String = (0..99999).map(|num| num.to_string() + "\n").collect();

        let mut dictionary_manager = DictionaryManager::default();
        dictionary_manager.add_dictionary(ENGLISH_KEY, words.as_str());
        dictionary_manager.load_dictionaries(dictionary_lists);

        let mut cracker: HashRipper = HashRipper::new();
        cracker.set_dictionary(&mut dictionary_manager);
        cracker.set_algorithm(HashAlgorithm::Md5);
        cracker.set_input("e57023ed682d83a41d25acb650c877da");
        cracker.start_matching();

        while cracker.is_checking() {
            cracker.check(500f64);
        }

        assert_eq!("99998", cracker.get_match());
        assert_ne!(0.0, cracker.get_elapsed_seconds());
        assert_ne!(0, cracker.get_progress());
        assert!(!cracker.get_last_word().is_empty());
    }

    #[wasm_bindgen_test]
    fn given_dictionary_list_and_lucky_ripper_when_checking_then_gets_match() {
        let dictionary_lists: Vec<JsValue> = vec![JsValue::from_str(ENGLISH_KEY)];
        let words: String = (0..60).map(|num| num.to_string() + "\n").collect();

        let mut dictionary_manager = DictionaryManager::default();
        dictionary_manager.add_dictionary(ENGLISH_KEY, words.as_str());
        dictionary_manager.load_dictionaries(dictionary_lists);

        let mut cracker: LuckyRipper = LuckyRipper::new();
        cracker.set_dictionary(&mut dictionary_manager);
        cracker.set_input("daa136908bd66810f306b788c644f470");
        cracker.start_matching();

        while cracker.is_checking() {
            cracker.check(500f64);
        }

        assert_eq!("20", cracker.get_match());
        assert_ne!(0.0, cracker.get_elapsed_seconds());
        assert_ne!(0, cracker.get_progress());
    }

    #[wasm_bindgen_test]
    fn given_dictionary_maker_and_hash_ripper_when_checking_then_gets_match() {
        let mut dictionary_manager = DictionaryManager::default();
        dictionary_manager.set_type(1);

        let mut cracker: HashRipper = HashRipper::new();
        cracker.set_dictionary(&mut dictionary_manager);
        cracker.set_algorithm(HashAlgorithm::Md5);
        cracker.set_input("dc5c7986daef50c1e02ab09b442ee34f");
        cracker.start_matching();

        while cracker.is_checking() {
            cracker.check(500f64);
        }

        assert_eq!("001", cracker.get_match());
        assert_ne!(0.0, cracker.get_elapsed_seconds());
        assert_ne!(0, cracker.get_progress());
    }

    #[wasm_bindgen_test]
    fn given_dictionary_manager_has_words_cached_when_loading_selection_then_resets_and_gets_expected_word_list() {
        const WORD_LIST_ONE: &str = "one\r\ntwo\r\nthree";
        const WORD_LIST_TWO: &str = "one\r\ntwo\r\nthree\r\nfour";

        let mut dictionary_manager = DictionaryManager::new();
        dictionary_manager.add_dictionary(ENGLISH_KEY, WORD_LIST_ONE);
        dictionary_manager.add_dictionary(FRENCH_KEY, WORD_LIST_TWO);
        dictionary_manager.load_dictionaries(vec![JsValue::from_str(ENGLISH_KEY)]);
        assert_eq!(3, dictionary_manager.get_word_list_count());
        dictionary_manager.load_dictionaries(vec![JsValue::from_str(FRENCH_KEY)]);
        assert_eq!(4, dictionary_manager.get_word_list_count());
        dictionary_manager.load_dictionaries(vec![
            JsValue::from_str(ENGLISH_KEY),
            JsValue::from_str(FRENCH_KEY),
        ]);
        assert_eq!(7, dictionary_manager.get_word_list_count());
    }

    #[wasm_bindgen_test]
    fn given_dictionary_manager_has_words_cached_when_loading_selection_then_gets_expected_word_list() {
        const WORD_LIST_ONE: &str = "one\r\ntwo\r\nthree";
        const WORD_LIST_TWO: &str = "one\r\ntwo\r\nthree\r\nfour";

        let mut dictionary_manager = DictionaryManager::new();
        dictionary_manager.add_dictionary(ENGLISH_KEY, WORD_LIST_ONE);
        dictionary_manager.add_dictionary(FRENCH_KEY, WORD_LIST_TWO);
        dictionary_manager.load_dictionaries(vec![
            JsValue::from_str(ENGLISH_KEY),
            JsValue::from_str(FRENCH_KEY),
        ]);
        let actual = dictionary_manager.get_word_list_count();
        assert_eq!(7, actual);
    }

    #[wasm_bindgen_test]
    fn given_inner_started_ticking_when_reset_then_reset_to_default_values() {
        let dictionary = DictionaryList::default();
        let mut inner = Inner::new(Box::new(dictionary));
        inner.word_match = Some(String::from("match"));
        inner.start_ticking();
        inner.reset();

        assert_eq!(None, inner.word_match);
        assert_eq!(None, inner.starting_at);
        assert_eq!(0.0, inner.get_elapsed_seconds());
        assert!(inner.get_match().is_empty());
    }

    #[wasm_bindgen_test]
    fn given_dictionary_list_and_lucky_ripper_when_checking_gets_progress_until_end() {
        const WORD_LIMIT: usize = 10000;
        const NUMBER_OF_HASHERS: usize = 18;

        let mut output: Vec<usize> = Vec::default();
        let dictionary_lists: Vec<JsValue> = vec![JsValue::from_str(ENGLISH_KEY)];
        let words: String = (0..WORD_LIMIT).map(|num| num.to_string() + "\n").collect();

        let mut dictionary_manager = DictionaryManager::default();
        dictionary_manager.add_dictionary(ENGLISH_KEY, words.as_str());
        dictionary_manager.load_dictionaries(dictionary_lists);

        let mut cracker: LuckyRipper = LuckyRipper::new();
        cracker.set_dictionary(&mut dictionary_manager);
        cracker.set_input("noway");
        cracker.start_matching();

        while cracker.is_checking() {
            cracker.check(500f64);
            let progress = cracker.get_progress();
            output.push(progress);
        }

        let mut former: usize = 0;
        println!("{:?}", output);
        for value in output.clone() {
            debug_assert!(
                value >= former,
                "Last value was greater: {} - {}",
                former,
                value
            );
            former = value;
        }

        let last: usize = output.into_iter().last().unwrap().clone();
        debug_assert_eq!(
            WORD_LIMIT * NUMBER_OF_HASHERS,
            last,
            "Last value is not the expected: {}",
            last
        );
    }
}
