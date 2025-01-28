By default project will build for wayland desktop. If you would like to change
that to x11 change default features in `Cargo.toml` to `x11`.

# Debug

```sh
cargo run --features asset_processor,file_watcher
```

# Release

```sh
cargo build --release
```

# WebAssembly

```sh
cargo build --release --target wasm32-unknown-unknown --features webgl2
cp -r assets ./webbuild/
cp -r assets_imported ./webbuild/
wasm-bindgen --out-dir ./webbuild/out/ --target web ./target/wasm32-unknown-unknown/release/space_invaders.wasm
zip -r webbuild.zip webbuild
```
