# Pitch math: WebAssembly bindings

WebAssembly build of [pitch-math](https://github.com/chethandvg/pitch-math) for browsers and Node. Function names and results are identical to the Rust crate; the shared golden vectors prove it in CI.

```bash
npm install
npm run build   # pkg/web for browsers, pkg/node for Node
npm test
```

Building needs Rust with the `wasm32-unknown-unknown` target and the `wasm-bindgen` CLI at the version pinned in `Cargo.toml`.
