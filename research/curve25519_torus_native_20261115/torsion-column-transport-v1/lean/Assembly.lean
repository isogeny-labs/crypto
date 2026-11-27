module
public import Mathlib.Algebra.Group.Hom.Basic

-- Anonymous existing-law probes, not a native Curve25519 instance.
example {G H : Type*} [AddGroup G] [AddGroup H]
    (psi : G →+ H) (r r0 : G) : psi (r-r0) = 0 ↔ psi r = psi r0 := by
  rw [map_sub, sub_eq_zero]

example {G H : Type*} [AddGroup G] [AddGroup H]
    (psi : G →+ H) (r0 t : G) (ht : psi t = 0) : psi (r0+t) = psi r0 := by
  rw [map_add, ht, add_zero]

#check map_sub
#check map_add
#check sub_eq_zero
#print axioms map_sub
#print axioms map_add
