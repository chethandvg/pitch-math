// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Chethan Dandebala Venkatarama

//! WebAssembly bindings for `pitch-math`. Every function keeps the Rust name, so
//! the golden vectors can call it by name; invalid input throws a JS `Error`.
//! Whole-number arguments arrive as JavaScript numbers (`f64`) and must be whole
//! numbers of at least -2^63 and less than 2^63 (the largest is 2^63 - 1024),
//! which convert to `i64` exactly; a fraction, NaN, an infinity or a number
//! outside that range throws instead of being truncated or wrapped. As for any
//! numeric argument, JavaScript first converts a value that is not a number
//! (`null` becomes 0, `true` 1), and a `BigInt` throws a `TypeError`. Integer
//! results are returned as `f64`, so JavaScript never sees a `BigInt`, and
//! tuples are returned as arrays.

use pitch_math_rs as imp;
use wasm_bindgen::prelude::*;

/// A whole-number argument `name`, given as a JavaScript number: a whole number
/// of at least -2^63 and less than 2^63 (every `i64` an `f64` can hold), converted exactly.
/// Anything else is an error in the crate's style, never a truncated integer.
fn whole(name: &str, x: f64) -> Result<i64, JsError> {
    const MIN: f64 = i64::MIN as f64; // -2^63, exact; 2^63 = -MIN is out of range
    if (MIN..-MIN).contains(&x) && x.fract() == 0.0 {
        Ok(x as i64)
    } else {
        Err(JsError::new(&format!(
            "{name} must be a whole number of at least -2^63 and less than 2^63, got {x}"
        )))
    }
}

/// The crate version.
#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_owned()
}

/// Equal-tempered frequency of MIDI note `midi`, with A4 = 69 = 440 Hz.
#[wasm_bindgen]
pub fn midi_to_hz(midi: f64) -> Result<f64, JsError> {
    Ok(imp::midi_to_hz(midi)?)
}

/// Equal-tempered frequency of MIDI note `midi` with A4 tuned to `a4_hz`.
#[wasm_bindgen]
pub fn midi_to_hz_with_reference(midi: f64, a4_hz: f64) -> Result<f64, JsError> {
    Ok(imp::midi_to_hz_with_reference(midi, a4_hz)?)
}

/// The fractional MIDI note number of `hz`: 69 + 12 · log2(hz / 440).
#[wasm_bindgen]
pub fn hz_to_midi(hz: f64) -> Result<f64, JsError> {
    Ok(imp::hz_to_midi(hz)?)
}

/// The interval from `from_hz` up to `to_hz` in cents: 1200 · log2(to / from).
#[wasm_bindgen]
pub fn cents(from_hz: f64, to_hz: f64) -> Result<f64, JsError> {
    Ok(imp::cents(from_hz, to_hz)?)
}

/// The name of MIDI note `midi`, with sharps and C4 = 60: 69 → "A4".
#[wasm_bindgen]
pub fn note_name(midi: f64) -> Result<String, JsError> {
    Ok(imp::note_name(whole("midi", midi)?))
}

/// The nearest note to `hz`: `[midi, name, cents]`, with cents in [−50, 50).
#[wasm_bindgen]
pub fn nearest_note(hz: f64) -> Result<Vec<JsValue>, JsError> {
    let (midi, name, cents) = imp::nearest_note(hz)?;
    Ok(vec![(midi as f64).into(), name.into(), cents.into()])
}

/// The frequency `n` octaves above `hz` (below it if `n < 0`): hz · 2ⁿ.
#[wasm_bindgen]
pub fn octave_transpose(hz: f64, n: f64) -> Result<f64, JsError> {
    Ok(imp::octave_transpose(hz, whole("n", n)?)?)
}

/// Folds `hz` into [low_hz, 2·low_hz): `[n, hz · 2ⁿ]` for the unique integer n.
#[wasm_bindgen]
pub fn octave_fold(hz: f64, low_hz: f64) -> Result<Vec<JsValue>, JsError> {
    let (n, folded) = imp::octave_fold(hz, low_hz)?;
    Ok(vec![(n as f64).into(), folded.into()])
}

/// The `n`-th harmonic of `f0`: n · f0, for n ≥ 1.
#[wasm_bindgen]
pub fn harmonic(f0: f64, n: f64) -> Result<f64, JsError> {
    Ok(imp::harmonic(f0, whole("n", n)?)?)
}

/// Whether the `n`-th harmonic (n ≥ 1) is an octave of the fundamental: n is a power of two.
#[wasm_bindgen]
pub fn is_octave_harmonic(n: f64) -> Result<bool, JsError> {
    Ok(imp::is_octave_harmonic(whole("n", n)?)?)
}
