module

public import ConditionalVariance

@[expose] public section

/-!
# Exact partial-label collision objective

This module models the scaled quadratic reference objective used by the native
first-minimum labeler.  It proves that averaging over every candidate label for
one previously unassigned vertex recovers the preceding objective, and hence
that at least one candidate is nonincreasing.
-/

open scoped BigOperators

namespace Curve25519ConditionalObjective

/-- Scaled conditional equality potential for one pair of endpoints.  Equal
endpoints contribute `q`; distinct fixed endpoints contribute `q` or zero;
and a comparison with an unknown label contributes one. -/
def equalityPotential
    (q : ℕ)
    {Vertex : Type*}
    [DecidableEq Vertex]
    (first second : Vertex)
    (assignment : Vertex → Option (Fin q)) : ℕ :=
  if first = second then
    q
  else
    match assignment first, assignment second with
    | some x, some y => if x = y then q else 0
    | _, _ => 1

/-- Assigning every possible label to one previously unknown vertex preserves
the old equality potential on average. -/
theorem equalityPotential_update_sum
    {q : ℕ}
    {Vertex : Type*}
    [DecidableEq Vertex]
    (assignment : Vertex → Option (Fin q))
    (vertex first second : Vertex)
    (hunknown : assignment vertex = none) :
    (∑ label : Fin q,
      equalityPotential q first second (Function.update assignment vertex (some label))) =
      q * equalityPotential q first second assignment := by
  classical
  by_cases hsame : first = second
  · simp [equalityPotential, hsame]
  by_cases hfirst : first = vertex
  · subst first
    have hsecond : second ≠ vertex := by
      intro h
      apply hsame
      simp [h]
    cases hother : assignment second with
    | none => simp [equalityPotential, hsame, hsecond, hunknown, hother]
    | some other => simp [equalityPotential, hsame, hsecond, hunknown, hother]
  by_cases hsecond : second = vertex
  · subst second
    cases hother : assignment first with
    | none => simp [equalityPotential, hsame, hunknown, hother]
    | some other => simp [equalityPotential, hsame, hunknown, hother]
  · simp [equalityPotential, hsame, hfirst, hsecond]

/-- The exact scaled conditional collision objective for ordered edge pairs in
a bipartite multigraph.  Distinct edge identifiers may share both endpoints,
so parallel edges are represented without a special case. -/
def conditionalObjective
    (q : ℕ)
    {Edge Row Column : Type*}
    [Fintype Edge]
    [DecidableEq Row]
    [DecidableEq Column]
    (left : Edge → Row)
    (right : Edge → Column)
    (rows : Row → Option (Fin q))
    (columns : Column → Option (Fin q)) : ℕ :=
  ∑ first : Edge, ∑ second : Edge,
    equalityPotential q (left first) (left second) rows *
      equalityPotential q (right first) (right second) columns

/-- Averaging all candidates for one unknown row vertex recovers `q` times the
old objective. -/
theorem row_update_objective_sum
    {q : ℕ}
    {Edge Row Column : Type*}
    [Fintype Edge]
    [DecidableEq Row]
    [DecidableEq Column]
    (left : Edge → Row)
    (right : Edge → Column)
    (rows : Row → Option (Fin q))
    (columns : Column → Option (Fin q))
    (vertex : Row)
    (hunknown : rows vertex = none) :
    (∑ label : Fin q,
      conditionalObjective q left right (Function.update rows vertex (some label)) columns) =
      q * conditionalObjective q left right rows columns := by
  classical
  unfold conditionalObjective
  calc
    (∑ label : Fin q, ∑ first : Edge, ∑ second : Edge,
      equalityPotential q (left first) (left second)
          (Function.update rows vertex (some label)) *
        equalityPotential q (right first) (right second) columns) =
        ∑ first : Edge, ∑ second : Edge, ∑ label : Fin q,
          equalityPotential q (left first) (left second)
              (Function.update rows vertex (some label)) *
            equalityPotential q (right first) (right second) columns := by
              rw [Finset.sum_comm]
              apply Finset.sum_congr rfl
              intro first _
              rw [Finset.sum_comm]
    _ = ∑ first : Edge, ∑ second : Edge,
        (q * equalityPotential q (left first) (left second) rows) *
          equalityPotential q (right first) (right second) columns := by
            apply Finset.sum_congr rfl
            intro first _
            apply Finset.sum_congr rfl
            intro second _
            rw [← Finset.sum_mul]
            rw [equalityPotential_update_sum rows vertex (left first) (left second) hunknown]
    _ = q * (∑ first : Edge, ∑ second : Edge,
        equalityPotential q (left first) (left second) rows *
          equalityPotential q (right first) (right second) columns) := by
            rw [Finset.mul_sum]
            apply Finset.sum_congr rfl
            intro first _
            rw [Finset.mul_sum]
            apply Finset.sum_congr rfl
            intro second _
            ring

/-- Averaging all candidates for one unknown column vertex recovers `q` times
the old objective. -/
theorem column_update_objective_sum
    {q : ℕ}
    {Edge Row Column : Type*}
    [Fintype Edge]
    [DecidableEq Row]
    [DecidableEq Column]
    (left : Edge → Row)
    (right : Edge → Column)
    (rows : Row → Option (Fin q))
    (columns : Column → Option (Fin q))
    (vertex : Column)
    (hunknown : columns vertex = none) :
    (∑ label : Fin q,
      conditionalObjective q left right rows
        (Function.update columns vertex (some label))) =
      q * conditionalObjective q left right rows columns := by
  classical
  unfold conditionalObjective
  calc
    (∑ label : Fin q, ∑ first : Edge, ∑ second : Edge,
      equalityPotential q (left first) (left second) rows *
        equalityPotential q (right first) (right second)
          (Function.update columns vertex (some label))) =
        ∑ first : Edge, ∑ second : Edge, ∑ label : Fin q,
          equalityPotential q (left first) (left second) rows *
            equalityPotential q (right first) (right second)
              (Function.update columns vertex (some label)) := by
                rw [Finset.sum_comm]
                apply Finset.sum_congr rfl
                intro first _
                rw [Finset.sum_comm]
    _ = ∑ first : Edge, ∑ second : Edge,
        equalityPotential q (left first) (left second) rows *
          (q * equalityPotential q (right first) (right second) columns) := by
            apply Finset.sum_congr rfl
            intro first _
            apply Finset.sum_congr rfl
            intro second _
            rw [← Finset.mul_sum]
            rw [equalityPotential_update_sum columns vertex (right first) (right second)
              hunknown]
    _ = q * (∑ first : Edge, ∑ second : Edge,
        equalityPotential q (left first) (left second) rows *
          equalityPotential q (right first) (right second) columns) := by
            rw [Finset.mul_sum]
            apply Finset.sum_congr rfl
            intro first _
            rw [Finset.mul_sum]
            apply Finset.sum_congr rfl
            intro second _
            ring

/-- If the sum of `q` candidate costs is `q` times the old cost, at least one
candidate is nonincreasing. -/
theorem exists_candidate_le_of_sum_eq
    {q : ℕ}
    (hq : 0 < q)
    (cost : Fin q → ℕ)
    (old : ℕ)
    (haverage : ∑ label, cost label = q * old) :
    ∃ label, cost label ≤ old := by
  by_contra h
  push Not at h
  let witness : Fin q := ⟨0, hq⟩
  have hstrict : (∑ _label : Fin q, old) < ∑ label : Fin q, cost label := by
    exact Finset.sum_lt_sum
      (fun label _ ↦ (h label).le)
      ⟨witness, Finset.mem_univ witness, h witness⟩
  simp only [Finset.sum_const, Finset.card_univ, Fintype.card_fin, nsmul_eq_mul] at hstrict
  rw [haverage] at hstrict
  exact (lt_irrefl _ hstrict)

/-- One row step of the exact conditional objective has a nonincreasing label
candidate. -/
theorem exists_nonincreasing_row_label
    {q : ℕ}
    (hq : 0 < q)
    {Edge Row Column : Type*}
    [Fintype Edge]
    [DecidableEq Row]
    [DecidableEq Column]
    (left : Edge → Row)
    (right : Edge → Column)
    (rows : Row → Option (Fin q))
    (columns : Column → Option (Fin q))
    (vertex : Row)
    (hunknown : rows vertex = none) :
    ∃ label,
      conditionalObjective q left right (Function.update rows vertex (some label)) columns ≤
        conditionalObjective q left right rows columns := by
  apply exists_candidate_le_of_sum_eq hq
  exact row_update_objective_sum left right rows columns vertex hunknown

/-- One column step of the exact conditional objective has a nonincreasing
label candidate. -/
theorem exists_nonincreasing_column_label
    {q : ℕ}
    (hq : 0 < q)
    {Edge Row Column : Type*}
    [Fintype Edge]
    [DecidableEq Row]
    [DecidableEq Column]
    (left : Edge → Row)
    (right : Edge → Column)
    (rows : Row → Option (Fin q))
    (columns : Column → Option (Fin q))
    (vertex : Column)
    (hunknown : columns vertex = none) :
    ∃ label,
      conditionalObjective q left right rows
          (Function.update columns vertex (some label)) ≤
        conditionalObjective q left right rows columns := by
  apply exists_candidate_le_of_sum_eq hq
  exact column_update_objective_sum left right rows columns vertex hunknown

/-- Any selected row label whose objective is minimal among all candidates is
nonincreasing.  The native first-minimum tie rule is stronger than `hminimum`
but is irrelevant to the bound. -/
theorem minimum_row_label_nonincreasing
    {q : ℕ}
    (hq : 0 < q)
    {Edge Row Column : Type*}
    [Fintype Edge]
    [DecidableEq Row]
    [DecidableEq Column]
    (left : Edge → Row)
    (right : Edge → Column)
    (rows : Row → Option (Fin q))
    (columns : Column → Option (Fin q))
    (vertex : Row)
    (selected : Fin q)
    (hunknown : rows vertex = none)
    (hminimum : ∀ label,
      conditionalObjective q left right (Function.update rows vertex (some selected)) columns ≤
        conditionalObjective q left right (Function.update rows vertex (some label)) columns) :
    conditionalObjective q left right (Function.update rows vertex (some selected)) columns ≤
      conditionalObjective q left right rows columns := by
  obtain ⟨candidate, hcandidate⟩ :=
    exists_nonincreasing_row_label hq left right rows columns vertex hunknown
  exact (hminimum candidate).trans hcandidate

/-- Any selected column label whose objective is minimal among all candidates
is nonincreasing. -/
theorem minimum_column_label_nonincreasing
    {q : ℕ}
    (hq : 0 < q)
    {Edge Row Column : Type*}
    [Fintype Edge]
    [DecidableEq Row]
    [DecidableEq Column]
    (left : Edge → Row)
    (right : Edge → Column)
    (rows : Row → Option (Fin q))
    (columns : Column → Option (Fin q))
    (vertex : Column)
    (selected : Fin q)
    (hunknown : columns vertex = none)
    (hminimum : ∀ label,
      conditionalObjective q left right rows
          (Function.update columns vertex (some selected)) ≤
        conditionalObjective q left right rows
          (Function.update columns vertex (some label))) :
    conditionalObjective q left right rows
        (Function.update columns vertex (some selected)) ≤
      conditionalObjective q left right rows columns := by
  obtain ⟨candidate, hcandidate⟩ :=
    exists_nonincreasing_column_label hq left right rows columns vertex hunknown
  exact (hminimum candidate).trans hcandidate

end Curve25519ConditionalObjective
