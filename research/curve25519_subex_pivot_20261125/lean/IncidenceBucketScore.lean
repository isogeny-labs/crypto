module

public import SparseLabelScore

@[expose] public section

/-!
# Incidence-bucket refinement of sparse label scores

This module factors the directed edge-pair score through the same incidence
counts used by the native conditional labeler.  The row phase uses degree and
right-endpoint bucket counts.  Once all rows are fixed, the column phase uses
the dot product of row-label incidence profiles.
-/

namespace Curve25519IncidenceBucketScore

open Curve25519ConditionalObjective
open Curve25519SparseLabelScore
open scoped BigOperators

/-- With every endpoint unknown, equality potential is one plus the repeated
endpoint correction `q-1`. -/
theorem equalityPotential_none
    (q : ℕ) (hq : 0 < q)
    {Vertex : Type*}
    [DecidableEq Vertex]
    (first second : Vertex) :
    equalityPotential q first second (fun _ => none) =
      1 + (q - 1) * (if first = second then 1 else 0) := by
  by_cases hsame : first = second
  · simp only [equalityPotential, ite_eq_left hsame]
    omega
  · simp [equalityPotential, hsame]

/-- With every endpoint assigned, equality potential is `q` exactly when the
two stored labels agree. -/
theorem equalityPotential_complete
    (q : ℕ)
    {Vertex : Type*}
    [DecidableEq Vertex]
    (assignment : Vertex → Option (Fin q))
    (hcomplete : ∀ vertex, ∃ label, assignment vertex = some label)
    (first second : Vertex) :
    equalityPotential q first second assignment =
      if assignment first = assignment second then q else 0 := by
  obtain ⟨firstLabel, hfirst⟩ := hcomplete first
  obtain ⟨secondLabel, hsecond⟩ := hcomplete second
  by_cases hsame : first = second
  · subst second
    simp [equalityPotential]
  by_cases hlabels : firstLabel = secondLabel
  · subst secondLabel
    simp [equalityPotential, hsame, hfirst, hsecond]
  · simp [equalityPotential, hsame, hfirst, hsecond, hlabels]

/-- Number of ports incident to the row vertex currently being labeled. -/
def rowVertexDegree
    {Edge Row : Type*}
    [Fintype Edge]
    [DecidableEq Row]
    (left : Edge → Row) (vertex : Row) : ℕ :=
  ∑ edge : Edge, if left edge = vertex then 1 else 0

/-- Total number of earlier row ports stored in one selected-label bucket. -/
def rowLabelBucketDegree
    {q : ℕ}
    {Edge Row : Type*}
    [Fintype Edge]
    [DecidableEq Row]
    (left : Edge → Row)
    (rows : Row → Option (Fin q))
    (vertex : Row) (label : Fin q) : ℕ :=
  ∑ edge : Edge,
    if left edge ≠ vertex ∧ rows (left edge) = some label then 1 else 0

/-- Earlier ports of one row-label bucket ending at a specified right
endpoint.  This is the immutable meaning of one native map entry. -/
def rowRightBucketPorts
    {q : ℕ}
    {Edge Row Column : Type*}
    [Fintype Edge]
    [DecidableEq Row]
    [DecidableEq Column]
    (left : Edge → Row)
    (right : Edge → Column)
    (rows : Row → Option (Fin q))
    (vertex : Row) (label : Fin q) (endpoint : Column) : ℕ :=
  ∑ edge : Edge,
    if left edge ≠ vertex ∧ rows (left edge) = some label ∧
        right edge = endpoint then 1 else 0

/-- Source-shaped row score: degree times bucket degree plus the repeated-right
endpoint correction accumulated by incidence-bucket lookup. -/
def rowIncidenceBucketScore
    (q : ℕ)
    {Edge Row Column : Type*}
    [Fintype Edge]
    [DecidableEq Row]
    [DecidableEq Column]
    (left : Edge → Row)
    (right : Edge → Column)
    (rows : Row → Option (Fin q))
    (vertex : Row) (label : Fin q) : ℕ :=
  rowVertexDegree left vertex * rowLabelBucketDegree left rows vertex label +
    (q - 1) *
      ∑ edge : Edge,
        if left edge = vertex then
          rowRightBucketPorts left right rows vertex label (right edge)
        else 0

/-- When columns are still unknown, the directed sparse row score is exactly
the native degree/incidence-bucket score. -/
theorem rowCandidateScore_none_eq_incidenceBucketScore
    (q : ℕ) (hq : 0 < q)
    {Edge Row Column : Type*}
    [Fintype Edge]
    [DecidableEq Row]
    [DecidableEq Column]
    (left : Edge → Row)
    (right : Edge → Column)
    (rows : Row → Option (Fin q))
    (vertex : Row) (label : Fin q) :
    rowCandidateScore q left right rows (fun _ => none) vertex label =
      rowIncidenceBucketScore q left right rows vertex label := by
  classical
  have hpoint (first second : Edge) :
      (if left first = vertex ∧ left second ≠ vertex ∧
          rows (left second) = some label then
        equalityPotential q (right first) (right second) (fun _ => none)
      else 0) =
        (if left first = vertex then 1 else 0) *
            (if left second ≠ vertex ∧ rows (left second) = some label then 1 else 0) +
          (q - 1) *
            (if left first = vertex ∧ left second ≠ vertex ∧
                rows (left second) = some label ∧ right second = right first then 1 else 0) := by
    by_cases hfirst : left first = vertex
    · by_cases hsecond : left second ≠ vertex ∧ rows (left second) = some label
      · by_cases hright : right first = right second
        · have hcondition : left first = vertex ∧ left second ≠ vertex ∧
              rows (left second) = some label :=
            ⟨hfirst, hsecond.1, hsecond.2⟩
          have hoverlap : left first = vertex ∧ left second ≠ vertex ∧
              rows (left second) = some label ∧ right second = right first :=
            ⟨hfirst, hsecond.1, hsecond.2, hright.symm⟩
          rw [ite_eq_left hcondition, ite_eq_left hfirst, ite_eq_left hsecond,
            ite_eq_left hoverlap]
          rw [equalityPotential_none q hq, ite_eq_left hright]
        · have hcondition : left first = vertex ∧ left second ≠ vertex ∧
              rows (left second) = some label :=
            ⟨hfirst, hsecond.1, hsecond.2⟩
          have hoverlap : ¬(left first = vertex ∧ left second ≠ vertex ∧
              rows (left second) = some label ∧ right second = right first) := by
            intro h
            exact hright h.2.2.2.symm
          rw [ite_eq_left hcondition, ite_eq_left hfirst, ite_eq_left hsecond,
            ite_eq_right hoverlap]
          rw [equalityPotential_none q hq, ite_eq_right hright]
      · have hcondition : ¬(left first = vertex ∧ left second ≠ vertex ∧
            rows (left second) = some label) := by
          intro h
          exact hsecond ⟨h.2.1, h.2.2⟩
        have hoverlap : ¬(left first = vertex ∧ left second ≠ vertex ∧
            rows (left second) = some label ∧ right second = right first) := by
          intro h
          exact hsecond ⟨h.2.1, h.2.2.1⟩
        rw [ite_eq_right hcondition, ite_eq_left hfirst, ite_eq_right hsecond,
          ite_eq_right hoverlap]
        simp
    · have hcondition : ¬(left first = vertex ∧ left second ≠ vertex ∧
          rows (left second) = some label) := fun h => hfirst h.1
      have hoverlap : ¬(left first = vertex ∧ left second ≠ vertex ∧
          rows (left second) = some label ∧ right second = right first) :=
        fun h => hfirst h.1
      rw [ite_eq_right hcondition, ite_eq_right hfirst, ite_eq_right hoverlap]
      simp
  have hbaseline :
      (∑ first : Edge, ∑ second : Edge,
        (if left first = vertex then 1 else 0) *
          (if left second ≠ vertex ∧ rows (left second) = some label then 1 else 0)) =
        rowVertexDegree left vertex *
          rowLabelBucketDegree left rows vertex label := by
    unfold rowVertexDegree rowLabelBucketDegree
    calc
      _ = ∑ first : Edge, (if left first = vertex then 1 else 0) *
          (∑ second : Edge,
            if left second ≠ vertex ∧ rows (left second) = some label then 1 else 0) := by
              apply Finset.sum_congr rfl
              intro first _
              rw [Finset.mul_sum]
      _ = _ := by rw [Finset.sum_mul]
  have hoverlap :
      (∑ first : Edge, ∑ second : Edge,
        if left first = vertex ∧ left second ≠ vertex ∧
            rows (left second) = some label ∧ right second = right first then 1 else 0) =
        ∑ first : Edge,
          if left first = vertex then
            rowRightBucketPorts left right rows vertex label (right first)
          else 0 := by
    apply Finset.sum_congr rfl
    intro first _
    by_cases hfirst : left first = vertex
    · simp [rowRightBucketPorts, hfirst]
    · simp [hfirst]
  unfold rowCandidateScore
  simp_rw [hpoint]
  rw [rowIncidenceBucketScore, ← hbaseline, ← hoverlap]
  simp only [Finset.sum_add_distrib, Finset.mul_sum]

/-- Earlier column ports having both a fixed row label and a selected column
label.  This is the immutable meaning of one transposed native map entry. -/
def columnRowLabelPorts
    {q : ℕ}
    {Edge Row Column : Type*}
    [Fintype Edge]
    [DecidableEq Row]
    [DecidableEq Column]
    (left : Edge → Row)
    (right : Edge → Column)
    (rows : Row → Option (Fin q))
    (columns : Column → Option (Fin q))
    (vertex : Column) (rowLabel columnLabel : Fin q) : ℕ :=
  ∑ edge : Edge,
    if right edge ≠ vertex ∧ columns (right edge) = some columnLabel ∧
        rows (left edge) = some rowLabel then 1 else 0

/-- Source-shaped column score: dot the current column's row-label incidence
profile with the earlier-port profile for one candidate column label. -/
def columnIncidenceProfileScore
    {q : ℕ}
    {Edge Row Column : Type*}
    [Fintype Edge]
    [DecidableEq Row]
    [DecidableEq Column]
    (left : Edge → Row)
    (right : Edge → Column)
    (rows : Row → Option (Fin q))
    (columns : Column → Option (Fin q))
    (vertex : Column) (label : Fin q) : ℕ :=
  ∑ edge : Edge,
    if right edge = vertex then
      match rows (left edge) with
      | some rowLabel =>
          columnRowLabelPorts left right rows columns vertex rowLabel label
      | none => 0
    else 0

/-- Once every row has a fixed label, the directed sparse column score is `q`
times the native incidence-profile dot product. -/
theorem columnCandidateScore_eq_incidenceProfileScore
    (q : ℕ)
    {Edge Row Column : Type*}
    [Fintype Edge]
    [DecidableEq Row]
    [DecidableEq Column]
    (left : Edge → Row)
    (right : Edge → Column)
    (rows : Row → Option (Fin q))
    (columns : Column → Option (Fin q))
    (hrows : ∀ row, ∃ label, rows row = some label)
    (vertex : Column) (label : Fin q) :
    columnCandidateScore q left right rows columns vertex label =
      q * columnIncidenceProfileScore left right rows columns vertex label := by
  classical
  have hpoint (first second : Edge) :
      (if right first = vertex ∧ right second ≠ vertex ∧
          columns (right second) = some label then
        equalityPotential q (left first) (left second) rows
      else 0) =
        q *
          (if right first = vertex ∧ right second ≠ vertex ∧
              columns (right second) = some label ∧
              rows (left second) = rows (left first) then 1 else 0) := by
    rw [equalityPotential_complete q rows hrows]
    by_cases hcondition : right first = vertex ∧ right second ≠ vertex ∧
        columns (right second) = some label
    · by_cases hlabels : rows (left first) = rows (left second)
      · have hfull : right first = vertex ∧ right second ≠ vertex ∧
            columns (right second) = some label ∧
            rows (left second) = rows (left first) :=
          ⟨hcondition.1, hcondition.2.1, hcondition.2.2, hlabels.symm⟩
        rw [ite_eq_left hcondition, ite_eq_left hlabels, ite_eq_left hfull]
        simp
      · have hreverse : rows (left second) ≠ rows (left first) := Ne.symm hlabels
        have hfull : ¬(right first = vertex ∧ right second ≠ vertex ∧
            columns (right second) = some label ∧
            rows (left second) = rows (left first)) := fun h => hreverse h.2.2.2
        rw [ite_eq_left hcondition, ite_eq_right hlabels, ite_eq_right hfull]
        simp
    · have hfull : ¬(right first = vertex ∧ right second ≠ vertex ∧
          columns (right second) = some label ∧
          rows (left second) = rows (left first)) := by
        intro h
        exact hcondition ⟨h.1, h.2.1, h.2.2.1⟩
      rw [ite_eq_right hcondition, ite_eq_right hfull]
      simp
  unfold columnCandidateScore rowCandidateScore columnIncidenceProfileScore
    columnRowLabelPorts
  simp_rw [hpoint]
  rw [Finset.mul_sum]
  apply Finset.sum_congr rfl
  intro first _
  obtain ⟨rowLabel, hrow⟩ := hrows (left first)
  rw [← Finset.mul_sum]
  by_cases hfirst : right first = vertex
  · simp only [hfirst, true_and, ite_true, hrow]
  · simp [hfirst]

/-- Row first-minimum labels can be computed from the native incidence-bucket
score without changing strict tie behavior. -/
theorem firstMinimumRowLabel_eq_incidenceBucketScore
    {q : ℕ} (hq : 0 < q)
    {Edge Row Column : Type*}
    [Fintype Edge]
    [DecidableEq Row]
    [DecidableEq Column]
    (left : Edge → Row)
    (right : Edge → Column)
    (rows : Row → Option (Fin q))
    (vertex : Row) :
    firstMinimumRowLabel hq left right rows (fun _ => none) vertex =
      firstMinimumLabel hq
        (rowIncidenceBucketScore q left right rows vertex) := by
  rw [firstMinimumRowLabel_eq_sparseScore]
  congr 1
  funext label
  exact rowCandidateScore_none_eq_incidenceBucketScore q hq left right rows vertex label

/-- With complete row labels, column first minima can be computed from the
native incidence-profile score without changing strict tie behavior. -/
theorem firstMinimumColumnLabel_eq_incidenceProfileScore
    {q : ℕ} (hq : 0 < q)
    {Edge Row Column : Type*}
    [Fintype Edge]
    [DecidableEq Row]
    [DecidableEq Column]
    (left : Edge → Row)
    (right : Edge → Column)
    (rows : Row → Option (Fin q))
    (columns : Column → Option (Fin q))
    (hrows : ∀ row, ∃ label, rows row = some label)
    (vertex : Column) :
    firstMinimumColumnLabel hq left right rows columns vertex =
      firstMinimumLabel hq
        (columnIncidenceProfileScore left right rows columns vertex) := by
  rw [firstMinimumColumnLabel_eq_sparseScore]
  apply firstMinimumLabel_eq_of_affine hq _ _ 0 q hq
  intro label
  simp only [Nat.zero_add]
  exact columnCandidateScore_eq_incidenceProfileScore q left right rows columns
    hrows vertex label

end Curve25519IncidenceBucketScore
