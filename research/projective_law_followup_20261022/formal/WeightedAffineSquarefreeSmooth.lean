import WeightedJacobianIdeal
import Mathlib.Algebra.MvPolynomial.PDeriv

set_option autoImplicit false
noncomputable section
namespace WeightedAffineSquarefreeSmooth

open MvPolynomial
variable {K : Type*} [Field K]

private def affDeriv (a3 a2 a1 a0 : K) (i : Fin 2) :
    Common22SeparateChartMap.AffineRing a3 a2 a1 a0 :=
  Ideal.Quotient.mk (Common22SeparateChartMap.affineIdeal a3 a2 a1 a0)
    ((Common22SeparateChartMap.affineEquation a3 a2 a1 a0).pderiv i)

/-- The Jacobian-span bridge isolates the two derivative computations from the
squarefree certificate. -/
theorem affine_deriv_span_top_of_ids [PerfectField K]
    (a3 a2 a1 a0 : K) (h2 : (2 : K) ≠ 0)
    (hsq : Squarefree (WeightedProjSmoothPoints.quintic a3 a2 a1 a0))
    (h0 : affDeriv a3 a2 a1 a0 0 =
      -WeightedJacobianIdeal.affAeval a3 a2 a1 a0
        (WeightedProjSmoothPoints.quintic a3 a2 a1 a0).derivative)
    (h1 : affDeriv a3 a2 a1 a0 1 =
      (2 : Common22SeparateChartMap.AffineRing a3 a2 a1 a0) *
        Common22SeparateChartMap.ybar a3 a2 a1 a0) :
    Ideal.span (Set.range (affDeriv a3 a2 a1 a0)) = ⊤ := by
  let J := Ideal.span (Set.range (affDeriv a3 a2 a1 a0))
  have hz : affDeriv a3 a2 a1 a0 0 ∈ J :=
    Ideal.subset_span ⟨0, rfl⟩
  have hy : affDeriv a3 a2 a1 a0 1 ∈ J :=
    Ideal.subset_span ⟨1, rfl⟩
  rw [h0] at hz
  rw [h1] at hy
  have hp : WeightedJacobianIdeal.affAeval a3 a2 a1 a0
      (WeightedProjSmoothPoints.quintic a3 a2 a1 a0).derivative ∈ J := by
    simpa using J.neg_mem hz
  have hle : Ideal.span
      ({(2 : Common22SeparateChartMap.AffineRing a3 a2 a1 a0) *
          Common22SeparateChartMap.ybar a3 a2 a1 a0,
        WeightedJacobianIdeal.affAeval a3 a2 a1 a0
          (WeightedProjSmoothPoints.quintic a3 a2 a1 a0).derivative} :
        Set (Common22SeparateChartMap.AffineRing a3 a2 a1 a0)) ≤ J := by
    apply Ideal.span_le.mpr
    intro x hx
    simp only [Set.mem_insert_iff, Set.mem_singleton_iff] at hx
    rcases hx with rfl | rfl
    · exact hy
    · exact hp
  rw [WeightedJacobianIdeal.affine_jacobian_ideal_top_of_squarefree
    a3 a2 a1 a0 h2 hsq] at hle
  exact top_unique hle

#print axioms WeightedAffineSquarefreeSmooth.affine_deriv_span_top_of_ids
end WeightedAffineSquarefreeSmooth
