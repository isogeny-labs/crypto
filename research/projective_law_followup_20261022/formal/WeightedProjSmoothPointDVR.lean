import WeightedProjDerivativePointDVR

/-!
# Rational smooth-point DVR criterion

The two pointwise local-parameter proofs combine under the disjunction of
nonzero partial derivatives. This file identifies that disjunction with the
affine equation's Jacobian condition at a rational point; it does not assert
global smoothness, Picard-group transport, or runtime correctness.
-/

set_option autoImplicit false
noncomputable section

namespace WeightedProjSmoothPointDVR

open Common22SeparateChartMap Common22BranchStalk Common22BranchParameter
  WeightedProjNonbranchZeroOrder WeightedProjSmoothPoints

variable {K : Type*} [Field K]

theorem derivativeAt_eq_quintic_derivative
    (a3 a2 a1 a0 a : K) :
    derivativeAt a3 a2 a1 a0 a =
      (quintic a3 a2 a1 a0).derivative.eval a := by
  simp [derivativeAt, quintic, Polynomial.derivative_add,
    Polynomial.derivative_mul, Polynomial.derivative_pow]
  ring

theorem pointwise_jacobian_iff
    (a3 a2 a1 a0 a y : K) :
    (2 * y ≠ 0 ∨ derivativeAt a3 a2 a1 a0 a ≠ 0) ↔
      (affineEquationY a3 a2 a1 a0 a).derivative.eval y ≠ 0 ∨
      (affineEquationX a3 a2 a1 a0 y).derivative.eval a ≠ 0 := by
  rw [affineEquationY_derivative, affineEquationX_derivative,
    ← derivativeAt_eq_quintic_derivative]
  simp

theorem point_local_dvr (a3 a2 a1 a0 a y : K)
    (hcurve : y ^ 2 = fAt a3 a2 a1 a0 a)
    (hsep : (quintic a3 a2 a1 a0).Separable)
    (hsmooth : 2 * y ≠ 0 ∨ derivativeAt a3 a2 a1 a0 a ≠ 0) :
    @IsDiscreteValuationRing
      (Localization.AtPrime (pointIdeal a3 a2 a1 a0 a y hcurve)) _
      (Common22SeparableAffineDomain.point_local_domain_of_separable
        a3 a2 a1 a0 a y hcurve hsep) := by
  rcases hsmooth with h2y | hderiv
  · exact WeightedProjNonbranchDVR.point_local_dvr a3 a2 a1 a0 a y
      hcurve hsep h2y
  · exact WeightedProjDerivativePointDVR.point_local_dvr a3 a2 a1 a0 a y
      hcurve hsep hderiv

theorem projective_point_dvr (a3 a2 a1 a0 a y : K)
    (hcurve : y ^ 2 = fAt a3 a2 a1 a0 a)
    (hsep : (quintic a3 a2 a1 a0).Separable)
    (hsmooth : 2 * y ≠ 0 ∨ derivativeAt a3 a2 a1 a0 a ≠ 0) :
    @IsDiscreteValuationRing (ProjPointRing a3 a2 a1 a0 a y hcurve) _
      (Common22SeparableAffineDomain.projective_point_domain_of_separable
        a3 a2 a1 a0 a y hcurve hsep) := by
  rcases hsmooth with h2y | hderiv
  · exact WeightedProjNonbranchDVR.projective_point_dvr a3 a2 a1 a0 a y
      hcurve hsep h2y
  · exact WeightedProjDerivativePointDVR.projective_point_dvr a3 a2 a1 a0 a y
      hcurve hsep hderiv

theorem projective_point_dvr_of_jacobian
    (a3 a2 a1 a0 a y : K)
    (hcurve : y ^ 2 = fAt a3 a2 a1 a0 a)
    (hsep : (quintic a3 a2 a1 a0).Separable)
    (hjac : (affineEquationY a3 a2 a1 a0 a).derivative.eval y ≠ 0 ∨
      (affineEquationX a3 a2 a1 a0 y).derivative.eval a ≠ 0) :
    @IsDiscreteValuationRing (ProjPointRing a3 a2 a1 a0 a y hcurve) _
      (Common22SeparableAffineDomain.projective_point_domain_of_separable
        a3 a2 a1 a0 a y hcurve hsep) := by
  exact projective_point_dvr a3 a2 a1 a0 a y hcurve hsep
    ((pointwise_jacobian_iff a3 a2 a1 a0 a y).mpr hjac)

#print axioms derivativeAt_eq_quintic_derivative
#print axioms pointwise_jacobian_iff
#print axioms point_local_dvr
#print axioms projective_point_dvr
#print axioms projective_point_dvr_of_jacobian

end WeightedProjSmoothPointDVR
