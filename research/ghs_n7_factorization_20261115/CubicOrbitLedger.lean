import Init

/-!
Finite F₂[T]/(T³+T+1) orbit ledger. Element masks use the basis 1,T,T².
Multiplication by T is Frobenius on the relevant irreducible cubic module.
This checks the seven-cycle combinatorics and exact integer cofactors only.
It does not formalize Artin–Schreier covers, Hess's L-polynomial theorem,
Jacobian isogeny, or the measured Magma transfer.
-/

namespace GHSCubicOrbitLedger

def sigma : Fin 8 → Fin 8
  | 0 => 0
  | 1 => 2
  | 2 => 4
  | 3 => 6
  | 4 => 3
  | 5 => 1
  | 6 => 7
  | 7 => 5

def sigma7 (x : Fin 8) : Fin 8 :=
  sigma (sigma (sigma (sigma (sigma (sigma (sigma x))))))

theorem cubic_all_orbits_close : ∀ x : Fin 8, sigma7 x = x := by
  decide

theorem cubic_unique_fixed_point : ∀ x : Fin 8, sigma x = x ↔ x = 0 := by
  decide

theorem q4_order_and_projection :
    (16634 : Nat) * 2 = 33268 ∧ 33268 / 8317 = 4 ∧
    16634 = 2 * 8317 := by
  decide

theorem q16_order_and_projection :
    (268413186 : Nat) * 18 = 4831437348 ∧
    4831437348 / 44735531 = 108 ∧
    268413186 = 6 * 44735531 := by
  decide

#print axioms cubic_all_orbits_close
#print axioms cubic_unique_fixed_point
#print axioms q4_order_and_projection
#print axioms q16_order_and_projection

end GHSCubicOrbitLedger
