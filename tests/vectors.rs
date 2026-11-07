// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Chethan Dandebala Venkatarama

//! Golden-vector tests. The Python and WebAssembly bindings check the same
//! `vectors/*.json` cases, so all three implementations give the same answers.

use golden_vectors::{Outcome, f64_arg, i64_arg};
use pitch_math as pm;
use serde_json::{Value, json};

/// Maps each vector's `fn` name to this crate's implementation. Tuples become
/// JSON arrays, as they become lists in Python and arrays in JavaScript.
fn dispatch(name: &str, args: &[Value]) -> Outcome {
    let x = |i| f64_arg(args, i);
    let n = |i| i64_arg(args, i);
    match name {
        "midi_to_hz" => pm::midi_to_hz(x(0)).into(),
        "midi_to_hz_with_reference" => pm::midi_to_hz_with_reference(x(0), x(1)).into(),
        "hz_to_midi" => pm::hz_to_midi(x(0)).into(),
        "cents" => pm::cents(x(0), x(1)).into(),
        "note_name" => Outcome::value(pm::note_name(n(0))),
        "nearest_note" => pm::nearest_note(x(0))
            .map(|(midi, note, cents)| json!([midi, note, cents]))
            .into(),
        "octave_transpose" => pm::octave_transpose(x(0), n(1)).into(),
        "octave_fold" => pm::octave_fold(x(0), x(1))
            .map(|(k, hz)| json!([k, hz]))
            .into(),
        "harmonic" => pm::harmonic(x(0), n(1)).into(),
        "is_octave_harmonic" => pm::is_octave_harmonic(n(0)).into(),
        _ => Outcome::unknown(name, args),
    }
}

#[test]
fn golden_vectors() {
    golden_vectors::check_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/vectors"), dispatch);
}
