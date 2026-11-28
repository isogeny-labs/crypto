module

public import ParametricRefinement

@[expose] public section

/-!
# Conditional-label variance support lemmas

These theorems formalize the degree-sequence budget and the conversion from
the scaled collision objective to the class variance and per-class cap.  They
do not yet prove that the native first-minimum labeling makes the final
collision objective no larger than its initial conditional expectation.
-/

open scoped BigOperators

namespace Curve25519ConditionalVariance

open Curve25519ParametricRefinement

/-- Two bipartite degree sequences with total mass `M` and cap `δ` have at
most `2Mδ` ordered endpoint-pair charges.  Zero-degree entries model isolates;
degree multiplicity includes parallel ports. -/
theorem bipartite_degree_square_budget
    {α β : Type*}
    [Fintype α]
    [Fintype β]
    (leftDegree : α → ℕ)
    (rightDegree : β → ℕ)
    (M δ : ℕ)
    (hleftMass : ∑ v, leftDegree v = M)
    (hrightMass : ∑ v, rightDegree v = M)
    (hleftCap : ∀ v, leftDegree v ≤ δ)
    (hrightCap : ∀ v, rightDegree v ≤ δ) :
    (∑ v, (leftDegree v) ^ 2) + (∑ v, (rightDegree v) ^ 2) ≤ 2 * M * δ := by
  have hleftPoint : ∀ v, (leftDegree v) ^ 2 ≤ δ * leftDegree v := by
    intro v
    simpa [pow_two, mul_comm] using Nat.mul_le_mul_right (leftDegree v) (hleftCap v)
  have hrightPoint : ∀ v, (rightDegree v) ^ 2 ≤ δ * rightDegree v := by
    intro v
    simpa [pow_two, mul_comm] using Nat.mul_le_mul_right (rightDegree v) (hrightCap v)
  calc
    (∑ v, (leftDegree v) ^ 2) + (∑ v, (rightDegree v) ^ 2)
        ≤ (∑ v, δ * leftDegree v) + (∑ v, δ * rightDegree v) := by
          exact Nat.add_le_add
            (Finset.sum_le_sum fun v _ ↦ hleftPoint v)
            (Finset.sum_le_sum fun v _ ↦ hrightPoint v)
    _ = δ * M + δ * M := by
      rw [← Finset.mul_sum, ← Finset.mul_sum, hleftMass, hrightMass]
    _ = 2 * M * δ := by ring

/-- Expanding the centered class squares gives the exact collision/variance
identity for `D` classes. -/
theorem class_variance_identity
    {D : ℕ}
    (hD : 0 < D)
    (classes : Fin D → ℝ)
    (M : ℝ)
    (hmass : ∑ γ, classes γ = M) :
    (∑ γ, (classes γ - M / (D : ℝ)) ^ 2) =
      (∑ γ, (classes γ) ^ 2) - M ^ 2 / (D : ℝ) := by
  have hDReal : (D : ℝ) ≠ 0 := by exact_mod_cast (Nat.ne_of_gt hD)
  simp_rw [sub_sq]
  rw [Finset.sum_add_distrib, Finset.sum_sub_distrib]
  have hcross :
      (∑ γ, 2 * classes γ * (M / (D : ℝ))) = 2 * (M / (D : ℝ)) * M := by
    calc
      (∑ γ, 2 * classes γ * (M / (D : ℝ))) =
          2 * (M / (D : ℝ)) * (∑ γ, classes γ) := by
            rw [Finset.mul_sum]
            apply Finset.sum_congr rfl
            intro γ _
            ring
      _ = 2 * (M / (D : ℝ)) * M := by rw [hmass]
  rw [hcross]
  simp only [Finset.sum_const, Finset.card_univ, Fintype.card_fin]
  simp only [nsmul_eq_mul]
  field_simp
  ring

/-- Dividing the scaled collision-objective inequality by positive `D`
produces the centered class-variance bound. -/
theorem variance_bound_of_scaled_objective
    {D : ℕ}
    (hD : 0 < D)
    (classes : Fin D → ℝ)
    (M δ : ℝ)
    (hmass : ∑ γ, classes γ = M)
    (hobjective :
      (D : ℝ) * (∑ γ, (classes γ) ^ 2) - M ^ 2 ≤
        2 * M * δ * (D : ℝ)) :
    (∑ γ, (classes γ - M / (D : ℝ)) ^ 2) ≤ 2 * M * δ := by
  rw [class_variance_identity hD classes M hmass]
  have hDPos : (0 : ℝ) < D := by exact_mod_cast hD
  apply le_of_mul_le_mul_left ?_ hDPos
  calc
    (D : ℝ) * ((∑ γ, (classes γ) ^ 2) - M ^ 2 / (D : ℝ)) =
        (D : ℝ) * (∑ γ, (classes γ) ^ 2) - M ^ 2 := by
          field_simp
    _ ≤ 2 * M * δ * (D : ℝ) := hobjective
    _ = (D : ℝ) * (2 * M * δ) := by ring

/-- The scaled objective, nonnegative source size, and strongest source guard
force every individual class below the `2M/D` cap. -/
theorem class_cap_of_scaled_objective
    {D : ℕ}
    (hD : 0 < D)
    (classes : Fin D → ℝ)
    (M δ : ℝ)
    (hM : 0 ≤ M)
    (hmass : ∑ γ, classes γ = M)
    (hobjective :
      (D : ℝ) * (∑ γ, (classes γ) ^ 2) - M ^ 2 ≤
        2 * M * δ * (D : ℝ))
    (hguard : 2 * (D : ℝ) ^ 2 * δ ≤ M) :
    ∀ γ, classes γ * (D : ℝ) ≤ 2 * M := by
  intro γ
  have hvariance := variance_bound_of_scaled_objective hD classes M δ hmass hobjective
  have hterm :
      (classes γ - M / (D : ℝ)) ^ 2 ≤
        ∑ η, (classes η - M / (D : ℝ)) ^ 2 := by
    exact Finset.single_le_sum (fun η _ ↦ sq_nonneg (classes η - M / (D : ℝ)))
      (Finset.mem_univ γ)
  have hDNonneg : 0 ≤ (D : ℝ) ^ 2 := sq_nonneg _
  have hscaled := mul_le_mul_of_nonneg_left (hterm.trans hvariance) hDNonneg
  have hDReal : (D : ℝ) ≠ 0 := by exact_mod_cast (Nat.ne_of_gt hD)
  have hclassVariance :
      (classes γ * (D : ℝ) - M) ^ 2 ≤
        2 * M * δ * (D : ℝ) ^ 2 := by
    calc
      (classes γ * (D : ℝ) - M) ^ 2 =
          (D : ℝ) ^ 2 * (classes γ - M / (D : ℝ)) ^ 2 := by
            field_simp
      _ ≤ (D : ℝ) ^ 2 * (2 * M * δ) := hscaled
      _ = 2 * M * δ * (D : ℝ) ^ 2 := by ring
  exact class_cap_of_scaled_variance M (D : ℝ) δ (classes γ) hM hclassVariance hguard

end Curve25519ConditionalVariance
