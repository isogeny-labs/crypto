import Mathlib.RingTheory.Polynomial.Chebyshev

set_option autoImplicit false

namespace Curve25519TraceDegree
open Polynomial

example (F : Type*) [Field F] [NeZero (2 : F)] (q : ℤ) :
    (Chebyshev.T F q).natDegree = q.natAbs := Chebyshev.natDegree_T F q

example (F : Type*) [Field F] [NeZero (2 : F)] (q : ℕ) :
    (Chebyshev.U F q).natDegree = q := Chebyshev.natDegree_U_natCast F q

example (F : Type*) [CommRing F] (q : ℤ) :
    derivative (Chebyshev.T F q) = q * Chebyshev.U F (q-1) :=
  Chebyshev.T_derivative_eq_U (R := F) q

theorem actual_trace_degree (F : Type*) [Field F] [NeZero (2 : F)] :
    (Chebyshev.T F 213156431).natDegree = 213156431 :=
  Chebyshev.natDegree_T F 213156431

theorem actual_lift_u_degree (F : Type*) [Field F] [NeZero (2 : F)] :
    (Chebyshev.U F 213156430).natDegree = 213156430 :=
  Chebyshev.natDegree_U_natCast F 213156430

theorem actual_derivative_identity (F : Type*) [CommRing F] :
    derivative (Chebyshev.T F 213156431) =
      ((213156431 : ℤ) : F[X]) * Chebyshev.U F ((213156431 : ℤ) - 1) :=
  Chebyshev.T_derivative_eq_U (R := F) 213156431

#print axioms actual_trace_degree
#print axioms actual_lift_u_degree
#print axioms actual_derivative_identity
end Curve25519TraceDegree
