[![Rust](https://github.com/gcastellov/rust-ripper-wasm/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/gcastellov/rust-ripper-wasm/actions/workflows/rust.yml)

# WASM rust the ripper 
Password cracker made in WASM Rust, inspired by the popular John the Ripper.
The app uses a collection of password dictionaries and different algorithms to perform a brute attack in order to get the ciphered password.
<br/>

## Available algorithms
Please, refer to each crate documentation and check its licensing.

| Name        | Algorithm  | Crates.io |
|-------------|------------|-----------|
| `md4`       | MD4      | [![crates.io](https://img.shields.io/crates/v/md4.svg)](https://crates.io/crates/md4)      |
| `md5`       | MD5     | [![crates.io](https://img.shields.io/crates/v/md5.svg)](https://crates.io/crates/md5)     |
| `sha1`      | SHA-1   | [![crates.io](https://img.shields.io/crates/v/sha1.svg)](https://crates.io/crates/sha-1)   |
| `sha256`    | SHA-2 256| [![crates.io](https://img.shields.io/crates/v/sha256.svg)](https://crates.io/crates/sha256)|
| `base64`    | Base64   | [![crates.io](https://img.shields.io/crates/v/base64.svg)](https://crates.io/crates/base64)|

<br/>

![UI](doc/ui.png)

<br/>

## Build and run in development environment

Download and install **wasm-pack** in case you don't have it.
```
cd /src/ripper_wasm
cargo install wasm-pack
```

Generate the WASM package
```
cargo build
wasm-pack build
```

Link locally the NPM package
```
cd pkg
npm link
cd /src/site
npm link ripper_wasm
```

Install NPM dependecies
```
npm install
```

Run
```
npm run serve
```
## Docker
```
docker build -f ./Docker/Dockerfile  -t rust-ripper-wasm .
```
```
docker run -p 8080:80 rust-ripper-wasm
```

Enjoy

<br/>

## License
This project is licensed under the terms of the MIT license. 
Check the [LICENSE](LICENSE.md) file out for license rights and limitations.
