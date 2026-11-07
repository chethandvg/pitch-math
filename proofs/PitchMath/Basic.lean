/-
SPDX-License-Identifier: Apache-2.0
SPDX-FileCopyrightText: 2026 Chethan Dandebala Venkatarama
-/
import Mathlib.Analysis.SpecialFunctions.Log.Base
import Mathlib.Analysis.SpecialFunctions.Pow.Real

/-!
# Pitch math: equal temperament and cents

Definitions mirror the Rust functions of the same names (`midi_to_hz` is
`midiToHz`, `cents` is `cents`). The theorems are about exact real numbers; the
Rust code computes the same formulas in floating point, and the golden vectors
check its rounding.
-/

namespace PitchMath

/-- Equal-tempered frequency of MIDI note `m`, with A4 = note 69 = 440 Hz. -/
noncomputable def midiToHz (m : ℝ) : ℝ := 440 * (2 : ℝ) ^ ((m - 69) / 12)

/-- The interval from frequency `f` up to frequency `g`, in cents. -/
noncomputable def cents (f g : ℝ) : ℝ := 1200 * Real.logb 2 (g / f)

/-- `pitch:ET-A4`: MIDI note 69 (A4) is 440 Hz. -/
theorem et_a4 : midiToHz 69 = 440 := by
  simp [midiToHz]

/-- `pitch:ET-OCTAVE`: twelve semitones up doubles the frequency. -/
theorem et_octave (m : ℝ) : midiToHz (m + 12) = 2 * midiToHz m := by
  have h : (m + 12 - 69) / 12 = (m - 69) / 12 + 1 := by ring
  rw [midiToHz, midiToHz, h, Real.rpow_add (by norm_num : (0 : ℝ) < 2), Real.rpow_one]
  ring

/-- `pitch:CENTS-OCTAVE`: an octave is 1200 cents. -/
theorem cents_octave {f : ℝ} (hf : 0 < f) : cents f (2 * f) = 1200 := by
  rw [cents, mul_div_assoc, div_self hf.ne', mul_one, Real.logb_self_eq_one one_lt_two, mul_one]

/-- `pitch:CENTS-ADDITIVE`: the interval from `a` to `c` is the interval from `a`
to `b` plus the interval from `b` to `c`. -/
theorem cents_additive {a b c : ℝ} (ha : 0 < a) (hb : 0 < b) (hc : 0 < c) :
    cents a c = cents a b + cents b c := by
  have h : c / a = b / a * (c / b) := by field_simp
  rw [cents, cents, cents, h, Real.logb_mul (div_pos hb ha).ne' (div_pos hc hb).ne', mul_add]

end PitchMath
