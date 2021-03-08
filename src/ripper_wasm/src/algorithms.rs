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
        fn get_encoder(&self) -> Option<&dyn HashEncoder>;
    }

    pub trait SymetricEncoderFactory {
        fn get_encoder(&self) -> Option<&dyn SymetricEncoder>;
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
        fn get_encoder(&self) -> Option<&dyn HashEncoder> { 
            match self {
                HashAlgorithm::Md5 => Some(&Md5Wrapper { }),
                HashAlgorithm::Sha256 => Some(&Sha256Wrapper { }),
                HashAlgorithm::Base64 => Some(&Base64Wrapper { }),
                HashAlgorithm::Md4 => Some(&Md4Wrapper { }),
                HashAlgorithm::Sha1 => Some(&Sha1Wrapper { }),
                _ => None,
            }
        }
    }

    impl SymetricEncoderFactory for SymetricAlgorithm {
        fn get_encoder(&self) -> Option<&dyn SymetricEncoder> { 
            match self {
                SymetricAlgorithm::Des => Some(&DesWrapper { }),
                SymetricAlgorithm::Des3 => Some(&Des3Wrapper { }),
                _ => None,
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
        fn encode(&self, key: &String, input: &String) -> String { 
            todo!() 
        }
    }

    impl SymetricEncoder for Des3Wrapper {        
        fn encode(&self, key: &String, input: &String) -> String { 
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
                let (input, expected) = $value;
                let encoder: Option<&dyn HashEncoder> = input.get_encoder();
                assert_eq!(encoder.is_some(), expected);
            }
        )*
        }
    }

    macro_rules! symetric_encoder_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                let encoder: Option<&dyn SymetricEncoder> = input.get_encoder();
                assert_eq!(encoder.is_some(), expected);
            }
        )*
        }
    }

    hash_encoder_tests! {
        hash_md4: (HashAlgorithm::Md4, true),
        hash_md5: (HashAlgorithm::Md5, true),
        hash_sha1: (HashAlgorithm::Sha1, true),
        hash_sha256: (HashAlgorithm::Sha256, true),
        hash_base64: (HashAlgorithm::Base64, true),
    }

    symetric_encoder_tests! {
        symetric_des: (SymetricAlgorithm::Des, true),
        symetric_des3: (SymetricAlgorithm::Des3, true),
    }

}