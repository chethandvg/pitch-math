# Pitch math: WebAssembly bindings

WebAssembly build of [pitch-math](https://github.com/chethandvg/pitch-math) for browsers and Node. Function names and argument order match the Rust crate. `midi_to_hz`, `midi_to_hz_with_reference`, `hz_to_midi`, `cents` and `nearest_note` call `exp2` or `log2` and can differ from the native build in the last bit, because WebAssembly has its own math library; the other functions are exact and match it bit for bit. Both reproduce the shared golden vectors in CI within their tolerances.

The whole-number arguments of `note_name`, `octave_transpose`, `harmonic` and `is_octave_harmonic` are JavaScript numbers that must be whole numbers of at least -2^63 and less than 2^63, which reach the Rust core exactly. The largest is 2^63 - 1024, the last JavaScript number below 2^63, so `2 ** 63 - 1`, which rounds to 2^63, is rejected. A fraction, NaN, an infinity or a number outside that range throws an `Error` instead of being truncated or wrapped. As for any numeric argument, JavaScript first converts a value that is not a number: `null`, `""` and `[]` become 0, `true` becomes 1 and `"69"` becomes 69, while `undefined` becomes NaN and throws, and a `BigInt` throws a `TypeError`.

```bash
npm install
npm run build   # pkg/web for browsers, pkg/node for Node
npm test
```

Building needs Rust with the `wasm32-unknown-unknown` target and the `wasm-bindgen` CLI at the version pinned in `Cargo.toml`.
