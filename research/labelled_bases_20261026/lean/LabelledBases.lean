import Mathlib

/-!
# The labelled-base lower bound

`research/labelled_bases_20261026/README.md`, section "Bound": in the README's cost
model, a labelled base of `B ≤ s` points over a structured set of size `s`, in a group
of order `N`, used with `m`-term decompositions, costs at least

  `C = B·N/s + (m!·N/B^m) · max(1, s^m/(m!·N))`,

and `C ≥ 2√N`. This file proves that inequality for all real `N > 0`, `0 < B ≤ s` and
`m ≥ 1`, covering both cases of the README (`s^m ≥ m!·N` and `s^m < m!·N`) at once.

What is *not* formalized: the cost model itself (that a useful label costs `N/s` draws,
and that an oracle pays at least one operation per decomposition it returns). Those are
modelling assumptions about attacks, not theorems about elliptic curves.
-/

namespace LabelledBases

/-- The README's cost: labelling plus decomposition. -/
noncomputable def cost (N s B : ℝ) (m : ℕ) : ℝ :=
  B * N / s + ((m.factorial : ℝ) * N / B ^ m) * max 1 (s ^ m / ((m.factorial : ℝ) * N))

/-- The two cost terms multiply to at least `N`. -/
theorem terms_mul_ge (N s B : ℝ) (m : ℕ) (hN : 0 < N) (hB : 0 < B) (hBs : B ≤ s)
    (hm : 1 ≤ m) :
    N ≤ (B * N / s) * (((m.factorial : ℝ) * N / B ^ m) *
      max 1 (s ^ m / ((m.factorial : ℝ) * N))) := by
  have hs : 0 < s := lt_of_lt_of_le hB hBs
  have hf : (0 : ℝ) < m.factorial := by exact_mod_cast Nat.factorial_pos m
  have hfN : 0 < (m.factorial : ℝ) * N := mul_pos hf hN
  have hBm : 0 < B ^ m := pow_pos hB m
  -- y ≥ (m!N/B^m)·(s^m/(m!N)) = s^m/B^m
  have hy : s ^ m / B ^ m ≤ ((m.factorial : ℝ) * N / B ^ m) *
      max 1 (s ^ m / ((m.factorial : ℝ) * N)) := by
    calc s ^ m / B ^ m
        = ((m.factorial : ℝ) * N / B ^ m) * (s ^ m / ((m.factorial : ℝ) * N)) := by
          field_simp
      _ ≤ ((m.factorial : ℝ) * N / B ^ m) * max 1 (s ^ m / ((m.factorial : ℝ) * N)) :=
          mul_le_mul_of_nonneg_left (le_max_right _ _) (div_nonneg hfN.le hBm.le)
  -- (s/B)^(m-1) ≥ 1
  have hratio : 1 ≤ (s / B) ^ (m - 1) := one_le_pow₀ ((one_le_div hB).2 hBs)
  have hx : 0 ≤ B * N / s := div_nonneg (mul_nonneg hB.le hN.le) hs.le
  calc N ≤ N * (s / B) ^ (m - 1) := le_mul_of_one_le_right hN.le hratio
    _ = (B * N / s) * (s ^ m / B ^ m) := by
        obtain ⟨k, rfl⟩ : ∃ k, m = k + 1 := ⟨m - 1, by omega⟩
        simp only [Nat.add_sub_cancel, div_pow, pow_succ]
        field_simp
    _ ≤ (B * N / s) * (((m.factorial : ℝ) * N / B ^ m) *
          max 1 (s ^ m / ((m.factorial : ℝ) * N))) :=
        mul_le_mul_of_nonneg_left hy hx

/-- **The labelled-base lower bound**: `cost ≥ 2√N`. -/
theorem labelled_base_lower (N s B : ℝ) (m : ℕ) (hN : 0 < N) (hB : 0 < B) (hBs : B ≤ s)
    (hm : 1 ≤ m) : 2 * Real.sqrt N ≤ cost N s B m := by
  have hs : 0 < s := lt_of_lt_of_le hB hBs
  have hf : (0 : ℝ) < m.factorial := by exact_mod_cast Nat.factorial_pos m
  set x := B * N / s with hxdef
  set y := ((m.factorial : ℝ) * N / B ^ m) * max 1 (s ^ m / ((m.factorial : ℝ) * N))
    with hydef
  have hx : 0 ≤ x := div_nonneg (mul_nonneg hB.le hN.le) hs.le
  have hy : 0 ≤ y :=
    mul_nonneg (div_nonneg (mul_nonneg hf.le hN.le) (pow_pos hB m).le)
      (le_trans zero_le_one (le_max_left _ _))
  have hxy : N ≤ x * y := terms_mul_ge N s B m hN hB hBs hm
  -- (x+y)² ≥ 4xy ≥ 4N, and x + y ≥ 0.
  have hsq : (2 * Real.sqrt N) ^ 2 ≤ (x + y) ^ 2 := by
    rw [mul_pow, Real.sq_sqrt hN.le]
    nlinarith [sq_nonneg (x - y)]
  have hcost : cost N s B m = x + y := rfl
  rw [hcost]
  exact (pow_le_pow_iff_left₀ (by positivity) (add_nonneg hx hy) two_ne_zero).1 hsq

end LabelledBases
