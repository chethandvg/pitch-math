// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Chethan Dandebala Venkatarama

//! # Pitch math
//!
//! Equal temperament, cents and octave folding, with a Lean 4 proof that every frequency folds into exactly one octave.
//!
//! Part of [Frequencies of Life](https://chethandvg.github.io/frequencies-of-life/). Each public function corresponds to
//! claims in `claims.yaml`; the README lists what is proved and how.
//!
//! Notes use 12-tone equal temperament with A4 = MIDI note 69 = 440 Hz, sharps,
//! and C4 = MIDI note 60. Moving a frequency by octaves is speeding time up by a
//! power of two, so it is built on [`rhythm_units::scale_frequency`].
//!
//! ```
//! use pitch_math::{nearest_note, octave_fold};
//!
//! // A rhythm with a 10-minute period (1/600 Hz), folded into the octave [220, 440) Hz.
//! let (n, hz) = octave_fold(1.0 / 600.0, 220.0)?;
//! assert_eq!(n, 18);
//! let (midi, name, cents) = nearest_note(hz)?;
//! assert_eq!((midi, name.as_str()), (69, "A4"));
//! assert!((cents + 12.21).abs() < 0.01); // 436.91 Hz is A4 minus 12.21 cents
//! # Ok::<(), pitch_math::PitchError>(())
//! ```
//!
//! Every frequency must be finite and greater than zero, and every result must
//! fit in an `f64`; anything else is a [`PitchError`].

use std::fmt;

/// Frequency of A4 in standard tuning, in hertz.
pub const A4_HZ: f64 = 440.0;

/// MIDI note number of A4.
pub const A4_MIDI: f64 = 69.0;

/// The twelve note names of an octave, starting at C, spelled with sharps.
pub const NOTE_NAMES: [&str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];

const FINITE: &str = "a finite number";
const POSITIVE: &str = "a finite number greater than 0";
const WHOLE: &str = "a whole number of at least 1";
const NORMAL_OCTAVE: &str =
    "between f64::MIN_POSITIVE and f64::MAX / 2, so that the octave is in the normal f64 range";

/// An argument (or the result) outside the range the function accepts.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PitchError {
    name: &'static str,
    value: f64,
    requirement: &'static str,
}

impl PitchError {
    /// The parameter (or `"result"`) that was out of range.
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// The offending value.
    pub fn value(&self) -> f64 {
        self.value
    }
}

impl fmt::Display for PitchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} must be {}, got {}",
            self.name, self.requirement, self.value
        )
    }
}

impl std::error::Error for PitchError {}

/// The result type of this crate.
pub type Result<T> = std::result::Result<T, PitchError>;

fn require(ok: bool, name: &'static str, value: f64, requirement: &'static str) -> Result<f64> {
    if ok {
        Ok(value)
    } else {
        Err(PitchError {
            name,
            value,
            requirement,
        })
    }
}

fn finite(name: &'static str, value: f64) -> Result<f64> {
    require(value.is_finite(), name, value, FINITE)
}

fn positive(name: &'static str, value: f64) -> Result<f64> {
    require(value.is_finite() && value > 0.0, name, value, POSITIVE)
}

fn harmonic_number(n: i64) -> Result<i64> {
    require(n >= 1, "n", n as f64, WHOLE).map(|_| n)
}

/// A value computed by `rhythm-units` from arguments already checked here, so
/// an error can only mean that the result left the `f64` range.
fn in_range(r: rhythm_units::Result<f64>) -> Result<f64> {
    r.map_err(|e| PitchError {
        name: "result",
        value: e.value(),
        requirement: POSITIVE,
    })
}

/// 2ⁿ, exact for every `n`: 0 below the `f64` range, infinity above it.
fn pow2(n: i64) -> f64 {
    match n {
        -1022..=1023 => f64::from_bits(((n + 1023) as u64) << 52),
        -1074..=-1023 => f64::from_bits(1 << (n + 1074)),
        ..-1074 => 0.0,
        _ => f64::INFINITY,
    }
}

/// Splits a finite `x > 0` exactly into `(m, e)` with `x = m · 2ᵉ` and `1 ≤ m < 2`,
/// so `e` is ⌊log₂ x⌋.
fn split(x: f64) -> (f64, i64) {
    const FRACTION: u64 = (1 << 52) - 1;
    let bits = x.to_bits();
    match (bits >> 52) as i64 {
        // Subnormal: scaling up by 2⁶⁴ is exact and makes it normal.
        0 => {
            let (m, e) = split(x * pow2(64));
            (m, e - 64)
        }
        biased => (
            f64::from_bits((bits & FRACTION) | (1023 << 52)),
            biased - 1023,
        ),
    }
}

/// log₂(to / from): the interval in octaves, for checked arguments.
fn octaves(from_hz: f64, to_hz: f64) -> Result<f64> {
    Ok(in_range(rhythm_units::ratio(to_hz, from_hz))?.log2())
}

/// Equal-tempered frequency of MIDI note `midi` (fractional notes allowed), with A4 = 69 = 440 Hz.
pub fn midi_to_hz(midi: f64) -> Result<f64> {
    midi_to_hz_with_reference(midi, A4_HZ)
}

/// Equal-tempered frequency of MIDI note `midi` when A4 is tuned to `a4_hz`:
/// `a4_hz · 2^((midi − 69) / 12)`.
pub fn midi_to_hz_with_reference(midi: f64, a4_hz: f64) -> Result<f64> {
    let semitones = finite("midi", midi)? - A4_MIDI;
    in_range(rhythm_units::scale_frequency(
        positive("a4_hz", a4_hz)?,
        (semitones / 12.0).exp2(),
    ))
}

/// The fractional MIDI note number of `hz`: `69 + 12 · log₂(hz / 440)`.
pub fn hz_to_midi(hz: f64) -> Result<f64> {
    Ok(A4_MIDI + 12.0 * octaves(A4_HZ, positive("hz", hz)?)?)
}

/// The interval from `from_hz` up to `to_hz` in cents: `1200 · log₂(to / from)`.
/// Negative when `to_hz` is lower.
pub fn cents(from_hz: f64, to_hz: f64) -> Result<f64> {
    Ok(1200.0 * octaves(positive("from_hz", from_hz)?, positive("to_hz", to_hz)?)?)
}

/// The name of MIDI note `midi` with sharps and octave number, C4 = 60:
/// 69 → `"A4"`, 61 → `"C#4"`, 0 → `"C-1"`.
pub fn note_name(midi: i64) -> String {
    format!(
        "{}{}",
        NOTE_NAMES[midi.rem_euclid(12) as usize],
        midi.div_euclid(12) - 1
    )
}

/// Rounds a fractional MIDI number to the nearest note, halves upwards, and
/// returns the note and the remainder, which is in [−0.5, 0.5).
fn nearest(midi: f64) -> (i64, f64) {
    let mut note = midi.round(); // rounds halves away from zero …
    if midi - note >= 0.5 {
        note += 1.0; // … so move negative halves up too
    }
    (note as i64, midi - note)
}

/// The nearest equal-tempered note to `hz`: `(midi, name, cents)`, where
/// `cents`, in [−50, 50), is how far `hz` lies above (or below) that note.
pub fn nearest_note(hz: f64) -> Result<(i64, String, f64)> {
    let (midi, remainder) = nearest(hz_to_midi(hz)?);
    Ok((midi, note_name(midi), 100.0 * remainder))
}

/// The frequency `n` octaves above `hz` (below it if `n < 0`): `hz · 2ⁿ`, which
/// is time sped up by the factor 2ⁿ. Exact unless the result is subnormal.
///
/// The factor 2ⁿ must itself be an `f64`, so `n` must be in −1074..=1023.
pub fn octave_transpose(hz: f64, n: i64) -> Result<f64> {
    in_range(rhythm_units::scale_frequency(positive("hz", hz)?, pow2(n)))
}

/// Folds `hz` into the octave `[low_hz, 2·low_hz)`: returns `(n, hz · 2ⁿ)` for the
/// unique integer `n` that lands there (proved in Lean as `pitch:OCTAVE-FOLD-UNIQUE`).
///
/// `n` comes from the exact binary logarithms (⌊log₂ low_hz⌋ − ⌊log₂ hz⌋, read
/// from the exponents) and is corrected by +1 when the significand of `hz` is
/// below that of `low_hz`. Scaling by 2ⁿ is exact in the normal range, so
/// `low_hz ≤ folded < 2·low_hz` holds exactly in floating point, at the
/// boundaries too. `low_hz` must lie between `f64::MIN_POSITIVE` and `f64::MAX / 2`.
pub fn octave_fold(hz: f64, low_hz: f64) -> Result<(i64, f64)> {
    let (m, e) = split(positive("hz", hz)?);
    let low = require(
        (f64::MIN_POSITIVE..=f64::MAX / 2.0).contains(&low_hz),
        "low_hz",
        low_hz,
        NORMAL_OCTAVE,
    )?;
    let (low_m, low_e) = split(low);
    let up = i64::from(m < low_m);
    Ok((low_e - e + up, m * pow2(low_e + up)))
}

/// The `n`-th harmonic of `f0`: `n · f0`, for `n ≥ 1` (`n = 1` is `f0` itself).
pub fn harmonic(f0: f64, n: i64) -> Result<f64> {
    let f0 = positive("f0", f0)?;
    in_range(rhythm_units::scale_frequency(
        f0,
        harmonic_number(n)? as f64,
    ))
}

/// Whether the `n`-th harmonic (`n ≥ 1`) is a whole number of octaves above the
/// fundamental, which is when `n` is a power of two (`pitch:HARMONIC-OCTAVE-IFF-POW2`).
pub fn is_octave_harmonic(n: i64) -> Result<bool> {
    Ok((harmonic_number(n)? as u64).is_power_of_two())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pow2_is_exact_across_the_whole_range() {
        assert_eq!(pow2(0), 1.0);
        assert_eq!(pow2(-1), 0.5);
        assert_eq!(pow2(1023), 2f64.powi(1023));
        assert_eq!(pow2(-1022), f64::MIN_POSITIVE);
        assert_eq!(pow2(-1074), f64::from_bits(1));
        assert_eq!((pow2(-1075), pow2(1024)), (0.0, f64::INFINITY));
    }

    #[test]
    fn split_reads_the_exact_exponent() {
        assert_eq!(split(1.0), (1.0, 0));
        assert_eq!(split(440.0), (1.71875, 8));
        assert_eq!(split(f64::from_bits(1)), (1.0, -1074));
        assert_eq!(split(f64::MAX), (2.0 - f64::EPSILON, 1023));
    }

    #[test]
    fn nearest_rounds_halves_up() {
        assert_eq!(nearest(69.5), (70, -0.5));
        assert_eq!(nearest(68.5), (69, -0.5));
        assert_eq!(nearest(-0.5), (0, -0.5));
        assert_eq!(nearest(-1.5), (-1, -0.5));
        let below_half = 0.5 - f64::EPSILON / 4.0;
        assert_eq!(nearest(below_half), (0, below_half));
        assert!(100.0 * below_half < 50.0);
    }
}
