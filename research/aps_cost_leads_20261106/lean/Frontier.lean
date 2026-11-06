import Std

namespace APSLeads

/-- A finite tuple outside the old product has a first old-coordinate failure. -/
theorem first_failure (P : Nat → Prop) : ∀ n,
    (¬ ∀ i, i < n → P i) →
    ∃ j, j < n ∧ ¬ P j ∧ ∀ i, i < j → P i := by
  classical
  intro n
  induction n with
  | zero =>
      intro h
      exact False.elim (h (by intro i hi; omega))
  | succ n ih =>
      intro h
      by_cases hp : ∀ i, i < n → P i
      · refine ⟨n, by omega, ?_, hp⟩
        intro hn
        apply h
        intro i hi
        by_cases hil : i < n
        · exact hp i hil
        · have heq : i = n := by omega
          simpa [heq] using hn
      · obtain ⟨j, hj, hnot, hpre⟩ := ih hp
        exact ⟨j, by omega, hnot, hpre⟩

/-- First-new-coordinate cell of a Cartesian product of nested domains. -/
def Region (old new : Nat → Prop) (n j : Nat) : Prop :=
  j < n ∧ (∀ i, i < j → old i) ∧ new j ∧ ¬ old j ∧
    (∀ i, j < i → i < n → new i)

/-- The new product minus the old product is exactly the frontier union.
`old i` and `new i` denote membership of the tuple's coordinate in that
partition. The subset hypothesis is precisely the builder's nesting check. -/
theorem frontier_cover (old new : Nat → Prop) (n : Nat)
    (subset : ∀ i, i < n → old i → new i) :
    ((∀ i, i < n → new i) ∧ ¬ (∀ i, i < n → old i)) ↔
    ∃ j, Region old new n j := by
  classical
  constructor
  · rintro ⟨hnew, hold⟩
    obtain ⟨j, hj, hnot, hpre⟩ := first_failure old n hold
    exact ⟨j, hj, hpre, hnew j hj, hnot, by intro i _ hi; exact hnew i hi⟩
  · rintro ⟨j, hj, hpre, hnew, hnot, hafter⟩
    constructor
    · intro i hi
      by_cases before : i < j
      · exact subset i hi (hpre i before)
      · by_cases equal : i = j
        · simpa [equal] using hnew
        · exact hafter i (by omega) hi
    · intro hall
      exact hnot (hall j hj)

/-- Different first-new-coordinate cells are disjoint. -/
theorem frontier_unique (old new : Nat → Prop) (n j k : Nat)
    (a : Region old new n j) (b : Region old new n k) : j = k := by
  rcases a with ⟨_, ja, _, jnot, _⟩
  rcases b with ⟨_, ka, _, knot, _⟩
  by_cases h : j = k
  · exact h
  · by_cases lt : j < k
    · exact False.elim (jnot (ka j lt))
    · exact False.elim (knot (ja k (by omega)))

#print axioms first_failure
#print axioms frontier_cover
#print axioms frontier_unique

end APSLeads
