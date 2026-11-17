import Mathlib.Tactic.Ring

/-! Exact commutative-ring identities for the secant presentation.
    These statements do not assume, or assert, an elliptic-curve group law. -/
namespace SecantIdentity
variable {K : Type*} [CommRing K]

/-- The existing IDC1h621ed1a51b5f3ca8 identity, now proved in Lean. -/
theorem cubic_identity (A B r s x z m T : K) :
    T^3 + A*T + B - (m*(T-r)-s)^2 =
      (T-r)*((T-x)*(T-z) - T*(m^2-x-z-r) -
        (x*z-r^2-r*m^2-A-2*m*s)) - (s^2-r^3-A*r-B) := by
  ring

/-- Vanishing of the actual three residuals gives the cubic factorization. -/
theorem intersection_factorization (A B r s x z m T : K)
    (hcurve : s^2-r^3-A*r-B = 0)
    (hsum : m^2-x-z-r = 0)
    (hprod : x*z-r^2-r*m^2-A-2*m*s = 0) :
    T^3 + A*T + B - (m*(T-r)-s)^2 = (T-r)*(T-x)*(T-z) := by
  rw [cubic_identity, hcurve, hsum, hprod]
  ring

/-- Each selected ordinate is genuinely on the curve, even on the diagonal. -/
theorem selected_point_on_curve (A B r s x z m y : K)
    (hcurve : s^2-r^3-A*r-B = 0)
    (hsum : m^2-x-z-r = 0)
    (hprod : x*z-r^2-r*m^2-A-2*m*s = 0)
    (hy : y = m*(x-r)-s) :
    y^2 = x^3 + A*x + B := by
  have h := intersection_factorization A B r s x z m x hcurve hsum hprod
  rw [sub_self, mul_zero, zero_mul] at h
  rw [hy]
  exact (sub_eq_zero.mp h).symm

end SecantIdentity

#print axioms SecantIdentity.cubic_identity
#print axioms SecantIdentity.intersection_factorization
#print axioms SecantIdentity.selected_point_on_curve
