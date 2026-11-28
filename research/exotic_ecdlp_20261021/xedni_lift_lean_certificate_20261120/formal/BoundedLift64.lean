import Mathlib.Tactic

/-!
The canonical rational-coordinate model used by the public-point search is
`x = a / b²`, `y = c / b³`, with positive `b`.  Reduction to `(744,244)`
modulo 1009 gives the two congruences below.  This module certifies the
complete height-64 instance for `E₉ : y² = x³ + 9x + 1`.
-/

namespace XedniBoundedLift64

private theorem x_candidates (a : ℤ) (b : ℕ)
    (ha₁ : -64 ≤ a) (ha₂ : a ≤ 64)
    (hb₁ : 1 ≤ b) (hb₂ : b ≤ 64)
    (hx : (a - 744 * (b : ℤ)^2) % 1009 = 0) :
    (a = -51 ∧ b = 2) ∨ (a = -55 ∧ b = 20) ∨
    (a = 61 ∧ b = 32) ∨ (a = -11 ∧ b = 33) := by
  interval_cases b <;> norm_num at hx ⊢ <;> omega

private theorem not_square_mod_three (c : ℤ) : c^2 % 3 ≠ 2 := by
  have hr : c % 3 = 0 ∨ c % 3 = 1 ∨ c % 3 = 2 := by omega
  rcases hr with h | h | h
  · have hc : c = 3 * (c / 3) := by omega
    rw [hc]
    ring_nf
    omega
  · have hc : c = 3 * (c / 3) + 1 := by omega
    rw [hc]
    ring_nf
    omega
  · have hc : c = 3 * (c / 3) + 2 := by omega
    rw [hc]
    ring_nf
    omega

/-- Complete bounded-height obstruction for the public point `(744,244)` on
`E₉/F₁₀₀₉`.  The hypotheses are exactly the integral canonical-coordinate
equation and both reduction congruences; no group-law or rank hypothesis is
substituted for a candidate check. -/
theorem no_canonical_lift_height64 (a c : ℤ) (b : ℕ)
    (ha₁ : -64 ≤ a) (ha₂ : a ≤ 64)
    (hb₁ : 1 ≤ b) (hb₂ : b ≤ 64)
    (hcoprime : a.natAbs.Coprime b)
    (hx : (a - 744 * (b : ℤ)^2) % 1009 = 0)
    (hy : (c - 244 * (b : ℤ)^3) % 1009 = 0)
    (hcurve : c^2 = a^3 + 9 * a * (b : ℤ)^4 + (b : ℤ)^6) : False := by
  rcases x_candidates a b ha₁ ha₂ hb₁ hb₂ hx with h | h | h | h
  · rcases h with ⟨rfl, rfl⟩
    norm_num at hcurve
    nlinarith [sq_nonneg c]
  · rcases h with ⟨rfl, rfl⟩
    norm_num at hcoprime
  · rcases h with ⟨rfl, rfl⟩
    have hm : c^2 % 3 = 2 := by omega
    exact (not_square_mod_three c hm)
  · rcases h with ⟨rfl, rfl⟩
    norm_num at hcoprime

#print axioms x_candidates
#print axioms not_square_mod_three
#print axioms no_canonical_lift_height64

end XedniBoundedLift64
