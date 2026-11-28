module

public import ConditionalObjective

@[expose] public section

/-!
# Sparse candidate-score refinement

The native conditional labeler drops every candidate-independent term from the
quadratic objective.  This module isolates the pointwise equality-potential
decomposition used to justify that sparse score.
-/

namespace Curve25519SparseLabelScore

open Curve25519ConditionalObjective
open scoped BigOperators

/-- The scaled equality potential is symmetric in its two endpoints. -/
theorem equalityPotential_comm
    (q : ℕ)
    {Vertex : Type*}
    [DecidableEq Vertex]
    (assignment : Vertex → Option (Fin q))
    (first second : Vertex) :
    equalityPotential q first second assignment =
      equalityPotential q second first assignment := by
  by_cases hsame : first = second
  · subst second
    rfl
  have hreverse : second ≠ first := Ne.symm hsame
  cases hfirst : assignment first <;>
    cases hsecond : assignment second <;>
    simp [equalityPotential, hsame, hreverse, hfirst, hsecond, eq_comm]

/-- Swapping the bipartite sides preserves the quadratic objective. -/
theorem conditionalObjective_swap
    (q : ℕ)
    {Edge Row Column : Type*}
    [Fintype Edge]
    [DecidableEq Row]
    [DecidableEq Column]
    (left : Edge → Row)
    (right : Edge → Column)
    (rows : Row → Option (Fin q))
    (columns : Column → Option (Fin q)) :
    conditionalObjective q left right rows columns =
      conditionalObjective q right left columns rows := by
  unfold conditionalObjective
  apply Finset.sum_congr rfl
  intro first _
  apply Finset.sum_congr rfl
  intro second _
  exact Nat.mul_comm _ _

/-- Candidate-independent equality potential after reserving one vertex for a
new label.  Comparisons with an unknown other endpoint retain their unit cost;
comparisons with an already labeled endpoint contribute through the separate
interaction term below. -/
def equalityPotentialOffset
    (q : ℕ)
    {Vertex : Type*}
    [DecidableEq Vertex]
    (assignment : Vertex → Option (Fin q))
    (vertex first second : Vertex) : ℕ :=
  if first = second then q
  else if first = vertex then
    match assignment second with
    | none => 1
    | some _ => 0
  else if second = vertex then
    match assignment first with
    | none => 1
    | some _ => 0
  else equalityPotential q first second assignment

/-- One directed interaction between the newly labeled vertex and an already
labeled distinct endpoint with the selected label. -/
def selectedEqualityInteraction
    (q : ℕ)
    {Vertex : Type*}
    [DecidableEq Vertex]
    (assignment : Vertex → Option (Fin q))
    (vertex : Vertex) (label : Fin q)
    (first second : Vertex) : ℕ :=
  if first = vertex ∧ second ≠ vertex ∧ assignment second = some label then q else 0

/-- Updating one vertex splits its equality potential into a
candidate-independent offset and the two directed selected-label
interactions. -/
theorem equalityPotential_update_decomposition
    (q : ℕ)
    {Vertex : Type*}
    [DecidableEq Vertex]
    (assignment : Vertex → Option (Fin q))
    (vertex : Vertex) (label : Fin q)
    (first second : Vertex) :
    equalityPotential q first second
        (Function.update assignment vertex (some label)) =
      equalityPotentialOffset q assignment vertex first second +
        selectedEqualityInteraction q assignment vertex label first second +
        selectedEqualityInteraction q assignment vertex label second first := by
  classical
  by_cases hsame : first = second
  · subst second
    by_cases hvertex : first = vertex <;>
      simp [equalityPotential, equalityPotentialOffset,
        selectedEqualityInteraction, hvertex]
  by_cases hfirst : first = vertex
  · subst first
    have hsecond : second ≠ vertex := by
      intro h
      apply hsame
      exact h.symm
    cases hother : assignment second with
    | none =>
        simp [equalityPotential, equalityPotentialOffset,
          selectedEqualityInteraction, hsame, hsecond, hother]
    | some other =>
        by_cases hequal : other = label
        · subst other
          simp [equalityPotential, equalityPotentialOffset,
            selectedEqualityInteraction, hsame, hsecond, hother]
        · simp [equalityPotential, equalityPotentialOffset,
            selectedEqualityInteraction, hsame, hsecond, hother,
            hequal, Ne.symm hequal]
  by_cases hsecond : second = vertex
  · subst second
    cases hother : assignment first with
    | none =>
        simp [equalityPotential, equalityPotentialOffset,
          selectedEqualityInteraction, hsame, hother]
    | some other =>
        by_cases hequal : other = label
        · subst other
          simp [equalityPotential, equalityPotentialOffset,
            selectedEqualityInteraction, hsame, hother]
        · simp [equalityPotential, equalityPotentialOffset,
            selectedEqualityInteraction, hsame, hother, hequal]
  · simp [equalityPotential, equalityPotentialOffset,
      selectedEqualityInteraction, hsame, hfirst, hsecond]

/-- Candidate-independent portion of one row update in the full quadratic
objective. -/
def rowCandidateOffset
    (q : ℕ)
    {Edge Row Column : Type*}
    [Fintype Edge]
    [DecidableEq Row]
    [DecidableEq Column]
    (left : Edge → Row)
    (right : Edge → Column)
    (rows : Row → Option (Fin q))
    (columns : Column → Option (Fin q))
    (vertex : Row) : ℕ :=
  ∑ first : Edge, ∑ second : Edge,
    equalityPotentialOffset q rows vertex (left first) (left second) *
      equalityPotential q (right first) (right second) columns

/-- Directed selected-label interaction score for one row candidate.  This is
the edge-pair form that the native incidence buckets compress. -/
def rowCandidateScore
    (q : ℕ)
    {Edge Row Column : Type*}
    [Fintype Edge]
    [DecidableEq Row]
    [DecidableEq Column]
    (left : Edge → Row)
    (right : Edge → Column)
    (rows : Row → Option (Fin q))
    (columns : Column → Option (Fin q))
    (vertex : Row) (label : Fin q) : ℕ :=
  ∑ first : Edge, ∑ second : Edge,
    if left first = vertex ∧ left second ≠ vertex ∧
        rows (left second) = some label then
      equalityPotential q (right first) (right second) columns
    else 0

/-- The exact quadratic objective for a row candidate is a
candidate-independent offset plus `2q` times its directed interaction score. -/
theorem rowCandidateObjective_eq_affineScore
    (q : ℕ)
    {Edge Row Column : Type*}
    [Fintype Edge]
    [DecidableEq Row]
    [DecidableEq Column]
    (left : Edge → Row)
    (right : Edge → Column)
    (rows : Row → Option (Fin q))
    (columns : Column → Option (Fin q))
    (vertex : Row) (label : Fin q) :
    conditionalObjective q left right
        (Function.update rows vertex (some label)) columns =
      rowCandidateOffset q left right rows columns vertex +
        (2 * q) * rowCandidateScore q left right rows columns vertex label := by
  classical
  let rightPotential := fun first second : Edge =>
    equalityPotential q (right first) (right second) columns
  let forward := ∑ first : Edge, ∑ second : Edge,
    selectedEqualityInteraction q rows vertex label
        (left first) (left second) * rightPotential first second
  let reverse := ∑ first : Edge, ∑ second : Edge,
    selectedEqualityInteraction q rows vertex label
        (left second) (left first) * rightPotential first second
  have hforward :
      forward = q * rowCandidateScore q left right rows columns vertex label := by
    simp only [forward, rowCandidateScore, rightPotential, Finset.mul_sum]
    apply Finset.sum_congr rfl
    intro first _
    apply Finset.sum_congr rfl
    intro second _
    by_cases hinteraction :
        left first = vertex ∧ left second ≠ vertex ∧
          rows (left second) = some label
    · simp [selectedEqualityInteraction, hinteraction]
    · simp [selectedEqualityInteraction, hinteraction]
  have hreverse : reverse = forward := by
    simp only [reverse, forward]
    rw [Finset.sum_comm]
    apply Finset.sum_congr rfl
    intro first _
    apply Finset.sum_congr rfl
    intro second _
    change selectedEqualityInteraction q rows vertex label
          (left first) (left second) *
        equalityPotential q (right second) (right first) columns =
      selectedEqualityInteraction q rows vertex label
          (left first) (left second) *
        equalityPotential q (right first) (right second) columns
    rw [equalityPotential_comm]
  rw [conditionalObjective]
  calc
    (∑ first : Edge, ∑ second : Edge,
        equalityPotential q (left first) (left second)
            (Function.update rows vertex (some label)) *
          rightPotential first second) =
        ∑ first : Edge, ∑ second : Edge,
          (equalityPotentialOffset q rows vertex (left first) (left second) +
              selectedEqualityInteraction q rows vertex label
                (left first) (left second) +
              selectedEqualityInteraction q rows vertex label
                (left second) (left first)) *
            rightPotential first second := by
              apply Finset.sum_congr rfl
              intro first _
              apply Finset.sum_congr rfl
              intro second _
              rw [equalityPotential_update_decomposition]
    _ = rowCandidateOffset q left right rows columns vertex + forward + reverse := by
          simp only [rowCandidateOffset, forward, reverse, rightPotential,
            add_mul, Finset.sum_add_distrib]
    _ = rowCandidateOffset q left right rows columns vertex +
          (2 * q) * rowCandidateScore q left right rows columns vertex label := by
            rw [hreverse, hforward]
            ring

/-- Candidate-independent portion of one column update. -/
def columnCandidateOffset
    (q : ℕ)
    {Edge Row Column : Type*}
    [Fintype Edge]
    [DecidableEq Row]
    [DecidableEq Column]
    (left : Edge → Row)
    (right : Edge → Column)
    (rows : Row → Option (Fin q))
    (columns : Column → Option (Fin q))
    (vertex : Column) : ℕ :=
  rowCandidateOffset q right left columns rows vertex

/-- Directed selected-label interaction score for one column candidate. -/
def columnCandidateScore
    (q : ℕ)
    {Edge Row Column : Type*}
    [Fintype Edge]
    [DecidableEq Row]
    [DecidableEq Column]
    (left : Edge → Row)
    (right : Edge → Column)
    (rows : Row → Option (Fin q))
    (columns : Column → Option (Fin q))
    (vertex : Column) (label : Fin q) : ℕ :=
  rowCandidateScore q right left columns rows vertex label

/-- The exact quadratic objective for a column candidate is a
candidate-independent offset plus `2q` times its directed interaction score. -/
theorem columnCandidateObjective_eq_affineScore
    (q : ℕ)
    {Edge Row Column : Type*}
    [Fintype Edge]
    [DecidableEq Row]
    [DecidableEq Column]
    (left : Edge → Row)
    (right : Edge → Column)
    (rows : Row → Option (Fin q))
    (columns : Column → Option (Fin q))
    (vertex : Column) (label : Fin q) :
    conditionalObjective q left right rows
        (Function.update columns vertex (some label)) =
      columnCandidateOffset q left right rows columns vertex +
        (2 * q) * columnCandidateScore q left right rows columns vertex label := by
  rw [conditionalObjective_swap]
  exact rowCandidateObjective_eq_affineScore q right left columns rows vertex label

/-- The strict first-minimum row label can be computed from the directed
interaction score without evaluating candidate-independent objective terms. -/
theorem firstMinimumRowLabel_eq_sparseScore
    {q : ℕ} (hq : 0 < q)
    {Edge Row Column : Type*}
    [Fintype Edge]
    [DecidableEq Row]
    [DecidableEq Column]
    (left : Edge → Row)
    (right : Edge → Column)
    (rows : Row → Option (Fin q))
    (columns : Column → Option (Fin q))
    (vertex : Row) :
    firstMinimumRowLabel hq left right rows columns vertex =
      firstMinimumLabel hq
        (rowCandidateScore q left right rows columns vertex) := by
  apply firstMinimumRowLabel_eq_of_affine_score hq left right rows columns
    vertex _ (rowCandidateOffset q left right rows columns vertex) (2 * q)
  · positivity
  · exact rowCandidateObjective_eq_affineScore q left right rows columns vertex

/-- The strict first-minimum column label can be computed from the directed
interaction score without evaluating candidate-independent objective terms. -/
theorem firstMinimumColumnLabel_eq_sparseScore
    {q : ℕ} (hq : 0 < q)
    {Edge Row Column : Type*}
    [Fintype Edge]
    [DecidableEq Row]
    [DecidableEq Column]
    (left : Edge → Row)
    (right : Edge → Column)
    (rows : Row → Option (Fin q))
    (columns : Column → Option (Fin q))
    (vertex : Column) :
    firstMinimumColumnLabel hq left right rows columns vertex =
      firstMinimumLabel hq
        (columnCandidateScore q left right rows columns vertex) := by
  apply firstMinimumColumnLabel_eq_of_affine_score hq left right rows columns
    vertex _ (columnCandidateOffset q left right rows columns vertex) (2 * q)
  · positivity
  · exact columnCandidateObjective_eq_affineScore q left right rows columns vertex

end Curve25519SparseLabelScore
