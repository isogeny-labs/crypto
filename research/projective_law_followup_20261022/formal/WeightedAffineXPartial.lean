import WeightedAffineRingSmooth
import WeightedAffineSquarefreeSmooth

set_option autoImplicit false
noncomputable section
namespace WeightedAffineXPartial

open MvPolynomial
variable {K : Type*} [Field K]

private theorem eval_C (a3 a2 a1 a0 r : K) :
    WeightedJacobianIdeal.affAeval a3 a2 a1 a0 (Polynomial.C r) =
      Ideal.Quotient.mk (Common22SeparateChartMap.affineIdeal a3 a2 a1 a0)
        (MvPolynomial.C r) := by
  change Polynomial.aeval (Common22SeparateChartMap.xbar a3 a2 a1 a0)
    (Polynomial.C r) = _
  calc
    _ = algebraMap K (Common22SeparateChartMap.AffineRing a3 a2 a1 a0) r :=
      Polynomial.aeval_C (Common22SeparateChartMap.xbar a3 a2 a1 a0) r
    _ = _ := by
      rw [← Ideal.Quotient.mk_algebraMap]
      rfl

private theorem eval_X (a3 a2 a1 a0 : K) :
    WeightedJacobianIdeal.affAeval a3 a2 a1 a0 Polynomial.X =
      Ideal.Quotient.mk (Common22SeparateChartMap.affineIdeal a3 a2 a1 a0)
        (MvPolynomial.X 0) := by
  simp [WeightedJacobianIdeal.affAeval, Common22SeparateChartMap.xbar]

private theorem mk_mul_two (I : Ideal (Common22SeparateChartMap.Poly K))
    (x : Common22SeparateChartMap.Poly K) :
    Ideal.Quotient.mk I (x * 2) =
      (2 : Common22SeparateChartMap.Poly K ⧸ I) * Ideal.Quotient.mk I x := by
  rw [map_mul, map_ofNat]
  ring

theorem affine_x_partial (a3 a2 a1 a0 : K) :
    Ideal.Quotient.mk (Common22SeparateChartMap.affineIdeal a3 a2 a1 a0)
      ((Common22SeparateChartMap.affineEquation a3 a2 a1 a0).pderiv 0) =
      - WeightedJacobianIdeal.affAeval a3 a2 a1 a0
        (WeightedProjSmoothPoints.quintic a3 a2 a1 a0).derivative := by
  simp [Common22SeparateChartMap.affineEquation,
    Common22SeparateChartMap.affineIdeal, WeightedJacobianIdeal.affAeval,
    WeightedProjSmoothPoints.quintic, MvPolynomial.pderiv_pow,
    MvPolynomial.pderiv_mul, Polynomial.derivative_pow]
  let φ : Common22SeparateChartMap.Poly K →+*
      Common22SeparateChartMap.AffineRing a3 a2 a1 a0 :=
    Ideal.Quotient.mk (Common22SeparateChartMap.affineIdeal a3 a2 a1 a0)
  change φ _ = _
  simp only [map_sub, map_add, map_mul, map_neg, map_pow, map_ofNat]
  simp [φ, Common22SeparateChartMap.xbar, ← Ideal.Quotient.mk_algebraMap,
    MvPolynomial.algebraMap_eq]

theorem affine_y_partial (a3 a2 a1 a0 : K) :
    Ideal.Quotient.mk (Common22SeparateChartMap.affineIdeal a3 a2 a1 a0)
      ((Common22SeparateChartMap.affineEquation a3 a2 a1 a0).pderiv 1) =
      (2 : Common22SeparateChartMap.AffineRing a3 a2 a1 a0) *
        Common22SeparateChartMap.ybar a3 a2 a1 a0 := by
  simp [Common22SeparateChartMap.affineEquation,
    Common22SeparateChartMap.ybar, Common22SeparateChartMap.affineIdeal,
    map_sub, map_add, map_mul, map_pow,
    MvPolynomial.pderiv_pow, MvPolynomial.pderiv_mul]
  rw [show (2 : Common22SeparateChartMap.Poly K) * X 1 = X 1 * 2 by ring]
  exact mk_mul_two (Common22SeparateChartMap.affineIdeal a3 a2 a1 a0) (X 1)

#print axioms WeightedAffineXPartial.affine_x_partial
#print axioms WeightedAffineXPartial.affine_y_partial

end WeightedAffineXPartial
