# A Tetris implementation in Rust
Using Rust and WASM (Web Assembly) to build a client side Tetris game that runs in a browser.

## Live Demo
- https://vincepr.github.io/rs_tetris/

## Notes while doing the Project
- install wasp-pack: `cargo install wasm-pack`
- imported librarys: 
    - wasm for rust: `cargo add wasm-bindgen`
    - rng numbers: `cargo add rand`
    - react wrapper for rust: `cargo add wasm-react`


### Making wasm ready (including rand)
- `cargo add getrandom --features js`
- `cargo add js-sys`
- compile with wasm: `wasm-pack build --target web`
- `cargo add web-sys` and include window, to get access to window (and therefor timer) 
    - web-sys gates each interface behind a cargo feature, so we have to specify what feature we want (in Cargo.toml)
```
web-sys = {version="0.3.61", features = ["Window"]}
```