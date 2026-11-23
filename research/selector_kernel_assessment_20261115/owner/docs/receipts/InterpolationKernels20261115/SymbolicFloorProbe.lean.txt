import Mathlib.LinearAlgebra.Lagrange
import Mathlib.Tactic
set_option autoImplicit false
open scoped BigOperators
open Polynomial

example {F : Type*} [Field F] (b : ℕ) (f : Fin b → F) :
    (∏ j, f j) = 0 ↔ ∃ j, f j = 0 := by
  simp only [Finset.prod_eq_zero_iff, Finset.mem_univ, true_and]

example {F : Type*} [Field F] (t b : ℕ) (f : Fin t → Fin b → F)
    (h : ∀ i, ∃ j, f i j = 0) :
    ∃ c : Fin t → Fin b, ∀ i, f i (c i) = 0 :=
  Classical.axiomOfChoice h

example {F : Type*} [Field F] (t b : ℕ) (x y : Fin t × Fin b → F)
    (u v : F[X]) (i : Fin t) :
    (∏ j, (u.eval (x (i,j)) + y (i,j) * v.eval (x (i,j)))) = 0 ↔
      ∃ j, u.eval (x (i,j)) + y (i,j) * v.eval (x (i,j)) = 0 := by
  simp only [Finset.prod_eq_zero_iff, Finset.mem_univ, true_and]

#check Equiv.ofInjective
#check Nat.card_congr
#check Finset.prod_eq_zero_iff
