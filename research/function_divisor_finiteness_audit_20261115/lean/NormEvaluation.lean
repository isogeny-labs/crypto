module
public import RecursionCertificate

@[expose] public section
namespace FunctionDivisorFinitenessAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine WeierstrassCurve.Affine.CoordinateRing
open MillerPrincipalIdealAudit MillerFinitePointAudit
variable {K : Type*} [Field K]

/-- Actual coordinate polynomial classes evaluate as their actual one-variable polynomials. -/
theorem point_eval_polynomial (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Equation x y) (p : K[X]) :
    pointEval W x y h (mk W (C p)) = p.eval x := by
  have he : (pointEval W x y h).comp ((mk W).comp C) = evalRingHom x := by
    apply Polynomial.ringHom_ext
    · intro c
      change pointEval W x y h (mk W (C (C c))) = (C c).eval x
      rw [eval_C]
      change pointEval W x y h (algebraMap K W.CoordinateRing c) = c
      exact point_eval_constant W h c
    · change pointEval W x y h (mk W (C X)) = X.eval x
      rw [eval_X]
      simpa only [XClass, map_zero, sub_zero] using point_eval_XClass W h 0
  exact RingHom.congr_fun he p

/-- Every actual affine zero lies above a root of the actual algebra norm polynomial. -/
theorem zero_eval_norm_root (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Equation x y) (a : W.CoordinateRing) (ha : pointEval W x y h a = 0) :
    IsRoot (Algebra.norm K[X] a) x := by
  obtain ⟨p, q, rfl⟩ := exists_smul_basis_eq a
  have hy : pointEval W x y h (mk W X) = y := by
    simpa only [YClass, map_zero, sub_zero, eval_zero] using point_eval_YClass W h (0 : K[X])
  have hz : p.eval x + q.eval x * y = 0 := by
    simpa only [smul, map_add, map_mul, map_one, mul_one, point_eval_polynomial, hy] using ha
  change (Algebra.norm K[X] (p • (1 : W.CoordinateRing) + q • mk W X)).eval x = 0
  rw [norm_smul_basis]
  simp only [eval_sub, eval_pow, eval_mul, eval_add, eval_C, eval_X]
  have hc := (W.equation_iff x y).mp h
  calc
    _ = (p.eval x + q.eval x * y) * (p.eval x - q.eval x * (y + W.a₁ * x + W.a₃)) := by
      linear_combination (q.eval x) ^ 2 * hc
    _ = 0 := by rw [hz, zero_mul]

theorem norm_polynomial_nonzero (W : WeierstrassCurve.Affine K) {a : W.CoordinateRing}
    (ha : a ≠ 0) : Algebra.norm K[X] a ≠ 0 := by
  intro h
  exact ha (Algebra.norm_eq_zero_iff.mp h)

/-- Actual polynomial functions in the actual elliptic function field. -/
def polynomialFunction (W : WeierstrassCurve.Affine K) : K[X] →+* W.FunctionField :=
  (algebraMap W.CoordinateRing W.FunctionField).comp ((mk W).comp C)

theorem polynomial_function_injective (W : WeierstrassCurve.Affine K) :
    Function.Injective (polynomialFunction W) := by
  apply (IsFractionRing.injective W.CoordinateRing W.FunctionField).comp
  intro p q hpq
  change mk W (C p) = mk W (C q) at hpq
  have hz : (p - q) • (1 : W.CoordinateRing) + (0 : K[X]) • mk W X = 0 := by
    simpa only [smul, zero_smul, mul_one, add_zero, map_sub] using sub_eq_zero.mpr hpq
  exact sub_eq_zero.mp (smul_basis_eq_zero hz).1

#print axioms zero_eval_norm_root
#print axioms polynomial_function_injective
end
end FunctionDivisorFinitenessAudit
