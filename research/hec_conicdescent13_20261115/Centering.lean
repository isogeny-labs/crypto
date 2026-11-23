import Mathlib
namespace ConicDescent
/-- Scalar logarithmic identity, conditional on geometric identities. -/
theorem centered_conic_doubled {R : Type*} [Field R]
    (x e d beta k a b r : R)
    (hx : x = e - d + beta) (hb : 2 * beta = 2 * d - k)
    (hc : e + a + b + r = 2 * k) :
    2 * x = 3 * k - 2 * (a + b + r) := by
  linear_combination 2 * hx + hb + 2 * hc
/-- Division by two is valid in the odd-order log field. -/
theorem centered_conic {R : Type*} [Field R]
    (htwo : (2 : R) ≠ 0) (x e d beta k a b r : R)
    (hx : x = e - d + beta) (hb : 2 * beta = 2 * d - k)
    (hc : e + a + b + r = 2 * k) :
    x = 3 * k / 2 - (a + b + r) := by
  have hd := centered_conic_doubled x e d beta k a b r hx hb hc
  apply (mul_left_cancel₀ htwo)
  calc
    2 * x = 3 * k - 2 * (a + b + r) := hd
    _ = 2 * (3 * k / 2 - (a + b + r)) := by field_simp
end ConicDescent
#print axioms ConicDescent.centered_conic
