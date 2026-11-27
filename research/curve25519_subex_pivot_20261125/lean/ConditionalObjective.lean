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

/-- Source-shaped strict-improvement fold used by the native first-minimum
selector. Equal costs retain the earlier candidate. -/
def firstMinimumFrom {q : ℕ} (cost : Fin q → ℕ) :
    Fin q → List (Fin q) → Fin q
  | best, [] => best
  | best, candidate :: candidates =>
      firstMinimumFrom cost
        (if cost candidate < cost best then candidate else best) candidates

private theorem firstMinimumFrom_le_best
    {q : ℕ} (cost : Fin q → ℕ) (best : Fin q)
    (candidates : List (Fin q)) :
    cost (firstMinimumFrom cost best candidates) ≤ cost best := by
  induction candidates generalizing best with
  | nil => simp [firstMinimumFrom]
  | cons candidate candidates ih =>
      rw [firstMinimumFrom]
      by_cases hcost : cost candidate < cost best
      · simp only [hcost, ↓reduceIte]
        exact (ih candidate).trans (Nat.le_of_lt hcost)
      · simp only [hcost, ↓reduceIte]
        exact ih best

private theorem firstMinimumFrom_le_of_mem
    {q : ℕ} (cost : Fin q → ℕ) (best candidate : Fin q)
    (candidates : List (Fin q)) (hmem : candidate ∈ candidates) :
    cost (firstMinimumFrom cost best candidates) ≤ cost candidate := by
  induction candidates generalizing best with
  | nil => simp at hmem
  | cons head tail ih =>
      rw [firstMinimumFrom]
      simp only [List.mem_cons] at hmem
      by_cases hcost : cost head < cost best
      · simp only [hcost, ↓reduceIte]
        rcases hmem with rfl | hmem
        · exact firstMinimumFrom_le_best cost candidate tail
        · exact ih head hmem
      · simp only [hcost, ↓reduceIte]
        rcases hmem with rfl | hmem
        · exact (firstMinimumFrom_le_best cost best tail).trans
            (Nat.le_of_not_lt hcost)
        · exact ih best hmem

/-- Execute the native ascending-label, strict-improvement first-minimum rule
over a nonempty label alphabet. -/
def firstMinimumLabel {q : ℕ} (hq : 0 < q)
    (cost : Fin q → ℕ) : Fin q :=
  firstMinimumFrom cost ⟨0, hq⟩ (List.finRange q)

private theorem firstMinimumFrom_affine
    {q : ℕ} (cost : Fin q → ℕ) (offset scale : ℕ)
    (hscale : 0 < scale) (best : Fin q) (candidates : List (Fin q)) :
    firstMinimumFrom (fun label => offset + scale * cost label) best candidates =
      firstMinimumFrom cost best candidates := by
  induction candidates generalizing best with
  | nil => rfl
  | cons candidate candidates ih =>
      simp only [firstMinimumFrom]
      have hcomparison :
          offset + scale * cost candidate < offset + scale * cost best ↔
            cost candidate < cost best := by
        rw [Nat.add_lt_add_iff_left, Nat.mul_lt_mul_left hscale]
      simp only [hcomparison]
      exact ih _

/-- Strict first-minimum selection is invariant under a label-independent
offset and positive scale, including its first-label tie rule. -/
theorem firstMinimumLabel_affine
    {q : ℕ} (hq : 0 < q) (cost : Fin q → ℕ)
    (offset scale : ℕ) (hscale : 0 < scale) :
    firstMinimumLabel hq (fun label => offset + scale * cost label) =
      firstMinimumLabel hq cost := by
  unfold firstMinimumLabel
  exact firstMinimumFrom_affine cost offset scale hscale _ _

/-- Any exact positive-affine score decomposition can replace the original
candidate cost without changing the strict first-minimum result. -/
theorem firstMinimumLabel_eq_of_affine
    {q : ℕ} (hq : 0 < q) (objective score : Fin q → ℕ)
    (offset scale : ℕ) (hscale : 0 < scale)
    (hobjective : ∀ label, objective label = offset + scale * score label) :
    firstMinimumLabel hq objective = firstMinimumLabel hq score := by
  have hfunctions : objective = fun label => offset + scale * score label := by
    funext label
    exact hobjective label
  rw [hfunctions]
  exact firstMinimumLabel_affine hq score offset scale hscale

/-- The executable first-minimum selector has cost no larger than every
candidate in the alphabet. -/
theorem firstMinimumLabel_le
    {q : ℕ} (hq : 0 < q) (cost : Fin q → ℕ) (candidate : Fin q) :
    cost (firstMinimumLabel hq cost) ≤ cost candidate := by
  apply firstMinimumFrom_le_of_mem
  simp

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

/-- Execute the first-minimum rule on the exact row-candidate objectives. -/
def firstMinimumRowLabel
    {q : ℕ} (hq : 0 < q)
    {Edge Row Column : Type*}
    [Fintype Edge]
    [DecidableEq Row]
    [DecidableEq Column]
    (left : Edge → Row)
    (right : Edge → Column)
    (rows : Row → Option (Fin q))
    (columns : Column → Option (Fin q))
    (vertex : Row) : Fin q :=
  firstMinimumLabel hq fun label =>
    conditionalObjective q left right
      (Function.update rows vertex (some label)) columns

/-- Replace a row candidate's quadratic objective by any exact positive-affine
score decomposition while preserving strict first-minimum tie breaking. -/
theorem firstMinimumRowLabel_eq_of_affine_score
    {q : ℕ} (hq : 0 < q)
    {Edge Row Column : Type*}
    [Fintype Edge]
    [DecidableEq Row]
    [DecidableEq Column]
    (left : Edge → Row)
    (right : Edge → Column)
    (rows : Row → Option (Fin q))
    (columns : Column → Option (Fin q))
    (vertex : Row) (score : Fin q → ℕ)
    (offset scale : ℕ) (hscale : 0 < scale)
    (hobjective : ∀ label,
      conditionalObjective q left right
          (Function.update rows vertex (some label)) columns =
        offset + scale * score label) :
    firstMinimumRowLabel hq left right rows columns vertex =
      firstMinimumLabel hq score := by
  unfold firstMinimumRowLabel
  exact firstMinimumLabel_eq_of_affine hq _ score offset scale hscale hobjective

/-- The executable first-minimum row choice discharges the abstract minimizer
premise and cannot increase the exact conditional objective. -/
theorem firstMinimumRowLabel_nonincreasing
    {q : ℕ} (hq : 0 < q)
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
    conditionalObjective q left right
        (Function.update rows vertex
          (some (firstMinimumRowLabel hq left right rows columns vertex))) columns ≤
      conditionalObjective q left right rows columns := by
  apply minimum_row_label_nonincreasing hq left right rows columns vertex _ hunknown
  intro label
  simpa only [firstMinimumRowLabel] using
    (firstMinimumLabel_le hq
      (fun candidate => conditionalObjective q left right
        (Function.update rows vertex (some candidate)) columns) label)

/-- Execute the first-minimum rule on the exact column-candidate objectives. -/
def firstMinimumColumnLabel
    {q : ℕ} (hq : 0 < q)
    {Edge Row Column : Type*}
    [Fintype Edge]
    [DecidableEq Row]
    [DecidableEq Column]
    (left : Edge → Row)
    (right : Edge → Column)
    (rows : Row → Option (Fin q))
    (columns : Column → Option (Fin q))
    (vertex : Column) : Fin q :=
  firstMinimumLabel hq fun label =>
    conditionalObjective q left right rows
      (Function.update columns vertex (some label))

/-- Replace a column candidate's quadratic objective by any exact
positive-affine score decomposition while preserving strict first-minimum tie
breaking. -/
theorem firstMinimumColumnLabel_eq_of_affine_score
    {q : ℕ} (hq : 0 < q)
    {Edge Row Column : Type*}
    [Fintype Edge]
    [DecidableEq Row]
    [DecidableEq Column]
    (left : Edge → Row)
    (right : Edge → Column)
    (rows : Row → Option (Fin q))
    (columns : Column → Option (Fin q))
    (vertex : Column) (score : Fin q → ℕ)
    (offset scale : ℕ) (hscale : 0 < scale)
    (hobjective : ∀ label,
      conditionalObjective q left right rows
          (Function.update columns vertex (some label)) =
        offset + scale * score label) :
    firstMinimumColumnLabel hq left right rows columns vertex =
      firstMinimumLabel hq score := by
  unfold firstMinimumColumnLabel
  exact firstMinimumLabel_eq_of_affine hq _ score offset scale hscale hobjective

/-- The executable first-minimum column choice discharges the abstract minimizer
premise and cannot increase the exact conditional objective. -/
theorem firstMinimumColumnLabel_nonincreasing
    {q : ℕ} (hq : 0 < q)
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
    conditionalObjective q left right rows
        (Function.update columns vertex
          (some (firstMinimumColumnLabel hq left right rows columns vertex))) ≤
      conditionalObjective q left right rows columns := by
  apply minimum_column_label_nonincreasing hq left right rows columns vertex _ hunknown
  intro label
  simpa only [firstMinimumColumnLabel] using
    (firstMinimumLabel_le hq
      (fun candidate => conditionalObjective q left right rows
        (Function.update columns vertex (some candidate))) label)

end Curve25519ConditionalObjective
