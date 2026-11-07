# SPDX-License-Identifier: Apache-2.0
# SPDX-FileCopyrightText: 2026 Chethan Dandebala Venkatarama
"""Write the golden vectors for pitch-math from the reference implementation.

The oracle mirrors `midi_to_hz`, `hz_to_note` and `octave_transpose` of the
project's earlier Python toolkit (`lifesound.py`), so the new code agrees with
the old. It uses exact `Fraction` arithmetic wherever the result is rational:
whole octaves, octave scaling, folding and harmonics. Only the irrational
results (fractional powers of two and logarithms) come from `math`.
"""

import json
import math
from fractions import Fraction
from pathlib import Path

HERE = Path(__file__).resolve().parent

A4_HZ, A4_MIDI = 440.0, 69
NOTE_NAMES = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"]
TEN_MINUTES = 1 / 600  # Hz: a rhythm with a 10-minute period
ONE_DAY = 1 / 86400  # Hz: a 24-hour (circadian) rhythm
EXACT = {"rel": 0, "abs": 0}


def write(name: str, description: str, cases: list[dict], rel: float = 1e-12, abs_: float = 0.0) -> None:
    doc = {"schema": "golden-vectors@1", "description": description, "tolerance": {"rel": rel, "abs": abs_}, "cases": cases}
    (HERE / f"{name}.json").write_text(json.dumps(doc, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")


# The reference implementation -------------------------------------------------


def midi_to_hz_with_reference(midi: float, a4_hz: float) -> float:
    steps = Fraction(midi) - A4_MIDI
    if steps.denominator == 1 and steps.numerator % 12 == 0:  # whole octaves: exact
        return float(Fraction(a4_hz) * Fraction(2) ** (steps.numerator // 12))
    return a4_hz * 2 ** ((midi - A4_MIDI) / 12)  # lifesound.midi_to_hz


def midi_to_hz(midi: float) -> float:
    return midi_to_hz_with_reference(midi, A4_HZ)


def hz_to_midi(hz: float) -> float:
    return A4_MIDI + 12 * math.log2(hz / A4_HZ)  # as in lifesound.hz_to_note


def cents(from_hz: float, to_hz: float) -> float:
    return 1200 * math.log2(to_hz / from_hz)


def note_name(midi: int) -> str:
    return f"{NOTE_NAMES[midi % 12]}{midi // 12 - 1}"  # as in lifesound.hz_to_note


def nearest_note(hz: float) -> list:
    m = hz_to_midi(hz)
    note = round(m)  # lifesound.hz_to_note rounds the same way, but to whole cents
    note += m - note >= 0.5  # halves go up, so the offset is in [-50, 50) cents
    return [note, note_name(note), (m - note) * 100]


def octave_transpose(hz: float, n: int) -> float:
    return float(Fraction(hz) * Fraction(2) ** n)


def octave_fold(hz: float, low_hz: float) -> list:
    """lifesound.octave_transpose (which returns (f, n)), in exact arithmetic."""
    f, low, n = Fraction(hz), Fraction(low_hz), 0
    while f >= 2 * low:
        f /= 2
        n -= 1
    while f < low:
        f *= 2
        n += 1
    return [n, float(f)]


def harmonic(f0: float, n: int) -> float:
    return float(Fraction(f0) * n)


def is_octave_harmonic(n: int) -> bool:
    return n & (n - 1) == 0  # for n >= 1


ORACLE = {f.__name__: f for f in (midi_to_hz, midi_to_hz_with_reference, hz_to_midi, cents, note_name, nearest_note,
                                  octave_transpose, octave_fold, harmonic, is_octave_harmonic)}


def case(claim: str, fn: str, *args, note: str | None = None, tolerance: dict | None = None) -> dict:
    """A case whose expected value comes from the oracle."""
    c = {"claim": claim, "fn": fn, "args": list(args), "expected": ORACLE[fn](*args)}
    if tolerance is not None:
        c["tolerance"] = tolerance
    if note:
        c["note"] = note
    return c


def error(fn: str, *args, note: str) -> dict:
    return {"claim": "pitch:INPUT-VALIDATION", "fn": fn, "args": list(args), "error": True, "note": note}


# The cases --------------------------------------------------------------------


def equal_temperament() -> list[dict]:
    low_notes = [(16, "E0", "20.60"), (21, "A0", "27.50"), (28, "E1", "41.20"), (29, "F1", "43.65")]
    return [
        case("pitch:ET-A4", "midi_to_hz", 69),
        case("pitch:ET-OCTAVE", "midi_to_hz", 81, note="A5: one octave above A4"),
        case("pitch:ET-OCTAVE", "midi_to_hz", 57, note="A3: one octave below A4"),
        case("pitch:ET-OCTAVE", "midi_to_hz", 9, note="A-1: five octaves below A4"),
        *(case("pitch:ET-LOW-NOTES", "midi_to_hz", m, note=f"{name}: {hz} Hz to two decimals")
          for m, name, hz in low_notes),
        case("pitch:ET-FORMULA", "midi_to_hz", 60, note="C4 (middle C)"),
        case("pitch:ET-FORMULA", "midi_to_hz", 0, note="C-1, the lowest MIDI note"),
        case("pitch:ET-FORMULA", "midi_to_hz", 127, note="G9, the highest MIDI note"),
        case("pitch:ET-FORMULA", "midi_to_hz", 69.5, note="a quarter tone above A4"),
        case("pitch:ET-FORMULA", "midi_to_hz", -24.25, note="fractional and below MIDI 0"),
        case("pitch:ET-A4", "midi_to_hz_with_reference", 69, 432.0),
        case("pitch:ET-OCTAVE", "midi_to_hz_with_reference", 57, 415.0),
        case("pitch:ET-FORMULA", "midi_to_hz_with_reference", 60, 442.0),
        case("pitch:ET-A4", "hz_to_midi", 440.0),
        case("pitch:ET-OCTAVE", "hz_to_midi", 880.0),
        case("pitch:ET-OCTAVE", "hz_to_midi", 27.5),
        case("pitch:ET-FORMULA", "hz_to_midi", midi_to_hz(60), note="round trip of C4"),
        case("pitch:ET-FORMULA", "hz_to_midi", 1000.0),
        case("pitch:ET-FORMULA", "hz_to_midi", 1.0),
    ]


def intervals() -> list[dict]:
    return [
        case("pitch:CENTS-OCTAVE", "cents", 440.0, 880.0),
        case("pitch:CENTS-OCTAVE", "cents", 0.1, 0.2),
        case("pitch:CENTS-OCTAVE", "cents", 440.0, 220.0, note="an octave down is -1200"),
        case("pitch:CENTS-ADDITIVE", "cents", 200.0, 300.0, note="perfect fifth 3/2 = 701.955 cents"),
        case("pitch:CENTS-ADDITIVE", "cents", 300.0, 400.0, note="perfect fourth 4/3 = 498.045 cents"),
        case("pitch:CENTS-ADDITIVE", "cents", 200.0, 400.0, note="fifth + fourth = octave"),
        case("pitch:ET-FORMULA", "cents", 440.0, midi_to_hz(70), note="one equal-tempered semitone"),
        case("pitch:ET-FORMULA", "cents", 440.0, 440.0),
    ]


def notes() -> list[dict]:
    names = [(69, "A4"), (61, "C#4"), (60, "C4"), (0, "C-1"), (11, "B-1"), (12, "C0"), (127, "G9"),
             (-1, "B-2"), (-12, "C-2"), (70, "A#4")]
    return [
        *(case("pitch:NOTE-NAMES", "note_name", m, note=name) for m, name in names),
        case("pitch:NOTE-NAMES", "nearest_note", 440.0),
        case("pitch:NOTE-NAMES", "nearest_note", midi_to_hz(60)),
        case("pitch:NOTE-NAMES", "nearest_note", 450.0, note="A4 + 38.9 cents"),
        case("pitch:NOTE-NAMES", "nearest_note", 455.0, note="A#4 - 42.0 cents: past the half-way point"),
        case("pitch:NOTE-NAMES", "nearest_note", 27.5, note="A0"),
        case("pitch:NOTE-NAMES", "nearest_note", 1.0, note="far below the piano"),
    ]


def octaves() -> list[dict]:
    largest_below = math.nextafter(440.0, 0.0)
    return [
        case("pitch:OCTAVE-TRANSPOSE-RATIOS", "octave_transpose", 440.0, 1),
        case("pitch:OCTAVE-TRANSPOSE-RATIOS", "octave_transpose", 440.0, -1),
        case("pitch:OCTAVE-TRANSPOSE-RATIOS", "octave_transpose", 440.0, 0),
        case("pitch:OCTAVE-TRANSPOSE-RATIOS", "octave_transpose", 330.0, -10),
        case("pitch:OCTAVE-TRANSPOSE-RATIOS", "octave_transpose", 1.0, 1023, note="2^1023, the largest power of two"),
        case("pitch:OCTAVE-TRANSPOSE-RATIOS", "octave_transpose", 1.0, -1074, note="2^-1074, the smallest subnormal"),
        case("pitch:OCTAVE-FOLD-EXACT", "octave_fold", 220.0, 220.0, note="lower boundary: included"),
        case("pitch:OCTAVE-FOLD-EXACT", "octave_fold", 440.0, 220.0, note="upper boundary: excluded, folds down"),
        case("pitch:OCTAVE-FOLD-EXACT", "octave_fold", largest_below, 220.0, note="largest f64 below 440"),
        case("pitch:OCTAVE-FOLD-EXACT", "octave_fold", 110.0, 220.0),
        case("pitch:OCTAVE-FOLD-EXACT", "octave_fold", math.nextafter(13.75, 0.0), 220.0,
             note="just below A-1: a float log2 gives ceil(log2(220/hz)) = 4, one octave short"),
        case("pitch:OCTAVE-FOLD-EXACT", "octave_fold", 1.0, 1.0),
        case("pitch:OCTAVE-FOLD-EXACT", "octave_fold", 2.0, 1.0),
        case("pitch:OCTAVE-FOLD-EXACT", "octave_fold", 0.5, 1.0),
        case("pitch:OCTAVE-FOLD-EXACT", "octave_fold", 1024.0, 1.0),
        case("pitch:OCTAVE-FOLD-EXACT", "octave_fold", 2.0**-40, 1.0),
        case("pitch:OCTAVE-FOLD-EXACT", "octave_fold", 3.0, 1.5, note="power-of-two multiple of a non-power-of-two"),
        case("pitch:OCTAVE-FOLD-EXACT", "octave_fold", 12.0, 1.5),
        case("pitch:OCTAVE-FOLD-EXACT", "octave_fold", 1000.0, 261.6255653005986, note="into the octave above C4"),
        case("pitch:OCTAVE-FOLD-EXACT", "octave_fold", 5e-324, 1.0, note="smallest subnormal"),
        case("pitch:OCTAVE-FOLD-EXACT", "octave_fold", 1.7976931348623157e308, 1.0, note="largest f64"),
        case("pitch:OCTAVE-FOLD-EXACT", "octave_fold", 1e-300, 1e300),
        case("pitch:OCTAVE-FOLD-EXACT", "octave_fold", 440.0, 2.2250738585072014e-308, note="smallest allowed low_hz"),
    ]


def harmonics() -> list[dict]:
    claim = "pitch:HARMONIC-OCTAVE-IFF-POW2"
    return [
        *(case(claim, "harmonic", 110.0, n) for n in (1, 2, 3, 5, 8)),
        case(claim, "harmonic", 0.1, 3, note="0.1 is not exact in binary; the product is rounded once"),
        *(case(claim, "is_octave_harmonic", n) for n in (1, 2, 3, 4, 6, 7, 8, 1000, 1024, 2**30, 2**30 + 1)),
    ]


def rhythms() -> list[dict]:
    ten, day = "pitch:TEN-MINUTE-RHYTHM-NOTE", "pitch:CIRCADIAN-RHYTHM-NOTE"
    ten_hz, day_hz = octave_transpose(TEN_MINUTES, 18), octave_transpose(ONE_DAY, 25)
    return [
        case("pitch:TEN-MINUTE-RHYTHM-PITCH", "octave_transpose", TEN_MINUTES, 18, tolerance=EXACT,
             note="1/600 Hz raised 18 octaves: 32768/75 = 436.907 Hz"),
        case(ten, "octave_fold", TEN_MINUTES, 220.0, tolerance=EXACT),
        case(ten, "nearest_note", ten_hz, note="A4 minus 12.21 cents"),
        case(ten, "cents", 440.0, ten_hz),
        case(day, "octave_transpose", ONE_DAY, 25, tolerance=EXACT, note="1/86400 Hz raised 25 octaves: 388.36 Hz"),
        case(day, "octave_fold", ONE_DAY, 220.0, tolerance=EXACT),
        case(day, "nearest_note", day_hz, note="G4 minus 16.12 cents"),
    ]


def errors() -> list[dict]:
    return [
        error("midi_to_hz", 1e6, note="2^((1e6 - 69)/12) overflows"),
        error("midi_to_hz", -1e6, note="underflows to 0"),
        error("midi_to_hz_with_reference", 69, 0.0, note="a4_hz must be > 0"),
        error("midi_to_hz_with_reference", 69, -440.0, note="a4_hz must be > 0"),
        error("hz_to_midi", 0.0, note="hz must be > 0"),
        error("hz_to_midi", -440.0, note="hz must be > 0"),
        error("cents", 0.0, 440.0, note="from_hz must be > 0"),
        error("cents", 440.0, -1.0, note="to_hz must be > 0"),
        error("cents", 1e-300, 1e300, note="the ratio overflows"),
        error("nearest_note", 0.0, note="hz must be > 0"),
        error("octave_transpose", 0.0, 1, note="hz must be > 0"),
        error("octave_transpose", -440.0, 1, note="hz must be > 0"),
        error("octave_transpose", 440.0, 2000, note="2^2000 overflows"),
        error("octave_transpose", 440.0, -2000, note="underflows to 0"),
        error("octave_transpose", 1e300, 100, note="the result overflows"),
        error("octave_transpose", 5e-324, 1074, note="2^1074 itself is not an f64, though the result would be"),
        error("octave_fold", 0.0, 220.0, note="hz must be > 0"),
        error("octave_fold", 440.0, 0.0, note="low_hz must be > 0"),
        error("octave_fold", 440.0, -220.0, note="low_hz must be > 0"),
        error("octave_fold", 440.0, 1e308, note="2 * low_hz overflows"),
        error("octave_fold", 440.0, 1e-310, note="a subnormal low_hz"),
        error("harmonic", 110.0, 0, note="harmonic numbers start at 1"),
        error("harmonic", 110.0, -1, note="harmonic numbers start at 1"),
        error("harmonic", 0.0, 2, note="f0 must be > 0"),
        error("harmonic", 1e308, 2, note="the result overflows"),
        error("is_octave_harmonic", 0, note="harmonic numbers start at 1"),
        error("is_octave_harmonic", -4, note="harmonic numbers start at 1"),
    ]


if __name__ == "__main__":
    write("equal_temperament", "midi_to_hz, midi_to_hz_with_reference and hz_to_midi (12-TET, A4 = 69 = 440 Hz).",
          equal_temperament())
    write("intervals", "cents: 1200 * log2(to / from).", intervals(), abs_=1e-12)
    write("notes", "note_name and nearest_note (sharps, C4 = 60, offsets in [-50, 50) cents).", notes(), abs_=1e-9)
    write("octaves", "octave_transpose and octave_fold, both exact in floating point.", octaves(), rel=0.0)
    write("harmonics", "harmonic and is_octave_harmonic.", harmonics(), rel=0.0)
    write("rhythms", "Biological rhythms raised by whole octaves into the audible range.", rhythms(), abs_=1e-9)
    write("errors", "Invalid input is rejected with an error in every binding.", errors())
