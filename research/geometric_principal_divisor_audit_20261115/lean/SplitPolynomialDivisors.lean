module
public import GeometricFibers

@[expose] public section
namespace GeometricPrincipalDivisorAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine
open MillerNormValuationAudit MillerFinitePointAudit FunctionDivisorFinitenessAudit
variable {K : Type*} [Field K]

theorem polynomial_degree_mul (W : WeierstrassCurve.Affine K) {p q : K[X]}
    (hp : p ≠ 0) (hq : q ≠ 0)
    (hdp : divisorDegree W (functionDivisor W (polynomialFunction W p)
      (polynomial_function_nonzero W hp)) = 0)
    (hdq : divisorDegree W (functionDivisor W (polynomialFunction W q)
      (polynomial_function_nonzero W hq)) = 0) :
    divisorDegree W (functionDivisor W (polynomialFunction W (p * q))
      (polynomial_function_nonzero W (mul_ne_zero hp hq))) = 0 := by
  have he := function_divisor_congr W (polynomial_function_nonzero W (mul_ne_zero hp hq))
    (mul_ne_zero (polynomial_function_nonzero W hp) (polynomial_function_nonzero W hq))
    ((polynomialFunction W).map_mul p q)
  rw [he, function_divisor_mul W (polynomial_function_nonzero W hp)
    (polynomial_function_nonzero W hq), map_add, hdp, hdq, add_zero]

theorem vertical_polynomial_degree [IsAlgClosed K] (W : WeierstrassCurve.Affine K)
    (hΔ : W.Δ ≠ 0) (x : K) :
    divisorDegree W (functionDivisor W (polynomialFunction W (X - C x))
      (polynomial_function_nonzero W (X_sub_C_ne_zero x))) = 0 := by
  classical
  obtain ⟨y, h⟩ := geometric_fiber W hΔ x
  have he : functionDivisor W (polynomialFunction W (X - C x))
      (polynomial_function_nonzero W (X_sub_C_ne_zero x)) = verticalDivisor W (Point.some x y h) := by
    ext R
    exact vertical_point_order W h R
  rw [he, verticalDivisor, map_sub, map_add, degree_single, degree_single, degree_single]
  norm_num

def rootProduct (s : Multiset K) : K[X] := (s.map fun a => X - C a).prod

theorem root_product_nonzero (s : Multiset K) : rootProduct s ≠ 0 := by
  induction s using Multiset.induction_on with
  | empty => simp [rootProduct]
  | cons x s ih =>
    simpa only [rootProduct, Multiset.map_cons, Multiset.prod_cons] using
      mul_ne_zero (X_sub_C_ne_zero x) ih

theorem root_product_degree [IsAlgClosed K] (W : WeierstrassCurve.Affine K)
    (hΔ : W.Δ ≠ 0) (s : Multiset K) :
    divisorDegree W (functionDivisor W (polynomialFunction W (rootProduct s))
      (polynomial_function_nonzero W (root_product_nonzero s))) = 0 := by
  induction s using Multiset.induction_on with
  | empty => simp [rootProduct, function_divisor_one]
  | cons x s ih =>
    simpa only [rootProduct, Multiset.map_cons, Multiset.prod_cons] using
      polynomial_degree_mul W (X_sub_C_ne_zero x) (root_product_nonzero s)
        (vertical_polynomial_degree W hΔ x) ih

/-- Degree zero of every actual nonzero polynomial function, using actual geometric fibers. -/
theorem polynomial_divisor_degree_zero [IsAlgClosed K] (W : WeierstrassCurve.Affine K)
    (hΔ : W.Δ ≠ 0) {p : K[X]} (hp : p ≠ 0) :
    divisorDegree W (functionDivisor W (polynomialFunction W p)
      (polynomial_function_nonzero W hp)) = 0 := by
  have hc : p.leadingCoeff ≠ 0 := leadingCoeff_ne_zero.mpr hp
  have he := (IsAlgClosed.splits p).eq_prod_roots
  have hd := polynomial_degree_mul W (C_ne_zero.mpr hc) (root_product_nonzero p.roots)
    (constant_polynomial_degree_zero W hc) (root_product_degree W hΔ p.roots)
  have hm : polynomialFunction W (C p.leadingCoeff * rootProduct p.roots) = polynomialFunction W p :=
    congrArg (polynomialFunction W) he.symm
  have hD := function_divisor_congr W
    (polynomial_function_nonzero W (mul_ne_zero (C_ne_zero.mpr hc) (root_product_nonzero p.roots)))
    (polynomial_function_nonzero W hp) hm
  rw [hD] at hd
  exact hd

#print axioms polynomial_divisor_degree_zero
end
end GeometricPrincipalDivisorAudit
