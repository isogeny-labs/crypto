import Mathlib.LinearAlgebra.Matrix.Rank
import Mathlib.Algebra.Polynomial.Roots
import Mathlib.Algebra.Field.ZMod
import Mathlib.LinearAlgebra.Matrix.Charpoly.Coeff
import Mathlib.Analysis.SpecificLimits.Basic

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
