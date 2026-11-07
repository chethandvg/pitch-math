# Pitch math

**Equal temperament, cents and octave folding, with a Lean 4 proof that every frequency folds into exactly one octave.**

[![CI](https://github.com/chethandvg/pitch-math/actions/workflows/ci.yml/badge.svg)](https://github.com/chethandvg/pitch-math/actions/workflows/ci.yml)
[![Proofs](https://github.com/chethandvg/pitch-math/actions/workflows/proofs.yml/badge.svg)](https://github.com/chethandvg/pitch-math/actions/workflows/proofs.yml)
[![Licence](https://img.shields.io/badge/licence-Apache--2.0-blue)](LICENSE)
[![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.24035856.svg)](https://doi.org/10.5281/zenodo.24035856)

Part of **[Frequencies of Life](https://chethandvg.github.io/frequencies-of-life/)**, open code for the rhythms of life in which every claim says how it is known: proved, verified, tested, referenced or assumed.

## Claims and evidence

<!-- claims-table:start -->
| Claim | Level | Statement | Evidence |
|---|---|---|---|
| `pitch:ET-A4` | P | In equal temperament, MIDI note 69 (A4) is 440 Hz. | [`PitchMath.et_a4`](proofs/PitchMath/Basic.lean) |
| `pitch:ET-OCTAVE` | P | Twelve equal-tempered semitones up doubles the frequency: midiToHz(m + 12) = 2 · midiToHz(m) for every real m. | [`PitchMath.et_octave`](proofs/PitchMath/Basic.lean) |
| `pitch:CENTS-OCTAVE` | P | An octave is 1200 cents: cents(f, 2f) = 1200 for every f > 0. | [`PitchMath.cents_octave`](proofs/PitchMath/Basic.lean) |
| `pitch:CENTS-ADDITIVE` | P | Cents add up: for a, b, c > 0, the interval from a to c is the interval from a to b plus the interval from b to c. | [`PitchMath.cents_additive`](proofs/PitchMath/Basic.lean) |
| `pitch:OCTAVE-FOLD-UNIQUE` | P | For every f > 0 and a > 0 there is exactly one integer n with a ≤ f·2^n < 2a, so folding a frequency into an octave is well defined. | [`PitchMath.octave_fold_unique`](proofs/PitchMath/OctaveFold.lean) |
| `pitch:OCTAVE-TRANSPOSE-RATIOS` | P | Moving two frequencies by the same number of octaves leaves their ratio unchanged. | [`PitchMath.octave_transpose_ratios`](proofs/PitchMath/Octaves.lean) |
| `pitch:HARMONIC-OCTAVE-IFF-POW2` | P | For f0 > 0, the N-th harmonic N·f0 is a whole number of octaves above f0 (2^k·f0 for some natural k) exactly when N is a power of two. | [`PitchMath.harmonic_octave_iff_pow_two`](proofs/PitchMath/Octaves.lean) |
| `pitch:TEN-MINUTE-RHYTHM-PITCH` | P | A rhythm with a 10-minute (600 s) period, raised 18 octaves, is exactly 32768/75 Hz: (1/600)·2^18 = 32768/75 ≈ 436.91 Hz. | [`PitchMath.ten_minute_rhythm_pitch`](proofs/PitchMath/Octaves.lean) |
| `pitch:ET-FORMULA` | T | midi_to_hz(m) computes a4·2^((m − 69)/12) and hz_to_midi computes 69 + 12·log2(hz/440), within 1e-12 relative of the reference implementation, and they invert each other. | [`vectors/equal_temperament.json`](vectors/equal_temperament.json), [`vectors/intervals.json`](vectors/intervals.json), [`tests/properties.rs`](tests/properties.rs) |
| `pitch:ET-LOW-NOTES` | T | With A4 = 440 Hz, the equal-tempered notes E0, A0, E1 and F1 (MIDI 16, 21, 28, 29) are 20.6017, 27.5, 41.2034 and 43.6535 Hz, which round to the two-decimal values 20.60, 27.50, 41.20 and 43.65 Hz quoted in the project's earlier draft. | [`vectors/equal_temperament.json`](vectors/equal_temperament.json) |
| `pitch:NOTE-NAMES` | T | note_name spells notes with sharps and numbers octaves with C4 = MIDI 60 (69 → A4, 61 → C#4, 0 → C-1); nearest_note returns the nearest note and an offset in [-50, 50) cents, rounding exact halves up. | [`vectors/notes.json`](vectors/notes.json), [`tests/properties.rs`](tests/properties.rs) |
| `pitch:OCTAVE-FOLD-EXACT` | T | octave_fold(hz, low) returns the unique n of pitch:OCTAVE-FOLD-UNIQUE and hz·2^n, and low ≤ hz·2^n < 2·low holds exactly in floating point, including at exact powers of two and at the ends of the f64 range. | [`vectors/octaves.json`](vectors/octaves.json), [`tests/properties.rs`](tests/properties.rs) |
| `pitch:TEN-MINUTE-RHYTHM-NOTE` | T | A 10-minute rhythm (1/600 Hz) folds into [220, 440) Hz after exactly 18 octaves, at 436.907 Hz, which is the note A4 minus 12.21 cents. | [`vectors/rhythms.json`](vectors/rhythms.json) |
| `pitch:CIRCADIAN-RHYTHM-NOTE` | T | A 24-hour rhythm (1/86400 Hz) folds into [220, 440) Hz after exactly 25 octaves, at 388.36 Hz (262144/675 Hz), which is the note G4 minus 16.12 cents. | [`vectors/rhythms.json`](vectors/rhythms.json) |
| `pitch:INPUT-VALIDATION` | T | Every function rejects frequencies that are zero, negative, NaN or infinite, harmonic numbers below 1, a fold octave outside the normal f64 range, and results that overflow or underflow; Python raises ValueError and JavaScript throws an Error. In JavaScript the whole-number arguments of note_name, octave_transpose, harmonic and is_octave_harmonic must be whole numbers of at least -2^63 and less than 2^63 (the largest is 2^63 - 1024), so a fraction, NaN, an infinity or a number outside that range throws instead of being truncated or wrapped. | [`vectors/errors.json`](vectors/errors.json), [`tests/properties.rs`](tests/properties.rs), [`bindings/wasm/tests/vectors.test.mjs`](bindings/wasm/tests/vectors.test.mjs) |
<!-- claims-table:end -->

Levels: **P** machine-checked proof (Lean 4) · **V** verified numerics · **T** tested · **R** referenced (DOI or PMID) · **M** model assumption.

## Use

Notes are 12-tone equal temperament with A4 = MIDI 69 = 440 Hz, sharps, and C4 = MIDI 60. Every frequency must be finite and greater than 0; anything else is an error.

**Rust**

```toml
[dependencies]
pitch-math = { git = "https://github.com/chethandvg/pitch-math", tag = "v0.1.1" }
```

```rust
use pitch_math::{cents, midi_to_hz, nearest_note, note_name, octave_fold};

// A rhythm with a 10-minute period (1/600 Hz), folded into the octave [220, 440) Hz.
let (n, hz) = octave_fold(1.0 / 600.0, 220.0)?; // (18, 436.9066666666667)
let (midi, name, offset) = nearest_note(hz)?;   // (69, "A4", -12.214084824648808)
assert_eq!(midi_to_hz(69.0)?, 440.0);
assert_eq!(note_name(61), "C#4");
assert_eq!(cents(440.0, 880.0)?, 1200.0);
```

**Python** (builds the Rust core with maturin)

```bash
pip install "git+https://github.com/chethandvg/pitch-math@v0.1.1#subdirectory=bindings/python"
```

```python
import pitch_math as pm

pm.octave_fold(1 / 600, 220)  # [18, 436.9066666666667]
pm.nearest_note(436.9066666666667)  # [69, 'A4', -12.214084824648808]
pm.octave_transpose(1 / 86400, 25)  # 388.36148148148146: a 24-hour rhythm, 25 octaves up
pm.is_octave_harmonic(6)  # False
pm.cents(440, 0)  # ValueError: to_hz must be a finite number greater than 0, got 0
```

**JavaScript / WebAssembly** (build steps in [bindings/wasm](bindings/wasm))

```js
import init, { octave_fold, nearest_note } from "./pkg/web/pitch_math_wasm.js";

await init();
octave_fold(1 / 600, 220);        // [18, 436.9066666666667]
nearest_note(388.36148148148146); // [67, "G4", -16.12408655542339]
```

The functions are `midi_to_hz`, `midi_to_hz_with_reference`, `hz_to_midi`, `cents`, `note_name`, `nearest_note`, `octave_transpose`, `octave_fold`, `harmonic` and `is_octave_harmonic`, with the same names in all three languages. Tuples come back as lists in Python and arrays in JavaScript; errors raise `ValueError` in Python and throw `Error` in JavaScript. The whole-number arguments of `note_name`, `octave_transpose`, `harmonic` and `is_octave_harmonic` must be Python `int`s in the 64-bit range (a float raises `TypeError` instead, and a larger `int` `OverflowError`) and, in JavaScript, whole numbers of at least -2^63 and less than 2^63 (a fraction, NaN, an infinity or a number outside that range throws instead of being truncated or wrapped).

## How it is verified

- **Proofs:** the Lean 4 files in [`proofs/`](proofs) prove the eight P-level claims about the mathematical definitions `midiToHz`, `cents`, `octaveTranspose` and `harmonic`, which mirror the Rust functions. Octave transposition and harmonics reuse `RhythmUnits.scaleFrequency` from [rhythm-units](https://github.com/chethandvg/rhythm-units), so `pitch:OCTAVE-TRANSPOSE-RATIOS` follows from its proved ratio invariance. CI rebuilds the proofs with Mathlib, rejects `sorry`, and allows only the standard axioms (`propext`, `Classical.choice`, `Quot.sound`).
- **Implementation:** one Rust implementation, compiled to Python (PyO3) and WebAssembly. The same golden vectors in [`vectors/`](vectors) are checked in all three: Rust and Python on Linux and Windows, WebAssembly on Linux. Their oracle, [`vectors/generate.py`](vectors/generate.py), mirrors the project's earlier Python toolkit and uses exact fractions wherever the answer is rational. Property tests in [`tests/properties.rs`](tests/properties.rs) add NaN and infinity, round trips and invariants over thousands of inputs.
- **Exact in floating point:** `octave_transpose`, `octave_fold` and `harmonic` are exact (or correctly rounded once), and their vectors have zero tolerance. `octave_fold` reads n from the binary exponents, so `low ≤ folded < 2·low` holds exactly, at the boundaries too.
- **What the proofs don't cover:** they are about exact real-number mathematics. The Rust code uses 64-bit floating point; `midi_to_hz`, `hz_to_midi`, `cents` and `nearest_note` call `exp2` and `log2`, whose rounding is checked by the golden-vector tolerances (1e-12 relative), not by the proofs. Note names, the nearest-note rounding, `midi_to_hz_with_reference` and input validation are tested, not proved. The proofs say nothing about biology: a 10-minute rhythm is an example input, not a claim about cells.

## Cite

If you use this work, please cite it. GitHub's **Cite this repository** button gives APA and BibTeX from [CITATION.cff](CITATION.cff).

## Licence

Code: Apache-2.0 ([LICENSE](LICENSE)). Copyright notices are in [NOTICE](NOTICE).
