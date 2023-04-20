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


### Making WASM ready (including rand)
- `cargo add getrandom --features js`
- `cargo add js-sys`
- compile with wasm for local testing: `wasm-pack build --out-dir ./build/pkg --target web`
- `cargo add web-sys` and include window, to get access to window (and therefor timer) 
    - web-sys gates each interface behind a cargo feature, so we have to specify what feature we want (in Cargo.toml)
```
web-sys = {version="0.3.61", features = ["Window"]}
```

## Testing locally
- generate the WASM and JS bindings from the Rust sourcecode:`wasm-pack build --out-dir ./build/pkg --target web` 
- then host the build folder in FiveServer or nginx

## Notes on CI with github Actions
- as of this moment wasm-pack build generates a .gitignore for the folder (and no flag to turn that optional)
    - so it is necessary before `actions/upload-artifact@v3` to copy those files or delete that gitignore
- rust (and especially cargo downloads) are really slow.
    - caching the dependencies (github stores them for 7-30 days) really speeds things up
    - from:
        - Run Test : 49s
        - build website: 20s
    - to:
        - Run Test; 11s
        - build website: 6s
```yml
name: Custom Deploy to GitHub Pages

on:
  push:
    branches:
      - master
  pull_request:
    brnches:
      - master
jobs:
  build:
    name: Build for Github Pages
    runs-on: ubuntu-latest

    steps:
      - name: Checkout repo
        uses: actions/checkout@v3
      
      - name: Install
        run: curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

      ## caching for faster rust compiles ->
      - name: Cargo Cache
        uses: actions/cache@v3
        with:
          path: ~/.cargo
          key: ${{ runner.os }}-cargo-${{ hashFiles('Cargo.toml') }}
          restore-keys: |
            ${{ runner.os }}-cargo-${{ hashFiles('Cargo.toml') }}
            ${{ runner.os }}-cargo

      - name: Cargo Target Cache
        uses: actions/cache@v3
        with:
          path: target
          key: ${{ runner.os }}-cargo-target-${{ hashFiles('Cargo.toml') }}
          restore-keys: |
            ${{ runner.os }}-cargo-target-${{ hashFiles('Cargo.toml') }}
            ${{ runner.os }}-cargo-target
      ## <- caching for faster rust compiles
        
      - name: Run Tests
        run: cargo test
      # # This will not run without wasm-bindgen-test as a dependence in Cargo.toml
      # - name: Run wasm-pack tests against headless browser of choice
      #   run: wasm-pack test --headless --chrome

      - name: build website
        run: wasm-pack build --target web --out-dir ./build --release

        # current wasm-pack creates a gitignore in the out-dir (and no flag to turn that off):
      - name: clean up gitignore from wasm-pack   
        run: rm -f ./build/.gitignore

        # store our files for the next step:
      - name: Upload production-ready build files
        uses: actions/upload-artifact@v3
        with:
          name: production-files
          path: ./build
      
  deploy:
    name: Deploy to Github Pages
    needs: build
    runs-on: ubuntu-latest
    if: github.ref == 'refs/heads/master'

    steps:
      - name: Download artifact
        uses: actions/download-artifact@v3
        with:
          name: production-files
          path: ./dist
      - name: Deploy to gh-pages branch
        uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./dist
```