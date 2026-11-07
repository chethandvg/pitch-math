// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Chethan Dandebala Venkatarama

//! Python bindings for `pitch-math`. Every function keeps the Rust name, so the
//! golden vectors can call it by name. Invalid input raises `ValueError`, and
//! tuples are returned as lists.

use pyo3::prelude::*;

#[pymodule]
mod pitch_math {
    use pitch_math_rs as imp;
    use pyo3::exceptions::PyValueError;
    use pyo3::prelude::*;
    use pyo3::types::PyList;

    fn ok<T>(r: imp::Result<T>) -> PyResult<T> {
        r.map_err(|e| PyValueError::new_err(e.to_string()))
    }

    #[pymodule_init]
    fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
        m.add("__version__", env!("CARGO_PKG_VERSION"))
    }

    /// Equal-tempered frequency of MIDI note `midi`, with A4 = 69 = 440 Hz.
    #[pyfunction]
    fn midi_to_hz(midi: f64) -> PyResult<f64> {
        ok(imp::midi_to_hz(midi))
    }

    /// Equal-tempered frequency of MIDI note `midi` with A4 tuned to `a4_hz`.
    #[pyfunction]
    fn midi_to_hz_with_reference(midi: f64, a4_hz: f64) -> PyResult<f64> {
        ok(imp::midi_to_hz_with_reference(midi, a4_hz))
    }

    /// The fractional MIDI note number of `hz`: 69 + 12 * log2(hz / 440).
    #[pyfunction]
    fn hz_to_midi(hz: f64) -> PyResult<f64> {
        ok(imp::hz_to_midi(hz))
    }

    /// The interval from `from_hz` up to `to_hz` in cents: 1200 * log2(to / from).
    #[pyfunction]
    fn cents(from_hz: f64, to_hz: f64) -> PyResult<f64> {
        ok(imp::cents(from_hz, to_hz))
    }

    /// The name of MIDI note `midi`, with sharps and C4 = 60: 69 -> "A4".
    #[pyfunction]
    fn note_name(midi: i64) -> String {
        imp::note_name(midi)
    }

    /// The nearest note to `hz`: [midi, name, cents], with cents in [-50, 50).
    #[pyfunction]
    fn nearest_note(py: Python<'_>, hz: f64) -> PyResult<Bound<'_, PyList>> {
        let (midi, name, cents) = ok(imp::nearest_note(hz))?;
        PyList::new(
            py,
            [
                midi.into_pyobject(py)?.into_any(),
                name.into_pyobject(py)?.into_any(),
                cents.into_pyobject(py)?.into_any(),
            ],
        )
    }

    /// The frequency `n` octaves above `hz` (below it if `n < 0`): hz * 2**n.
    #[pyfunction]
    fn octave_transpose(hz: f64, n: i64) -> PyResult<f64> {
        ok(imp::octave_transpose(hz, n))
    }

    /// Folds `hz` into [low_hz, 2 * low_hz): [n, hz * 2**n] for the unique integer n.
    #[pyfunction]
    fn octave_fold(py: Python<'_>, hz: f64, low_hz: f64) -> PyResult<Bound<'_, PyList>> {
        let (n, folded) = ok(imp::octave_fold(hz, low_hz))?;
        PyList::new(
            py,
            [
                n.into_pyobject(py)?.into_any(),
                folded.into_pyobject(py)?.into_any(),
            ],
        )
    }

    /// The `n`-th harmonic of `f0`: n * f0, for n >= 1.
    #[pyfunction]
    fn harmonic(f0: f64, n: i64) -> PyResult<f64> {
        ok(imp::harmonic(f0, n))
    }

    /// Whether the `n`-th harmonic (n >= 1) is an octave of the fundamental: n is a power of two.
    #[pyfunction]
    fn is_octave_harmonic(n: i64) -> PyResult<bool> {
        ok(imp::is_octave_harmonic(n))
    }
}
