import WeightedProjSourceFunctionField
import WeightedProjYIndependent

/-! Source-shaped common 2+2 numerator and rational function are nonzero on
the actual weighted-Proj curve when the source coefficient B is nonzero. -/

set_option autoImplicit false
set_option maxHeartbeats 1000000
noncomputable section

namespace WeightedProjSourceNonzero
open MvPolynomial Common22SeparateChartMap AlgebraicGeometry WeightedRingGraded
variable {K : Type} [Field K]

theorem sourceNumerator_sub_B_ybar_ne_zero
    (a3 a2 a1 a0 q1 r1 s1 t1 q2 r2 s2 t2 : K)
    (hB : (Common22SupportHead.coefficientData
      q1 r1 s1 t1 q2 r2 s2 t2).1 ≠ 0) :
    WeightedProjSourceFunctionField.sourceNumerator
      a3 a2 a1 a0 q1 r1 s1 t1 q2 r2 s2 t2 -
      algebraMap K (Common22WeightedProjQuotient.AffineRing a3 a2 a1 a0)
        (Common22SupportHead.coefficientData
          q1 r1 s1 t1 q2 r2 s2 t2).1 *
      Common22WeightedProjQuotient.ybar a3 a2 a1 a0 ≠ 0 := by
  rcases hdata : Common22SupportHead.coefficientData q1 r1 s1 t1 q2 r2 s2 t2 with
    ⟨B, A, cCoef, D, E⟩
  let p : Poly K :=
    C B * (C s1 * X 0 + C t1) -
      (C cCoef * X 0 + C A) * (X 0 ^ 2 + C q1 * X 0 + C r1)
  let q : MvPolynomial (Fin 1) K :=
    C B * (C s1 * X 0 + C t1) -
      (C cCoef * X 0 + C A) * (X 0 ^ 2 + C q1 * X 0 + C r1)
  have hX0 : Common22Domain.nestedEquiv (X 0 : Poly K) =
      Polynomial.C (X 0 : MvPolynomial (Fin 1) K) := by
    have hX1 : MvPolynomial.finSuccEquiv K 1
        (MvPolynomial.X (1 : Fin 2)) =
        Polynomial.C (MvPolynomial.X (0 : Fin 1)) := by
      simpa using (MvPolynomial.finSuccEquiv_X_succ (R := K) (j := (0 : Fin 1)))
    simp [Common22Domain.nestedEquiv, MvPolynomial.renameEquiv_apply, hX1]
  have hC (b : K) : Common22Domain.nestedEquiv (C b : Poly K) =
      Polynomial.C (C b : MvPolynomial (Fin 1) K) := by
    simp [Common22Domain.nestedEquiv, MvPolynomial.renameEquiv_apply,
      MvPolynomial.finSuccEquiv_apply]
  have hp : Common22Domain.nestedEquiv p = Polynomial.C q := by
    simp [p, q, hX0, hC]
  have hconst (b : K) :
      (Ideal.Quotient.mk (Common22WeightedProjQuotient.affineIdeal a3 a2 a1 a0) (C b) :
        Common22WeightedProjQuotient.AffineRing a3 a2 a1 a0) =
      algebraMap K (Common22WeightedProjQuotient.AffineRing a3 a2 a1 a0) b := by
    change Ideal.Quotient.mk (Common22WeightedProjQuotient.affineIdeal a3 a2 a1 a0)
      (algebraMap K (Poly K) b) = _
    rfl
  have hmk : Ideal.Quotient.mk (Common22WeightedProjQuotient.affineIdeal a3 a2 a1 a0) p =
      WeightedProjSourceFunctionField.sourceNumerator
        a3 a2 a1 a0 q1 r1 s1 t1 q2 r2 s2 t2 := by
    simp only [p, map_sub, map_mul, map_add, map_pow, hconst]
    dsimp only [WeightedProjSourceFunctionField.sourceNumerator]
    rw [hdata]
    rfl
  have hB' : B ≠ 0 := by simpa [hdata] using hB
  simpa only [hmk, hdata] using
    (WeightedProjYIndependent.affine_x_only_sub_B_ybar_ne_zero
      a3 a2 a1 a0 B p q hp hB')

theorem sourceFunction_ne_zero
    (a3 a2 a1 a0 q1 r1 s1 t1 q2 r2 s2 t2 : K)
    (hsep : (WeightedProjSmoothPoints.quintic a3 a2 a1 a0).Separable)
    (hB : (Common22SupportHead.coefficientData
      q1 r1 s1 t1 q2 r2 s2 t2).1 ≠ 0) :
    WeightedProjSourceFunctionField.sourceFunction
      a3 a2 a1 a0 q1 r1 s1 t1 q2 r2 s2 t2 hsep ≠ 0 := by
  letI : IsIntegral (Proj (degreeSubmodule a3 a2 a1 a0)) :=
    WeightedProjIsIntegral.whole_curve_integral a3 a2 a1 a0 hsep
  let φ := WeightedProjSourceFunctionField.affineToFunctionField a3 a2 a1 a0 hsep
  let B := (Common22SupportHead.coefficientData
    q1 r1 s1 t1 q2 r2 s2 t2).1
  let ι := algebraMap K (Common22WeightedProjQuotient.AffineRing a3 a2 a1 a0)
  let N := WeightedProjSourceFunctionField.sourceNumerator
    a3 a2 a1 a0 q1 r1 s1 t1 q2 r2 s2 t2
  have hd : φ (ι B) ≠ 0 :=
    WeightedProjSourceFunctionField.sourceDenominator_ne_zero
      a3 a2 a1 a0 q1 r1 s1 t1 q2 r2 s2 t2 hsep hB
  have hn : N - ι B * Common22WeightedProjQuotient.ybar a3 a2 a1 a0 ≠ 0 :=
    sourceNumerator_sub_B_ybar_ne_zero
      a3 a2 a1 a0 q1 r1 s1 t1 q2 r2 s2 t2 hB
  change φ N / φ (ι B) - φ (Common22WeightedProjQuotient.ybar a3 a2 a1 a0) ≠ 0
  intro hf
  have heq : φ N / φ (ι B) = φ (Common22WeightedProjQuotient.ybar a3 a2 a1 a0) :=
    sub_eq_zero.mp hf
  have hm : φ N = φ (ι B) * φ (Common22WeightedProjQuotient.ybar a3 a2 a1 a0) := by
    calc
      φ N = φ (Common22WeightedProjQuotient.ybar a3 a2 a1 a0) * φ (ι B) :=
        (div_eq_iff hd).mp heq
      _ = _ := mul_comm _ _
  have hpre : N = ι B * Common22WeightedProjQuotient.ybar a3 a2 a1 a0 :=
    (WeightedProjSourceFunctionField.affineToFunctionField_injective
      a3 a2 a1 a0 hsep) (by simpa only [map_mul] using hm)
  exact hn (sub_eq_zero.mpr hpre)

#print axioms sourceNumerator_sub_B_ybar_ne_zero
#print axioms sourceFunction_ne_zero
end WeightedProjSourceNonzero
