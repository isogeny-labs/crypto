import Mathlib.Algebra.Polynomial.Degree.Domain
import Mathlib.RingTheory.Coprime.Basic

/-!
The degree/CRT uniqueness step used in the common 2+2 branch.
This lemma does not establish that the runtime residual and input curve
have the required residues or degree bound; those remain separate obligations.
-/

set_option autoImplicit false

namespace Common22Uniqueness

open Polynomial

variable {K : Type*} [Field K]

/-- A polynomial below the degree of two coprime supports is zero if
it vanishes modulo each support. -/
theorem zero_of_coprime_supports
    (u1 u2 h : K[X])
    (coprime : IsCoprime u1 u2)
    (mod_first : u1 ∣ h)
    (mod_second : u2 ∣ h)
    (small : degree h < degree (u1 * u2)) : h = 0 := by
  exact eq_zero_of_dvd_of_degree_lt (coprime.mul_dvd mod_first mod_second) small

/-- In the residual equation, the two proposed curve polynomials agree
once the cleared difference has the explicit support and degree facts. -/
theorem residual_eq_curve
    (u1 u2 residual curve scale : K[X])
    (coprime : IsCoprime u1 u2)
    (mod_first : u1 ∣ residual - scale * curve)
    (mod_second : u2 ∣ residual - scale * curve)
    (small : degree (residual - scale * curve) < degree (u1 * u2)) :
    residual = scale * curve := by
  exact sub_eq_zero.mp (zero_of_coprime_supports u1 u2
    (residual - scale * curve) coprime mod_first mod_second small)

end Common22Uniqueness

#print axioms Common22Uniqueness.zero_of_coprime_supports
#print axioms Common22Uniqueness.residual_eq_curve
