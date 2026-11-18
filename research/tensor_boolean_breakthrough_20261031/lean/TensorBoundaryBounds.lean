import Mathlib.LinearAlgebra.Matrix.Rank

/-!
# Exact separator-message bounds

This module formalizes the algebraic bridge used by the boundary-rank
certificate report.  It intentionally assumes an exact linear factorization;
it says nothing about nonlinear, approximate, or randomized representations.
-/

namespace TensorBoundaryBounds

/-- Universal finite-matrix rank upper bound. -/
theorem matrixRankUpperBound
    {K I J : Type*} [Field K] [Fintype I] [Fintype J]
    (matrix : Matrix I J K) :
    matrix.rank ≤ min (Fintype.card I) (Fintype.card J) :=
  le_min (Matrix.rank_le_card_height matrix) (Matrix.rank_le_card_width matrix)

/-- A Boolean flattening with `rowBits` and `columnBits` has rank no larger
than its smaller side. -/
theorem booleanFlatteningRankUpperBound
    {K : Type*} [Field K] (rowBits columnBits : ℕ)
    (matrix : Matrix (Fin (2 ^ rowBits)) (Fin (2 ^ columnBits)) K) :
    matrix.rank ≤ min (2 ^ rowBits) (2 ^ columnBits) := by
  simpa using matrixRankUpperBound matrix

/-- An exact factorization through a `D`-indexed linear message has matrix rank
at most the number of message coordinates. -/
theorem exactLinearSeparatorRankBound
    {K I D J : Type*} [Field K] [Fintype I] [Fintype D] [Fintype J]
    (left : Matrix I D K) (right : Matrix D J K) :
    (left * right).rank ≤ Fintype.card D :=
  (Matrix.rank_mul_le_left left right).trans (Matrix.rank_le_card_width left)

/-- A nonzero `k × k` minor is a kernel-checked certificate that the containing
matrix has rank at least `k`. -/
theorem nonzeroMinorRankLowerBound
    {K I J : Type*} [Field K] [Fintype I] [Fintype J]
    (matrix : Matrix I J K) (k : ℕ) (rows : Fin k → I) (columns : Fin k → J)
    (minor_nonzero : (matrix.submatrix rows columns).det ≠ 0) :
    k ≤ matrix.rank := by
  calc
    k = Fintype.card (Fin k) := by simp
    _ = (matrix.submatrix rows columns).rank :=
      (Matrix.rank_of_det_ne_zero minor_nonzero).symm
    _ ≤ matrix.rank := Matrix.rank_submatrix_le matrix rows columns

/-- A certified nonzero minor forces every exact linear separator message to
have at least that many coordinates. -/
theorem nonzeroMinorForcesSeparatorDimension
    {K I D J : Type*} [Field K] [Fintype I] [Fintype D] [Fintype J]
    (left : Matrix I D K) (right : Matrix D J K)
    (k : ℕ) (rows : Fin k → I) (columns : Fin k → J)
    (minor_nonzero : ((left * right).submatrix rows columns).det ≠ 0) :
    k ≤ Fintype.card D :=
  (nonzeroMinorRankLowerBound (left * right) k rows columns minor_nonzero).trans
    (exactLinearSeparatorRankBound left right)

/-- Pointwise family bridge.  The hypotheses deliberately expose the missing
family-specific rank lemma (`certified_rank`) and the architecture-specific
separator lemma (`separator_bound`). -/
theorem exponentialRankForcesExponentialMessage
    (rank message exponent : ℕ → ℕ)
    (certified_rank : ∀ n, 2 ^ exponent n ≤ rank n)
    (separator_bound : ∀ n, rank n ≤ message n) :
    ∀ n, 2 ^ exponent n ≤ message n := fun n =>
  (certified_rank n).trans (separator_bound n)

/-- If the certified exponent itself grows at least linearly, the exact linear
message dimension has an explicit exponential lower bound. -/
theorem linearExponentForcesExponentialMessage
    (rank message exponent : ℕ → ℕ) (slope : ℕ)
    (certified_rank : ∀ n, 2 ^ exponent n ≤ rank n)
    (separator_bound : ∀ n, rank n ≤ message n)
    (linear_exponent : ∀ n, slope * n ≤ exponent n) :
    ∀ n, 2 ^ (slope * n) ≤ message n := by
  intro n
  exact (Nat.pow_le_pow_right (by decide) (linear_exponent n)).trans
    (exponentialRankForcesExponentialMessage rank message exponent certified_rank
      separator_bound n)

/-- The explicit work corollary for architectures that must process at least
one unit per exact linear-message coordinate. -/
theorem linearExponentForcesExponentialWork
    (rank message work exponent : ℕ → ℕ) (slope : ℕ)
    (certified_rank : ∀ n, 2 ^ exponent n ≤ rank n)
    (separator_bound : ∀ n, rank n ≤ message n)
    (message_work : ∀ n, message n ≤ work n)
    (linear_exponent : ∀ n, slope * n ≤ exponent n) :
    ∀ n, 2 ^ (slope * n) ≤ work n := by
  intro n
  exact (linearExponentForcesExponentialMessage rank message exponent slope certified_rank
    separator_bound linear_exponent n).trans (message_work n)

/-! ## Semantic decomposition relation

The following declarations separate the encoding-independent relation matrix
from a low-level Boolean-network boundary flattening. A row and column form a
complete decomposition exactly when `rel` holds.
-/

/-- The zero-one semantic matrix of a finite decomposition relation. -/
def decompositionRelationMatrix
    {Left Right : Type*} [Fintype Left] [Fintype Right]
    (rel : Left → Right → Prop) [DecidableRel rel] : Matrix Left Right ℚ :=
  fun left right => if rel left right then 1 else 0

private def matchedLeft
    {Left Right : Type*} [Fintype Left] [Fintype Right]
    (rel : Left → Right → Prop) [DecidableRel rel] : Finset Left :=
  Finset.univ.filter fun left => ∃ right, rel left right

private def decompositions
    {Left Right : Type*} [Fintype Left] [Fintype Right]
    (rel : Left → Right → Prop) [DecidableRel rel] : Finset (Left × Right) :=
  (Finset.univ.product Finset.univ).filter fun pair => rel pair.1 pair.2

/-- Only left tuples that extend to a complete decomposition can support a
nonzero row. This is a direct application of Mathlib's row-support rank
bound. -/
theorem decompositionRelationRankLeMatchedLeft
    {Left Right : Type*} [Fintype Left] [Fintype Right]
    (rel : Left → Right → Prop) [DecidableRel rel] :
    (decompositionRelationMatrix rel).rank ≤ (matchedLeft rel).card := by
  apply Matrix.rank_le_card_of_support_subset
  intro left hrow
  refine Finset.mem_filter.mpr ⟨Finset.mem_univ left, ?_⟩
  by_contra hnone
  apply hrow
  funext right
  have hnot : ¬ rel left right := fun hrel => hnone ⟨right, hrel⟩
  simp [Matrix.row, decompositionRelationMatrix, hnot]

/-- Selecting one right witness per matched left tuple injects matched rows
into complete decompositions. -/
theorem matchedLeftCardLeDecompositions
    {Left Right : Type*} [Fintype Left] [Fintype Right]
    (rel : Left → Right → Prop) [DecidableRel rel] :
    (matchedLeft rel).card ≤ (decompositions rel).card := by
  classical
  let choosePair : {left // left ∈ matchedLeft rel} →
      {pair // pair ∈ decompositions rel} := fun left => by
    have hex : ∃ right, rel left.1 right := by
      simpa [matchedLeft] using left.2
    let right := Classical.choose hex
    have hright : rel left.1 right := Classical.choose_spec hex
    refine ⟨(left.1, right), ?_⟩
    simp [decompositions, hright]
  have hinjective : Function.Injective choosePair := by
    intro left₁ left₂ heq
    apply Subtype.ext
    exact congrArg (fun pair => pair.1.1) heq
  simpa using Fintype.card_le_of_injective choosePair hinjective

/-- The rank of the semantic relation matrix is at most the exact number of
complete decompositions. -/
theorem decompositionRelationRankLeDecompositions
    {Left Right : Type*} [Fintype Left] [Fintype Right]
    (rel : Left → Right → Prop) [DecidableRel rel] :
    (decompositionRelationMatrix rel).rank ≤ (decompositions rel).card :=
  (decompositionRelationRankLeMatchedLeft rel).trans
    (matchedLeftCardLeDecompositions rel)

/-- Finite counting form of Markov's inequality: the number of seeds whose
rank reaches a threshold is controlled by the total solution count. -/
theorem highRankSeedCountBound
    {Seed : Type*} [Fintype Seed]
    (rankCount solutionCount : Seed → ℕ) (threshold : ℕ)
    (pointwise : ∀ seed, rankCount seed ≤ solutionCount seed) :
    threshold * (Finset.univ.filter fun seed => threshold ≤ rankCount seed).card ≤
      ∑ seed, solutionCount seed := by
  classical
  let high := Finset.univ.filter fun seed => threshold ≤ rankCount seed
  calc
    threshold * high.card = ∑ _seed ∈ high, threshold := by simp [Nat.mul_comm]
    _ ≤ ∑ seed ∈ high, rankCount seed := by
      exact Finset.sum_le_sum fun seed hseed => (Finset.mem_filter.mp hseed).2
    _ ≤ ∑ seed ∈ high, solutionCount seed := by
      exact Finset.sum_le_sum fun seed _ => pointwise seed
    _ ≤ ∑ seed ∈ Finset.univ, solutionCount seed := by
      exact Finset.sum_le_sum_of_subset_of_nonneg (Finset.filter_subset _ _) (by simp)
    _ = ∑ seed, solutionCount seed := rfl

/-- A rank floor that holds for every seed cannot exceed a supplied average
solution-count upper bound. -/
theorem uniformRankLeAverageSolutionBound
    {Seed : Type*} [Fintype Seed] [Nonempty Seed]
    (rankCount solutionCount : Seed → ℕ) (threshold averageBound : ℕ)
    (pointwise : ∀ seed, rankCount seed ≤ solutionCount seed)
    (uniform : ∀ seed, threshold ≤ rankCount seed)
    (totalBound : (∑ seed, solutionCount seed) ≤ averageBound * Fintype.card Seed) :
    threshold ≤ averageBound := by
  classical
  have hhigh := highRankSeedCountBound rankCount solutionCount threshold pointwise
  have hcard : (Finset.univ.filter fun seed => threshold ≤ rankCount seed).card =
      Fintype.card Seed := by
    simp [uniform]
  rw [hcard] at hhigh
  exact Nat.le_of_mul_le_mul_right (hhigh.trans totalBound) Fintype.card_pos

end TensorBoundaryBounds
