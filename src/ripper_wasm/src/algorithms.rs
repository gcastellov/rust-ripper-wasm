pub mod implementations {

    use wasm_bindgen::prelude::*;
    use md4::{Md4,Digest};

    #[wasm_bindgen]
    pub enum Algorithm {
        Md5 = 1,
        Base64 = 2,
        Sha256 = 3,
        Md4 = 4,
        Sha1 = 5,
    }

    pub trait EncoderFactory {
        fn get_encoder(&self) -> &dyn Encoder;
    }
    
    pub trait Encoder {
        fn encode(&self, input: &String) -> String;
    }

    struct Md5Wrapper {}
    struct Base64Wrapper {}
    struct Sha256Wrapper {}
    struct Md4Wrapper {}
    struct Sha1Wrapper {}

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
}