import Init

/-!
Finite algebra check for the irreducible cubic T^3+T+1. The theorem
checks the eight elements of F_2[T]/(T^3+T+1) using kernel reduction.
The identification of the quotient's deck-character endomorphism ring
with this field, and the curve geometry, are mathematical arguments in
THEOREM.md and are not formalized here.
-/

namespace GHSN7NaturalQuotient

def xtime (a : Nat) : Nat :=
  if a < 4 then 2 * a else Nat.xor (2 * a) 11

def mul8 (a b : Fin 8) : Nat :=
  let a1 := xtime a.val
  let a2 := xtime a1
  Nat.xor
    (Nat.xor (if b.val % 2 = 1 then a.val else 0)
             (if b.val / 2 % 2 = 1 then a1 else 0))
    (if b.val / 4 = 1 then a2 else 0)

theorem square_one_only_one :
    ∀ x : Fin 8, mul8 x x = 1 → x.val = 1 := by
  decide

end GHSN7NaturalQuotient
