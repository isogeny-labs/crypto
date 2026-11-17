import Common22SourceHead
import Mathlib.Algebra.Polynomial.Eval.Defs
import Mathlib.Algebra.Polynomial.Derivative
import Mathlib.Algebra.Polynomial.FieldDivision
import Mathlib.Tactic.FieldSimp

/-!
# The infinity chart of a normalized genus-2 quintic

These identities hold in a field at affine points with `x ≠ 0`. They do not
construct the projective curve or prove local-ring valuation statements.
-/

set_option autoImplicit false

namespace Common22InfinityChart

open Polynomial

variable {K : Type*} [Field K]

/-- The near-infinity equation after `z=1/x`, `w=y/x³`. The missing `x⁴`
coefficient of the normalized quintic is visible as a missing `z²` term. -/
def chartRight (a3 a2 a1 a0 z : K) : K :=
  z * (1 + a3 * z ^ 2 + a2 * z ^ 3 + a1 * z ^ 4 + a0 * z ^ 5)

/-- The infinity-chart equation, viewed as a polynomial in `z` with `w`
fixed. This orientation makes the derivative transverse to the `w`-axis. -/
noncomputable def chartEquationZ (a3 a2 a1 a0 w : K) : K[X] :=
  C (w ^ 2) - X *
    (1 + C a3 * X ^ 2 + C a2 * X ^ 3 + C a1 * X ^ 4 + C a0 * X ^ 5)

theorem chartEquationZ_eval (a3 a2 a1 a0 w z : K) :
    (chartEquationZ a3 a2 a1 a0 w).eval z =
      w ^ 2 - chartRight a3 a2 a1 a0 z := by
  simp [chartEquationZ, chartRight]

/-- The point `(z,w)=(0,0)` lies on the chart. -/
theorem chartEquationZ_origin (a3 a2 a1 a0 : K) :
    (chartEquationZ a3 a2 a1 a0 0).eval 0 = 0 := by
  simp [chartEquationZ_eval, chartRight]

/-- The `z` partial derivative at infinity is a unit (`-1`) for every
coefficient tuple, including positive-characteristic fields. This is the
algebraic Jacobian-criterion input; it does not itself construct the local
ring of the smooth projective completion. -/
theorem chartEquationZ_derivative_origin (a3 a2 a1 a0 : K) :
    (chartEquationZ a3 a2 a1 a0 0).derivative.eval 0 = -1 := by
  simp [chartEquationZ, derivative_mul]

theorem chartEquationZ_derivative_nonzero (a3 a2 a1 a0 : K) :
    (chartEquationZ a3 a2 a1 a0 0).derivative.eval 0 ≠ 0 := by
  rw [chartEquationZ_derivative_origin]
  exact neg_ne_zero.mpr one_ne_zero

/-- The `w=0` section has a simple root at `z=0`. This follows from the
nonzero transverse derivative. It is a polynomial statement, not a theorem
about the completed curve's local ring or its uniformizer. -/
theorem chartEquationZ_simple_section (a3 a2 a1 a0 : K) :
    (chartEquationZ a3 a2 a1 a0 0).rootMultiplicity 0 = 1 := by
  let p := chartEquationZ a3 a2 a1 a0 0
  have hp : p ≠ 0 := by
    intro h
    have hd := chartEquationZ_derivative_nonzero a3 a2 a1 a0
    simp [p, h] at hd
  have hroot : p.IsRoot 0 := by
    simpa [p, IsRoot] using chartEquationZ_origin a3 a2 a1 a0
  have hpos : 0 < p.rootMultiplicity 0 := (rootMultiplicity_pos hp).mpr hroot
  have hle : p.rootMultiplicity 0 ≤ 1 := by
    apply Nat.le_of_not_gt
    intro hlt
    have hder := (one_lt_rootMultiplicity_iff_isRoot hp).mp hlt |>.2
    exact (chartEquationZ_derivative_nonzero a3 a2 a1 a0) (by simpa [p, IsRoot] using hder)
  exact Nat.le_antisymm hle (Nat.succ_le_iff.mpr hpos)

theorem curve_chart_equation
    (a3 a2 a1 a0 x y : K) (hx : x ≠ 0)
    (hcurve : y ^ 2 = x ^ 5 + a3 * x ^ 3 + a2 * x ^ 2 + a1 * x + a0) :
    (y * x⁻¹ ^ 3) ^ 2 = chartRight a3 a2 a1 a0 x⁻¹ := by
  dsimp [chartRight]
  field_simp [hx]
  convert hcurve using 1; ring

/-- The right-hand factor of the chart equation has residue one at the
origin. This is the algebraic input for showing it is a unit in the local
ring at infinity; the local-ring theorem itself is not included. -/
theorem chart_factor_at_origin (a3 a2 a1 a0 : K) :
    1 + a3 * (0 : K) ^ 2 + a2 * 0 ^ 3 + a1 * 0 ^ 4 + a0 * 0 ^ 5 = 1 := by
  ring

/-- Cleared interpolation function in the same chart. The source-style
cubic coefficients instantiate `c,n2,n1,n0`; `b` clears its denominator. -/
theorem interpolant_chart
    (b c n2 n1 n0 x y : K) (hx : x ≠ 0) :
    x⁻¹ ^ 3 * ((-c) * x ^ 3 + n2 * x ^ 2 + n1 * x + n0 - b * y) =
      -c + n2 * x⁻¹ + n1 * x⁻¹ ^ 2 + n0 * x⁻¹ ^ 3 -
        b * (y * x⁻¹ ^ 3) := by
  field_simp [hx]

/-- The chart identity with the exact source-style common 2+2 coefficient
tuple substituted into the cleared cubic interpolant. -/
theorem source_interpolant_chart
    (q1 r1 s1 t1 q2 r2 s2 t2 x y : K) (hx : x ≠ 0) :
    let (b, a, c, _, _) :=
      Common22SupportHead.coefficientData q1 r1 s1 t1 q2 r2 s2 t2
    let n : K[X] := C (-c) * X ^ 3 + C (-(c * q1 + a)) * X ^ 2 +
      C (b * s1 - c * r1 - a * q1) * X + C (b * t1 - a * r1)
    x⁻¹ ^ 3 * (n.eval x - b * y) =
      -c + (-(c * q1 + a)) * x⁻¹ +
      (b * s1 - c * r1 - a * q1) * x⁻¹ ^ 2 +
      (b * t1 - a * r1) * x⁻¹ ^ 3 - b * (y * x⁻¹ ^ 3) := by
  dsimp only
  simp only [eval_add, eval_mul, eval_pow, eval_C, eval_X]
  exact interpolant_chart _ _ _ _ _ x y hx

/-- The constant residue of the cleared interpolation function at the
chart origin is `-c`. Hence `c ≠ 0` supplies a local-unit candidate. -/
theorem interpolant_residue (b c n2 n1 n0 : K) :
    -c + n2 * (0 : K) + n1 * 0 ^ 2 + n0 * 0 ^ 3 - b * 0 = -c := by
  ring

/-- When `c=0`, the linear `w` coefficient is `-b`; the other terms start
in `z`. This is the algebraic input for the pole-order-five branch once `w`
is proved to be a uniformizer and `z` has order two. -/
theorem interpolant_linear_w_ne_zero (b c : K) (hb : b ≠ 0) (hc : c = 0) :
    -c = 0 ∧ -b ≠ 0 := by
  simp [hc, hb]

end Common22InfinityChart

#print axioms Common22InfinityChart.curve_chart_equation
#print axioms Common22InfinityChart.chartEquationZ_eval
#print axioms Common22InfinityChart.chartEquationZ_origin
#print axioms Common22InfinityChart.chartEquationZ_derivative_origin
#print axioms Common22InfinityChart.chartEquationZ_derivative_nonzero
#print axioms Common22InfinityChart.chartEquationZ_simple_section
#print axioms Common22InfinityChart.chart_factor_at_origin
#print axioms Common22InfinityChart.interpolant_chart
#print axioms Common22InfinityChart.source_interpolant_chart
#print axioms Common22InfinityChart.interpolant_residue
#print axioms Common22InfinityChart.interpolant_linear_w_ne_zero
