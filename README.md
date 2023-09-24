# minesweeper

## Build
```
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --target web .\target\wasm32-unknown-unknown\release\minesweeper.wasm --out-dir .\out\
```