
# How to deploy a Rust WebAssembly project

## Install the tools

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-server-runner
cargo install wasm-bindgen-cli
```

## Build WebAssembly

```bash
cargo build --release --target wasm32-unknown-unknown
```

## Generate JavaScript bindings

```bash
wasm-bindgen --out-dir ./webbuild/out/ --target web ./target/wasm32-unknown-unknown/release/space_invaders.wasm
cp -r assets ./webbuild/
```

## Modify index.html

In `webbuild/index.html`, replace `/out/space_invaders.js` with `/out/<PKG_NAME>.js`.

## Serve the project

```bash
npx serve webbuild  # then navigate to http://localhost:3000
```
## Package the project

```bash
zip -r webbuild.zip webbuild
```
