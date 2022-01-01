use std::slice::Iter;
use wasm_bindgen::prelude::*;

macro_rules! define_types {
    ($($type:ident),*) => {
        $(
        struct $type;
        )*
    }
}

macro_rules! impl_digest {
    ($($type:ident: $wrapper:ident),*) => {
        $(
            impl HashEncoder for $wrapper {
                fn encode(&self, input: &str) -> String {
                    let result = $type::digest(input.as_bytes());
                    format!("{:x}", result)
                }
            }
        )*
    };
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum HashAlgorithm {
    Md5 = 1,
    Base64 = 2,
    Sha256 = 3,
    Md4 = 4,
    Sha1 = 5,
    Ripemd128 = 6,
    Ripemd320 = 7,
    Whirlpool = 8,
    Md2 = 9,
    Ripemd160 = 10,
    Blake2b = 11,
    Blake2s = 12,
    Tiger = 13,
    Shabal192 = 14,
    Shabal224 = 15,
    Shabal256 = 16,
    Shabal384 = 17,
    Shabal512 = 18,
}

#[wasm_bindgen]
#[derive(Clone)]
pub enum SymetricAlgorithm {
    Des = 20,
    Des3 = 21,
}

pub trait HashEncoderFactory {
    fn get_encoder(&self) -> Option<(u8, Box<dyn HashEncoder>)>;
}

pub trait SymetricEncoderFactory {
    fn get_encoder(&self) -> Option<Box<dyn SymetricEncoder>>;
}

pub trait HashEncoder {
    fn encode(&self, input: &str) -> String;
}

pub trait SymetricEncoder {
    fn encode(&self, key: &str, input: &str) -> String;
}

define_types!(
    Md5Wrapper,
    Base64Wrapper,
    Sha256Wrapper,
    Md4Wrapper,
    Sha1Wrapper,
    DesWrapper,
    Des3Wrapper,
    Ripemd128Wrapper,
    Ripemd160Wrapper,
    Ripemd320Wrapper,
    WhirlpoolWrapper,
    Md2Wrapper,
    Blake2bWrapper,
    Blake2sWrapper,
    TigerWrapper,
    Shabal192Wrapper,
    Shabal224Wrapper,
    Shabal256Wrapper,
    Shabal384Wrapper,
    Shabal512Wrapper
);

static ALGORITHMS: [HashAlgorithm; 18] = [
    HashAlgorithm::Md5,
    HashAlgorithm::Base64,
    HashAlgorithm::Sha256,
    HashAlgorithm::Md4,
    HashAlgorithm::Sha1,
    HashAlgorithm::Ripemd128,
    HashAlgorithm::Ripemd320,
    HashAlgorithm::Whirlpool,
    HashAlgorithm::Md2,
    HashAlgorithm::Ripemd160,
    HashAlgorithm::Blake2b,
    HashAlgorithm::Blake2s,
    HashAlgorithm::Tiger,
    HashAlgorithm::Shabal192,
    HashAlgorithm::Shabal224,
    HashAlgorithm::Shabal256,
    HashAlgorithm::Shabal384,
    HashAlgorithm::Shabal512,
];

impl HashAlgorithm {
    pub fn iterator() -> Iter<'static, HashAlgorithm> {
        ALGORITHMS.iter()
    }

    fn get_code(&self) -> u8 {
        let variant = *self;
        variant as u8
    }
}

impl HashEncoderFactory for HashAlgorithm {
    fn get_encoder(&self) -> Option<(u8, Box<dyn HashEncoder>)> {
        let code = self.get_code();
        match self {
            HashAlgorithm::Md5 => Some((code, Box::new(Md5Wrapper {}))),
            HashAlgorithm::Sha256 => Some((code, Box::new(Sha256Wrapper {}))),
            HashAlgorithm::Base64 => Some((code, Box::new(Base64Wrapper {}))),
            HashAlgorithm::Md4 => Some((code, Box::new(Md4Wrapper {}))),
            HashAlgorithm::Sha1 => Some((code, Box::new(Sha1Wrapper {}))),
            HashAlgorithm::Ripemd128 => Some((code, Box::new(Ripemd128Wrapper {}))),
            HashAlgorithm::Ripemd160 => Some((code, Box::new(Ripemd160Wrapper {}))),
            HashAlgorithm::Ripemd320 => Some((code, Box::new(Ripemd320Wrapper {}))),
            HashAlgorithm::Whirlpool => Some((code, Box::new(WhirlpoolWrapper {}))),
            HashAlgorithm::Md2 => Some((code, Box::new(Md2Wrapper {}))),
            HashAlgorithm::Blake2b => Some((code, Box::new(Blake2bWrapper {}))),
            HashAlgorithm::Blake2s => Some((code, Box::new(Blake2sWrapper {}))),
            HashAlgorithm::Tiger => Some((code, Box::new(TigerWrapper {}))),
            HashAlgorithm::Shabal192 => Some((code, Box::new(Shabal192Wrapper {}))),
            HashAlgorithm::Shabal224 => Some((code, Box::new(Shabal224Wrapper {}))),
            HashAlgorithm::Shabal256 => Some((code, Box::new(Shabal256Wrapper {}))),
            HashAlgorithm::Shabal384 => Some((code, Box::new(Shabal384Wrapper {}))),
            HashAlgorithm::Shabal512 => Some((code, Box::new(Shabal512Wrapper {}))),
        }
    }
}

impl SymetricEncoderFactory for SymetricAlgorithm {
    fn get_encoder(&self) -> Option<Box<dyn SymetricEncoder>> {
        match self {
            SymetricAlgorithm::Des => Some(Box::new(DesWrapper {})),
            SymetricAlgorithm::Des3 => Some(Box::new(Des3Wrapper {})),
        }
    }
}

mod md5 {
    use super::{Md5Wrapper, HashEncoder};

    impl HashEncoder for Md5Wrapper {
        fn encode(&self, input: &str) -> String {
            let digest = md5::compute(input);
            format!("{:x}", digest)
        }
    }
}

mod base64 {
    use super::{Base64Wrapper,HashEncoder};

    impl HashEncoder for Base64Wrapper {
        fn encode(&self, input: &str) -> String {
            base64::encode(input)
        }
    }
}

mod sha256 {
    use super::{Sha256Wrapper,HashEncoder};

    impl HashEncoder for Sha256Wrapper {
        fn encode(&self, input: &str) -> String {
            sha256::digest(input)
        }
    }
}

mod md4 {
    use super::{Md4Wrapper,HashEncoder};
    use md4::{Digest, Md4};

    impl_digest!(Md4: Md4Wrapper);
}

mod sha1 {
    use super::{Sha1Wrapper,HashEncoder};
    use sha1::Sha1;

    impl HashEncoder for Sha1Wrapper {
        fn encode(&self, input: &str) -> String {
            let mut hasher = Sha1::new();
            hasher.update(input.as_bytes());
            hasher.digest().to_string()
        }
    }
}

mod ripemd320 {
    use super::{Ripemd320Wrapper,HashEncoder};
    use ripemd320::{Digest, Ripemd320};

    impl_digest!(Ripemd320: Ripemd320Wrapper);
}

mod ripemd160 {
    use super::{Ripemd160Wrapper,HashEncoder};
    use ripemd160::{Digest, Ripemd160};

    impl_digest!(Ripemd160: Ripemd160Wrapper);
}

mod ripemd128 {
    use super::{Ripemd128Wrapper,HashEncoder};
    use ripemd128::{Digest, Ripemd128};

    impl_digest!(Ripemd128: Ripemd128Wrapper);
}

mod whirlpool {
    use super::{WhirlpoolWrapper,HashEncoder};
    use whirlpool::{Digest, Whirlpool};

    impl_digest!(Whirlpool: WhirlpoolWrapper);
}

mod md2 {
    use super::{Md2Wrapper,HashEncoder};
    use md2::{Digest, Md2};

    impl_digest!(Md2: Md2Wrapper);
}

mod blake2 {
    use super::{Blake2bWrapper,Blake2sWrapper,HashEncoder};
    use blake2::{Blake2b, Blake2s, Digest};

    impl_digest!(Blake2b: Blake2bWrapper, Blake2s: Blake2sWrapper);
}

mod tiger {
    use super::{TigerWrapper,HashEncoder};
    use digest::Digest;
    use tiger::Tiger;

    impl_digest!(Tiger: TigerWrapper);
}

mod shabal {
    use super::{
        Shabal192Wrapper, Shabal224Wrapper, Shabal256Wrapper, Shabal384Wrapper,
        Shabal512Wrapper, HashEncoder
    };
    use shabal::{Digest, Shabal192, Shabal224, Shabal256, Shabal384, Shabal512};

    impl_digest!(
        Shabal192: Shabal192Wrapper,
        Shabal224: Shabal224Wrapper,
        Shabal256: Shabal256Wrapper,
        Shabal384: Shabal384Wrapper,
        Shabal512: Shabal512Wrapper
    );
}

impl SymetricEncoder for DesWrapper {
    fn encode(&self, _key: &str, _input: &str) -> String {
        todo!()
    }
}

impl SymetricEncoder for Des3Wrapper {
    fn encode(&self, _key: &str, _input: &str) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {

    use super::*;    

    macro_rules! hash_algorithm_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let input: String = String::from("Hello world!");
                let (algorithm, expected) = $value;
                let (_, encoder): (u8, Box<dyn HashEncoder>) = algorithm.get_encoder().unwrap();
                let actual = encoder.encode(&input);
                assert_eq!(actual, expected);
            }
        )*
        }
    }

    macro_rules! hash_encoder_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let input = $value;
                let encoder: Option<(u8, Box<dyn HashEncoder>)> = input.get_encoder();
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
        hash_md2: HashAlgorithm::Md2,
        hash_md4: HashAlgorithm::Md4,
        hash_md5: HashAlgorithm::Md5,
        hash_sha1: HashAlgorithm::Sha1,
        hash_sha256: HashAlgorithm::Sha256,
        hash_base64: HashAlgorithm::Base64,
        hash_ripemd128: HashAlgorithm::Ripemd128,
        hash_ripemd160: HashAlgorithm::Ripemd160,
        hash_ripemd320: HashAlgorithm::Ripemd320,
        hash_whirlpool: HashAlgorithm::Whirlpool,
        hash_blacke2b: HashAlgorithm::Blake2b,
        hash_blacke2s: HashAlgorithm::Blake2s,
        hash_tiger: HashAlgorithm::Tiger,
        hash_shabal192: HashAlgorithm::Shabal192,
        hash_shabal224: HashAlgorithm::Shabal224,
        hash_shabal256: HashAlgorithm::Shabal256,
        hash_shabal384: HashAlgorithm::Shabal384,
        hash_shabal512: HashAlgorithm::Shabal512,
    }

    symetric_encoder_tests! {
        symetric_des: SymetricAlgorithm::Des,
        symetric_des3: SymetricAlgorithm::Des3,
    }

    hash_algorithm_tests! {
        md2: (HashAlgorithm::Md2, "63503d3117ad33f941d20f57144ece64"),
        md4: (HashAlgorithm::Md4, "0d7a9db5a3bed4ae5738ee6d1909649c"),
        md5: (HashAlgorithm::Md5, "86fb269d190d2c85f6e0468ceca42a20"),
        sha1: (HashAlgorithm::Sha1, "d3486ae9136e7856bc42212385ea797094475802"),
        sha256: (HashAlgorithm::Sha256, "c0535e4be2b79ffd93291305436bf889314e4a3faec05ecffcbb7df31ad9e51a"),
        base64: (HashAlgorithm::Base64, "SGVsbG8gd29ybGQh"),
        ripemd128: (HashAlgorithm::Ripemd128, "d917d92bc5591a0915f70acebbc2b126"),
        ripemd160: (HashAlgorithm::Ripemd160, "7f772647d88750add82d8e1a7a3e5c0902a346a3"),
        ripemd320: (HashAlgorithm::Ripemd320, "f1c1c231d301abcf2d7daae0269ff3e7bc68e623ad723aa068d316b056d26b7d1bb6f0cc0f28336d"),
        whirlpool: (HashAlgorithm::Whirlpool, "bb4f1451ec1b8326643d25d74547591619cb01dd1f104d729a13494cbd95382d3526b00a2d3fdf448e1e4b39887c54fe2aea9767872b58ed361eb3a12075c5b5"),
        blake2b: (HashAlgorithm::Blake2b, "0389abc5ab1e8e170e95aff19d341ecbf88b83a12dd657291ec1254108ea97352c2ff5116902b9fe4021bfe5a6a4372b0f7c9fc2d7dd810c29f85511d1e04c59"),
        blake2s: (HashAlgorithm::Blake2s, "c63813a8f804abece06213a46acd04a2d738c8e7a58fbf94bfe066a9c7f89197"),
        tiger: (HashAlgorithm::Tiger, "432b916300b93d2849bca4629ad04e6d8acff835aa42a8fa"),
        shabal192: (HashAlgorithm::Shabal192, "4975ce359cb4097944b622ca2fb86aeedaca18a49fbd7c2c"),
        shabal224: (HashAlgorithm::Shabal224, "4953a339d7ef6042fb35ad2ec7bed9ecaae8873719746c38431a7503"),
        shabal256: (HashAlgorithm::Shabal256, "dee2cacc573a950a436d80bad166695c88e421bf02d0c8f063f74394ff6947b3"),
        shabal384: (HashAlgorithm::Shabal384, "4dfb72ceecbf6d8c908c8694966316f04e366d519a411cfbacd4ed642428c33da1ceb17ddf7d00801ac8e438ec1cee28"),
        shabal512: (HashAlgorithm::Shabal512, "bdeadae0daa07e0947738d3e6c0569b23efa865ed5a601fe9409f8f5473c51d4dc465470640cb9805179ff13cef9f5682958343b0cac67cd737a927c3c178b46"),
    }

    #[test]
    fn get_all_hash_algorithms() {
        let all: Vec<HashAlgorithm> = HashAlgorithm::iterator().cloned().collect();
        assert!(!all.is_empty())
    }
}