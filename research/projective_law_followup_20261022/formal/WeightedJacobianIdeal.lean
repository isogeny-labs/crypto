import WeightedProjSmoothPoints
import Common22SeparateChartMap
import Mathlib.RingTheory.Ideal.Operations

set_option autoImplicit false
noncomputable section
namespace WeightedJacobianIdeal
open Polynomial Common22SeparateChartMap WeightedProjSmoothPoints
variable {K : Type*} [Field K]

def affAeval (a3 a2 a1 a0 : K) : K[X] →ₐ[K] AffineRing a3 a2 a1 a0 :=
  Polynomial.aeval (xbar a3 a2 a1 a0)

private theorem eval_quintic_eq_y_sq (a3 a2 a1 a0 : K) :
    affAeval a3 a2 a1 a0 (quintic a3 a2 a1 a0) =
      ybar a3 a2 a1 a0 ^ 2 := by
  rw [Common22SeparateChartMap.quotient_affine_relation]
  simp [affAeval, quintic, map_add, map_mul, map_pow]

private theorem coprime_y_sq_derivative
    (a3 a2 a1 a0 : K) (hsep : (quintic a3 a2 a1 a0).Separable) :
    IsCoprime (ybar a3 a2 a1 a0 ^ 2)
      (affAeval a3 a2 a1 a0 (quintic a3 a2 a1 a0).derivative) := by
  have hcop : IsCoprime (quintic a3 a2 a1 a0) (quintic a3 a2 a1 a0).derivative :=
    (Polynomial.separable_def _).mp hsep
  have h := hcop.map (affAeval a3 a2 a1 a0).toRingHom
  change IsCoprime (affAeval a3 a2 a1 a0 (quintic a3 a2 a1 a0))
    (affAeval a3 a2 a1 a0 (quintic a3 a2 a1 a0).derivative) at h
  rw [eval_quintic_eq_y_sq] at h
  exact h

/-- The affine Jacobian ideal is the whole coordinate ring. -/
theorem affine_jacobian_ideal_top_of_separable
    (a3 a2 a1 a0 : K) (h2 : (2 : K) ≠ 0)
    (hsep : (quintic a3 a2 a1 a0).Separable) :
    Ideal.span ({(2 : AffineRing a3 a2 a1 a0) * ybar a3 a2 a1 a0,
      affAeval a3 a2 a1 a0 (quintic a3 a2 a1 a0).derivative} :
      Set (AffineRing a3 a2 a1 a0)) = ⊤ := by
  have huK : IsUnit (2 : K) := (isUnit_iff_ne_zero).mpr h2
  have hu : IsUnit (2 : AffineRing a3 a2 a1 a0) := by
    exact IsUnit.map (algebraMap K (AffineRing a3 a2 a1 a0)) huK
  have hcop := coprime_y_sq_derivative a3 a2 a1 a0 hsep
  have hcopY : IsCoprime (ybar a3 a2 a1 a0)
      (affAeval a3 a2 a1 a0 (quintic a3 a2 a1 a0).derivative) := by
    have hmul : IsCoprime
        (ybar a3 a2 a1 a0 * ybar a3 a2 a1 a0)
        (affAeval a3 a2 a1 a0 (quintic a3 a2 a1 a0).derivative) := by
      simpa only [pow_two] using hcop
    exact hmul.of_mul_left_left
  have hcop2 : IsCoprime
      ((2 : AffineRing a3 a2 a1 a0) * ybar a3 a2 a1 a0)
      (affAeval a3 a2 a1 a0 (quintic a3 a2 a1 a0).derivative) := by
    exact (isCoprime_mul_unit_left_left hu _ _).mpr hcopY
  exact (Ideal.eq_top_iff_one _).mpr (Ideal.mem_span_pair.mpr hcop2)

/-- Finite fields meet the separability input when the quintic is squarefree. -/
theorem affine_jacobian_ideal_top_of_squarefree
    [PerfectField K] (a3 a2 a1 a0 : K) (h2 : (2 : K) ≠ 0)
    (hsq : Squarefree (quintic a3 a2 a1 a0)) :
    Ideal.span ({(2 : AffineRing a3 a2 a1 a0) * ybar a3 a2 a1 a0,
      affAeval a3 a2 a1 a0 (quintic a3 a2 a1 a0).derivative} :
      Set (AffineRing a3 a2 a1 a0)) = ⊤ := by
  exact affine_jacobian_ideal_top_of_separable a3 a2 a1 a0 h2
    (PerfectField.separable_iff_squarefree.mpr hsq)

#print axioms WeightedJacobianIdeal.affine_jacobian_ideal_top_of_squarefree

end WeightedJacobianIdeal
