import Mathlib

/-!
# From a height ratio to an exact packing base

The JKSST geometry-of-numbers argument uses the norm `sqrt(hhat)` rather than
the quadratic canonical height itself.  This file records the elementary but
sign-sensitive bridge from a height ratio `M / m ≤ K` to the separation ratio
needed by the finite packing theorem.
-/

namespace JKSSTHeightToPacking

/-- Taking square roots converts a canonical-height ratio into the
corresponding norm ratio. -/
theorem sqrt_ratio_le_sqrt
    {M m K : ℝ} (M_nonneg : 0 ≤ M) (_m_pos : 0 < m)
    (ratio_bound : M / m ≤ K) :
    Real.sqrt M / Real.sqrt m ≤ Real.sqrt K := by
  calc
    Real.sqrt M / Real.sqrt m = Real.sqrt (M / m) := by
      symm
      exact Real.sqrt_div M_nonneg m
    _ ≤ Real.sqrt K := Real.sqrt_le_sqrt ratio_bound

/-- If a coefficient-`C` partial-sum ball has radius at most
`r*C*sqrt(M)`, then its packing base is at most
`C * (1 + 2*r*sqrt(K))` whenever `M/m ≤ K`.

The harmless-looking `1 ≤ C` hypothesis is exactly what absorbs the additive
`1` in the packing estimate; it is made explicit rather than hidden in an
asymptotic constant.
-/
theorem packing_base_le_scaled
    {M m K R : ℝ} (r C : ℕ)
    (M_nonneg : 0 ≤ M) (m_pos : 0 < m)
    (ratio_bound : M / m ≤ K)
    (radius_bound : R ≤ (r : ℝ) * (C : ℝ) * Real.sqrt M)
    (C_pos : 1 ≤ C) :
    1 + 2 * R / Real.sqrt m ≤
      (C : ℝ) * (1 + 2 * (r : ℝ) * Real.sqrt K) := by
  have sqrt_m_pos : 0 < Real.sqrt m := Real.sqrt_pos.2 m_pos
  have sqrt_ratio := sqrt_ratio_le_sqrt M_nonneg m_pos ratio_bound
  have rc_nonneg : 0 ≤ (r : ℝ) * (C : ℝ) := by positivity
  have scaled_ratio :
      (r : ℝ) * (C : ℝ) *
          (Real.sqrt M / Real.sqrt m) ≤
        (r : ℝ) * (C : ℝ) * Real.sqrt K :=
    mul_le_mul_of_nonneg_left sqrt_ratio rc_nonneg
  have radius_ratio :
      R / Real.sqrt m ≤
        (r : ℝ) * (C : ℝ) * Real.sqrt K := by
    calc
      R / Real.sqrt m ≤
          ((r : ℝ) * (C : ℝ) * Real.sqrt M) / Real.sqrt m :=
        (div_le_div_iff_of_pos_right sqrt_m_pos).2 radius_bound
      _ = (r : ℝ) * (C : ℝ) *
          (Real.sqrt M / Real.sqrt m) := by ring
      _ ≤ (r : ℝ) * (C : ℝ) * Real.sqrt K := scaled_ratio
  have C_pos_real : (1 : ℝ) ≤ (C : ℝ) := by exact_mod_cast C_pos
  calc
    1 + 2 * R / Real.sqrt m =
        1 + 2 * (R / Real.sqrt m) := by ring
    _ ≤ 1 + 2 * ((r : ℝ) * (C : ℝ) * Real.sqrt K) := by
      gcongr
    _ ≤ (C : ℝ) * (1 + 2 * (r : ℝ) * Real.sqrt K) := by
      have sqrt_K_nonneg : 0 ≤ Real.sqrt K := Real.sqrt_nonneg K
      nlinarith

/-- A completely explicit natural cutoff for the finite packing theorem.
Taking

`A = ceil(1 + 2*r*sqrt(K))`

makes the real packing base at most the natural product `A*C`. -/
theorem packing_base_le_natCeil_mul
    {M m K R : ℝ} (r C : ℕ)
    (M_nonneg : 0 ≤ M) (m_pos : 0 < m)
    (ratio_bound : M / m ≤ K)
    (radius_bound : R ≤ (r : ℝ) * (C : ℝ) * Real.sqrt M)
    (C_pos : 1 ≤ C) :
    let A := Nat.ceil (1 + 2 * (r : ℝ) * Real.sqrt K)
    1 + 2 * R / Real.sqrt m ≤ ((A * C : ℕ) : ℝ) := by
  dsimp
  let base : ℝ := 1 + 2 * (r : ℝ) * Real.sqrt K
  let A : ℕ := Nat.ceil base
  have packing_le :
      1 + 2 * R / Real.sqrt m ≤ (C : ℝ) * base := by
    simpa [base] using packing_base_le_scaled r C M_nonneg m_pos
      ratio_bound radius_bound C_pos
  have base_le_A : base ≤ (A : ℝ) := by
    exact Nat.le_ceil base
  have scaled_le : (C : ℝ) * base ≤ (C : ℝ) * (A : ℝ) :=
    mul_le_mul_of_nonneg_left base_le_A (Nat.cast_nonneg C)
  calc
    1 + 2 * R / Real.sqrt m ≤ (C : ℝ) * base := packing_le
    _ ≤ (C : ℝ) * (A : ℝ) := scaled_le
    _ = ((A * C : ℕ) : ℝ) := by push_cast; ring

/-- Existential form of `packing_base_le_natCeil_mul`, convenient when a
caller only needs some certified natural packing cutoff. -/
theorem exists_natural_packing_cutoff
    {M m K R : ℝ} (r C : ℕ)
    (M_nonneg : 0 ≤ M) (m_pos : 0 < m)
    (ratio_bound : M / m ≤ K)
    (radius_bound : R ≤ (r : ℝ) * (C : ℝ) * Real.sqrt M)
    (C_pos : 1 ≤ C) :
    ∃ A : ℕ, 1 + 2 * R / Real.sqrt m ≤ ((A * C : ℕ) : ℝ) := by
  refine ⟨Nat.ceil (1 + 2 * (r : ℝ) * Real.sqrt K), ?_⟩
  exact packing_base_le_natCeil_mul r C M_nonneg m_pos ratio_bound
    radius_bound C_pos

end JKSSTHeightToPacking
