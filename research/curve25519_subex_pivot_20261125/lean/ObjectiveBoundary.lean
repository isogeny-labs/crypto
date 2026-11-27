module

public import LabelSequence

@[expose] public section

/-!
# Initial and final conditional-objective boundaries

The initial completely unknown objective is bounded by ordered endpoint-pair
charges.  The final fully labeled objective is exactly the label-pair fiber
collision count.  A generic fiber lemma turns both counts into sums of squared
fiber sizes.
-/

open scoped BigOperators

namespace Curve25519ObjectiveBoundary

open Curve25519ConditionalObjective
open Curve25519ConditionalVariance
open Curve25519LabelSequence

/-- Cardinality of one fiber of a finite map. -/
def fiberSize
    {Source Target : Type*}
    [Fintype Source]
    [DecidableEq Source]
    [DecidableEq Target]
    (map : Source → Target)
    (target : Target) : ℕ :=
  (Finset.univ.filter fun source ↦ map source = target).card

/-- Number of ordered source pairs in a common fiber. -/
def endpointPairCount
    {Source Target : Type*}
    [Fintype Source]
    [DecidableEq Target]
    (map : Source → Target) : ℕ :=
  ∑ first : Source, ∑ second : Source, if map first = map second then 1 else 0

/-- Fiber sizes partition the finite source. -/
theorem sum_fiberSize
    {Source Target : Type*}
    [Fintype Source]
    [Fintype Target]
    [DecidableEq Source]
    [DecidableEq Target]
    (map : Source → Target) :
    ∑ target, fiberSize map target = Fintype.card Source := by
  classical
  rw [← Finset.card_univ]
  symm
  simpa [fiberSize] using
    (Finset.card_eq_sum_card_fiberwise
      (s := Finset.univ) (t := Finset.univ) (f := map) (by simp))

/-- Ordered common-fiber pairs are counted by squared fiber sizes. -/
theorem endpointPairCount_eq_sum_fiberSize_sq
    {Source Target : Type*}
    [Fintype Source]
    [Fintype Target]
    [DecidableEq Source]
    [DecidableEq Target]
    (map : Source → Target) :
    endpointPairCount map = ∑ target, (fiberSize map target) ^ 2 := by
  classical
  unfold endpointPairCount
  calc
    (∑ first : Source, ∑ second : Source,
        if map first = map second then 1 else 0) =
      ∑ first : Source, fiberSize map (map first) := by
        apply Finset.sum_congr rfl
        intro first _
        simp [fiberSize, eq_comm]
    _ = ∑ first : Source, ∑ target : Target,
        if map first = target then fiberSize map target else 0 := by
          apply Finset.sum_congr rfl
          intro first _
          simp
    _ = ∑ target : Target, ∑ first : Source,
        if map first = target then fiberSize map target else 0 := by
          exact Finset.sum_comm
    _ = ∑ target : Target, (fiberSize map target) ^ 2 := by
          apply Finset.sum_congr rfl
          intro target _
          rw [Finset.sum_ite]
          simp [fiberSize, pow_two]

/-- With every endpoint unknown, one ordered edge pair is bounded by a baseline
unit plus `q²` for each shared endpoint. -/
theorem unknown_pair_potential_bound
    {q : ℕ}
    (hq : 0 < q)
    {Row Column : Type*}
    [DecidableEq Row]
    [DecidableEq Column]
    (leftFirst leftSecond : Row)
    (rightFirst rightSecond : Column) :
    equalityPotential q leftFirst leftSecond (fun _ ↦ none) *
        equalityPotential q rightFirst rightSecond (fun _ ↦ none) ≤
      1 + q ^ 2 *
        ((if leftFirst = leftSecond then 1 else 0) +
          (if rightFirst = rightSecond then 1 else 0)) := by
  have hqSquare : q ≤ q * q := by
    simpa using Nat.le_mul_of_pos_left q hq
  by_cases hleft : leftFirst = leftSecond <;>
    by_cases hright : rightFirst = rightSecond <;>
    simp [equalityPotential, hleft, hright, pow_two] <;> omega

/-- The completely unknown objective is bounded by the independent baseline
plus the two ordered endpoint-pair charges. -/
theorem initial_objective_bound
    {q edgeCount rowCount columnCount : ℕ}
    (hq : 0 < q)
    (left : Fin edgeCount → Fin rowCount)
    (right : Fin edgeCount → Fin columnCount) :
    conditionalObjective q left right (fun _ ↦ none) (fun _ ↦ none) ≤
      edgeCount ^ 2 + q ^ 2 * (endpointPairCount left + endpointPairCount right) := by
  unfold conditionalObjective
  calc
    (∑ first : Fin edgeCount, ∑ second : Fin edgeCount,
      equalityPotential q (left first) (left second) (fun _ ↦ none) *
        equalityPotential q (right first) (right second) (fun _ ↦ none)) ≤
      ∑ first : Fin edgeCount, ∑ second : Fin edgeCount,
        (1 + q ^ 2 *
          ((if left first = left second then 1 else 0) +
            (if right first = right second then 1 else 0))) := by
              exact Finset.sum_le_sum fun first _ ↦
                Finset.sum_le_sum fun second _ ↦
                  unknown_pair_potential_bound hq
                    (left first) (left second) (right first) (right second)
    _ = edgeCount ^ 2 + q ^ 2 * (endpointPairCount left + endpointPairCount right) := by
      simp_rw [Nat.mul_add, Finset.sum_add_distrib]
      simp only [Finset.sum_const, Finset.card_univ, Fintype.card_fin, nsmul_eq_mul]
      unfold endpointPairCount
      simp_rw [← Finset.mul_sum]
      simp [pow_two]

/-- With every endpoint fixed, equality potential depends only on equality of
the assigned labels. -/
theorem equalityPotential_fully_labeled
    (q : ℕ)
    {Vertex : Type*}
    [DecidableEq Vertex]
    (labels : Vertex → Fin q)
    (first second : Vertex) :
    equalityPotential q first second (fun vertex ↦ some (labels vertex)) =
      if labels first = labels second then q else 0 := by
  by_cases hvertex : first = second
  · subst second
    simp [equalityPotential]
  · by_cases hlabel : labels first = labels second <;>
      simp [equalityPotential, hvertex, hlabel]

/-- The fully labeled objective is `q²` times the ordered collision count of
the row/column label-pair map. -/
theorem final_objective_eq_pair_collisions
    {q edgeCount rowCount columnCount : ℕ}
    (left : Fin edgeCount → Fin rowCount)
    (right : Fin edgeCount → Fin columnCount)
    (rowLabels : Fin rowCount → Fin q)
    (columnLabels : Fin columnCount → Fin q) :
    conditionalObjective q left right
        (fun row ↦ some (rowLabels row))
        (fun column ↦ some (columnLabels column)) =
      q ^ 2 * endpointPairCount
        (fun edge ↦ (rowLabels (left edge), columnLabels (right edge))) := by
  unfold conditionalObjective endpointPairCount
  rw [Finset.mul_sum]
  apply Finset.sum_congr rfl
  intro first _
  rw [Finset.mul_sum]
  apply Finset.sum_congr rfl
  intro second _
  rw [equalityPotential_fully_labeled, equalityPotential_fully_labeled]
  by_cases hrow : rowLabels (left first) = rowLabels (left second) <;>
    by_cases hcolumn : columnLabels (right first) = columnLabels (right second) <;>
    simp [hrow, hcolumn, Prod.ext_iff, pow_two]

/-- Encode the row/column label pair as one of exactly `q²` classes. -/
def classMap
    {q edgeCount rowCount columnCount : ℕ}
    (left : Fin edgeCount → Fin rowCount)
    (right : Fin edgeCount → Fin columnCount)
    (rowLabels : Fin rowCount → Fin q)
    (columnLabels : Fin columnCount → Fin q) :
    Fin edgeCount → Fin (q * q) :=
  fun edge ↦ finProdFinEquiv (rowLabels (left edge), columnLabels (right edge))

/-- The final objective is `q²` times the sum of squared encoded class
sizes. -/
theorem final_objective_eq_class_squares
    {q edgeCount rowCount columnCount : ℕ}
    (left : Fin edgeCount → Fin rowCount)
    (right : Fin edgeCount → Fin columnCount)
    (rowLabels : Fin rowCount → Fin q)
    (columnLabels : Fin columnCount → Fin q) :
    conditionalObjective q left right
        (fun row ↦ some (rowLabels row))
        (fun column ↦ some (columnLabels column)) =
      q ^ 2 * ∑ γ : Fin (q * q),
        (fiberSize (classMap left right rowLabels columnLabels) γ) ^ 2 := by
  rw [final_objective_eq_pair_collisions]
  have hencode :
      endpointPairCount
          (fun edge ↦ (rowLabels (left edge), columnLabels (right edge))) =
        endpointPairCount (classMap left right rowLabels columnLabels) := by
    unfold endpointPairCount classMap
    apply Finset.sum_congr rfl
    intro first _
    apply Finset.sum_congr rfl
    intro second _
    simp
  rw [hencode, endpointPairCount_eq_sum_fiberSize_sq]

/-- Minimum labels, capped degrees, and the exact objective boundaries give the
scaled collision inequality over the encoded `q²` class vector. -/
theorem scaled_class_collision_bound
    {q edgeCount rowCount columnCount : ℕ}
    (hq : 0 < q)
    (left : Fin edgeCount → Fin rowCount)
    (right : Fin edgeCount → Fin columnCount)
    (rowLabels : Fin rowCount → Fin q)
    (columnLabels : Fin columnCount → Fin q)
    (δ : ℕ)
    (hleftCap : ∀ row, fiberSize left row ≤ δ)
    (hrightCap : ∀ column, fiberSize right column ≤ δ)
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
    q ^ 2 * ∑ γ : Fin (q * q),
        (fiberSize (classMap left right rowLabels columnLabels) γ) ^ 2 ≤
      edgeCount ^ 2 + 2 * edgeCount * δ * q ^ 2 := by
  have hdegree := bipartite_degree_square_budget
    (fiberSize left) (fiberSize right) edgeCount δ
    (by simpa using sum_fiberSize left)
    (by simpa using sum_fiberSize right)
    hleftCap hrightCap
  have hpairs : endpointPairCount left + endpointPairCount right ≤ 2 * edgeCount * δ := by
    simpa [endpointPairCount_eq_sum_fiberSize_sq] using hdegree
  have hinitial := initial_objective_bound hq left right
  have hinitialCapped :
      conditionalObjective q left right (fun _ ↦ none) (fun _ ↦ none) ≤
        edgeCount ^ 2 + q ^ 2 * (2 * edgeCount * δ) :=
    hinitial.trans (Nat.add_le_add_left (Nat.mul_le_mul_left (q ^ 2) hpairs) _)
  have hsequence := full_label_sequence_nonincreasing hq left right
    rowLabels columnLabels hrowMinimum hcolumnMinimum
  calc
    q ^ 2 * ∑ γ : Fin (q * q),
        (fiberSize (classMap left right rowLabels columnLabels) γ) ^ 2 =
      conditionalObjective q left right
        (fun row ↦ some (rowLabels row))
        (fun column ↦ some (columnLabels column)) :=
          (final_objective_eq_class_squares left right rowLabels columnLabels).symm
    _ ≤ conditionalObjective q left right (fun _ ↦ none) (fun _ ↦ none) := hsequence
    _ ≤ edgeCount ^ 2 + q ^ 2 * (2 * edgeCount * δ) := hinitialCapped
    _ = edgeCount ^ 2 + 2 * edgeCount * δ * q ^ 2 := by ring

/-- Under the strongest source guard, the fully typed graph/objective chain
forces every encoded class below `2M/D`, with `D=q²`. -/
theorem class_cap_for_minimum_labels
    {q edgeCount rowCount columnCount : ℕ}
    (hq : 0 < q)
    (left : Fin edgeCount → Fin rowCount)
    (right : Fin edgeCount → Fin columnCount)
    (rowLabels : Fin rowCount → Fin q)
    (columnLabels : Fin columnCount → Fin q)
    (δ : ℕ)
    (hleftCap : ∀ row, fiberSize left row ≤ δ)
    (hrightCap : ∀ column, fiberSize right column ≤ δ)
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
            (some candidate)))
    (hguard : 2 * ((q * q : ℕ) : ℝ) ^ 2 * (δ : ℝ) ≤ (edgeCount : ℝ)) :
    ∀ γ : Fin (q * q),
      (fiberSize (classMap left right rowLabels columnLabels) γ : ℝ) *
          ((q * q : ℕ) : ℝ) ≤
        2 * (edgeCount : ℝ) := by
  let encoded := classMap left right rowLabels columnLabels
  let classes : Fin (q * q) → ℝ := fun γ ↦ (fiberSize encoded γ : ℝ)
  have hcollision := scaled_class_collision_bound hq left right rowLabels columnLabels δ
    hleftCap hrightCap hrowMinimum hcolumnMinimum
  have hcollisionMul :
      (q * q) * ∑ γ : Fin (q * q),
          (fiberSize (classMap left right rowLabels columnLabels) γ) ^ 2 ≤
        edgeCount ^ 2 + 2 * edgeCount * δ * (q * q) := by
    simpa [pow_two] using hcollision
  have hcollisionReal :
      ((q * q : ℕ) : ℝ) * (∑ γ, (classes γ) ^ 2) ≤
        (edgeCount : ℝ) ^ 2 +
          2 * (edgeCount : ℝ) * (δ : ℝ) * ((q * q : ℕ) : ℝ) := by
    dsimp [classes, encoded]
    exact_mod_cast hcollisionMul
  have hobjective :
      ((q * q : ℕ) : ℝ) * (∑ γ, (classes γ) ^ 2) -
          (edgeCount : ℝ) ^ 2 ≤
        2 * (edgeCount : ℝ) * (δ : ℝ) * ((q * q : ℕ) : ℝ) := by
    linarith
  have hmass : ∑ γ, classes γ = (edgeCount : ℝ) := by
    dsimp [classes]
    exact_mod_cast (by simpa using sum_fiberSize encoded)
  have hD : 0 < q * q := Nat.mul_pos hq hq
  simpa [classes, encoded] using
    (class_cap_of_scaled_objective hD classes (edgeCount : ℝ) (δ : ℝ)
      (by positivity) hmass hobjective hguard)

/-- The strongest source guard bounds every class produced by the complete
executable first-minimum row-then-column schedule, with no external label or
minimum-family premise. -/
theorem class_cap_for_executable_labels
    {q edgeCount rowCount columnCount : ℕ}
    (hq : 0 < q)
    (left : Fin edgeCount → Fin rowCount)
    (right : Fin edgeCount → Fin columnCount)
    (δ : ℕ)
    (hleftCap : ∀ row, fiberSize left row ≤ δ)
    (hrightCap : ∀ column, fiberSize right column ≤ δ)
    (hguard : 2 * ((q * q : ℕ) : ℝ) ^ 2 * (δ : ℝ) ≤ (edgeCount : ℝ)) :
    ∀ γ : Fin (q * q),
      (fiberSize (classMap left right
          (executableRowLabels hq left right)
          (executableColumnLabels hq left right
            (executableRowLabels hq left right))) γ : ℝ) *
          ((q * q : ℕ) : ℝ) ≤
        2 * (edgeCount : ℝ) := by
  apply class_cap_for_minimum_labels hq left right
    (executableRowLabels hq left right)
    (executableColumnLabels hq left right (executableRowLabels hq left right))
    δ hleftCap hrightCap
  · exact executableRowLabels_minimum hq left right
  · exact executableColumnLabels_minimum hq left right
      (executableRowLabels hq left right)
  · exact hguard

end Curve25519ObjectiveBoundary
