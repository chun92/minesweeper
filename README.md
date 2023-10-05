# minesweeper

## Requirement
* Cargo (with rust)

## Build Setup
```
rustup target add wasm32-unknown-unknow
cargo install -f wasm-bindgen-cli
```

## Build
```
cargo build --release --target wasm32-unknown-unknown
/* window */
wasm-bindgen --target web .\target\wasm32-unknown-unknown\release\minesweeper.wasm --out-dir .\out\
/* linux */
wasm-bindgen --target web ./target/wasm32-unknown-unknown/release/minesweeper.wasm --out-dir ./out/
```

## Server Deploy
```
firebase login
firebase init functions
firebase deploy --only functions
```

## Config files
```
/assets/yaml/config.yaml.example => /assets/yaml/config.yaml
/functions/config.yaml.example => /functions/config.yaml
```