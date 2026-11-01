/-!
# The projected degree-one plus degree-two fiber identity

The computational certificate shows that the two closed components of a
generic odd-degree fiber push to `P` and `[2]P`.  Their total therefore gives
only the group-law identity `[3]P = P + [2]P`.  This file formalizes that
last unconditional algebraic step.  It does not formalize the finite-field
factorization or the Rust norm implementation.
-/

set_option autoImplicit false

namespace N83ImageClosedDegreeTwo

structure AdditiveSemigroup (A : Type) where
  add : A → A → A
  add_assoc : ∀ a b c, add (add a b) c = add a (add b c)

variable {A : Type} (S : AdditiveSemigroup A)

local infixl:65 " +ₛ " => S.add

def twice (point : A) : A := point +ₛ point

def thrice (point : A) : A := point +ₛ (point +ₛ point)

/-- A projected `1+2` closed-point fiber contributes the tautological scalar
identity, not a new linear relation. -/
theorem degree_one_plus_degree_two (point : A) :
    thrice S point = point +ₛ twice S point := by
  rfl

end N83ImageClosedDegreeTwo

#print axioms N83ImageClosedDegreeTwo.degree_one_plus_degree_two
