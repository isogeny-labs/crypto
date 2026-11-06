import Common22InfinityChart
import Mathlib.Algebra.Polynomial.Derivative
import Mathlib.FieldTheory.Separable
import Mathlib.FieldTheory.Perfect

set_option autoImplicit false

namespace WeightedProjSmoothPoints

open Polynomial

variable {K : Type*} [Field K]

noncomputable def quintic (a3 a2 a1 a0 : K) : K[X] :=
  X ^ 5 + C a3 * X ^ 3 + C a2 * X ^ 2 + C a1 * X + C a0

noncomputable def affineEquationY (a3 a2 a1 a0 x : K) : K[X] :=
  X ^ 2 - C ((quintic a3 a2 a1 a0).eval x)

noncomputable def affineEquationX (a3 a2 a1 a0 y : K) : K[X] :=
  C (y ^ 2) - quintic a3 a2 a1 a0

theorem affineEquationY_eval (a3 a2 a1 a0 x y : K) :
    (affineEquationY a3 a2 a1 a0 x).eval y =
      y ^ 2 - (quintic a3 a2 a1 a0).eval x := by
  simp [affineEquationY]

theorem affineEquationY_derivative (a3 a2 a1 a0 x y : K) :
    (affineEquationY a3 a2 a1 a0 x).derivative.eval y = 2 * y := by
  simp [affineEquationY, derivative_pow]

theorem affineEquationX_derivative (a3 a2 a1 a0 x y : K) :
    (affineEquationX a3 a2 a1 a0 y).derivative.eval x =
      - (quintic a3 a2 a1 a0).derivative.eval x := by
  simp [affineEquationX, derivative_sub, derivative_pow]

/-- Every K-point of the affine chart has a nonzero partial derivative,
provided 2 is invertible and every root of the quintic is simple. -/
theorem affine_jacobian_nonzero
    (a3 a2 a1 a0 x y : K) (h2 : (2 : K) ≠ 0)
    (hsimple : ∀ t : K, (quintic a3 a2 a1 a0).eval t = 0 →
      (quintic a3 a2 a1 a0).derivative.eval t ≠ 0)
    (hon : (affineEquationY a3 a2 a1 a0 x).eval y = 0) :
    (affineEquationY a3 a2 a1 a0 x).derivative.eval y ≠ 0 ∨
      (affineEquationX a3 a2 a1 a0 y).derivative.eval x ≠ 0 := by
  by_cases hy : y = 0
  · right
    have hf : (quintic a3 a2 a1 a0).eval x = 0 := by
      simpa [affineEquationY_eval, hy] using hon.symm
    simpa [affineEquationX_derivative] using hsimple x hf
  · left
    rw [affineEquationY_derivative]
    exact mul_ne_zero h2 hy


/-- A separable quintic supplies the simple-root hypothesis over K. -/
theorem affine_jacobian_nonzero_of_separable
    (a3 a2 a1 a0 x y : K) (h2 : (2 : K) ≠ 0)
    (hsep : (quintic a3 a2 a1 a0).Separable)
    (hon : (affineEquationY a3 a2 a1 a0 x).eval y = 0) :
    (affineEquationY a3 a2 a1 a0 x).derivative.eval y ≠ 0 ∨
      (affineEquationX a3 a2 a1 a0 y).derivative.eval x ≠ 0 := by
  apply affine_jacobian_nonzero a3 a2 a1 a0 x y h2 ?_ hon
  intro t ht
  exact hsep.eval₂_derivative_ne_zero (RingHom.id K) (by simpa using ht)


/-- Over a perfect field, a squarefree quintic gives the affine pointwise
Jacobian condition. Finite fields are perfect. -/
theorem affine_jacobian_nonzero_of_squarefree
    [PerfectField K] (a3 a2 a1 a0 x y : K) (h2 : (2 : K) ≠ 0)
    (hsq : Squarefree (quintic a3 a2 a1 a0))
    (hon : (affineEquationY a3 a2 a1 a0 x).eval y = 0) :
    (affineEquationY a3 a2 a1 a0 x).derivative.eval y ≠ 0 ∨
      (affineEquationX a3 a2 a1 a0 y).derivative.eval x ≠ 0 := by
  exact affine_jacobian_nonzero_of_separable a3 a2 a1 a0 x y h2
    (PerfectField.separable_iff_squarefree.mpr hsq) hon

/-- The only infinity-chart point with z=0 is (0,0), where the z partial
is -1, independently of the quintic coefficients or characteristic. -/
theorem infinity_boundary_jacobian_nonzero
    (a3 a2 a1 a0 w : K)
    (hon : (Common22InfinityChart.chartEquationZ a3 a2 a1 a0 w).eval 0 = 0) :
    (Common22InfinityChart.chartEquationZ a3 a2 a1 a0 w).derivative.eval 0 ≠ 0 := by
  have hw : w = 0 := by
    have hw2 : w ^ 2 = 0 := by
      simpa [Common22InfinityChart.chartEquationZ_eval,
        Common22InfinityChart.chartRight] using hon
    simpa using hw2
  subst w
  exact Common22InfinityChart.chartEquationZ_derivative_nonzero a3 a2 a1 a0

#print axioms WeightedProjSmoothPoints.affine_jacobian_nonzero
#print axioms WeightedProjSmoothPoints.affine_jacobian_nonzero_of_separable
#print axioms WeightedProjSmoothPoints.affine_jacobian_nonzero_of_squarefree
#print axioms WeightedProjSmoothPoints.infinity_boundary_jacobian_nonzero

end WeightedProjSmoothPoints
