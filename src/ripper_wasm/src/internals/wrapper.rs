use super::{dictionarylist::{DictionaryList}, management::Dictionary};

pub struct Inner {
    pub input: String,
    pub word_match: Option<String>,
    pub starting_at: Option<f64>,
    pub dictionary: Box<dyn Dictionary<Item=String>>,
}    

impl Default for Inner {
    fn default() -> Self {
        let dictionary_list = DictionaryList::default();
        Inner::new(Box::new(dictionary_list))
    }
}

impl Inner {
    pub fn new(dictionary: Box<dyn Dictionary<Item=String>>) -> Self {
        Inner {
            input: String::default(),
            dictionary,
            word_match: None,
            starting_at: None,
        }
    }

    pub fn reset(&mut self) {
        self.word_match = None;
        self.starting_at = None;
        self.dictionary.start();
    }

    pub fn get_match(&self) -> String {
        self.word_match.clone().unwrap_or_default()
    }

    pub fn get_elapsed_seconds(&self) -> f64 {
        if let Some(starting_at) = self.starting_at {
            (js_sys::Date::now() - starting_at) / 1_000f64
        } else {
            0.0f64
        }
    }

    pub fn start_ticking(&mut self) {
        self.starting_at = Some(js_sys::Date::now());
    }
}