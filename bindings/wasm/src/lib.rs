// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Chethan Dandebala Venkatarama

//! WebAssembly bindings for `pitch-math`. Every function keeps the Rust name, so
//! the golden vectors can call it by name; invalid input throws a JS `Error`.
//! Integers cross the boundary as `i32` or `f64`, so JavaScript never sees a
//! `BigInt`, and tuples are returned as arrays.

use pitch_math_rs as imp;
use wasm_bindgen::prelude::*;

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
pub fn note_name(midi: i32) -> String {
    imp::note_name(midi.into())
}

/// The nearest note to `hz`: `[midi, name, cents]`, with cents in [−50, 50).
#[wasm_bindgen]
pub fn nearest_note(hz: f64) -> Result<Vec<JsValue>, JsError> {
    let (midi, name, cents) = imp::nearest_note(hz)?;
    Ok(vec![(midi as f64).into(), name.into(), cents.into()])
}

/// The frequency `n` octaves above `hz` (below it if `n < 0`): hz · 2ⁿ.
#[wasm_bindgen]
pub fn octave_transpose(hz: f64, n: i32) -> Result<f64, JsError> {
    Ok(imp::octave_transpose(hz, n.into())?)
}

/// Folds `hz` into [low_hz, 2·low_hz): `[n, hz · 2ⁿ]` for the unique integer n.
#[wasm_bindgen]
pub fn octave_fold(hz: f64, low_hz: f64) -> Result<Vec<JsValue>, JsError> {
    let (n, folded) = imp::octave_fold(hz, low_hz)?;
    Ok(vec![(n as f64).into(), folded.into()])
}

/// The `n`-th harmonic of `f0`: n · f0, for n ≥ 1.
#[wasm_bindgen]
pub fn harmonic(f0: f64, n: i32) -> Result<f64, JsError> {
    Ok(imp::harmonic(f0, n.into())?)
}

/// Whether the `n`-th harmonic (n ≥ 1) is an octave of the fundamental: n is a power of two.
#[wasm_bindgen]
pub fn is_octave_harmonic(n: i32) -> Result<bool, JsError> {
    Ok(imp::is_octave_harmonic(n.into())?)
}
