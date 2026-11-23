import JKSSTFiniteHeightRepair
import JKSSTHeightToPacking

/-!
# The repaired conditional bounded-relation core

This module composes the height-ratio, square-root, finite packing, torsion
fiber, natural-ceiling, and pigeonhole steps.  It is deliberately conditional
on a discrete group/projection satisfying the displayed Mordell--Weil-style
hypotheses; it does not assert the unresolved JKSST height/discriminant or
Lang inputs.
-/

open scoped BigOperators

namespace JKSSTConditionalHeightCore

/-- Natural packing cutoff obtained from the height ratio `K`. -/
noncomputable def packingCutoff (r : ℕ) (K : ℝ) : ℕ :=
  Nat.ceil (1 + 2 * (r : ℝ) * Real.sqrt K)

/-- Explicit coefficient radius after including a fiber/torsion bound `T`. -/
noncomputable def coefficientCutoff (T k r : ℕ) (K : ℝ) : ℕ :=
  T * packingCutoff r K ^ k

variable {G E ι : Type*} [AddCommGroup G] [Fintype ι]
  [NormedAddCommGroup E] [NormedSpace ℝ E] [FiniteDimensional ℝ E]

/-- A finite, explicit replacement for the conditional core of JKSST
Lemma 4.1.

The conclusion follows from a certified ratio `M/m ≤ K`; no discriminant
conjecture is hidden.  `π`, `fiberBound`, and `separated` expose the exact
Mordell--Weil quotient/torsion obligations. -/
theorem bounded_relation_of_certified_height_ratio
    (P : ι → G) (T : ℕ) (T_pos : 0 < T)
    (rank_gap : Module.finrank ℝ E + 1 ≤ Fintype.card ι)
    (M m K R : ℝ) (M_nonneg : 0 ≤ M) (m_pos : 0 < m)
    (ratio_bound : M / m ≤ K) (hR : 0 ≤ R)
    (radius_bound : R ≤
      (Fintype.card ι : ℝ) *
        (coefficientCutoff T (Module.finrank ℝ E) (Fintype.card ι) K : ℕ) *
        Real.sqrt M)
    (S : Finset G) (s : Finset E) (π : G →+ E)
    (range_mem : ∀ c : ι → Fin
        (coefficientCutoff T (Module.finrank ℝ E) (Fintype.card ι) K + 1),
      (∑ i, (c i : ℤ) • P i) ∈ S)
    (mapsTo : ∀ x ∈ S, π x ∈ s)
    (fiberBound : ∀ y ∈ s, {x : G | x ∈ S ∧ π x = y}.ncard ≤ T)
    (inside : ∀ y ∈ s, ‖y‖ ≤ R)
    (separated : ∀ y ∈ s, ∀ z ∈ s, y ≠ z →
      Real.sqrt m ≤ ‖y - z‖) :
    ∃ n : ι → ℤ,
      n ≠ 0 ∧
      (∀ i, |n i| ≤
        coefficientCutoff T (Module.finrank ℝ E) (Fintype.card ι) K) ∧
      ∑ i, n i • P i = 0 := by
  let r := Fintype.card ι
  let k := Module.finrank ℝ E
  let A := packingCutoff r K
  let C := coefficientCutoff T k r K
  have base_one_le : (1 : ℝ) ≤
      1 + 2 * (r : ℝ) * Real.sqrt K := by
    have : 0 ≤ 2 * (r : ℝ) * Real.sqrt K := by positivity
    linarith
  have one_le_A_real : (1 : ℝ) ≤ (A : ℝ) := by
    exact base_one_le.trans (Nat.le_ceil _)
  have A_pos : 0 < A := by
    exact_mod_cast (lt_of_lt_of_le (zero_lt_one : (0 : ℝ) < 1) one_le_A_real)
  have C_pos : 1 ≤ C := by
    have power_pos : 0 < A ^ k := pow_pos A_pos _
    dsimp [C, coefficientCutoff]
    exact Nat.one_le_iff_ne_zero.mpr
      (mul_ne_zero (Nat.ne_of_gt T_pos) (Nat.ne_of_gt power_pos))
  have exact_base : 1 + 2 * R / Real.sqrt m ≤ ((A * C : ℕ) : ℝ) := by
    simpa [A, C, r, k, packingCutoff, coefficientCutoff] using
      (JKSSTHeightToPacking.packing_base_le_natCeil_mul
        (Fintype.card ι)
        (coefficientCutoff T (Module.finrank ℝ E) (Fintype.card ι) K)
        M_nonneg m_pos ratio_bound radius_bound C_pos)
  apply JKSSTFiniteHeightRepair.bounded_relation_from_exact_packing
    P T A rank_gap S s π hR (Real.sqrt_pos.2 m_pos)
  · simpa [A, C, r, k, coefficientCutoff] using range_mem
  · exact mapsTo
  · exact fiberBound
  · exact inside
  · exact separated
  · simpa [A, C, r, k, coefficientCutoff] using exact_base

end JKSSTConditionalHeightCore
