import Common22BranchDomain
import Common22WeightedProjQuotient

/-! The affine ordinate of the actual weighted-Proj chart is not an x-only
polynomial, and no nonzero scalar multiple of it is x-only. -/

set_option autoImplicit false
set_option maxHeartbeats 600000
noncomputable section

namespace WeightedProjYIndependent
open MvPolynomial Common22SeparateChartMap
variable {K : Type*} [Field K]

theorem affine_ybar_sub_base_ne_zero (a3 a2 a1 a0 : K)
    (p : Poly K) (q : MvPolynomial (Fin 1) K)
    (hp : Common22Domain.nestedEquiv p = Polynomial.C q) :
    Common22WeightedProjQuotient.ybar a3 a2 a1 a0 -
      Ideal.Quotient.mk (Common22WeightedProjQuotient.affineIdeal a3 a2 a1 a0) p ≠ 0 := by
  intro hy
  have hmem : (X 1 - p : Poly K) ∈ Common22WeightedProjQuotient.affineIdeal a3 a2 a1 a0 := by
    apply (Ideal.Quotient.eq_zero_iff_mem).mp
    simpa [Common22WeightedProjQuotient.ybar, map_sub] using hy
  have hdvd : Common22WeightedProjQuotient.affineEquation a3 a2 a1 a0 ∣ (X 1 - p : Poly K) := by
    simpa only [Common22WeightedProjQuotient.affineIdeal, Ideal.mem_span_singleton] using hmem
  have hdvd' : Common22Domain.nestedEquiv (Common22WeightedProjQuotient.affineEquation a3 a2 a1 a0) ∣
      Common22Domain.nestedEquiv (X 1 - p : Poly K) := by
    obtain ⟨r, hr⟩ := hdvd
    exact ⟨Common22Domain.nestedEquiv r,
      by simpa using congrArg Common22Domain.nestedEquiv hr⟩
  have hmapy : Common22Domain.nestedEquiv (X 1 : Poly K) = Polynomial.X := by
    simp [Common22Domain.nestedEquiv, MvPolynomial.renameEquiv_apply,
      MvPolynomial.finSuccEquiv_X_zero]
  have hmap : Common22Domain.nestedEquiv (X 1 - p : Poly K) =
      Polynomial.X - Polynomial.C q := by
    rw [map_sub, hmapy, hp]
  have hEq : Common22WeightedProjQuotient.affineEquation a3 a2 a1 a0 =
      Common22SeparateChartMap.affineEquation a3 a2 a1 a0 := by rfl
  rw [hEq, Common22BranchDomain.nested_affine_equation, hmap] at hdvd'
  have hdeg := Polynomial.natDegree_le_of_dvd hdvd'
    (Polynomial.X_sub_C_ne_zero q)
  rw [Polynomial.natDegree_X_pow_sub_C,
    Polynomial.natDegree_X_sub_C] at hdeg
  omega


theorem affine_x_only_sub_B_ybar_ne_zero (a3 a2 a1 a0 B : K)
    (p : Poly K) (q : MvPolynomial (Fin 1) K)
    (hp : Common22Domain.nestedEquiv p = Polynomial.C q)
    (hB : B ≠ 0) :
    Ideal.Quotient.mk (Common22WeightedProjQuotient.affineIdeal a3 a2 a1 a0) p -
      algebraMap K (Common22WeightedProjQuotient.AffineRing a3 a2 a1 a0) B * Common22WeightedProjQuotient.ybar a3 a2 a1 a0 ≠ 0 := by
  let R := Common22WeightedProjQuotient.AffineRing a3 a2 a1 a0
  let ι := algebraMap K R
  have hC : Common22Domain.nestedEquiv (C B⁻¹ : Poly K) =
      Polynomial.C (C B⁻¹ : MvPolynomial (Fin 1) K) := by
    simp [Common22Domain.nestedEquiv, MvPolynomial.renameEquiv_apply,
      MvPolynomial.finSuccEquiv_apply]
  have hpInv : Common22Domain.nestedEquiv (C B⁻¹ * p : Poly K) =
      Polynomial.C (C B⁻¹ * q : MvPolynomial (Fin 1) K) := by
    rw [map_mul, hC, hp, map_mul]
  have hmk : Ideal.Quotient.mk (Common22WeightedProjQuotient.affineIdeal a3 a2 a1 a0) (C B⁻¹ * p) =
      ι B⁻¹ * Ideal.Quotient.mk (Common22WeightedProjQuotient.affineIdeal a3 a2 a1 a0) p := by
    rw [map_mul]
    congr 1
  have hunit : ι B⁻¹ * ι B = 1 := by
    rw [← map_mul, inv_mul_cancel₀ hB, map_one]
  intro hz
  have heq : Ideal.Quotient.mk (Common22WeightedProjQuotient.affineIdeal a3 a2 a1 a0) p =
      ι B * Common22WeightedProjQuotient.ybar a3 a2 a1 a0 := sub_eq_zero.mp hz
  have hy : Common22WeightedProjQuotient.ybar a3 a2 a1 a0 =
      Ideal.Quotient.mk (Common22WeightedProjQuotient.affineIdeal a3 a2 a1 a0) (C B⁻¹ * p) := by
    rw [hmk, heq, ← mul_assoc, hunit, one_mul]
  exact affine_ybar_sub_base_ne_zero a3 a2 a1 a0
    (C B⁻¹ * p) (C B⁻¹ * q) hpInv (sub_eq_zero.mpr hy)

#print axioms affine_ybar_sub_base_ne_zero
#print axioms affine_x_only_sub_B_ybar_ne_zero
end WeightedProjYIndependent
