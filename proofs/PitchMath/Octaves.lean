/-
SPDX-License-Identifier: Apache-2.0
SPDX-FileCopyrightText: 2026 Chethan Dandebala Venkatarama
-/
import RhythmUnits.Basic

/-!
# Octave transposition and harmonics

Both are built on `RhythmUnits.scaleFrequency`, as in Rust: moving `n` octaves is
speeding time up by `2ⁿ`, and the `n`-th harmonic is speeding it up by `n`.
-/

namespace PitchMath

open RhythmUnits

/-- Rust `octave_transpose(hz, n)`: the frequency `n` octaves up (down if `n < 0`). -/
noncomputable def octaveTranspose (f : ℝ) (n : ℤ) : ℝ := scaleFrequency f (2 ^ n)

/-- Rust `harmonic(f0, n)`: the `n`-th harmonic of `f0`. -/
def harmonic (f0 : ℝ) (n : ℕ) : ℝ := scaleFrequency f0 n

/-- `pitch:OCTAVE-TRANSPOSE-RATIOS`: moving two frequencies by the same number of
octaves keeps their ratio. This is `rhythm:SCALE-RATIO-INVARIANT` with `k = 2ⁿ`. -/
theorem octave_transpose_ratios (f₁ f₂ : ℝ) (n : ℤ) :
    ratio (octaveTranspose f₁ n) (octaveTranspose f₂ n) = ratio f₁ f₂ :=
  scale_ratio_invariant (zpow_ne_zero n two_ne_zero)

/-- `pitch:HARMONIC-OCTAVE-IFF-POW2`: for `f0 > 0`, the `N`-th harmonic is an
octave of `f0` (`2ᵏ·f0` for some `k`) exactly when `N` is a power of two. -/
theorem harmonic_octave_iff_pow_two {f0 : ℝ} (hf0 : 0 < f0) (N : ℕ) :
    (∃ k : ℕ, harmonic f0 N = 2 ^ k * f0) ↔ ∃ k : ℕ, N = 2 ^ k := by
  simp only [harmonic, scaleFrequency, mul_comm f0, mul_left_inj' hf0.ne']
  constructor <;> rintro ⟨k, hk⟩ <;> exact ⟨k, by exact_mod_cast hk⟩

/-- `pitch:TEN-MINUTE-RHYTHM-PITCH`: a rhythm with a 10-minute (600 s) period,
raised 18 octaves, is exactly `32768/75` Hz (436.91 Hz); that is,
`(1/600) · 2¹⁸ = 32768/75`. -/
theorem ten_minute_rhythm_pitch : octaveTranspose (hzFromPeriod 600) 18 = 32768 / 75 := by
  norm_num [octaveTranspose, hzFromPeriod, scaleFrequency]

end PitchMath
