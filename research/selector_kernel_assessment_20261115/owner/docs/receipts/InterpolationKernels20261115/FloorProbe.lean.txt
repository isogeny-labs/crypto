import Mathlib.LinearAlgebra.Lagrange
import Mathlib.AlgebraicGeometry.EllipticCurve.Affine.Point
import Mathlib.Tactic

set_option autoImplicit false
open Polynomial

-- Inherited polynomial facts, before any new shared law.
example {F I : Type*} [Field F] (s : Finset I) (x : I → F) :
    (Lagrange.nodal s x).natDegree = s.card := Lagrange.natDegree_nodal

example {F I : Type*} [Field F] (s : Finset I) (x : I → F) :
    Lagrange.nodal s x ≠ 0 := Lagrange.nodal_ne_zero

example {F I : Type*} [Field F] (s : Finset I) (x : I → F) (i : I)
    (hi : i ∈ s) : (Lagrange.nodal s x).eval (x i) = 0 :=
  Lagrange.eval_nodal_at_node hi

example {F I : Type*} [Field F] (s : Finset I) (x : I → F)
    (hx : Function.Injective x) (j : I) (hj : j ∉ s) :
    (Lagrange.nodal s x).eval (x j) ≠ 0 := by
  apply Lagrange.eval_nodal_not_at_node
  intro i hi h
  exact hj ((hx h).symm ▸ hi)

example (t b : ℕ) : Fintype.card (Fin t → Fin b) = b ^ t := by simp

-- Actual Mathlib Weierstrass coordinates; the generic law does not need
-- the equation, so it applies unchanged when that stronger condition holds.
example {F I : Type*} [Field F] (W : WeierstrassCurve.Affine F)
    (q : I → F × F) (_hEq : ∀ i, W.Equation (q i).1 (q i).2)
    (hx : Function.Injective (fun i => (q i).1))
    (s : Finset I) (j : I) (hj : j ∉ s) :
    (Lagrange.nodal s (fun i => (q i).1)).eval (q j).1 ≠ 0 := by
  apply Lagrange.eval_nodal_not_at_node
  intro i hi h
  exact hj ((hx h).symm ▸ hi)

#check Finset.card_image_of_injective
#check Finset.card_image_iff
#check Fintype.card_fun
#print axioms Lagrange.natDegree_nodal
#print axioms Lagrange.eval_nodal_not_at_node
