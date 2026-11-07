/-
SPDX-License-Identifier: Apache-2.0
SPDX-FileCopyrightText: 2026 Chethan Dandebala Venkatarama
-/
import Mathlib.Algebra.Order.Archimedean.Basic
import Mathlib.Algebra.Order.Archimedean.Real.Basic
import Mathlib.Tactic.Linarith

/-!
# Octave folding is unique

Rust's `octave_fold(hz, low_hz)` returns the number of octaves `n` that moves
`hz` into `[low_hz, 2·low_hz)`. This file proves that such an `n` exists and is
unique, so the function is well defined.
-/

namespace PitchMath

/-- `pitch:OCTAVE-FOLD-UNIQUE`: for `f, a > 0` there is exactly one integer `n`
with `a ≤ f·2ⁿ < 2a`. -/
theorem octave_fold_unique {f a : ℝ} (hf : 0 < f) (ha : 0 < a) :
    ∃! n : ℤ, a ≤ f * 2 ^ n ∧ f * 2 ^ n < 2 * a := by
  -- If `n` reaches the octave, any `m > n` overshoots it: f·2^m ≥ f·2^(n+1) ≥ 2a.
  have le_of_fold : ∀ n m : ℤ, a ≤ f * 2 ^ n → f * 2 ^ m < 2 * a → m ≤ n := by
    intro n m hn hm
    by_contra h
    have h2 : (2 : ℝ) ^ (n + 1) ≤ 2 ^ m := zpow_le_zpow_right₀ one_le_two (by omega)
    rw [zpow_add_one₀ two_ne_zero] at h2
    linarith [mul_le_mul_of_nonneg_left h2 hf.le]
  -- Existence: a / f lies in (2^k, 2^(k+1)] for some k, so n = k + 1 works.
  obtain ⟨k, hk₁, hk₂⟩ := exists_mem_Ioc_zpow (div_pos ha hf) one_lt_two
  have low : a ≤ f * 2 ^ (k + 1) := by
    rw [mul_comm]
    exact (div_le_iff₀ hf).1 hk₂
  have high : f * 2 ^ (k + 1) < 2 * a := by
    have := (lt_div_iff₀ hf).1 hk₁
    rw [zpow_add_one₀ two_ne_zero]
    linarith
  exact ⟨k + 1, ⟨low, high⟩, fun m hm => le_antisymm (le_of_fold _ _ low hm.2) (le_of_fold _ _ hm.1 high)⟩

end PitchMath
