import Std

/- These are recorded software-work counts, not mathematical prime
certificates or measurements of the original generator. The bridge from
transcripts to these constants is the separate native 210-cell check. -/
namespace NistWorkLimits

abbrev Counts := Fin 10 → Nat

def vector (a b c d e f g h i j : Nat) : Counts := fun k =>
  match k.val with
  | 0 => a | 1 => b | 2 => c | 3 => d | 4 => e
  | 5 => f | 6 => g | 7 => h | 8 => i | _ => j

def skipped1562 : Counts := vector 1 5 31 1 26 14 8 52 42 0
def skipped2682 : Counts := vector 1 9 42 1 33 18 9 55 43 0
def published2185 : Counts := vector 2 19 77 2 46 20 12 55 86 0
def published3048 : Counts := vector 10 10 63 5 52 27 20 59 92 0

def CountsLE (a b : Counts) : Prop := ∀ k, a k ≤ b k

def MonotoneCost (cost : Counts → Nat) : Prop :=
  ∀ a b, CountsLE a b → cost a ≤ cost b

theorem monotone_budget (cost : Counts → Nat) (limit : Nat)
    (a b : Counts) (monotone : MonotoneCost cost)
    (dominated : CountsLE a b) (published_kept : cost b ≤ limit) :
    cost a ≤ limit :=
  Nat.le_trans (monotone a b dominated) published_kept

theorem skipped1562_le_2185 : CountsLE skipped1562 published2185 := by
  unfold CountsLE
  decide +kernel

theorem skipped1562_le_3048 : CountsLE skipped1562 published3048 := by
  unfold CountsLE
  decide +kernel

theorem skipped2682_le_2185 : CountsLE skipped2682 published2185 := by
  unfold CountsLE
  decide +kernel

theorem skipped2682_le_3048 : CountsLE skipped2682 published3048 := by
  unfold CountsLE
  decide +kernel

theorem budget_keeps_both (cost : Counts → Nat) (limit : Nat)
    (monotone : MonotoneCost cost) (published_kept : cost published2185 ≤ limit) :
    cost skipped1562 ≤ limit ∧ cost skipped2682 ≤ limit := by
  exact ⟨monotone_budget cost limit skipped1562 published2185 monotone
      skipped1562_le_2185 published_kept,
    monotone_budget cost limit skipped2682 published2185 monotone
      skipped2682_le_2185 published_kept⟩

/- This form also covers non-integer costs. Transitivity and monotonicity
are explicit model assumptions, not properties asserted of historical time. -/
theorem ordered_budget {α : Type} (rel : α → α → Prop)
    (transitive : ∀ x y z, rel x y → rel y z → rel x z)
    (cost : Counts → α) (limit : α) (a b : Counts)
    (monotone : ∀ u v, CountsLE u v → rel (cost u) (cost v))
    (dominated : CountsLE a b) (published_kept : rel (cost b) limit) :
    rel (cost a) limit :=
  transitive (cost a) (cost b) limit (monotone a b dominated) published_kept

theorem ordered_budget_keeps_both {α : Type} (rel : α → α → Prop)
    (transitive : ∀ x y z, rel x y → rel y z → rel x z)
    (cost : Counts → α) (limit : α)
    (monotone : ∀ u v, CountsLE u v → rel (cost u) (cost v))
    (published_kept : rel (cost published2185) limit) :
    rel (cost skipped1562) limit ∧ rel (cost skipped2682) limit := by
  exact ⟨ordered_budget rel transitive cost limit skipped1562 published2185
      monotone skipped1562_le_2185 published_kept,
    ordered_budget rel transitive cost limit skipped2682 published2185
      monotone skipped2682_le_2185 published_kept⟩

#print axioms monotone_budget
#print axioms skipped1562_le_2185
#print axioms skipped1562_le_3048
#print axioms skipped2682_le_2185
#print axioms skipped2682_le_3048
#print axioms budget_keeps_both
#print axioms ordered_budget
#print axioms ordered_budget_keeps_both

end NistWorkLimits
