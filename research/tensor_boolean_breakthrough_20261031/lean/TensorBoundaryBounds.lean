import Mathlib.LinearAlgebra.Matrix.Rank
import Mathlib.Algebra.Polynomial.Roots
import Mathlib.Algebra.Field.ZMod
import Mathlib.LinearAlgebra.Matrix.Charpoly.Coeff
import Mathlib.Analysis.SpecificLimits.Basic
import Mathlib.LinearAlgebra.Matrix.Determinant.Bird.Correctness
import Mathlib.Data.Matrix.ColumnRowPartitioned

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

/-! ## No-subexponential-work goal

The solver predicate below is a parameter on purpose.  This file does not
define a machine model or assert that any work function solves ECDLP.  It
separates the asymptotic implication from that missing correctness-and-cost
bridge.
-/

/-- Operational binary-work definition of subexponential growth.  For every
positive rational rate `a / b`, the `b`-th power of the work is eventually
strictly below `2^(a*n)`. -/
def IsSubexponentialBinaryWork (work : ℕ → ℕ) : Prop :=
  ∀ a b : ℕ, 0 < a → 0 < b →
    ∃ N : ℕ, ∀ n : ℕ, N ≤ n → work n ^ b < 2 ^ (a * n)

/-- A positive rational-rate exponential lower bound on binary work. -/
def HasPositiveExponentialLowerBound (work : ℕ → ℕ) : Prop :=
  ∃ a b : ℕ, 0 < a ∧ 0 < b ∧
    ∃ N : ℕ, ∀ n : ℕ, N ≤ n → 2 ^ (a * n) ≤ work n ^ b

/-- The requested goal, parameterized by the predicate specifying which work
functions belong to correct ECDLP solvers. -/
def NoSubexponentialECDLPGoal (SolvesECDLP : (ℕ → ℕ) → Prop) : Prop :=
  ∀ work, SolvesECDLP work → ¬ IsSubexponentialBinaryWork work

/-- Constant work satisfies the operational subexponential definition. -/
theorem constantOneWorkIsSubexponential :
    IsSubexponentialBinaryWork (fun _ => 1) := by
  intro a b ha _hb
  refine ⟨1, fun n hn => ?_⟩
  simp only [one_pow]
  apply Nat.one_lt_two_pow
  exact Nat.mul_ne_zero ha.ne' (Nat.ne_of_gt (lt_of_lt_of_le Nat.zero_lt_one hn))

/-- A positive exponential lower bound and subexponential work are
incompatible. -/
theorem exponentialLowerBoundNotSubexponential
    {work : ℕ → ℕ} (lower : HasPositiveExponentialLowerBound work) :
    ¬ IsSubexponentialBinaryWork work := by
  rintro subexponential
  rcases lower with ⟨a, b, ha, hb, lowerStart, lowerBound⟩
  rcases subexponential a b ha hb with ⟨subStart, subBound⟩
  let n := max lowerStart subStart
  exact Nat.not_lt_of_ge (lowerBound n (le_max_left _ _))
    (subBound n (le_max_right _ _))

/-- A pointwise binary-work floor with a positive integer slope supplies the
positive rational-rate exponential lower bound used by the asymptotic goal. -/
theorem positiveLinearWorkLowerBoundHasPositiveExponentialLowerBound
    (work : ℕ → ℕ) (slope : ℕ) (slopePositive : 0 < slope)
    (lower : ∀ n, 2 ^ (slope * n) ≤ work n) :
    HasPositiveExponentialLowerBound work := by
  refine ⟨slope, 1, slopePositive, Nat.zero_lt_one, 0, fun n _ => ?_⟩
  simpa using lower n

/-- The registered exact-linear-message chain excludes subexponential work
for the named backend once its family rank exponent has positive slope. -/
theorem linearRankBridgeNotSubexponential
    (rank message work exponent : ℕ → ℕ) (slope : ℕ)
    (slopePositive : 0 < slope)
    (certifiedRank : ∀ n, 2 ^ exponent n ≤ rank n)
    (separatorBound : ∀ n, rank n ≤ message n)
    (messageWork : ∀ n, message n ≤ work n)
    (linearExponent : ∀ n, slope * n ≤ exponent n) :
    ¬ IsSubexponentialBinaryWork work := by
  apply exponentialLowerBoundNotSubexponential
  apply positiveLinearWorkLowerBoundHasPositiveExponentialLowerBound work slope slopePositive
  exact linearExponentForcesExponentialWork rank message work exponent slope certifiedRank
    separatorBound messageWork linearExponent

/-- An ECDLP-wide solver-to-exponential-work bridge would establish the
requested no-subexponential goal. -/
theorem noSubexponentialECDLPGoalOfExponentialLowerBound
    {SolvesECDLP : (ℕ → ℕ) → Prop}
    (bridge : ∀ work, SolvesECDLP work → HasPositiveExponentialLowerBound work) :
    NoSubexponentialECDLPGoal SolvesECDLP := by
  intro work solves
  exact exponentialLowerBoundNotSubexponential (bridge work solves)

/-- The semantic rank/count premises alone do not establish the requested
no-subexponential-ECDLP goal.  The one-seed zero-rank model satisfies those
premises, while a predicate accepting constant-one work supplies a formal
countermodel.  This does not assert that constant-one work solves real ECDLP;
it exposes the missing solver-to-rank/work bridge. -/
theorem semanticRankPremisesDoNotEstablishNoSubexponentialECDLPGoal :
    ¬ (∀ (SolvesECDLP : (ℕ → ℕ) → Prop)
        (rankCount solutionCount : PUnit → ℕ),
      (∀ seed, rankCount seed ≤ solutionCount seed) →
      (∑ seed, solutionCount seed) ≤ 2 * Fintype.card PUnit →
      NoSubexponentialECDLPGoal SolvesECDLP) := by
  intro claimedImplication
  let oneWork : ℕ → ℕ := fun _ => 1
  let solvesOne : (ℕ → ℕ) → Prop := fun work => work = oneWork
  have rulesOut := claimedImplication solvesOne (fun _ => 0) (fun _ => 0)
    (by intro seed; exact le_rfl) (by simp)
  have solves : solvesOne oneWork := rfl
  exact (rulesOut oneWork solves) constantOneWorkIsSubexponential

/-! ## Universal scheduling of fixed-arity candidates

These interfaces separate the generic diagonal argument from the missing
probabilistic machine model.  A universal verified scheduler may enumerate all
finite program descriptions; it need not compute a preferred program from an
arity.  The first interface records only the resulting polynomial simulation
bound for each candidate work function.
-/

/-- A universal expected-work bound polynomially simulates every fixed
candidate, with a constant overhead that may depend on that candidate. -/
def HasPolynomialUniversalSimulationBound
    (universal : ℕ → ℕ) (candidate : ℕ → ℕ → ℕ) (degree : ℕ) : Prop :=
  0 < degree ∧ ∀ m, ∃ overhead : ℕ, 0 < overhead ∧
    ∀ n, universal n ≤ overhead * candidate m n ^ degree

/-- After the fixed simulation degree is charged, candidates achieve every
positive rational exponential rate.  The chosen candidate may depend on the
rate; its eventual bound absorbs every fixed positive description overhead. -/
def HasVanishingRateCandidateFamily
    (candidate : ℕ → ℕ → ℕ) (degree : ℕ) : Prop :=
  ∀ a b : ℕ, 0 < a → 0 < b →
    ∃ m : ℕ, ∀ overhead : ℕ, 0 < overhead →
      ∃ N : ℕ, ∀ n : ℕ, N ≤ n →
        (overhead * candidate m n ^ degree) ^ b < 2 ^ (a * n)

/-- A fixed-numerator rate family supplies the eventual work inequality for
every arity whose rate clears the requested rational exponent.  The intended
instance is a fixed-arity bound such as `p^(3/m+o(1))`, after charging the
universal simulator's fixed polynomial degree. -/
def HasFixedNumeratorCandidateFamily
    (candidate : ℕ → ℕ → ℕ) (degree numerator : ℕ) : Prop :=
  ∀ m a b : ℕ, 0 < a → 0 < b → numerator * degree * b < a * m →
    ∀ overhead : ℕ, 0 < overhead →
      ∃ N : ℕ, ∀ n : ℕ, N ≤ n →
        (overhead * candidate m n ^ degree) ^ b < 2 ^ (a * n)

/-- Every fixed numerator divided by arity has vanishing rate, even after a
fixed polynomial simulation degree.  In particular, the dense
`p^(3/m+o(1))` fixed-arity route is sufficient for the universal diagonal; the
qualitative subexponential conclusion does not need a sparse `p^(2/m+o(1))`
solver. -/
theorem fixedNumeratorCandidateFamilyHasVanishingRate
    (candidate : ℕ → ℕ → ℕ) (degree numerator : ℕ)
    (fixedRate : HasFixedNumeratorCandidateFamily candidate degree numerator) :
    HasVanishingRateCandidateFamily candidate degree := by
  intro a b aPositive bPositive
  let m := numerator * degree * b + 1
  refine ⟨m, fixedRate m a b aPositive bPositive ?_⟩
  have belowM : numerator * degree * b < m := by
    exact Nat.lt_succ_self _
  have oneLeA : 1 ≤ a := aPositive
  have mLe : m ≤ a * m := by
    simpa using Nat.mul_le_mul_right m oneLeA
  exact belowM.trans_le mLe

/-- Polynomial universal simulation of a vanishing-rate candidate family gives
one subexponential work bound.  This is the generic diagonal endpoint; applying
it to ECDLP still requires machine semantics, verified output checking and the
paper-to-work bridge for each fixed-arity candidate. -/
theorem polynomialUniversalSchedulerIsSubexponential
    (universal : ℕ → ℕ) (candidate : ℕ → ℕ → ℕ) (degree : ℕ)
    (simulation : HasPolynomialUniversalSimulationBound universal candidate degree)
    (vanishing : HasVanishingRateCandidateFamily candidate degree) :
    IsSubexponentialBinaryWork universal := by
  intro a b ha hb
  obtain ⟨m, hm⟩ := vanishing a b ha hb
  obtain ⟨overhead, overheadPositive, hsimulation⟩ := simulation.2 m
  obtain ⟨N, hcandidate⟩ := hm overhead overheadPositive
  refine ⟨N, fun n hn => ?_⟩
  exact (Nat.pow_le_pow_left (hsimulation n) b).trans_lt (hcandidate n hn)

/-- The exact geometric tail used after capped independent repetitions.  If
stage reach probability contributes `q^j` and stage cost contributes `B^j`,
their product has ratio `q*B`; this theorem applies with `ratio = q*B`. -/
theorem geometricSchedulerTailBound
    (scale ratio : ℝ) (ratioNonnegative : 0 ≤ ratio) (ratioLtOne : ratio < 1) :
    HasSum (fun stage : ℕ => scale * ratio ^ stage) (scale / (1 - ratio)) := by
  simpa [div_eq_mul_inv] using
    (hasSum_geometric_of_lt_one ratioNonnegative ratioLtOne).mul_left scale

/-! ## Near-uniform pair-event transfer

The following division-free lemmas compare a finite sampler with uniform
sampling.  They apply to generator events once every group element, including
the identity, has an explicit positive fibre in the sampler.
-/

/-- If every output has at least `lower` uniform seeds, the weighted count of
any pair event is at least `lower^2` times its uniform pair count. -/
theorem weightedPairEventLower
    {Output : Type*} [Fintype Output]
    (weight : Output → ℕ) (good : Output × Output → Prop)
    [DecidablePred good] (lower : ℕ)
    (fibreFloor : ∀ output, lower ≤ weight output) :
    lower ^ 2 * (Finset.univ.filter good).card ≤
      ∑ pair : Output × Output,
        if good pair then weight pair.1 * weight pair.2 else 0 := by
  classical
  calc
    lower ^ 2 * (Finset.univ.filter good).card =
        ∑ pair ∈ Finset.univ.filter good, lower ^ 2 := by
      simp [Nat.mul_comm]
    _ ≤ ∑ pair ∈ Finset.univ.filter good,
        weight pair.1 * weight pair.2 := by
      exact Finset.sum_le_sum fun pair _ => by
        simpa [pow_two] using
          Nat.mul_le_mul (fibreFloor pair.1) (fibreFloor pair.2)
    _ = ∑ pair : Output × Output,
        if good pair then weight pair.1 * weight pair.2 else 0 := by
      rw [Finset.sum_filter]

/-- A sampler whose total seed count is at most `distortion * |Output| * lower`
loses at most `distortion^2` on any pair event compared with uniform sampling.
The inequality is cross-multiplied, so it makes no divisibility or positivity
assumptions. -/
theorem nearUniformPairEventTransfer
    {Output : Type*} [Fintype Output]
    (weight : Output → ℕ) (good : Output × Output → Prop)
    [DecidablePred good] (lower distortion total : ℕ)
    (fibreFloor : ∀ output, lower ≤ weight output)
    (totalUpper : total ≤ distortion * Fintype.card Output * lower) :
    total ^ 2 * (Finset.univ.filter good).card ≤
      distortion ^ 2 * Fintype.card Output ^ 2 *
        (∑ pair : Output × Output,
          if good pair then weight pair.1 * weight pair.2 else 0) := by
  have pairLower := weightedPairEventLower weight good lower fibreFloor
  have totalSquare : total ^ 2 ≤
      (distortion * Fintype.card Output * lower) ^ 2 :=
    Nat.pow_le_pow_left totalUpper 2
  calc
    total ^ 2 * (Finset.univ.filter good).card ≤
        (distortion * Fintype.card Output * lower) ^ 2 *
          (Finset.univ.filter good).card :=
      Nat.mul_le_mul_right _ totalSquare
    _ = distortion ^ 2 * Fintype.card Output ^ 2 *
        (lower ^ 2 * (Finset.univ.filter good).card) := by ring
    _ ≤ distortion ^ 2 * Fintype.card Output ^ 2 *
        (∑ pair : Output × Output,
          if good pair then weight pair.1 * weight pair.2 else 0) :=
      Nat.mul_le_mul_left _ pairLower

/-! ## Dense fixed-arity elimination exponent -/

/-- If the dense relation dimension is at most `3X` and `X^m ≤ p`, its cubic
cost has the exact fixed-numerator bound
`rows^(3m) ≤ 3^(3m) * p^3`.  This is the natural-power form of the
`p^(3/m)` exponent; executable ring-operation and bit-cost bounds remain
separate obligations. -/
theorem denseCubicRowBoundAtFixedArity
    (p X m rows : ℕ) (rowBound : rows ≤ 3 * X) (scaleBound : X ^ m ≤ p) :
    rows ^ (3 * m) ≤ 3 ^ (3 * m) * p ^ 3 := by
  calc
    rows ^ (3 * m) ≤ (3 * X) ^ (3 * m) :=
      Nat.pow_le_pow_left rowBound _
    _ = 3 ^ (3 * m) * (X ^ m) ^ 3 := by
      rw [mul_pow, ← pow_mul]
      congr 2
      omega
    _ ≤ 3 ^ (3 * m) * p ^ 3 :=
      Nat.mul_le_mul_left _ (Nat.pow_le_pow_left scaleBound 3)

/-! ## Bird planted-dense fallback -/

/-- The public flat-array Bird implementation, specialized back to a square
matrix through its row-major array representation. -/
def birdMatrixDet
    {R : Type*} [CommRing R] {n : ℕ}
    (A : Matrix (Fin n) (Fin n) R) : R :=
  BirdDet.birdDet n (.ofFn fun k : Fin (n * n) => A k.divNat k.modNat)

/-- Mathlib's verified flat-array Bird implementation computes the matrix
determinant. -/
theorem det_eq_birdMatrixDet
    {R : Type*} [CommRing R] {n : ℕ}
    (A : Matrix (Fin n) (Fin n) R) :
    A.det = birdMatrixDet A := by
  simpa [birdMatrixDet, Matrix.ofArray_ofFn] using
    (BirdDet.det_eq_birdDet
      (.ofFn fun k : Fin (n * n) => A k.divNat k.modNat) Array.size_ofFn)

/-- Compute every adjugate entry with the verified Bird determinant
implementation applied to the row-updated matrix from Cramer's rule. -/
def birdAdjugate
    {R : Type*} [CommRing R] {n : ℕ}
    (A : Matrix (Fin n) (Fin n) R) : Matrix (Fin n) (Fin n) R :=
  Matrix.of fun i j =>
    birdMatrixDet (A.updateRow j (Pi.single i 1))

/-- The Bird-computed cofactor matrix is exactly the ordinary adjugate. -/
theorem birdAdjugate_eq_adjugate
    {R : Type*} [CommRing R] {n : ℕ}
    (A : Matrix (Fin n) (Fin n) R) :
    birdAdjugate A = A.adjugate := by
  ext i j
  rw [birdAdjugate, Matrix.of_apply, ← det_eq_birdMatrixDet,
    Matrix.adjugate_apply]

/-- An executable inverse candidate for matrices over `ZMod q`.  Unlike
`Matrix.inv`, this definition uses the computable `ZMod.inv` operation. -/
def birdZModInverse
    {q n : ℕ}
    (A : Matrix (Fin n) (Fin n) (ZMod q)) :
    Matrix (Fin n) (Fin n) (ZMod q) :=
  (birdMatrixDet A)⁻¹ • birdAdjugate A

/-- When the determinant is a unit, the executable Bird formula agrees with
Mathlib's nonsingular matrix inverse. -/
theorem birdZModInverse_eq_inv
    {q n : ℕ}
    (A : Matrix (Fin n) (Fin n) (ZMod q))
    (h : IsUnit A.det) :
    birdZModInverse A = A⁻¹ := by
  rw [birdZModInverse, birdAdjugate_eq_adjugate,
    Matrix.nonsing_inv_apply A h, ← det_eq_birdMatrixDet]
  congr 1
  calc
    A.det⁻¹ = (h.unit : ZMod q)⁻¹ := congrArg Inv.inv h.unit_spec.symm
    _ = (h.unit⁻¹ : (ZMod q)ˣ) := ZMod.inv_coe_unit h.unit

/-! ### Eager Bird recurrence -/

/-- A row-major square matrix stored in an array-backed fixed-length vector. -/
abbrev BirdFlatMatrix (R : Type*) (n : ℕ) := Vector R (n * n)

/-- Materialize a matrix as a row-major array-backed vector. -/
def birdMatrixToFlat
    {R : Type*} {n : ℕ}
    (A : Matrix (Fin n) (Fin n) R) : BirdFlatMatrix R n :=
  Vector.ofFn fun k => A k.divNat k.modNat

/-- Interpret a row-major vector as a square matrix. -/
def birdFlatToMatrix
    {R : Type*} {n : ℕ}
    (A : BirdFlatMatrix R n) : Matrix (Fin n) (Fin n) R :=
  Matrix.ofArray A.toArray A.size_toArray

/-- Materializing and then interpreting a matrix preserves every entry. -/
theorem birdFlatToMatrix_matrixToFlat
    {R : Type*} {n : ℕ}
    (A : Matrix (Fin n) (Fin n) R) :
    birdFlatToMatrix (birdMatrixToFlat A) = A := by
  simp [birdFlatToMatrix, birdMatrixToFlat]

/-- One eager Bird recurrence step. `birdMatrixToFlat` forces the complete
`n × n` next state into an array-backed vector instead of retaining a
function-valued state whose entries may be recomputed. -/
def eagerBirdStep
    {R : Type*} [CommRing R] {n : ℕ}
    (A F : BirdFlatMatrix R n) : BirdFlatMatrix R n :=
  birdMatrixToFlat <| Matrix.of fun i j =>
    BirdDet.stepEntry n A.toArray (BirdDet.get n F.toArray) i.val j.val

/-- One eager step refines the public scalar Bird recurrence entrywise. -/
theorem eagerBirdStep_refines
    {R : Type*} [CommRing R] {n : ℕ}
    (A F : BirdFlatMatrix R n) :
    birdFlatToMatrix (eagerBirdStep A F) =
      Matrix.of fun i j =>
        BirdDet.stepEntry n A.toArray (BirdDet.get n F.toArray) i.val j.val := by
  exact birdFlatToMatrix_matrixToFlat _

/-- Valid row-major vector entries agree with `BirdDet.get`. -/
theorem birdFlatToMatrix_eq_get
    {R : Type*} [CommRing R] {n : ℕ}
    (F : BirdFlatMatrix R n) (i j : Fin n) :
    birdFlatToMatrix F i j = BirdDet.get n F.toArray i.val j.val := by
  rw [birdFlatToMatrix, Matrix.ofArray_eq_of_getD]
  rfl

/-- Entrywise form of the eager-step refinement. -/
theorem eagerBirdStep_get
    {R : Type*} [CommRing R] {n : ℕ}
    (A F : BirdFlatMatrix R n) (i j : Fin n) :
    BirdDet.get n (eagerBirdStep A F).toArray i.val j.val =
      BirdDet.stepEntry n A.toArray (BirdDet.get n F.toArray) i.val j.val := by
  rw [← birdFlatToMatrix_eq_get, eagerBirdStep_refines, Matrix.of_apply]

/-- `sumFrom` respects pointwise equality on the suffix it actually reads. -/
theorem birdSumFrom_congr_valid
    {R : Type*} [CommRing R] (n lo : ℕ) (f g : ℕ → R)
    (h : ∀ k, lo ≤ k → k < n → f k = g k) :
    BirdDet.sumFrom n lo f = BirdDet.sumFrom n lo g := by
  induction lo using BirdDet.sumFrom_induct n with
  | step lo hlo ih =>
      rw [BirdDet.sumFrom_step n lo f hlo, BirdDet.sumFrom_step n lo g hlo,
        h lo le_rfl hlo, ih]
      intro k hk hkn
      exact h k (Nat.le_trans (Nat.le_succ lo) hk) hkn
  | stop lo hlo =>
      rw [BirdDet.sumFrom_stop n lo f hlo, BirdDet.sumFrom_stop n lo g hlo]

/-- A scalar Bird step only observes recurrence entries with valid indices. -/
theorem birdStepEntry_congr_valid
    {R : Type*} [CommRing R] (n : ℕ) (A : Array R)
    (F G : ℕ → ℕ → R)
    (h : ∀ i j, i < n → j < n → F i j = G i j)
    (i j : Fin n) :
    BirdDet.stepEntry n A F i.val j.val =
      BirdDet.stepEntry n A G i.val j.val := by
  rw [BirdDet.stepEntry_eq, BirdDet.stepEntry_eq]
  have hdiag :
      BirdDet.sumFrom n (i.val + 1) (fun k => F k k) =
        BirdDet.sumFrom n (i.val + 1) (fun k => G k k) := by
    apply birdSumFrom_congr_valid
    intro k _ hk
    exact h k k hk hk
  have hrow :
      BirdDet.sumFrom n (i.val + 1)
          (fun k => F i.val k * BirdDet.get n A k j.val) =
        BirdDet.sumFrom n (i.val + 1)
          (fun k => G i.val k * BirdDet.get n A k j.val) := by
    apply birdSumFrom_congr_valid
    intro k _ hk
    rw [h i.val k i.isLt hk]
  rw [hdiag, hrow]

/-- Every valid entry of every materialized state agrees with the corresponding
entry of Mathlib's function-valued scalar recurrence. -/
theorem eagerBirdIterate_get
    {R : Type*} [CommRing R] {n : ℕ}
    (A F : BirdFlatMatrix R n) (t : ℕ) (i j : Fin n) :
    BirdDet.get n (((eagerBirdStep A)^[t] F).toArray) i.val j.val =
      ((BirdDet.stepEntry n A.toArray)^[t] (BirdDet.get n F.toArray))
        i.val j.val := by
  induction t generalizing i j with
  | zero => rfl
  | succ t ih =>
      rw [Function.iterate_succ_apply', Function.iterate_succ_apply',
        eagerBirdStep_get]
      apply birdStepEntry_congr_valid n A.toArray
      intro i' j' hi' hj'
      exact ih ⟨i', hi'⟩ ⟨j', hj'⟩

/-- Bird determinant evaluation with every intermediate state materialized in
an array-backed vector. -/
def eagerBirdDet
    {R : Type*} [CommRing R] {n : ℕ}
    (A : BirdFlatMatrix R n) : R :=
  match n with
  | 0 => 1
  | k + 1 =>
      (-1 : R) ^ k *
        birdFlatToMatrix ((eagerBirdStep A)^[k] A) 0 0

/-- The eager array-backed Bird recurrence computes the determinant. -/
theorem eagerBirdDet_eq_det
    {R : Type*} [CommRing R] {n : ℕ}
    (A : Matrix (Fin n) (Fin n) R) :
    eagerBirdDet (birdMatrixToFlat A) = A.det := by
  rw [det_eq_birdMatrixDet]
  cases n with
  | zero => simp [eagerBirdDet, birdMatrixDet, BirdDet.birdDet]
  | succ k =>
      rw [eagerBirdDet, birdMatrixDet, BirdDet.birdDet_succ,
        birdFlatToMatrix_eq_get, eagerBirdIterate_get]
      rfl

/-! ### Instrumented scalar-ring operation count -/

/-- A scalar value paired with the number of additions, multiplications, and
negations used to construct it. Array access and allocation are outside this
scalar-ring model. -/
structure RingCounted (R : Type*) where
  value : R
  ops : ℕ

/-- Introduce an already available scalar at zero ring-operation cost. -/
def RingCounted.lift (x : R) : RingCounted R := ⟨x, 0⟩

/-- Count one scalar addition. -/
def RingCounted.add [Add R] (x y : RingCounted R) : RingCounted R :=
  ⟨x.value + y.value, x.ops + y.ops + 1⟩

/-- Count one scalar multiplication. -/
def RingCounted.mul [Mul R] (x y : RingCounted R) : RingCounted R :=
  ⟨x.value * y.value, x.ops + y.ops + 1⟩

/-- Count one scalar negation. -/
def RingCounted.neg [Neg R] (x : RingCounted R) : RingCounted R :=
  ⟨-x.value, x.ops + 1⟩

/-- Instrumented version of Bird's suffix sum. -/
def countedBirdSumFrom
    {R : Type*} [CommRing R] (n lo : ℕ) (f : ℕ → RingCounted R) :
    RingCounted R :=
  if h : lo < n then
    RingCounted.add (f lo) (countedBirdSumFrom n (lo + 1) f)
  else
    RingCounted.lift 0
termination_by n - lo
decreasing_by omega

/-- Erasing the count from the instrumented suffix sum recovers Mathlib's
`BirdDet.sumFrom`. -/
theorem countedBirdSumFrom_value
    {R : Type*} [CommRing R] (n lo : ℕ) (f : ℕ → RingCounted R) :
    (countedBirdSumFrom n lo f).value =
      BirdDet.sumFrom n lo fun k => (f k).value := by
  induction lo using BirdDet.sumFrom_induct n with
  | step lo hlo ih =>
      rw [countedBirdSumFrom, dite_eq_left hlo, BirdDet.sumFrom_step n lo _ hlo]
      simp [RingCounted.add, ih]
  | stop lo hlo =>
      rw [countedBirdSumFrom, dite_eq_right hlo, BirdDet.sumFrom_stop n lo _ hlo]
      rfl

/-- A suffix of length `n-lo`, with constant per-term cost `c`, uses exactly
`(n-lo)(c+1)` operations: the term cost plus one addition per term. -/
theorem countedBirdSumFrom_ops
    {R : Type*} [CommRing R] (n lo c : ℕ) (f : ℕ → RingCounted R)
    (hle : lo ≤ n)
    (hc : ∀ k, lo ≤ k → k < n → (f k).ops = c) :
    (countedBirdSumFrom n lo f).ops = (n - lo) * (c + 1) := by
  induction lo using BirdDet.sumFrom_induct n with
  | step lo hlo ih =>
      rw [countedBirdSumFrom, dite_eq_left hlo]
      simp only [RingCounted.add]
      rw [hc lo le_rfl hlo, ih (Nat.succ_le_iff.mpr hlo)]
      · have hn : n - lo = n - (lo + 1) + 1 := by omega
        rw [hn, Nat.add_mul]
        omega
      · intro k hk hkn
        exact hc k (Nat.le_trans (Nat.le_succ lo) hk) hkn
  | stop lo hlo =>
      rw [countedBirdSumFrom, dite_eq_right hlo]
      simp [RingCounted.lift]
      omega

/-- Bird's scalar recurrence entry with each ring operation instrumented. -/
def countedBirdStepEntry
    {R : Type*} [CommRing R]
    (n : ℕ) (A : Array R) (F : ℕ → ℕ → R) (i j : ℕ) :
    RingCounted R :=
  RingCounted.add
    (RingCounted.mul
      (RingCounted.neg
        (countedBirdSumFrom n (i + 1) fun k => RingCounted.lift (F k k)))
      (RingCounted.lift (BirdDet.get n A i j)))
    (countedBirdSumFrom n (i + 1) fun k =>
      RingCounted.mul (RingCounted.lift (F i k))
        (RingCounted.lift (BirdDet.get n A k j)))

/-- Erasing the counter recovers the public scalar Bird entry formula. -/
theorem countedBirdStepEntry_value
    {R : Type*} [CommRing R]
    (n : ℕ) (A : Array R) (F : ℕ → ℕ → R) (i j : ℕ) :
    (countedBirdStepEntry n A F i j).value = BirdDet.stepEntry n A F i j := by
  rw [countedBirdStepEntry, BirdDet.stepEntry_eq]
  change
    -(countedBirdSumFrom n (i + 1) fun k => RingCounted.lift (F k k)).value *
          BirdDet.get n A i j +
        (countedBirdSumFrom n (i + 1) fun k =>
          RingCounted.mul (RingCounted.lift (F i k))
            (RingCounted.lift (BirdDet.get n A k j))).value = _
  rw [countedBirdSumFrom_value, countedBirdSumFrom_value]
  rfl

/-- A valid Bird output entry uses exactly three operations per suffix term,
plus one negation, one multiplication, and one final addition. -/
theorem countedBirdStepEntry_ops
    {R : Type*} [CommRing R]
    (n : ℕ) (A : Array R) (F : ℕ → ℕ → R) (i j : ℕ)
    (hi : i < n) :
    (countedBirdStepEntry n A F i j).ops = 3 * (n - (i + 1)) + 3 := by
  have hle : i + 1 ≤ n := hi
  have hdiag := countedBirdSumFrom_ops n (i + 1) 0
    (fun k => RingCounted.lift (F k k)) hle (by simp [RingCounted.lift])
  have hrow := countedBirdSumFrom_ops n (i + 1) 1
    (fun k => RingCounted.mul (RingCounted.lift (F i k))
      (RingCounted.lift (BirdDet.get n A k j))) hle
      (by simp [RingCounted.mul, RingCounted.lift])
  rw [countedBirdStepEntry]
  change
    (countedBirdSumFrom n (i + 1) fun k => RingCounted.lift (F k k)).ops +
          1 + 0 + 1 +
        (countedBirdSumFrom n (i + 1) fun k =>
          RingCounted.mul (RingCounted.lift (F i k))
            (RingCounted.lift (BirdDet.get n A k j))).ops + 1 = _
  rw [hdiag, hrow]
  omega

/-- Every entry's scalar-ring cost is at most `3n+3`. -/
theorem countedBirdStepEntry_ops_le
    {R : Type*} [CommRing R]
    (n : ℕ) (A : Array R) (F : ℕ → ℕ → R) (i j : Fin n) :
    (countedBirdStepEntry n A F i.val j.val).ops ≤ 3 * n + 3 := by
  rw [countedBirdStepEntry_ops n A F i.val j.val i.isLt]
  omega

/-- Exact scalar-ring operation count obtained by summing the instrumented
entry formula over one materialized `n × n` state. -/
def eagerBirdStepRingOps (n : ℕ) : ℕ :=
  ∑ i : Fin n, ∑ _j : Fin n, (3 * (n - (i.val + 1)) + 3)

/-- One complete eager state costs at most `n²(3n+3)` scalar-ring
operations. -/
theorem eagerBirdStepRingOps_le (n : ℕ) :
    eagerBirdStepRingOps n ≤ n * n * (3 * n + 3) := by
  have houter :
      (∑ i : Fin n, ∑ _j : Fin n, (3 * (n - (i.val + 1)) + 3)) ≤
        ∑ _i : Fin n, ∑ _j : Fin n, (3 * n + 3) := by
    apply Finset.sum_le_sum
    intro i _
    apply Finset.sum_le_sum
    intro j _
    omega
  calc
    eagerBirdStepRingOps n ≤ ∑ _i : Fin n, ∑ _j : Fin n, (3 * n + 3) := by
      rw [eagerBirdStepRingOps]
      exact houter
    _ = n * n * (3 * n + 3) := by simp [mul_assoc]

/-- Conservative full determinant budget: at most `n` complete eager states,
plus at most `n` multiplications for the final sign power and product. -/
def eagerBirdDetRingOps (n : ℕ) : ℕ :=
  n * eagerBirdStepRingOps n + n

/-- The complete eager Bird determinant has an explicit quartic scalar-ring
operation budget. -/
theorem eagerBirdDetRingOps_le_quartic (n : ℕ) :
    eagerBirdDetRingOps n ≤ 4 * (n + 1) ^ 4 := by
  have hstep := eagerBirdStepRingOps_le n
  rw [eagerBirdDetRingOps]
  nlinarith [sq_nonneg (n : ℤ), sq_nonneg ((n : ℤ) + 1),
    sq_nonneg ((n : ℤ) ^ 2)]

/-- Scalar-ring budget for computing all `n²` Bird determinants in the
cofactor adjugate. -/
def eagerBirdAdjugateRingOps (n : ℕ) : ℕ :=
  n * n * eagerBirdDetRingOps n

/-- Computing all cofactors with the eager quartic determinant has an explicit
sixth-power scalar-ring operation budget. -/
theorem eagerBirdAdjugateRingOps_le_sixth (n : ℕ) :
    eagerBirdAdjugateRingOps n ≤ 4 * (n + 1) ^ 6 := by
  calc
    eagerBirdAdjugateRingOps n ≤ n * n * (4 * (n + 1) ^ 4) := by
      exact Nat.mul_le_mul_left (n * n) (eagerBirdDetRingOps_le_quartic n)
    _ ≤ (n + 1) * (n + 1) * (4 * (n + 1) ^ 4) := by
      gcongr <;> omega
    _ = 4 * (n + 1) ^ 6 := by ring

/-! ### Executable planted kernel -/

/-- The planted-kernel matrix with the executable Bird inverse substituted for
Mathlib's noncomputable matrix inverse. -/
def executableBirdPlantedKernel
    {q f s : ℕ}
    (B : Matrix (Fin f) (Fin f) (ZMod q))
    (Rr : Matrix (Fin s) (Fin f) (ZMod q)) :
    Matrix (Fin s) (Fin f ⊕ Fin s) (ZMod q) :=
  Matrix.fromCols (-(Rr * birdZModInverse B)) 1

/-- Under the planted block's unit-determinant hypothesis, the executable
kernel is exactly the canonical `(-R B⁻¹ | I)` construction. -/
theorem executableBirdPlantedKernel_eq
    {q f s : ℕ}
    (B : Matrix (Fin f) (Fin f) (ZMod q))
    (Rr : Matrix (Fin s) (Fin f) (ZMod q))
    (hB : IsUnit B.det) :
    executableBirdPlantedKernel B Rr =
      Matrix.fromCols (-(Rr * B⁻¹)) 1 := by
  rw [executableBirdPlantedKernel, birdZModInverse_eq_inv B hB]

/-- The executable planted kernel annihilates the stacked relation matrix. -/
theorem executableBirdPlantedKernel_mul
    {q f s : ℕ}
    (B : Matrix (Fin f) (Fin f) (ZMod q))
    (Rr : Matrix (Fin s) (Fin f) (ZMod q))
    (hB : IsUnit B.det) :
    executableBirdPlantedKernel B Rr * Matrix.fromRows B Rr = 0 := by
  rw [executableBirdPlantedKernel_eq B Rr hB,
    Matrix.fromCols_mul_fromRows, Matrix.neg_mul, Matrix.mul_assoc,
    Matrix.nonsing_inv_mul _ hB]
  simp

/-- The executable planted kernel has the identity right block required by the
upstream uniformity proof. -/
theorem executableBirdPlantedKernel_rightInv
    {q f s : ℕ}
    (B : Matrix (Fin f) (Fin f) (ZMod q))
    (Rr : Matrix (Fin s) (Fin f) (ZMod q))
    (hB : IsUnit B.det) :
    executableBirdPlantedKernel B Rr *
      Matrix.fromRows (0 : Matrix (Fin f) (Fin s) (ZMod q))
        (1 : Matrix (Fin s) (Fin s) (ZMod q)) = 1 := by
  rw [executableBirdPlantedKernel_eq B Rr hB,
    Matrix.fromCols_mul_fromRows]
  simp

/-- Combined executable version of the two laws consumed by the upstream
planted-kernel and conditional-uniformity argument. -/
theorem executableBirdPlantedKernel_spec
    {q f s : ℕ}
    (B : Matrix (Fin f) (Fin f) (ZMod q))
    (Rr : Matrix (Fin s) (Fin f) (ZMod q))
    (hB : IsUnit B.det) :
    executableBirdPlantedKernel B Rr * Matrix.fromRows B Rr = 0 ∧
      executableBirdPlantedKernel B Rr *
        Matrix.fromRows (0 : Matrix (Fin f) (Fin s) (ZMod q))
          (1 : Matrix (Fin s) (Fin s) (ZMod q)) = 1 :=
  ⟨executableBirdPlantedKernel_mul B Rr hB,
    executableBirdPlantedKernel_rightInv B Rr hB⟩

/-- A conservative `rows^6` planted-dense cost has fixed numerator six.  This
matches `rows^2` Bird determinant calls for the adjugate at an advertised
`rows^4` operations per determinant; the operation-count connection itself is
not asserted by this arithmetic theorem. -/
theorem birdSixthPowerRowBoundAtFixedArity
    (p X m rows : ℕ) (rowBound : rows ≤ 3 * X) (scaleBound : X ^ m ≤ p) :
    rows ^ (6 * m) ≤ 3 ^ (6 * m) * p ^ 6 := by
  calc
    rows ^ (6 * m) ≤ (3 * X) ^ (6 * m) :=
      Nat.pow_le_pow_left rowBound _
    _ = 3 ^ (6 * m) * (X ^ m) ^ 6 := by
      rw [mul_pow, ← pow_mul]
      congr 2
      omega
    _ ≤ 3 ^ (6 * m) * p ^ 6 :=
      Nat.mul_le_mul_left _ (Nat.pow_le_pow_left scaleBound 6)

/-! ## First-moment coverage repair

The following finite counting lemmas isolate the arithmetic in the corrected
fixed-arity coverage argument.  The curve-specific character sum and energy
estimates are deliberately hypotheses of the eventual specialization, not
claims of this module.
-/

/-- If a nonnegative finite population is pointwise at most `2X` and has mean
at least `3X/4`, at least one sixth of its seeds have value at least `X/2`.
The division-free conclusion is exact for a uniform finite seed space. -/
theorem firstMomentForcesLargeValueSeeds
    {Seed : Type*} [Fintype Seed]
    (value : Seed → ℕ) (X : ℕ) (XPositive : 0 < X)
    (upper : ∀ seed, value seed ≤ 2 * X)
    (totalLower : 3 * X * Fintype.card Seed ≤ 4 * ∑ seed, value seed) :
    Fintype.card Seed ≤
      6 * (Finset.univ.filter fun seed => X ≤ 2 * value seed).card := by
  classical
  let high := Finset.univ.filter fun seed => X ≤ 2 * value seed
  let low := Finset.univ.filter fun seed => ¬ X ≤ 2 * value seed
  have highBound : 2 * ∑ seed ∈ high, value seed ≤ 4 * X * high.card := by
    rw [Finset.mul_sum]
    calc
      (∑ seed ∈ high, 2 * value seed) ≤ ∑ _seed ∈ high, 4 * X := by
        exact Finset.sum_le_sum fun seed _ => by nlinarith [upper seed]
      _ = 4 * X * high.card := by simp [Nat.mul_comm]
  have lowBound : 2 * ∑ seed ∈ low, value seed ≤ X * low.card := by
    rw [Finset.mul_sum]
    calc
      (∑ seed ∈ low, 2 * value seed) ≤ ∑ _seed ∈ low, X := by
        exact Finset.sum_le_sum fun seed hseed => by
          have : ¬ X ≤ 2 * value seed := (Finset.mem_filter.mp hseed).2
          omega
      _ = X * low.card := by simp [Nat.mul_comm]
  have sumSplit : (∑ seed, value seed) =
      (∑ seed ∈ high, value seed) + ∑ seed ∈ low, value seed := by
    simpa [high, low] using
      (Finset.sum_filter_add_sum_filter_not Finset.univ
        (fun seed => X ≤ 2 * value seed) value).symm
  have cardSplit : high.card + low.card = Fintype.card Seed := by
    simpa [high, low] using
      Finset.card_filter_add_card_filter_not
        (s := Finset.univ) (fun seed => X ≤ 2 * value seed)
  have totalUpper : 4 * ∑ seed, value seed ≤
      2 * (4 * X * high.card + X * low.card) := by
    rw [sumSplit]
    nlinarith
  have scaled : X * Fintype.card Seed ≤ X * (6 * high.card) := by
    nlinarith
  exact Nat.le_of_mul_le_mul_left scaled XPositive

/-- Finite Markov specialization at factor twelve. If total energy is at most
`meanBound` times the population size, at most one twelfth of the seeds have
energy at least `12 * meanBound`. -/
theorem twelveMarkovBadSetBound
    {Seed : Type*} [Fintype Seed]
    (energy : Seed → ℕ) (meanBound : ℕ) (meanPositive : 0 < meanBound)
    (totalBound : (∑ seed, energy seed) ≤ meanBound * Fintype.card Seed) :
    12 * (Finset.univ.filter fun seed => 12 * meanBound ≤ energy seed).card ≤
      Fintype.card Seed := by
  classical
  have counted := highRankSeedCountBound energy energy (12 * meanBound)
    (fun seed => le_rfl)
  apply Nat.le_of_mul_le_mul_left (c := meanBound) ?_ meanPositive
  calc
    meanBound * (12 * (Finset.univ.filter fun seed =>
        12 * meanBound ≤ energy seed).card) =
        (12 * meanBound) * (Finset.univ.filter fun seed =>
          12 * meanBound ≤ energy seed).card := by ring
    _ ≤ ∑ seed, energy seed := counted
    _ ≤ meanBound * Fintype.card Seed := totalBound

/-- An event of finite uniform probability at least `1/6` still has probability
at least `1/12` after removing any bad event of probability at most `1/12`. -/
theorem oneSixthEventSurvivesOneTwelfthBadSet
    {Seed : Type*} [Fintype Seed] [DecidableEq Seed]
    (large bad : Finset Seed)
    (largeLower : Fintype.card Seed ≤ 6 * large.card)
    (badUpper : 12 * bad.card ≤ Fintype.card Seed) :
    Fintype.card Seed ≤ 12 * (large \ bad).card := by
  have split : large.card = (large \ bad).card + (large ∩ bad).card :=
    (Finset.card_sdiff_add_card_inter large bad).symm
  have interLe : (large ∩ bad).card ≤ bad.card :=
    Finset.card_le_card Finset.inter_subset_right
  omega

/-- Composed finite coverage endpoint. A `3/4` first-moment lower bound for a
quantity bounded by `2X`, together with an average energy upper bound, leaves
at least one twelfth of all seeds simultaneously large and below the
factor-twelve energy threshold. -/
theorem firstMomentAndEnergyGiveOneTwelfthGoodSeeds
    {Seed : Type*} [Fintype Seed] [DecidableEq Seed]
    (value energy : Seed → ℕ) (X meanBound : ℕ)
    (XPositive : 0 < X) (meanPositive : 0 < meanBound)
    (valueUpper : ∀ seed, value seed ≤ 2 * X)
    (valueTotalLower : 3 * X * Fintype.card Seed ≤ 4 * ∑ seed, value seed)
    (energyTotalUpper : (∑ seed, energy seed) ≤ meanBound * Fintype.card Seed) :
    Fintype.card Seed ≤ 12 *
      ((Finset.univ.filter fun seed => X ≤ 2 * value seed) \
        (Finset.univ.filter fun seed => 12 * meanBound ≤ energy seed)).card := by
  apply oneSixthEventSurvivesOneTwelfthBadSet
  · exact firstMomentForcesLargeValueSeeds value X XPositive valueUpper valueTotalLower
  · exact twelveMarkovBadSetBound energy meanBound meanPositive energyTotalUpper

/-! ## Planted-diagonal bad shifts

The following lemmas isolate the deterministic root-counting argument in the
planted-diagonal step. They do not assert that a paper-specific failed planting
is represented by one of these characteristic-polynomial roots.
-/

/-- Among any finite set of distinct field values, a nonzero polynomial has at
most `natDegree` roots. -/
theorem polynomialBadValuesCardLeDegree
    {K : Type*} [Field K] [DecidableEq K]
    (polynomial : Polynomial K) (candidates : Finset K) (nonzero : polynomial ≠ 0) :
    (candidates.filter fun value => polynomial.eval value = 0).card ≤
      polynomial.natDegree := by
  classical
  apply Polynomial.card_le_degree_of_subset_roots
  intro value valueMem
  rw [Polynomial.mem_roots nonzero]
  exact (Finset.mem_filter.mp valueMem).2

/-- An `n × n` matrix has at most `n` candidate scalar shifts for which
`value * I - matrix` is singular. This is the characteristic-polynomial root
bound, stated directly in the determinant form used by diagonal planting. -/
theorem charpolySingularShiftCardLeDimension
    {K Index : Type*} [Field K] [DecidableEq K]
    [Fintype Index] [DecidableEq Index]
    (matrix : Matrix Index Index K) (candidates : Finset K) :
    (candidates.filter fun value =>
      (Matrix.scalar Index value - matrix).det = 0).card ≤ Fintype.card Index := by
  classical
  have roots := polynomialBadValuesCardLeDegree matrix.charpoly candidates
    matrix.charpoly_monic.ne_zero
  rw [Matrix.charpoly_natDegree_eq_dim] at roots
  have filterEq :
      candidates.filter (fun value =>
        (Matrix.scalar Index value - matrix).det = 0) =
      candidates.filter (fun value => matrix.charpoly.eval value = 0) := by
    ext value
    simp only [Finset.mem_filter]
    rw [Matrix.eval_charpoly]
  rw [filterEq]
  exact roots

/-- The singular-shift bound pulls back along any injective encoding of an
abstract finite shift set into the field. -/
theorem charpolySingularEncodedShiftCardLeDimension
    {K Index Shift : Type*} [Field K] [DecidableEq K]
    [Fintype Index] [DecidableEq Index] [DecidableEq Shift]
    (matrix : Matrix Index Index K) (candidates : Finset Shift)
    (encode : Shift → K) (encodeInjective : Function.Injective encode) :
    (candidates.filter fun shift =>
      (Matrix.scalar Index (encode shift) - matrix).det = 0).card ≤
      Fintype.card Index := by
  classical
  let badShifts := candidates.filter fun shift =>
    (Matrix.scalar Index (encode shift) - matrix).det = 0
  let badValues := (candidates.image encode).filter fun value =>
    (Matrix.scalar Index value - matrix).det = 0
  have mapsBad : Set.MapsTo encode (badShifts : Set Shift) (badValues : Set K) := by
    intro shift shiftBad
    have shiftData := Finset.mem_filter.mp shiftBad
    exact Finset.mem_filter.mpr
      ⟨Finset.mem_image.mpr ⟨shift, shiftData.1, rfl⟩, shiftData.2⟩
  have badCard : badShifts.card ≤ badValues.card :=
    Finset.card_le_card_of_injOn encode mapsBad encodeInjective.injOn
  exact badCard.trans
    (charpolySingularShiftCardLeDimension matrix (candidates.image encode))

/-- Natural residues below a common bound embed injectively into `ZMod modulus`
whenever that bound is no larger than the modulus. -/
theorem zmodNatCastInjectiveOnFin
    (bound modulus : ℕ) (rangeBound : bound ≤ modulus) :
    Function.Injective (fun value : Fin bound => (value.val : ZMod modulus)) := by
  intro left right equalCasts
  apply Fin.ext
  have leftLt : left.val < modulus := left.isLt.trans_le rangeBound
  have rightLt : right.val < modulus := right.isLt.trans_le rangeBound
  have equalVals := congrArg ZMod.val equalCasts
  simpa [ZMod.val_natCast_of_lt leftLt, ZMod.val_natCast_of_lt rightLt] using equalVals

/-- For a prime modulus, at most `dimension` of the first `bound` natural
shifts make the residue-field matrix singular, provided `bound ≤ modulus`.
This discharges the distinct-residues premise in the planted-diagonal count. -/
theorem zmodCharpolySingularShiftCardLeDimension
    (bound modulus dimension : ℕ) [Fact (Nat.Prime modulus)]
    (rangeBound : bound ≤ modulus)
    (matrix : Matrix (Fin dimension) (Fin dimension) (ZMod modulus)) :
    (Finset.univ.filter fun shift : Fin bound =>
      (Matrix.scalar (Fin dimension) (shift.val : ZMod modulus) - matrix).det = 0).card ≤
      dimension := by
  simpa using charpolySingularEncodedShiftCardLeDimension matrix Finset.univ
    (fun value : Fin bound => (value.val : ZMod modulus))
    (zmodNatCastInjectiveOnFin bound modulus rangeBound)

/-- A union of finitely many bad-shift sets, each of size at most `bound`, has
size at most the number of sets times `bound`. -/
theorem finiteBadShiftUnionCardBound
    {Family Shift : Type*} [DecidableEq Shift]
    (families : Finset Family) (bad : Family → Finset Shift) (bound : ℕ)
    (badCard : ∀ family ∈ families, (bad family).card ≤ bound) :
    (families.biUnion bad).card ≤ families.card * bound := by
  exact Finset.card_biUnion_le_card_mul families bad bound badCard

/-- If every bad-shift set lies in the candidate pool, every one has size at
most `bound`, and the pool has size at least eight times the union-bound scale,
then at least seven eighths of the candidate shifts survive. The conclusion is
division-free and remains exact for finite uniform sampling. -/
theorem sevenEighthsShiftsSurvive
    {Family Shift : Type*} [DecidableEq Shift]
    (families : Finset Family) (candidates : Finset Shift)
    (bad : Family → Finset Shift) (bound : ℕ)
    (badSubset : ∀ family ∈ families, bad family ⊆ candidates)
    (badCard : ∀ family ∈ families, (bad family).card ≤ bound)
    (candidateLarge : 8 * families.card * bound ≤ candidates.card) :
    7 * candidates.card ≤
      8 * (candidates \ families.biUnion bad).card := by
  have unionSubset : families.biUnion bad ⊆ candidates := by
    intro value valueMem
    obtain ⟨family, familyMem, valueBad⟩ := Finset.mem_biUnion.mp valueMem
    exact badSubset family familyMem valueBad
  have unionCard : (families.biUnion bad).card ≤ families.card * bound :=
    finiteBadShiftUnionCardBound families bad bound badCard
  have eightUnion : 8 * (families.biUnion bad).card ≤ candidates.card :=
    (Nat.mul_le_mul_left 8 unionCard).trans (by
      simpa [Nat.mul_assoc] using candidateLarge)
  rw [Finset.card_sdiff_of_subset unionSubset]
  omega

end TensorBoundaryBounds
