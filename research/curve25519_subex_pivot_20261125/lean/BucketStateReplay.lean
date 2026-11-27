module

public import IncidenceBucketScore
public import LabelSequence

@[expose] public section

/-!
# Auxiliary bucket-state replay

This module gives the mutable degree/incidence maps an immutable formal
meaning.  Each update is proved equal to extending the corresponding partial
assignment by one selected label, and each state score refines the extensional
incidence formula.
-/

namespace Curve25519BucketStateReplay

open Curve25519ConditionalObjective
open Curve25519IncidenceBucketScore
open Curve25519LabelSequence
open scoped BigOperators

/-- Total assigned row-port mass in one label bucket. -/
def assignedRowBucketDegree
    {q : ℕ}
    {Edge Row : Type*}
    [Fintype Edge]
    (left : Edge → Row)
    (rows : Row → Option (Fin q))
    (label : Fin q) : ℕ :=
  ∑ edge : Edge, if rows (left edge) = some label then 1 else 0

/-- Assigned row ports in one label bucket ending at one right endpoint. -/
def assignedRightBucketPorts
    {q : ℕ}
    {Edge Row Column : Type*}
    [Fintype Edge]
    [DecidableEq Column]
    (left : Edge → Row)
    (right : Edge → Column)
    (rows : Row → Option (Fin q))
    (endpoint : Column) (label : Fin q) : ℕ :=
  ∑ edge : Edge,
    if rows (left edge) = some label ∧ right edge = endpoint then 1 else 0

/-- Ports of one row vertex ending at one right endpoint. -/
def rowVertexRightPorts
    {Edge Row Column : Type*}
    [Fintype Edge]
    [DecidableEq Row]
    [DecidableEq Column]
    (left : Edge → Row)
    (right : Edge → Column)
    (vertex : Row) (endpoint : Column) : ℕ :=
  ∑ edge : Edge,
    if left edge = vertex ∧ right edge = endpoint then 1 else 0

/-- The two auxiliary maps maintained during the row phase. -/
structure RowBucketState (q : ℕ) (Column : Type*) where
  bucketDegrees : Fin q → ℕ
  rightBucketPorts : Column → Fin q → ℕ

/-- Empty row-phase auxiliary state. -/
def RowBucketState.empty (q : ℕ) (Column : Type*) : RowBucketState q Column where
  bucketDegrees := fun _ => 0
  rightBucketPorts := fun _ _ => 0

/-- Extensional meaning of a row-phase auxiliary state. -/
def RowBucketState.Matches
    {q : ℕ}
    {Edge Row Column : Type*}
    [Fintype Edge]
    [DecidableEq Column]
    (state : RowBucketState q Column)
    (left : Edge → Row)
    (right : Edge → Column)
    (rows : Row → Option (Fin q)) : Prop :=
  (∀ label, state.bucketDegrees label =
      assignedRowBucketDegree left rows label) ∧
    (∀ endpoint label, state.rightBucketPorts endpoint label =
      assignedRightBucketPorts left right rows endpoint label)

/-- Source-shaped row score evaluated from auxiliary maps. -/
def RowBucketState.score
    (q : ℕ)
    {Edge Row Column : Type*}
    [Fintype Edge]
    [DecidableEq Row]
    (state : RowBucketState q Column)
    (left : Edge → Row)
    (right : Edge → Column)
    (vertex : Row) (label : Fin q) : ℕ :=
  rowVertexDegree left vertex * state.bucketDegrees label +
    (q - 1) *
      ∑ edge : Edge,
        if left edge = vertex then state.rightBucketPorts (right edge) label else 0

/-- Add every port of the newly labeled row vertex to both auxiliary maps. -/
def RowBucketState.addVertex
    {q : ℕ}
    {Edge Row Column : Type*}
    [Fintype Edge]
    [DecidableEq Row]
    [DecidableEq Column]
    (state : RowBucketState q Column)
    (left : Edge → Row)
    (right : Edge → Column)
    (vertex : Row) (selected : Fin q) : RowBucketState q Column where
  bucketDegrees := fun label =>
    state.bucketDegrees label +
      if label = selected then rowVertexDegree left vertex else 0
  rightBucketPorts := fun endpoint label =>
    state.rightBucketPorts endpoint label +
      if label = selected then rowVertexRightPorts left right vertex endpoint else 0

/-- Extending an unknown row vertex updates its label-bucket degree by exactly
the degree of that vertex in the selected bucket. -/
theorem assignedRowBucketDegree_update
    {q : ℕ}
    {Edge Row : Type*}
    [Fintype Edge]
    [DecidableEq Row]
    (left : Edge → Row)
    (rows : Row → Option (Fin q))
    (vertex : Row) (selected label : Fin q)
    (hunknown : rows vertex = none) :
    assignedRowBucketDegree left (Function.update rows vertex (some selected)) label =
      assignedRowBucketDegree left rows label +
        if label = selected then rowVertexDegree left vertex else 0 := by
  classical
  have hpoint (edge : Edge) :
      (if Function.update rows vertex (some selected) (left edge) = some label then 1 else 0) =
        (if rows (left edge) = some label then 1 else 0) +
          if label = selected then (if left edge = vertex then 1 else 0) else 0 := by
    by_cases hvertex : left edge = vertex
    · subst vertex
      by_cases hlabel : label = selected
      · subst selected
        simp [hunknown]
      · simp [hunknown, hlabel, Ne.symm hlabel]
    · simp [Function.update, hvertex]
  unfold assignedRowBucketDegree rowVertexDegree
  simp_rw [hpoint, Finset.sum_add_distrib]
  by_cases hlabel : label = selected
  · simp [hlabel]
  · simp [hlabel]

/-- Extending an unknown row vertex updates every right-endpoint incidence
bucket by exactly that vertex's port multiplicity at the endpoint. -/
theorem assignedRightBucketPorts_update
    {q : ℕ}
    {Edge Row Column : Type*}
    [Fintype Edge]
    [DecidableEq Row]
    [DecidableEq Column]
    (left : Edge → Row)
    (right : Edge → Column)
    (rows : Row → Option (Fin q))
    (vertex : Row) (selected : Fin q)
    (endpoint : Column) (label : Fin q)
    (hunknown : rows vertex = none) :
    assignedRightBucketPorts left right
        (Function.update rows vertex (some selected)) endpoint label =
      assignedRightBucketPorts left right rows endpoint label +
        if label = selected then rowVertexRightPorts left right vertex endpoint else 0 := by
  classical
  have hpoint (edge : Edge) :
      (if Function.update rows vertex (some selected) (left edge) = some label ∧
          right edge = endpoint then 1 else 0) =
        (if rows (left edge) = some label ∧ right edge = endpoint then 1 else 0) +
          if label = selected then
            (if left edge = vertex ∧ right edge = endpoint then 1 else 0)
          else 0 := by
    by_cases hvertex : left edge = vertex
    · subst vertex
      by_cases hlabel : label = selected
      · subst selected
        simp [hunknown]
      · simp [hunknown, hlabel, Ne.symm hlabel]
    · simp [Function.update, hvertex]
  unfold assignedRightBucketPorts rowVertexRightPorts
  simp_rw [hpoint, Finset.sum_add_distrib]
  by_cases hlabel : label = selected
  · simp [hlabel]
  · simp [hlabel]

/-- An empty auxiliary state exactly represents the all-unknown row
assignment. -/
theorem RowBucketState.empty_matches
    (q : ℕ)
    {Edge Row Column : Type*}
    [Fintype Edge]
    [DecidableEq Column]
    (left : Edge → Row)
    (right : Edge → Column) :
    (RowBucketState.empty q Column).Matches left right (fun _ => none) := by
  constructor
  · intro label
    simp [RowBucketState.empty, assignedRowBucketDegree]
  · intro endpoint label
    simp [RowBucketState.empty, assignedRightBucketPorts]

/-- A source-shaped row update preserves the exact meaning of both auxiliary
maps. -/
theorem RowBucketState.addVertex_matches
    {q : ℕ}
    {Edge Row Column : Type*}
    [Fintype Edge]
    [DecidableEq Row]
    [DecidableEq Column]
    (state : RowBucketState q Column)
    (left : Edge → Row)
    (right : Edge → Column)
    (rows : Row → Option (Fin q))
    (vertex : Row) (selected : Fin q)
    (hmatches : state.Matches left right rows)
    (hunknown : rows vertex = none) :
    (state.addVertex left right vertex selected).Matches left right
      (Function.update rows vertex (some selected)) := by
  constructor
  · intro label
    simp only [RowBucketState.addVertex]
    rw [hmatches.1 label]
    exact (assignedRowBucketDegree_update left rows vertex selected label hunknown).symm
  · intro endpoint label
    simp only [RowBucketState.addVertex]
    rw [hmatches.2 endpoint label]
    exact (assignedRightBucketPorts_update left right rows vertex selected endpoint label
      hunknown).symm

/-- Excluding the current unknown row vertex is redundant in the extensional
bucket degree. -/
theorem rowLabelBucketDegree_eq_assigned
    {q : ℕ}
    {Edge Row : Type*}
    [Fintype Edge]
    [DecidableEq Row]
    (left : Edge → Row)
    (rows : Row → Option (Fin q))
    (vertex : Row) (label : Fin q)
    (hunknown : rows vertex = none) :
    rowLabelBucketDegree left rows vertex label =
      assignedRowBucketDegree left rows label := by
  unfold rowLabelBucketDegree assignedRowBucketDegree
  apply Finset.sum_congr rfl
  intro edge _
  by_cases hvertex : left edge = vertex
  · subst vertex
    simp [hunknown]
  · simp [hvertex]

/-- Excluding the current unknown row vertex is redundant in each extensional
right-endpoint incidence bucket. -/
theorem rowRightBucketPorts_eq_assigned
    {q : ℕ}
    {Edge Row Column : Type*}
    [Fintype Edge]
    [DecidableEq Row]
    [DecidableEq Column]
    (left : Edge → Row)
    (right : Edge → Column)
    (rows : Row → Option (Fin q))
    (vertex : Row) (label : Fin q) (endpoint : Column)
    (hunknown : rows vertex = none) :
    rowRightBucketPorts left right rows vertex label endpoint =
      assignedRightBucketPorts left right rows endpoint label := by
  unfold rowRightBucketPorts assignedRightBucketPorts
  apply Finset.sum_congr rfl
  intro edge _
  by_cases hvertex : left edge = vertex
  · subst vertex
    simp [hunknown]
  · simp [hvertex]

/-- A matching row-state score is exactly the extensional incidence-bucket
score used in the first-minimum refinement. -/
theorem RowBucketState.score_eq_incidenceBucketScore
    (q : ℕ)
    {Edge Row Column : Type*}
    [Fintype Edge]
    [DecidableEq Row]
    [DecidableEq Column]
    (state : RowBucketState q Column)
    (left : Edge → Row)
    (right : Edge → Column)
    (rows : Row → Option (Fin q))
    (vertex : Row) (label : Fin q)
    (hmatches : state.Matches left right rows)
    (hunknown : rows vertex = none) :
    state.score q left right vertex label =
      rowIncidenceBucketScore q left right rows vertex label := by
  unfold RowBucketState.score rowIncidenceBucketScore
  rw [hmatches.1 label,
    rowLabelBucketDegree_eq_assigned left rows vertex label hunknown]
  apply congrArg₂ (fun degree overlap =>
    rowVertexDegree left vertex * degree + (q - 1) * overlap) rfl
  apply Finset.sum_congr rfl
  intro edge _
  by_cases hvertex : left edge = vertex
  · simp only [hvertex, ite_true]
    rw [hmatches.2 (right edge) label,
      rowRightBucketPorts_eq_assigned left right rows vertex label (right edge) hunknown]
  · simp [hvertex]

/-- Total assigned column ports for one fixed row-label/column-label profile
entry. -/
def assignedColumnProfilePorts
    {q : ℕ}
    {Edge Row Column : Type*}
    [Fintype Edge]
    (left : Edge → Row)
    (right : Edge → Column)
    (rows : Row → Option (Fin q))
    (columns : Column → Option (Fin q))
    (rowLabel columnLabel : Fin q) : ℕ :=
  ∑ edge : Edge,
    if columns (right edge) = some columnLabel ∧
        rows (left edge) = some rowLabel then 1 else 0

/-- Ports of one column vertex carrying one fixed row label. -/
def columnVertexRowLabelPorts
    {q : ℕ}
    {Edge Row Column : Type*}
    [Fintype Edge]
    [DecidableEq Column]
    (left : Edge → Row)
    (right : Edge → Column)
    (rows : Row → Option (Fin q))
    (vertex : Column) (rowLabel : Fin q) : ℕ :=
  ∑ edge : Edge,
    if right edge = vertex ∧ rows (left edge) = some rowLabel then 1 else 0

/-- Auxiliary transposed incidence map maintained during the column phase. -/
structure ColumnBucketState (q : ℕ) where
  rowLabelColumnPorts : Fin q → Fin q → ℕ

/-- Empty column-phase auxiliary state. -/
def ColumnBucketState.empty (q : ℕ) : ColumnBucketState q where
  rowLabelColumnPorts := fun _ _ => 0

/-- Extensional meaning of a column-phase auxiliary state. -/
def ColumnBucketState.Matches
    {q : ℕ}
    {Edge Row Column : Type*}
    [Fintype Edge]
    (state : ColumnBucketState q)
    (left : Edge → Row)
    (right : Edge → Column)
    (rows : Row → Option (Fin q))
    (columns : Column → Option (Fin q)) : Prop :=
  ∀ rowLabel columnLabel,
    state.rowLabelColumnPorts rowLabel columnLabel =
      assignedColumnProfilePorts left right rows columns rowLabel columnLabel

/-- Source-shaped column profile score evaluated from the auxiliary map. -/
def ColumnBucketState.score
    {q : ℕ}
    {Edge Row Column : Type*}
    [Fintype Edge]
    [DecidableEq Column]
    (state : ColumnBucketState q)
    (left : Edge → Row)
    (right : Edge → Column)
    (rows : Row → Option (Fin q))
    (vertex : Column) (label : Fin q) : ℕ :=
  ∑ edge : Edge,
    if right edge = vertex then
      match rows (left edge) with
      | some rowLabel => state.rowLabelColumnPorts rowLabel label
      | none => 0
    else 0

/-- Add one selected column's row-label incidence profile. -/
def ColumnBucketState.addVertex
    {q : ℕ}
    {Edge Row Column : Type*}
    [Fintype Edge]
    [DecidableEq Column]
    (state : ColumnBucketState q)
    (left : Edge → Row)
    (right : Edge → Column)
    (rows : Row → Option (Fin q))
    (vertex : Column) (selected : Fin q) : ColumnBucketState q where
  rowLabelColumnPorts := fun rowLabel columnLabel =>
    state.rowLabelColumnPorts rowLabel columnLabel +
      if columnLabel = selected then
        columnVertexRowLabelPorts left right rows vertex rowLabel
      else 0

/-- Extending an unknown column updates each transposed profile entry by its
current-vertex row-label port count. -/
theorem assignedColumnProfilePorts_update
    {q : ℕ}
    {Edge Row Column : Type*}
    [Fintype Edge]
    [DecidableEq Column]
    (left : Edge → Row)
    (right : Edge → Column)
    (rows : Row → Option (Fin q))
    (columns : Column → Option (Fin q))
    (vertex : Column) (selected rowLabel columnLabel : Fin q)
    (hunknown : columns vertex = none) :
    assignedColumnProfilePorts left right rows
        (Function.update columns vertex (some selected)) rowLabel columnLabel =
      assignedColumnProfilePorts left right rows columns rowLabel columnLabel +
        if columnLabel = selected then
          columnVertexRowLabelPorts left right rows vertex rowLabel
        else 0 := by
  classical
  have hpoint (edge : Edge) :
      (if Function.update columns vertex (some selected) (right edge) = some columnLabel ∧
          rows (left edge) = some rowLabel then 1 else 0) =
        (if columns (right edge) = some columnLabel ∧
            rows (left edge) = some rowLabel then 1 else 0) +
          if columnLabel = selected then
            (if right edge = vertex ∧ rows (left edge) = some rowLabel then 1 else 0)
          else 0 := by
    by_cases hvertex : right edge = vertex
    · subst vertex
      by_cases hlabel : columnLabel = selected
      · subst selected
        simp [hunknown]
      · simp [hunknown, hlabel, Ne.symm hlabel]
    · simp [Function.update, hvertex]
  unfold assignedColumnProfilePorts columnVertexRowLabelPorts
  simp_rw [hpoint, Finset.sum_add_distrib]
  by_cases hlabel : columnLabel = selected
  · simp [hlabel]
  · simp [hlabel]

/-- Empty column state exactly represents the all-unknown column assignment. -/
theorem ColumnBucketState.empty_matches
    (q : ℕ)
    {Edge Row Column : Type*}
    [Fintype Edge]
    (left : Edge → Row)
    (right : Edge → Column)
    (rows : Row → Option (Fin q)) :
    (ColumnBucketState.empty q).Matches left right rows (fun _ => none) := by
  intro rowLabel columnLabel
  simp [ColumnBucketState.empty, assignedColumnProfilePorts]

/-- A source-shaped column update preserves the exact meaning of the
transposed auxiliary profile. -/
theorem ColumnBucketState.addVertex_matches
    {q : ℕ}
    {Edge Row Column : Type*}
    [Fintype Edge]
    [DecidableEq Column]
    (state : ColumnBucketState q)
    (left : Edge → Row)
    (right : Edge → Column)
    (rows : Row → Option (Fin q))
    (columns : Column → Option (Fin q))
    (vertex : Column) (selected : Fin q)
    (hmatches : state.Matches left right rows columns)
    (hunknown : columns vertex = none) :
    (state.addVertex left right rows vertex selected).Matches left right rows
      (Function.update columns vertex (some selected)) := by
  intro rowLabel columnLabel
  simp only [ColumnBucketState.addVertex]
  rw [hmatches rowLabel columnLabel]
  exact (assignedColumnProfilePorts_update left right rows columns vertex selected
    rowLabel columnLabel hunknown).symm

/-- Excluding the current unknown column is redundant in the extensional
transposed profile. -/
theorem columnRowLabelPorts_eq_assigned
    {q : ℕ}
    {Edge Row Column : Type*}
    [Fintype Edge]
    [DecidableEq Row]
    [DecidableEq Column]
    (left : Edge → Row)
    (right : Edge → Column)
    (rows : Row → Option (Fin q))
    (columns : Column → Option (Fin q))
    (vertex : Column) (rowLabel columnLabel : Fin q)
    (hunknown : columns vertex = none) :
    columnRowLabelPorts left right rows columns vertex rowLabel columnLabel =
      assignedColumnProfilePorts left right rows columns rowLabel columnLabel := by
  unfold columnRowLabelPorts assignedColumnProfilePorts
  apply Finset.sum_congr rfl
  intro edge _
  by_cases hvertex : right edge = vertex
  · subst vertex
    simp [hunknown]
  · simp [hvertex]

/-- A matching column-state score is exactly the extensional incidence-profile
score used in the first-minimum refinement. -/
theorem ColumnBucketState.score_eq_incidenceProfileScore
    {q : ℕ}
    {Edge Row Column : Type*}
    [Fintype Edge]
    [DecidableEq Row]
    [DecidableEq Column]
    (state : ColumnBucketState q)
    (left : Edge → Row)
    (right : Edge → Column)
    (rows : Row → Option (Fin q))
    (columns : Column → Option (Fin q))
    (vertex : Column) (label : Fin q)
    (hmatches : state.Matches left right rows columns)
    (hunknown : columns vertex = none) :
    state.score left right rows vertex label =
      columnIncidenceProfileScore left right rows columns vertex label := by
  unfold ColumnBucketState.score columnIncidenceProfileScore
  apply Finset.sum_congr rfl
  intro edge _
  by_cases hvertex : right edge = vertex
  · simp only [hvertex, ite_true]
    cases hrow : rows (left edge) with
    | none => rfl
    | some rowLabel =>
        change state.rowLabelColumnPorts rowLabel label =
          columnRowLabelPorts left right rows columns vertex rowLabel label
        rw [hmatches rowLabel label,
          columnRowLabelPorts_eq_assigned left right rows columns vertex rowLabel label hunknown]
  · simp [hvertex]

/-- Single-pass row replay over the next `remaining` native-order vertices. -/
def replayRowStateAux
    {q edgeCount rowCount columnCount : ℕ}
    (hq : 0 < q)
    (left : Fin edgeCount → Fin rowCount)
    (right : Fin edgeCount → Fin columnCount)
    (labels : Fin rowCount → Fin q) :
    ℕ → ℕ → RowBucketState q (Fin columnCount) → Bool
  | _, 0, _ => true
  | k, remaining + 1, state =>
      if hk : k < rowCount then
        decide (labels ⟨k, hk⟩ =
          firstMinimumLabel hq (state.score q left right ⟨k, hk⟩)) &&
        replayRowStateAux hq left right labels (k + 1) remaining
          (state.addVertex left right ⟨k, hk⟩ (labels ⟨k, hk⟩))
      else false

/-- Complete single-pass row replay from empty auxiliary state. -/
def replayRowState
    {q edgeCount rowCount columnCount : ℕ}
    (hq : 0 < q)
    (left : Fin edgeCount → Fin rowCount)
    (right : Fin edgeCount → Fin columnCount)
    (labels : Fin rowCount → Fin q) : Bool :=
  replayRowStateAux hq left right labels 0 rowCount
    (RowBucketState.empty q (Fin columnCount))

/-- A successful row-state suffix replay gives the exact extensional
incidence-score first minimum at every covered native-order vertex. -/
theorem replayRowStateAux_sound
    {q edgeCount rowCount columnCount : ℕ}
    (hq : 0 < q)
    (left : Fin edgeCount → Fin rowCount)
    (right : Fin edgeCount → Fin columnCount)
    (labels : Fin rowCount → Fin q)
    (k remaining : ℕ)
    (state : RowBucketState q (Fin columnCount))
    (htotal : k + remaining = rowCount)
    (hmatches : state.Matches left right (prefixAssignment labels k))
    (htrace : replayRowStateAux hq left right labels k remaining state = true) :
    ∀ (i : ℕ) (hi : i < rowCount), k ≤ i →
      labels ⟨i, hi⟩ =
        firstMinimumLabel hq
          (rowIncidenceBucketScore q left right
            (prefixAssignment labels i) ⟨i, hi⟩) := by
  induction remaining generalizing k state with
  | zero =>
      intro i hi hki
      omega
  | succ remaining ih =>
      have hk : k < rowCount := by omega
      simp only [replayRowStateAux, hk, ↓reduceDIte, Bool.and_eq_true] at htrace
      have hcurrent := of_decide_eq_true htrace.1
      have hscore :
          (fun label => state.score q left right ⟨k, hk⟩ label) =
            rowIncidenceBucketScore q left right
              (prefixAssignment labels k) ⟨k, hk⟩ := by
        funext label
        exact state.score_eq_incidenceBucketScore q left right
          (prefixAssignment labels k) ⟨k, hk⟩ label hmatches
          (prefixAssignment_next_unknown labels hk)
      have hcurrent' : labels ⟨k, hk⟩ =
          firstMinimumLabel hq
            (fun label => state.score q left right ⟨k, hk⟩ label) := by
        simpa only using hcurrent
      rw [hscore] at hcurrent'
      have hnextMatches := state.addVertex_matches left right
        (prefixAssignment labels k) ⟨k, hk⟩ (labels ⟨k, hk⟩) hmatches
        (prefixAssignment_next_unknown labels hk)
      rw [prefixAssignment_step labels hk] at hnextMatches
      have hnextTotal : k + 1 + remaining = rowCount := by omega
      have hrest := ih (k + 1)
        (state.addVertex left right ⟨k, hk⟩ (labels ⟨k, hk⟩))
        hnextTotal hnextMatches htrace.2
      intro i hi hki
      by_cases hsame : i = k
      · subst i
        simpa only using hcurrent'
      · exact hrest i hi (by omega)

/-- Complete successful row-state replay gives every local extensional first
minimum. -/
theorem replayRowState_sound
    {q edgeCount rowCount columnCount : ℕ}
    (hq : 0 < q)
    (left : Fin edgeCount → Fin rowCount)
    (right : Fin edgeCount → Fin columnCount)
    (labels : Fin rowCount → Fin q)
    (htrace : replayRowState hq left right labels = true) :
    ∀ row : Fin rowCount,
      labels row = firstMinimumLabel hq
        (rowIncidenceBucketScore q left right
          (prefixAssignment labels row.val) row) := by
  have hmatches :
      (RowBucketState.empty q (Fin columnCount)).Matches left right
        (prefixAssignment labels 0) := by
    rw [prefixAssignment_zero]
    exact RowBucketState.empty_matches q left right
  intro row
  exact replayRowStateAux_sound hq left right labels 0 rowCount _ (by omega) hmatches
    htrace row.val row.isLt (Nat.zero_le _)

/-- Successful single-pass row replay is extensionally equal to the original
recursive executable row schedule. -/
theorem replayRowState_eq_executable
    {q edgeCount rowCount columnCount : ℕ}
    (hq : 0 < q)
    (left : Fin edgeCount → Fin rowCount)
    (right : Fin edgeCount → Fin columnCount)
    (labels : Fin rowCount → Fin q)
    (htrace : replayRowState hq left right labels = true) :
    labels = executableRowLabels hq left right := by
  funext row
  have hlabel : ∀ k (hk : k < rowCount),
      labels ⟨k, hk⟩ = executableRowLabels hq left right ⟨k, hk⟩ := by
    intro k
    induction k using Nat.strong_induction_on with
    | h k ih =>
      intro hk
      rw [executableRowLabels_eq, firstMinimumRowLabel_eq_incidenceBucketScore]
      have hlocal := replayRowState_sound hq left right labels htrace ⟨k, hk⟩
      rw [hlocal]
      congr 2
      funext vertex
      simp only [prefixAssignment]
      split_ifs with hbefore
      · congr 1
        exact ih vertex.val hbefore vertex.isLt
      · rfl
  exact hlabel row.val row.isLt

/-- Single-pass column replay over the next `remaining` native-order vertices. -/
def replayColumnStateAux
    {q edgeCount rowCount columnCount : ℕ}
    (hq : 0 < q)
    (left : Fin edgeCount → Fin rowCount)
    (right : Fin edgeCount → Fin columnCount)
    (rowLabels : Fin rowCount → Fin q)
    (columnLabels : Fin columnCount → Fin q) :
    ℕ → ℕ → ColumnBucketState q → Bool
  | _, 0, _ => true
  | k, remaining + 1, state =>
      if hk : k < columnCount then
        decide (columnLabels ⟨k, hk⟩ =
          firstMinimumLabel hq
            (state.score left right (fun row => some (rowLabels row)) ⟨k, hk⟩)) &&
        replayColumnStateAux hq left right rowLabels columnLabels (k + 1) remaining
          (state.addVertex left right (fun row => some (rowLabels row))
            ⟨k, hk⟩ (columnLabels ⟨k, hk⟩))
      else false

/-- Complete single-pass column replay from empty auxiliary state. -/
def replayColumnState
    {q edgeCount rowCount columnCount : ℕ}
    (hq : 0 < q)
    (left : Fin edgeCount → Fin rowCount)
    (right : Fin edgeCount → Fin columnCount)
    (rowLabels : Fin rowCount → Fin q)
    (columnLabels : Fin columnCount → Fin q) : Bool :=
  replayColumnStateAux hq left right rowLabels columnLabels 0 columnCount
    (ColumnBucketState.empty q)

/-- A successful column-state suffix replay gives the exact extensional
profile-score first minimum at every covered native-order vertex. -/
theorem replayColumnStateAux_sound
    {q edgeCount rowCount columnCount : ℕ}
    (hq : 0 < q)
    (left : Fin edgeCount → Fin rowCount)
    (right : Fin edgeCount → Fin columnCount)
    (rowLabels : Fin rowCount → Fin q)
    (columnLabels : Fin columnCount → Fin q)
    (k remaining : ℕ)
    (state : ColumnBucketState q)
    (htotal : k + remaining = columnCount)
    (hmatches : state.Matches left right (fun row => some (rowLabels row))
      (prefixAssignment columnLabels k))
    (htrace : replayColumnStateAux hq left right rowLabels columnLabels
      k remaining state = true) :
    ∀ (i : ℕ) (hi : i < columnCount), k ≤ i →
      columnLabels ⟨i, hi⟩ =
        firstMinimumLabel hq
          (columnIncidenceProfileScore left right
            (fun row => some (rowLabels row))
            (prefixAssignment columnLabels i) ⟨i, hi⟩) := by
  induction remaining generalizing k state with
  | zero =>
      intro i hi hki
      omega
  | succ remaining ih =>
      have hk : k < columnCount := by omega
      simp only [replayColumnStateAux, hk, ↓reduceDIte, Bool.and_eq_true] at htrace
      have hcurrent := of_decide_eq_true htrace.1
      have hscore :
          (fun label => state.score left right
            (fun row => some (rowLabels row)) ⟨k, hk⟩ label) =
            columnIncidenceProfileScore left right
              (fun row => some (rowLabels row))
              (prefixAssignment columnLabels k) ⟨k, hk⟩ := by
        funext label
        exact state.score_eq_incidenceProfileScore left right
          (fun row => some (rowLabels row)) (prefixAssignment columnLabels k)
          ⟨k, hk⟩ label hmatches (prefixAssignment_next_unknown columnLabels hk)
      have hcurrent' : columnLabels ⟨k, hk⟩ =
          firstMinimumLabel hq
            (fun label => state.score left right
              (fun row => some (rowLabels row)) ⟨k, hk⟩ label) := by
        simpa only using hcurrent
      rw [hscore] at hcurrent'
      have hnextMatches := state.addVertex_matches left right
        (fun row => some (rowLabels row)) (prefixAssignment columnLabels k)
        ⟨k, hk⟩ (columnLabels ⟨k, hk⟩) hmatches
        (prefixAssignment_next_unknown columnLabels hk)
      rw [prefixAssignment_step columnLabels hk] at hnextMatches
      have hnextTotal : k + 1 + remaining = columnCount := by omega
      have hrest := ih (k + 1)
        (state.addVertex left right (fun row => some (rowLabels row))
          ⟨k, hk⟩ (columnLabels ⟨k, hk⟩))
        hnextTotal hnextMatches htrace.2
      intro i hi hki
      by_cases hsame : i = k
      · subst i
        simpa only using hcurrent'
      · exact hrest i hi (by omega)

/-- Complete successful column-state replay gives every local extensional
first minimum. -/
theorem replayColumnState_sound
    {q edgeCount rowCount columnCount : ℕ}
    (hq : 0 < q)
    (left : Fin edgeCount → Fin rowCount)
    (right : Fin edgeCount → Fin columnCount)
    (rowLabels : Fin rowCount → Fin q)
    (columnLabels : Fin columnCount → Fin q)
    (htrace : replayColumnState hq left right rowLabels columnLabels = true) :
    ∀ column : Fin columnCount,
      columnLabels column = firstMinimumLabel hq
        (columnIncidenceProfileScore left right
          (fun row => some (rowLabels row))
          (prefixAssignment columnLabels column.val) column) := by
  have hmatches :
      (ColumnBucketState.empty q).Matches left right
        (fun row => some (rowLabels row)) (prefixAssignment columnLabels 0) := by
    rw [prefixAssignment_zero]
    exact ColumnBucketState.empty_matches q left right _
  intro column
  exact replayColumnStateAux_sound hq left right rowLabels columnLabels
    0 columnCount _ (by omega) hmatches htrace column.val column.isLt (Nat.zero_le _)

/-- Successful single-pass column replay is extensionally equal to the
original recursive executable column schedule. -/
theorem replayColumnState_eq_executable
    {q edgeCount rowCount columnCount : ℕ}
    (hq : 0 < q)
    (left : Fin edgeCount → Fin rowCount)
    (right : Fin edgeCount → Fin columnCount)
    (rowLabels : Fin rowCount → Fin q)
    (columnLabels : Fin columnCount → Fin q)
    (htrace : replayColumnState hq left right rowLabels columnLabels = true) :
    columnLabels = executableColumnLabels hq left right rowLabels := by
  funext column
  have hlabel : ∀ k (hk : k < columnCount),
      columnLabels ⟨k, hk⟩ =
        executableColumnLabels hq left right rowLabels ⟨k, hk⟩ := by
    intro k
    induction k using Nat.strong_induction_on with
    | h k ih =>
      intro hk
      rw [executableColumnLabels_eq,
        firstMinimumColumnLabel_eq_incidenceProfileScore hq _ _ _ _
          (fun row => ⟨rowLabels row, rfl⟩)]
      have hlocal := replayColumnState_sound hq left right rowLabels columnLabels
        htrace ⟨k, hk⟩
      rw [hlocal]
      congr 2
      funext vertex
      simp only [prefixAssignment]
      split_ifs with hbefore
      · congr 1
        exact ih vertex.val hbefore vertex.isLt
      · rfl
  exact hlabel column.val column.isLt

end Curve25519BucketStateReplay
