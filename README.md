# WASM rust the ripper 
Password cracker made in WASM Rust, inspired by the popular John the Ripper.
The app uses a collection of password dictionaries and different algorithms to perform a brute attack in order to get the ciphered password.

![UI](doc/ui.png)

## Available algorithms

1. Md4
2. Md5
3. Sha1
4. Sha256
5. Base64

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

Enjoy

## License
This project is licensed under the terms of the MIT license. 
Check the [LICENSE](LICENSE.md) file out for license rights and limitations.