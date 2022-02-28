# bevy-jam-1

## Building for the web
To build for the web, you first need to install the following prerequisistes:
```
rustup target add wasm32-unknown-unknown
cargo install -f wasm-bindgen-cli
```

Then, to make a release build, run the following commands:
```
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/bevy-jam-1.wasm
```

You can test the output by running `npx serve .` (requires you to have node.js installed).
