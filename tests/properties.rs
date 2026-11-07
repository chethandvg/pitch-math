// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Chethan Dandebala Venkatarama

//! Property tests over many inputs, for what single golden vectors cannot show:
//! NaN and infinity (which JSON cannot hold), round trips and invariants.

use pitch_math::*;

/// Frequencies from 1e-6 Hz to 1e6 Hz, spread evenly on a log scale.
fn frequencies() -> impl Iterator<Item = f64> {
    (-600..=600).map(|i| 10f64.powf(f64::from(i) / 100.0))
}

#[test]
fn rejects_nan_and_infinity_everywhere() {
    for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(midi_to_hz(bad).is_err());
        assert!(midi_to_hz_with_reference(69.0, bad).is_err());
        assert!(hz_to_midi(bad).is_err());
        assert!(cents(bad, 440.0).is_err() && cents(440.0, bad).is_err());
        assert!(nearest_note(bad).is_err());
        assert!(octave_transpose(bad, 1).is_err());
        assert!(octave_fold(bad, 220.0).is_err() && octave_fold(440.0, bad).is_err());
        assert!(harmonic(bad, 2).is_err());
    }
}

#[test]
fn errors_name_the_argument() {
    let e = octave_fold(440.0, -1.0).unwrap_err();
    assert_eq!((e.name(), e.value()), ("low_hz", -1.0));
    assert_eq!(
        cents(0.0, 440.0).unwrap_err().to_string(),
        "from_hz must be a finite number greater than 0, got 0"
    );
    assert_eq!(octave_transpose(1e300, 100).unwrap_err().name(), "result");
    assert_eq!(harmonic(1.0, 0).unwrap_err().name(), "n");
}

#[test]
fn midi_and_hz_round_trip() {
    for m in (-120..=250).map(|i| f64::from(i) * 0.5) {
        let back = hz_to_midi(midi_to_hz(m).unwrap()).unwrap();
        assert!((back - m).abs() < 1e-12, "{m} -> {back}");
    }
}

#[test]
fn cents_add_up_and_octaves_are_1200() {
    for (a, b) in frequencies().zip(frequencies().skip(37)) {
        let c = 3.7 * b;
        let sum = cents(a, b).unwrap() + cents(b, c).unwrap();
        assert!((cents(a, c).unwrap() - sum).abs() < 1e-9);
        assert!((cents(a, 2.0 * a).unwrap() - 1200.0).abs() < 1e-9);
    }
}

#[test]
fn octave_fold_lands_in_the_octave_exactly() {
    for low in [1.0, 220.0, 261.6255653005986, 0.37] {
        for hz in frequencies() {
            let (n, folded) = octave_fold(hz, low).unwrap();
            assert!(low <= folded && folded < 2.0 * low, "{hz} into {low}");
            assert_eq!(folded, octave_transpose(hz, n).unwrap());
        }
    }
}

#[test]
fn nearest_note_offsets_stay_in_range() {
    for hz in frequencies() {
        let (midi, name, offset) = nearest_note(hz).unwrap();
        assert!((-50.0..50.0).contains(&offset), "{hz}: {offset}");
        assert_eq!(name, note_name(midi));
        let back = cents(midi_to_hz(midi as f64).unwrap(), hz).unwrap();
        assert!((back - offset).abs() < 1e-8, "{hz}: {back} vs {offset}");
    }
}

#[test]
fn octave_harmonics_are_powers_of_two() {
    for n in 1..=4096 {
        let octave = (0..13).any(|k| n == 1 << k);
        assert_eq!(is_octave_harmonic(n).unwrap(), octave, "{n}");
        assert_eq!(harmonic(1.0, n).unwrap(), n as f64);
    }
}
