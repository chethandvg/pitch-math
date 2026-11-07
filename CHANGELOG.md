# Changelog

All notable changes are listed here. The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and versions follow [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1] - 2026-11-07

### Fixed

- WebAssembly: `note_name`, `octave_transpose`, `harmonic` and `is_octave_harmonic` took their integer arguments as `i32`, which JavaScript truncates or wraps, so `harmonic(440, 2.5)` computed `harmonic(440, 2)` and 2^32 + 2 became 2. They now take a number and throw an `Error` unless it is a whole number of at least -2^63 and less than 2^63, which reaches the Rust core exactly. The Rust and Python APIs, the maths and the golden vectors are unchanged.

### Changed

- `pitch:INPUT-VALIDATION` 2.0.0 adds the JavaScript whole-number rule (a new major version, because the statement's meaning changed); the WebAssembly tests check it with fractions, NaN, infinities, -0 and the numbers just inside and outside the range.
- README: "What is proved" is now "Claims and evidence", and it says the golden vectors are checked for Rust and Python on Linux and Windows and for WebAssembly on Linux, as CI runs them. It also says which errors Python raises for a whole-number argument that is a float or too large an `int`. The binding READMEs say the vectors check the results in CI; the WebAssembly README no longer says its results are identical to the Rust crate's: the functions that call `exp2` or `log2` can differ from the native build in the last bit, because WebAssembly has its own math library, and match the vectors within their tolerances. The crate docs say the README lists each claim, its level and its evidence.
- Python style: a `ruff.toml` (line length 120, as in signal-core), with `vectors/generate.py` and the README's Python example formatted by it. The generated vectors are byte for byte the same.

## [0.1.0] - 2026-11-07

### Added

- Equal temperament: `midi_to_hz`, `midi_to_hz_with_reference` and `hz_to_midi` (A4 = 69 = 440 Hz).
- Intervals and notes: `cents` (built on `rhythm_units::ratio`), `note_name` (sharps, C4 = 60) and `nearest_note` (offset in [-50, 50) cents).
- Octaves and harmonics: `octave_transpose` (built on `rhythm_units::scale_frequency`), `octave_fold`, exact in floating point at the boundaries, `harmonic` and `is_octave_harmonic`.
- `PitchError` for invalid input and out-of-range results.
- Python (PyO3) and WebAssembly bindings with the same function names.
- Lean 4 proofs of eight P-level claims, including `pitch:OCTAVE-FOLD-UNIQUE` (every frequency folds into exactly one octave).
- Golden vectors (120 cases) shared by Rust, Python and WebAssembly, and property tests.
