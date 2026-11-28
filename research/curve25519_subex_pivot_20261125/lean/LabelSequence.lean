module

public import ConditionalObjective

@[expose] public section

/-!
# Finite composition of minimum-label steps

Vertices are labeled in their native array order.  A prefix assignment exposes
exactly the first `k` chosen labels.  The local conditional-objective theorem
then composes over every row followed by every column.
-/

namespace Curve25519LabelSequence

open Curve25519ConditionalObjective

/-- The first `k` vertices are assigned their chosen labels; later vertices
remain unknown. -/
def prefixAssignment
    {n q : ℕ}
    (labels : Fin n → Fin q)
    (k : ℕ) : Fin n → Option (Fin q) :=
  fun vertex ↦ if vertex.val < k then some (labels vertex) else none

/-- The next vertex is unknown before its prefix step. -/
theorem prefixAssignment_next_unknown
    {n q k : ℕ}
    (labels : Fin n → Fin q)
    (hk : k < n) :
    prefixAssignment labels k ⟨k, hk⟩ = none := by
  simp [prefixAssignment]

/-- Updating the next vertex is exactly the successor prefix. -/
theorem prefixAssignment_step
    {n q k : ℕ}
    (labels : Fin n → Fin q)
    (hk : k < n) :
    Function.update (prefixAssignment labels k) ⟨k, hk⟩ (some (labels ⟨k, hk⟩)) =
      prefixAssignment labels (k + 1) := by
  classical
  funext vertex
  by_cases hcurrent : vertex = ⟨k, hk⟩
  · subst vertex
    simp [prefixAssignment]
  · have hval : vertex.val ≠ k := by
      intro h
      apply hcurrent
      apply Fin.ext
      exact h
    by_cases hbefore : vertex.val < k
    · have hbeforeNext : vertex.val < k + 1 := by omega
      simp [Function.update, hcurrent, prefixAssignment, hbefore, hbeforeNext]
    · have hafter : ¬ vertex.val < k + 1 := by omega
      simp [Function.update, hcurrent, prefixAssignment, hbefore, hafter]

/-- A zero-length prefix is the everywhere-unknown assignment. -/
theorem prefixAssignment_zero
    {n q : ℕ}
    (labels : Fin n → Fin q) :
    prefixAssignment labels 0 = fun _ ↦ none := by
  funext vertex
  simp [prefixAssignment]

/-- The full prefix assigns every vertex. -/
theorem prefixAssignment_full
    {n q : ℕ}
    (labels : Fin n → Fin q) :
    prefixAssignment labels n = fun vertex ↦ some (labels vertex) := by
  funext vertex
  simp [prefixAssignment, vertex.isLt]

/-- Minimum row choices compose over the complete native row order. -/
theorem row_sequence_nonincreasing
    {q edgeCount rowCount columnCount : ℕ}
    (hq : 0 < q)
    (left : Fin edgeCount → Fin rowCount)
    (right : Fin edgeCount → Fin columnCount)
    (rowLabels : Fin rowCount → Fin q)
    (columns : Fin columnCount → Option (Fin q))
    (hminimum : ∀ k (hk : k < rowCount) (candidate : Fin q),
      conditionalObjective q left right
          (Function.update (prefixAssignment rowLabels k) ⟨k, hk⟩
            (some (rowLabels ⟨k, hk⟩))) columns ≤
        conditionalObjective q left right
          (Function.update (prefixAssignment rowLabels k) ⟨k, hk⟩
            (some candidate)) columns) :
    conditionalObjective q left right (fun row ↦ some (rowLabels row)) columns ≤
      conditionalObjective q left right (fun _ ↦ none) columns := by
  have hstep : ∀ k (hk : k < rowCount),
      conditionalObjective q left right (prefixAssignment rowLabels (k + 1)) columns ≤
        conditionalObjective q left right (prefixAssignment rowLabels k) columns := by
    intro k hk
    rw [← prefixAssignment_step rowLabels hk]
    exact minimum_row_label_nonincreasing hq left right
      (prefixAssignment rowLabels k) columns ⟨k, hk⟩ (rowLabels ⟨k, hk⟩)
      (prefixAssignment_next_unknown rowLabels hk) (hminimum k hk)
  have hprefix : ∀ k, k ≤ rowCount →
      conditionalObjective q left right (prefixAssignment rowLabels k) columns ≤
        conditionalObjective q left right (prefixAssignment rowLabels 0) columns := by
    intro k hk
    induction k with
    | zero => exact le_rfl
    | succ k ih =>
        have hklt : k < rowCount := Nat.lt_of_succ_le hk
        exact (hstep k hklt).trans (ih (Nat.le_of_lt hklt))
  rw [← prefixAssignment_full rowLabels, ← prefixAssignment_zero rowLabels]
  exact hprefix rowCount le_rfl

/-- Minimum column choices compose over the complete native column order. -/
theorem column_sequence_nonincreasing
    {q edgeCount rowCount columnCount : ℕ}
    (hq : 0 < q)
    (left : Fin edgeCount → Fin rowCount)
    (right : Fin edgeCount → Fin columnCount)
    (rows : Fin rowCount → Option (Fin q))
    (columnLabels : Fin columnCount → Fin q)
    (hminimum : ∀ k (hk : k < columnCount) (candidate : Fin q),
      conditionalObjective q left right rows
          (Function.update (prefixAssignment columnLabels k) ⟨k, hk⟩
            (some (columnLabels ⟨k, hk⟩))) ≤
        conditionalObjective q left right rows
          (Function.update (prefixAssignment columnLabels k) ⟨k, hk⟩
            (some candidate))) :
    conditionalObjective q left right rows (fun column ↦ some (columnLabels column)) ≤
      conditionalObjective q left right rows (fun _ ↦ none) := by
  have hstep : ∀ k (hk : k < columnCount),
      conditionalObjective q left right rows (prefixAssignment columnLabels (k + 1)) ≤
        conditionalObjective q left right rows (prefixAssignment columnLabels k) := by
    intro k hk
    rw [← prefixAssignment_step columnLabels hk]
    exact minimum_column_label_nonincreasing hq left right rows
      (prefixAssignment columnLabels k) ⟨k, hk⟩ (columnLabels ⟨k, hk⟩)
      (prefixAssignment_next_unknown columnLabels hk) (hminimum k hk)
  have hprefix : ∀ k, k ≤ columnCount →
      conditionalObjective q left right rows (prefixAssignment columnLabels k) ≤
        conditionalObjective q left right rows (prefixAssignment columnLabels 0) := by
    intro k hk
    induction k with
    | zero => exact le_rfl
    | succ k ih =>
        have hklt : k < columnCount := Nat.lt_of_succ_le hk
        exact (hstep k hklt).trans (ih (Nat.le_of_lt hklt))
  rw [← prefixAssignment_full columnLabels, ← prefixAssignment_zero columnLabels]
  exact hprefix columnCount le_rfl

/-- Native-order row minima followed by column minima make the fully labeled
objective no larger than the completely unknown objective. -/
theorem full_label_sequence_nonincreasing
    {q edgeCount rowCount columnCount : ℕ}
    (hq : 0 < q)
    (left : Fin edgeCount → Fin rowCount)
    (right : Fin edgeCount → Fin columnCount)
    (rowLabels : Fin rowCount → Fin q)
    (columnLabels : Fin columnCount → Fin q)
    (hrowMinimum : ∀ k (hk : k < rowCount) (candidate : Fin q),
      conditionalObjective q left right
          (Function.update (prefixAssignment rowLabels k) ⟨k, hk⟩
            (some (rowLabels ⟨k, hk⟩))) (fun _ ↦ none) ≤
        conditionalObjective q left right
          (Function.update (prefixAssignment rowLabels k) ⟨k, hk⟩
            (some candidate)) (fun _ ↦ none))
    (hcolumnMinimum : ∀ k (hk : k < columnCount) (candidate : Fin q),
      conditionalObjective q left right (fun row ↦ some (rowLabels row))
          (Function.update (prefixAssignment columnLabels k) ⟨k, hk⟩
            (some (columnLabels ⟨k, hk⟩))) ≤
        conditionalObjective q left right (fun row ↦ some (rowLabels row))
          (Function.update (prefixAssignment columnLabels k) ⟨k, hk⟩
            (some candidate))) :
    conditionalObjective q left right
        (fun row ↦ some (rowLabels row))
        (fun column ↦ some (columnLabels column)) ≤
      conditionalObjective q left right (fun _ ↦ none) (fun _ ↦ none) := by
  exact (column_sequence_nonincreasing hq left right
    (fun row ↦ some (rowLabels row)) columnLabels hcolumnMinimum).trans
      (row_sequence_nonincreasing hq left right rowLabels (fun _ ↦ none) hrowMinimum)

/-- Construct every row label in native order from the exact first-minimum
objective over the already assigned row prefix. -/
def executableRowLabels
    {q edgeCount rowCount columnCount : ℕ}
    (hq : 0 < q)
    (left : Fin edgeCount → Fin rowCount)
    (right : Fin edgeCount → Fin columnCount)
    (vertex : Fin rowCount) : Fin q :=
  firstMinimumRowLabel hq left right
    (fun candidate =>
      if _h : candidate.val < vertex.val then
        some (executableRowLabels hq left right candidate)
      else none)
    (fun _ => none) vertex
termination_by vertex.val

/-- Unfold one constructed row label into the canonical prefix assignment. -/
theorem executableRowLabels_eq
    {q edgeCount rowCount columnCount : ℕ}
    (hq : 0 < q)
    (left : Fin edgeCount → Fin rowCount)
    (right : Fin edgeCount → Fin columnCount)
    (vertex : Fin rowCount) :
    executableRowLabels hq left right vertex =
      firstMinimumRowLabel hq left right
        (prefixAssignment (executableRowLabels hq left right) vertex.val)
        (fun _ => none) vertex := by
  rw [executableRowLabels]
  congr 2

/-- Every constructed row label is a minimum for its exact native prefix. -/
theorem executableRowLabels_minimum
    {q edgeCount rowCount columnCount : ℕ}
    (hq : 0 < q)
    (left : Fin edgeCount → Fin rowCount)
    (right : Fin edgeCount → Fin columnCount)
    (k : ℕ) (hk : k < rowCount) (candidate : Fin q) :
    conditionalObjective q left right
        (Function.update
          (prefixAssignment (executableRowLabels hq left right) k) ⟨k, hk⟩
          (some (executableRowLabels hq left right ⟨k, hk⟩)))
        (fun _ => none) ≤
      conditionalObjective q left right
        (Function.update
          (prefixAssignment (executableRowLabels hq left right) k) ⟨k, hk⟩
          (some candidate))
        (fun _ => none) := by
  rw [executableRowLabels_eq]
  simpa only [firstMinimumRowLabel] using
    (firstMinimumLabel_le hq
      (fun label => conditionalObjective q left right
        (Function.update
          (prefixAssignment (executableRowLabels hq left right) k) ⟨k, hk⟩
          (some label))
        (fun _ => none)) candidate)

/-- Construct every column label in native order after fixing all row labels. -/
def executableColumnLabels
    {q edgeCount rowCount columnCount : ℕ}
    (hq : 0 < q)
    (left : Fin edgeCount → Fin rowCount)
    (right : Fin edgeCount → Fin columnCount)
    (rowLabels : Fin rowCount → Fin q)
    (vertex : Fin columnCount) : Fin q :=
  firstMinimumColumnLabel hq left right (fun row => some (rowLabels row))
    (fun candidate =>
      if _h : candidate.val < vertex.val then
        some (executableColumnLabels hq left right rowLabels candidate)
      else none)
    vertex
termination_by vertex.val

/-- Unfold one constructed column label into the canonical prefix assignment. -/
theorem executableColumnLabels_eq
    {q edgeCount rowCount columnCount : ℕ}
    (hq : 0 < q)
    (left : Fin edgeCount → Fin rowCount)
    (right : Fin edgeCount → Fin columnCount)
    (rowLabels : Fin rowCount → Fin q)
    (vertex : Fin columnCount) :
    executableColumnLabels hq left right rowLabels vertex =
      firstMinimumColumnLabel hq left right (fun row => some (rowLabels row))
        (prefixAssignment
          (executableColumnLabels hq left right rowLabels) vertex.val)
        vertex := by
  rw [executableColumnLabels]
  congr 2

/-- Every constructed column label is a minimum for its exact native prefix. -/
theorem executableColumnLabels_minimum
    {q edgeCount rowCount columnCount : ℕ}
    (hq : 0 < q)
    (left : Fin edgeCount → Fin rowCount)
    (right : Fin edgeCount → Fin columnCount)
    (rowLabels : Fin rowCount → Fin q)
    (k : ℕ) (hk : k < columnCount) (candidate : Fin q) :
    conditionalObjective q left right (fun row => some (rowLabels row))
        (Function.update
          (prefixAssignment
            (executableColumnLabels hq left right rowLabels) k) ⟨k, hk⟩
          (some (executableColumnLabels hq left right rowLabels ⟨k, hk⟩))) ≤
      conditionalObjective q left right (fun row => some (rowLabels row))
        (Function.update
          (prefixAssignment
            (executableColumnLabels hq left right rowLabels) k) ⟨k, hk⟩
          (some candidate)) := by
  rw [executableColumnLabels_eq]
  simpa only [firstMinimumColumnLabel] using
    (firstMinimumLabel_le hq
      (fun label => conditionalObjective q left right
        (fun row => some (rowLabels row))
        (Function.update
          (prefixAssignment
            (executableColumnLabels hq left right rowLabels) k) ⟨k, hk⟩
          (some label))) candidate)

/-- The complete executable row-then-column first-minimum schedule cannot
increase the exact conditional objective. -/
theorem executableFullLabelSequence_nonincreasing
    {q edgeCount rowCount columnCount : ℕ}
    (hq : 0 < q)
    (left : Fin edgeCount → Fin rowCount)
    (right : Fin edgeCount → Fin columnCount) :
    conditionalObjective q left right
        (fun row => some (executableRowLabels hq left right row))
        (fun column => some (executableColumnLabels hq left right
          (executableRowLabels hq left right) column)) ≤
      conditionalObjective q left right (fun _ => none) (fun _ => none) := by
  apply full_label_sequence_nonincreasing hq left right
    (executableRowLabels hq left right)
    (executableColumnLabels hq left right (executableRowLabels hq left right))
  · exact executableRowLabels_minimum hq left right
  · exact executableColumnLabels_minimum hq left right
      (executableRowLabels hq left right)

end Curve25519LabelSequence
