# Changelog

All notable changes are listed here. The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and versions follow [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-11-07

### Added

- Equal temperament: `midi_to_hz`, `midi_to_hz_with_reference` and `hz_to_midi` (A4 = 69 = 440 Hz).
- Intervals and notes: `cents` (built on `rhythm_units::ratio`), `note_name` (sharps, C4 = 60) and `nearest_note` (offset in [-50, 50) cents).
- Octaves and harmonics: `octave_transpose` (built on `rhythm_units::scale_frequency`), `octave_fold`, exact in floating point at the boundaries, `harmonic` and `is_octave_harmonic`.
- `PitchError` for invalid input and out-of-range results.
- Python (PyO3) and WebAssembly bindings with the same function names.
- Lean 4 proofs of eight P-level claims, including `pitch:OCTAVE-FOLD-UNIQUE` (every frequency folds into exactly one octave).
- Golden vectors (120 cases) shared by Rust, Python and WebAssembly, and property tests.
