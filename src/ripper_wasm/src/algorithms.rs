pub mod implementations {

    use wasm_bindgen::prelude::*;
    use md4::{Md4,Digest};

    #[wasm_bindgen]
    #[derive(Clone)]
    pub enum HashAlgorithm {
        Md5 = 1,
        Base64 = 2,
        Sha256 = 3,
        Md4 = 4,
        Sha1 = 5,
    }

    #[wasm_bindgen]
    #[derive(Clone)]
    pub enum SymetricAlgorithm {
        Des = 6,
        Des3 = 7,
    }

    pub trait HashEncoderFactory {
        fn get_encoder(&self) -> Option<Box<dyn HashEncoder>>;
    }

    pub trait SymetricEncoderFactory {
        fn get_encoder(&self) -> Option<Box<dyn SymetricEncoder>>;
    }
    
    pub trait HashEncoder {
        fn encode(&self, input: &String) -> String;
    }

    pub trait SymetricEncoder {
        fn encode(&self, key: &String, input: &String) -> String;
    }

    struct Md5Wrapper {}
    struct Base64Wrapper {}
    struct Sha256Wrapper {}
    struct Md4Wrapper {}
    struct Sha1Wrapper {}
    struct DesWrapper {}
    struct Des3Wrapper {}

    impl HashEncoderFactory for HashAlgorithm {
        fn get_encoder(&self) -> Option<Box<dyn HashEncoder>> { 
            match self {
                HashAlgorithm::Md5 => Some(Box::new(Md5Wrapper { })),
                HashAlgorithm::Sha256 => Some(Box::new(Sha256Wrapper { })),
                HashAlgorithm::Base64 => Some(Box::new(Base64Wrapper { })),
                HashAlgorithm::Md4 => Some(Box::new(Md4Wrapper { })),
                HashAlgorithm::Sha1 => Some(Box::new(Sha1Wrapper { }))
            }
        }
    }

    impl SymetricEncoderFactory for SymetricAlgorithm {
        fn get_encoder(&self) -> Option<Box<dyn SymetricEncoder>> { 
            match self {
                SymetricAlgorithm::Des => Some(Box::new(DesWrapper { })),
                SymetricAlgorithm::Des3 => Some(Box::new(Des3Wrapper { }))
            }
        }
    }
    
    impl HashEncoder for Md5Wrapper {    
        fn encode(&self, input: &String) -> String { 
            let digest = md5::compute(input);
            format!("{:x}", digest)
        }
    }
    
    impl HashEncoder for Base64Wrapper {
        fn encode(&self, input: &String) -> String { 
            base64::encode(input)
        }
    }
    
    impl HashEncoder for Sha256Wrapper {
        fn encode(&self, input: &String) -> String { 
            sha256::digest(input)
        }
    }
    
    impl HashEncoder for Md4Wrapper {
        fn encode(&self, input: &String) -> String { 
            let mut hasher = Md4::new();
            hasher.update(input);
            let result = hasher.finalize();
            format!("{:x}", result)
        }
    }
    
    impl HashEncoder for Sha1Wrapper {
        fn encode(&self, input: &String) -> String { 
            let mut hasher = sha1::Sha1::new();
            let bytes: &[u8] = input.as_bytes();
            hasher.update(bytes);
            hasher.digest().to_string()
        }
    }

    impl SymetricEncoder for DesWrapper {        
        fn encode(&self, _key: &String, _input: &String) -> String { 
            todo!() 
        }
    }

    impl SymetricEncoder for Des3Wrapper {        
        fn encode(&self, _key: &String, _input: &String) -> String { 
            todo!() 
        }
    }
}

#[cfg(test)]
mod tests {
    
    use crate::algorithms::implementations::{HashEncoderFactory, SymetricEncoderFactory, SymetricAlgorithm, HashAlgorithm, HashEncoder, SymetricEncoder};

    macro_rules! hash_encoder_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let input = $value;
                let encoder: Option<Box<dyn HashEncoder>> = input.get_encoder();
                assert!(encoder.is_some());
            }
        )*
        }
    }

    macro_rules! symetric_encoder_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let input = $value;
                let encoder: Option<Box<dyn SymetricEncoder>> = input.get_encoder();
                assert!(encoder.is_some());
            }
        )*
        }
    }

    hash_encoder_tests! {
        hash_md4: HashAlgorithm::Md4,
        hash_md5: HashAlgorithm::Md5,
        hash_sha1: HashAlgorithm::Sha1,
        hash_sha256: HashAlgorithm::Sha256,
        hash_base64: HashAlgorithm::Base64,
    }

    symetric_encoder_tests! {
        symetric_des: SymetricAlgorithm::Des,
        symetric_des3: SymetricAlgorithm::Des3,
    }

}