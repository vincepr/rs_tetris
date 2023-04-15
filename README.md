# A Tetris implementation in Rust
Using Rust and WASM (Web Assembly) to build a client side Tetris game that runs in a browser.

Goal of this project is mainly to get familiar with rust.

## Notes while doing the Project
- imported librarys: 
    - wasm for rust: `cargo add wasm-bindgen`
    - rng numbers: `cargo add rand`
    - react wrapper for rust: `cargo add wasm-react`

### Making wasm ready (including rand)
`cargo add getrandom --features js`