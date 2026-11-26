module
public import Mathlib.Tactic.LinearCombination
public import Mathlib.Algebra.Polynomial.Div
public import Mathlib.Tactic.FieldSimp

public section

namespace ReceivedIdeal

/- Instance obligation for the displayed received-symbol polynomial.
   No field/cardinality/compiler/decoder/cost theorem is asserted. -/
theorem received_quartic {R : Type*} [CommRing R] (A b u v w : R)
    (hu : u^2 + A*u + b = 0)
    (hv : v^2 = u^3 + A*u^2 + u) (hw : w = -u*v) :
    w^4 + A*(1-b)*(A^2-3*b)*w^2 + b^3*(1-b)^2 = 0 := by
  have hlin : w^2 = (1-b)*((A^2-b)*u + A*b) := by
    rw [hw]
    linear_combination u^2 * hv + (u^3 + (1-b)*(u-A)) * hu
  linear_combination
    (w^2 + (1-b)*((A^2-b)*u + A*b) + A*(1-b)*(A^2-3*b)) * hlin +
      ((1-b)*(A^2-b))^2 * hu

theorem cleared_quartic {R : Type*} [CommRing R] (A b J B w : R)
    (hb : J*b = B)
    (hf : w^4 + A*(1-b)*(A^2-3*b)*w^2 + b^3*(1-b)^2 = 0) :
    J^5*w^4 + A*J^3*(J-B)*(A^2*J-3*B)*w^2 + B^3*(J-B)^2 = 0 := by
  rw [← hb]
  calc
    J^5*w^4 + A*J^3*(J-J*b)*(A^2*J-3*(J*b))*w^2 +
        (J*b)^3*(J-J*b)^2 =
        J^5*(w^4 + A*(1-b)*(A^2-3*b)*w^2 + b^3*(1-b)^2) := by ring
    _ = 0 := by rw [hf, mul_zero]

theorem raw_branch_quad {K : Type*} [Field K] (A tau e : K)
    (ht : tau ≠ 0) (h2 : (2 : K) ≠ 0) (he : e^2 = 1) :
    ((-A + A*e/tau)/2)^2 + A*((-A + A*e/tau)/2) +
      A^2*(tau^2-1)/(4*tau^2) = 0 := by
  have h4 : (4 : K) ≠ 0 := by
    simpa only [show (2 : K) * 2 = 4 by ring] using mul_ne_zero h2 h2
  field_simp [ht, h2, h4]
  linear_combination (4*A^2) * he

theorem branch_factorization {R : Type*} [CommRing R] (A t w : R) :
    (4*t^2)^5*w^4 +
      A*(4*t^2)^3*(4*t^2-A^2*(t^2-1))*(A^2*(4*t^2)-3*(A^2*(t^2-1)))*w^2 +
      (A^2*(t^2-1))^3*(4*t^2-A^2*(t^2-1))^2 =
    (32*t^5*w^2-A^3*(1-t)^3*(4*t^2-A^2*(t^2-1))) *
      (32*t^5*w^2-A^3*(-1-t)^3*(4*t^2-A^2*(t^2-1))) := by
  ring

#print axioms received_quartic
#print axioms cleared_quartic
#print axioms raw_branch_quad
#print axioms branch_factorization

end ReceivedIdeal
