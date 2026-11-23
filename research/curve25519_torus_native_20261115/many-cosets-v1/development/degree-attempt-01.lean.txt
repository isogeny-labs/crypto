import CosetInstance
import Mathlib.Algebra.Polynomial.Roots

set_option autoImplicit false

namespace Curve25519CosetInstance

theorem membership_polynomial_degree {F : Type*} [Field F] (h : F)
    (hh : IsPrimitiveRoot h 12965132697890207) (p : Polynomial F) (hp : p ≠ 0)
    (heval : ∀ i : Fin 20633 × Fin 213156431,
      p.eval (h^(i.1.val+1+60824497*i.2.val)) = 0) :
    4398056640823 ≤ p.natDegree := by
  by_contra hc
  apply hp
  apply Polynomial.eq_zero_of_natDegree_lt_card_of_eval_eq_zero p
    (indexed_injective h hh) heval
  simpa using (show p.natDegree < 4398056640823 from Nat.lt_of_not_ge hc)

#print axioms membership_polynomial_degree
end Curve25519CosetInstance
