import Mathlib.Data.Finset.Card
import Mathlib.Data.Fintype.Prod
import Mathlib.Algebra.Group.Commute.Basic
import Mathlib.Algebra.Module.Submodule.Range
import Lean.Elab.Tactic.Omega

namespace Scaling61

/-- The first ramification cap is forced by the three smaller caps. -/
theorem balanced_first_bound (e0 e1 e2 e3 a b c : Int)
    (hs : e0 + e1 + e2 + e3 = 0)
    (h1 : -a ≤ e1 ∧ e1 ≤ a) (h2 : -b ≤ e2 ∧ e2 ≤ b)
    (h3 : -c ≤ e3 ∧ e3 ≤ c) : -(a+b+c) ≤ e0 ∧ e0 ≤ a+b+c := by
  omega

/-- Conditional map degree d=2 sum positive(e), uniformly in the pole caps. -/
theorem balanced_degree_bound (e0 e1 e2 e3 a b c : Int)
    (hs : e0 + e1 + e2 + e3 = 0)
    (h1 : -a ≤ e1 ∧ e1 ≤ a) (h2 : -b ≤ e2 ∧ e2 ≤ b)
    (h3 : -c ≤ e3 ∧ e3 ≤ c) :
    2 * (max e0 0 + max e1 0 + max e2 0 + max e3 0) ≤ 2*(a+b+c) := by
  omega

/-- Every minor triple fixes the first entry; erase the zero triple. -/
theorem minor_pattern_count (a b c : Nat) :
    ((Finset.univ : Finset (Fin (2*a+1) × Fin (2*b+1) × Fin (2*c+1))).erase
      (⟨a, by omega⟩, ⟨b, by omega⟩, ⟨c, by omega⟩)).card =
      (2*a+1)*(2*b+1)*(2*c+1)-1 := by
  rw [Finset.card_erase_of_mem (Finset.mem_univ _)]
  simp only [Finset.card_univ, Fintype.card_prod, Fintype.card_fin, Nat.mul_assoc]

/-- The cubic infinity bound uses6,10,1; its complete cohort has818members. -/
theorem cubic_patterns_818 :
    ((Finset.univ : Finset (Fin 13 × Fin 21 × Fin 3)).erase
      ((6 : Fin 13), (10 : Fin 21), (1 : Fin 3))).card = 818 := by
  simpa using minor_pattern_count 6 10 1

/-- Actual F32 elements satisfy x^32=x; inverse squaring is x maps to x^16.
The finite-field model and semilinear Cartier interpretation are external. -/
theorem coefficient_inverse_square {R : Type*} [Monoid R] (x : R)
    (h : x^32=x) : (x^16)^2=x := by
  calc
    (x^16)^2=x^32 := by rw [← pow_mul]
    _=x := h

/-- A nonzero Cartier-compatible functional obstructs surjectivity of C-I.
The AS/Serre geometry producing the functional is an external premise. -/
theorem eigenfunctional_blocks_surjectivity {K V : Type*} [CommRing K]
    [AddCommGroup V] [Module K V] (C : V →ₗ[K] V) (ell : V →ₗ[K] K)
    (h : ∀ v, ell (C v) = ell v) (w : V) (hw : ell w ≠ 0) :
    ¬ Function.Surjective (fun v => C v - v) := by
  intro hs
  obtain ⟨v, hv⟩ := hs w
  have hz : ell w = 0 := by
    rw [← hv, map_sub, h, sub_self]
  exact hw hz

end Scaling61
