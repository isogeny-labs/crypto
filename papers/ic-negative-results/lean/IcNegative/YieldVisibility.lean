import IcNegative.YieldConservation

/-!
# Yield conservation does not imply solver visibility or distribution shape

The semantic decomposition map is `φ : τ → G`.  A solver exposes a subset of
its domain, represented by `accept`.  This module proves the generic subset
law and gives three finite countermodels used by the yield/rank frontier.
-/

namespace IcNegative

open Finset

variable {τ G : Type*} [Fintype τ] [DecidableEq G]

/-- Yield visible through a solver/filter predicate. -/
def visibleYieldAt (accept : τ → Prop) [DecidablePred accept]
    (φ : τ → G) (g : G) : ℕ :=
  (Finset.univ.filter fun t => accept t ∧ φ t = g).card

/-- A sound filter can only remove mathematical witnesses. -/
theorem visibleYieldAt_le_yieldAt (accept : τ → Prop) [DecidablePred accept]
    (φ : τ → G) (g : G) :
    visibleYieldAt accept φ g ≤ yieldAt φ g := by
  apply Finset.card_le_card
  intro t ht
  simp only [Finset.mem_filter, Finset.mem_univ, true_and] at ht ⊢
  exact ht.2

/-- Pointwise completeness is sufficient for solver-visible and mathematical
yield to agree target by target. -/
theorem visibleYieldAt_eq_of_complete (accept : τ → Prop) [DecidablePred accept]
    (φ : τ → G) (g : G) (hcomplete : ∀ t, accept t) :
    visibleYieldAt accept φ g = yieldAt φ g := by
  unfold visibleYieldAt yieldAt
  congr 1
  ext t
  simp [hcomplete]

namespace Countermodel

def constantFalse : Bool → Bool := fun _ => false

def identity : Bool → Bool := fun b => b

/-- The constant and identity maps have the same conserved total but different
coverage and target-wise fibres.  Hence the global mean does not determine the
zero mass or distribution shape. -/
theorem same_total_different_coverage :
    (∑ g, yieldAt constantFalse g) = ∑ g, yieldAt identity g ∧
    coverage constantFalse = 1 ∧ coverage identity = 2 ∧
    yieldAt constantFalse true = 0 ∧ yieldAt identity true = 1 := by
  decide

/-- An incomplete solver can report zero visible yield at a target that has a
mathematical witness. -/
theorem reject_all_hides_real_witness :
    visibleYieldAt (fun _ : Bool => False) identity false = 0 ∧
    yieldAt identity false = 1 := by
  decide

def populationFailures : ℕ :=
  (Finset.univ.filter fun b : Bool => b = false).card

def successOnlyFailures : ℕ :=
  ((Finset.univ.filter fun b : Bool => b = true).filter fun b => b = false).card

/-- Retaining successes only erases a real failure from the observed sample. -/
theorem success_only_censors_failure :
    populationFailures = 1 ∧ successOnlyFailures = 0 := by
  decide

end Countermodel

end IcNegative
