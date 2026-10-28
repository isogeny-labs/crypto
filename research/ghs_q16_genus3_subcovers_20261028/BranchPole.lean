/-!
The characteristic-two additive step in the order-three-pole obstruction.
This is abstract algebra: a characteristic-two field supplies these additive
axioms and injectivity of inversion on nonzero elements. It does not
formalize Laurent series, the cover geometry, or finite-field enumeration.
-/

universe u

namespace GHSQ16BranchPole

structure CharTwoAdd (F : Type u) where
  zero : F
  add : F → F → F
  add_assoc : ∀ x y z, add (add x y) z = add x (add y z)
  add_zero : ∀ x, add x zero = x
  zero_add : ∀ x, add zero x = x
  add_self : ∀ x, add x x = zero

theorem add_eq_zero_iff_eq {F : Type u} (A : CharTwoAdd F) (x y : F) :
    A.add x y = A.zero ↔ x = y := by
  constructor
  · intro h
    calc
      x = A.add x A.zero := (A.add_zero x).symm
      _ = A.add x (A.add x y) := by rw [h]
      _ = A.add (A.add x x) y := (A.add_assoc x x y).symm
      _ = A.add A.zero y := by rw [A.add_self]
      _ = y := A.zero_add y
  · intro h
    subst y
    exact A.add_self x

theorem inverse_sum_nonzero {F : Type u} (A : CharTwoAdd F)
    (inv : F → F) (h_injective : Function.Injective inv)
    (x y : F) (h_distinct : x ≠ y) :
    A.add (inv x) (inv y) ≠ A.zero := by
  intro hzero
  have h_inv : inv x = inv y := (add_eq_zero_iff_eq A (inv x) (inv y)).mp hzero
  exact h_distinct (h_injective h_inv)

theorem iterated_square_injective {F : Type u} (sq : F → F)
    (h_sq : Function.Injective sq) :
    Function.Injective (fun x => sq (sq (sq x))) := by
  intro x y h
  exact h_sq (h_sq (h_sq h))

end GHSQ16BranchPole
