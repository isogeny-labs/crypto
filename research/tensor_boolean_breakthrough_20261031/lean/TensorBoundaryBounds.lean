import Mathlib.LinearAlgebra.Matrix.Rank
import Mathlib.Algebra.Polynomial.Roots
import Mathlib.Algebra.Field.ZMod
import Mathlib.LinearAlgebra.Matrix.Charpoly.Coeff
import Mathlib.Analysis.SpecificLimits.Basic
import Mathlib.LinearAlgebra.Matrix.Determinant.Bird.Correctness
import Mathlib.Data.Matrix.ColumnRowPartitioned
import Mathlib.AlgebraicGeometry.EllipticCurve.Affine.Point
import Mathlib.FieldTheory.Finite.Basic
import Mathlib.Data.Nat.Size
import Mathlib.Data.Nat.Sqrt
import Mathlib.NumberTheory.LegendreSymbol.QuadraticChar.Basic
import Mathlib.SetTheory.Cardinal.NatCard
import MathlibExt.AlgebraicGeometry.EllipticCurve.HasseBound
import Mathlib.AlgebraicGeometry.EllipticCurve.DivisionPolynomial.Degree
import Mathlib.AlgebraicGeometry.EllipticCurve.DivisionPolynomial.ZSMul
import Mathlib.FieldTheory.Galois.Basic
import Mathlib.FieldTheory.SplittingField.Construction
import Mathlib.RingTheory.Localization.NumDen
import Mathlib.Tactic.Module
import ECDLP.L49_Main

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

/-- Add an explicit seed branch of mass `lower` for a distinguished identity
output to a sampler that may otherwise omit it. -/
def identityInclusiveWeight
    {Output : Type*} [DecidableEq Output]
    (identity : Output) (lower : ℕ) (weight : Output → ℕ) : Output → ℕ :=
  fun output => weight output + if output = identity then lower else 0

/-- The explicit identity branch extends a nonidentity fibre floor to every
output. -/
theorem identityInclusiveWeight_floor
    {Output : Type*} [DecidableEq Output]
    (identity : Output) (lower : ℕ) (weight : Output → ℕ)
    (nonidentityFloor : ∀ output, output ≠ identity → lower ≤ weight output) :
    ∀ output, lower ≤ identityInclusiveWeight identity lower weight output := by
  intro output
  by_cases h : output = identity
  · simp [identityInclusiveWeight, h]
  · simpa [identityInclusiveWeight, h] using nonidentityFloor output h

/-- The identity-inclusive sampler has exactly `lower` more seeds than its
base sampler. -/
theorem identityInclusiveWeight_total
    {Output : Type*} [Fintype Output] [DecidableEq Output]
    (identity : Output) (lower : ℕ) (weight : Output → ℕ) :
    (∑ output, identityInclusiveWeight identity lower weight output) =
      (∑ output, weight output) + lower := by
  simp [identityInclusiveWeight, Finset.sum_add_distrib]

/-- A distortion bound for a sampler on the nonidentity outputs extends to
the identity-inclusive sampler. -/
theorem identityInclusiveTotalUpper
    {Output : Type*} [Fintype Output] [DecidableEq Output]
    (identity : Output) (lower distortion : ℕ) (weight : Output → ℕ)
    (distortionPositive : 0 < distortion)
    (baseTotalUpper :
      (∑ output, weight output) ≤
        distortion * (Fintype.card Output - 1) * lower) :
    (∑ output, identityInclusiveWeight identity lower weight output) ≤
      distortion * Fintype.card Output * lower := by
  rw [identityInclusiveWeight_total]
  have hlower : lower ≤ distortion * lower := by
    calc
      lower = 1 * lower := by simp
      _ ≤ distortion * lower := Nat.mul_le_mul_right lower distortionPositive
  calc
    (∑ output, weight output) + lower ≤
        distortion * (Fintype.card Output - 1) * lower + lower :=
      Nat.add_le_add_right baseTotalUpper lower
    _ ≤ distortion * (Fintype.card Output - 1) * lower +
        distortion * lower := Nat.add_le_add_left hlower _
    _ = distortion * ((Fintype.card Output - 1) + 1) * lower := by ring
    _ = distortion * Fintype.card Output * lower := by
      have hcard : 0 < Fintype.card Output := Fintype.card_pos_iff.mpr ⟨identity⟩
      congr 2
      omega

/-- Exact pair-event transfer for an affine-style sampler after adding an
explicit identity branch. -/
theorem identityInclusiveNearUniformPairEventTransfer
    {Output : Type*} [Fintype Output] [DecidableEq Output]
    (identity : Output) (weight : Output → ℕ)
    (good : Output × Output → Prop) [DecidablePred good]
    (lower distortion : ℕ)
    (distortionPositive : 0 < distortion)
    (nonidentityFloor : ∀ output, output ≠ identity → lower ≤ weight output)
    (baseTotalUpper :
      (∑ output, weight output) ≤
        distortion * (Fintype.card Output - 1) * lower) :
    (∑ output, identityInclusiveWeight identity lower weight output) ^ 2 *
        (Finset.univ.filter good).card ≤
      distortion ^ 2 * Fintype.card Output ^ 2 *
        (∑ pair : Output × Output,
          if good pair then
            identityInclusiveWeight identity lower weight pair.1 *
              identityInclusiveWeight identity lower weight pair.2
          else 0) := by
  exact nearUniformPairEventTransfer
    (identityInclusiveWeight identity lower weight) good lower distortion
    (∑ output, identityInclusiveWeight identity lower weight output)
    (identityInclusiveWeight_floor identity lower weight nonidentityFloor)
    (identityInclusiveTotalUpper identity lower distortion weight
      distortionPositive baseTotalUpper)

/-! ### Exact finite rejection sampling -/

/-- Number of one-trial seeds accepted as a specified output. -/
def rejectionSamplerWeight
    {Seed Output : Type*} [Fintype Seed] [DecidableEq Output]
    (sample : Seed → Option Output) (output : Output) : ℕ :=
  (Finset.univ.filter fun seed => sample seed = some output).card

/-- Finite set of one-trial seeds that are accepted rather than rejected. -/
def rejectionSamplerAcceptedSeeds
    {Seed Output : Type*} [Fintype Seed]
    (sample : Seed → Option Output) : Finset Seed :=
  Finset.univ.filter fun seed => (sample seed).isSome

/-- Accepted one-trial seeds are partitioned exactly by their output fibres. -/
theorem rejectionSamplerWeight_sum
    {Seed Output : Type*} [Fintype Seed] [Fintype Output]
    [DecidableEq Output] (sample : Seed → Option Output) :
    (∑ output, rejectionSamplerWeight sample output) =
      (rejectionSamplerAcceptedSeeds sample).card := by
  classical
  calc
    (∑ output, rejectionSamplerWeight sample output) =
        ∑ output, ∑ seed, if sample seed = some output then 1 else 0 := by
      simp [rejectionSamplerWeight]
    _ = ∑ seed, ∑ output, if sample seed = some output then 1 else 0 :=
      Finset.sum_comm
    _ = ∑ seed, if (sample seed).isSome then 1 else 0 := by
      apply Finset.sum_congr rfl
      intro seed _
      cases h : sample seed with
      | none => simp
      | some output => simp
    _ = (rejectionSamplerAcceptedSeeds sample).card := by
      simp [rejectionSamplerAcceptedSeeds]

/-- Equal accepted fibres give the exact successful-seed count. -/
theorem uniformAcceptedFibres_card
    {Seed Output : Type*} [Fintype Seed] [Fintype Output]
    [DecidableEq Output] (sample : Seed → Option Output) (fibre : ℕ)
    (uniformFibres : ∀ output, rejectionSamplerWeight sample output = fibre) :
    (rejectionSamplerAcceptedSeeds sample).card =
      fibre * Fintype.card Output := by
  rw [← rejectionSamplerWeight_sum]
  simp [uniformFibres, Nat.mul_comm]

/-- With equal accepted fibres, every pair event has exactly its uniform count
times the square of the common fibre size. -/
theorem uniformAcceptedPairEventWeight
    {Seed Output : Type*} [Fintype Seed] [Fintype Output]
    [DecidableEq Output] (sample : Seed → Option Output)
    (good : Output × Output → Prop) [DecidablePred good] (fibre : ℕ)
    (uniformFibres : ∀ output, rejectionSamplerWeight sample output = fibre) :
    (∑ pair : Output × Output,
        if good pair then
          rejectionSamplerWeight sample pair.1 *
            rejectionSamplerWeight sample pair.2
        else 0) =
      fibre ^ 2 * (Finset.univ.filter good).card := by
  rw [← Finset.sum_filter]
  simp [uniformFibres, pow_two, Nat.mul_comm]

/-- The tail-sum series for independent rejection trials is exactly
`total / accepted`, the usual expected number of trials. -/
theorem rejectionTrialTailHasSum
    (total accepted : ℕ) (acceptedPositive : 0 < accepted)
    (acceptedLeTotal : accepted ≤ total) :
    HasSum
      (fun failures : ℕ =>
        (1 - (accepted : ℝ) / (total : ℝ)) ^ failures)
      ((total : ℝ) / (accepted : ℝ)) := by
  have totalPositive : (0 : ℝ) < total := by
    exact_mod_cast acceptedPositive.trans_le acceptedLeTotal
  have acceptedRealPositive : (0 : ℝ) < accepted := by exact_mod_cast acceptedPositive
  have ratioNonnegative :
      0 ≤ 1 - (accepted : ℝ) / (total : ℝ) := by
    rw [sub_nonneg]
    exact (div_le_one totalPositive).mpr (by exact_mod_cast acceptedLeTotal)
  have ratioLtOne : 1 - (accepted : ℝ) / (total : ℝ) < 1 := by
    have : 0 < (accepted : ℝ) / (total : ℝ) := div_pos acceptedRealPositive totalPositive
    linarith
  convert geometricSchedulerTailBound 1
    (1 - (accepted : ℝ) / (total : ℝ)) ratioNonnegative ratioLtOne using 1
  · simp
  · rw [sub_sub_cancel]
    field_simp

/-- If every output has one specified successful encoding and no other seed
decodes to it, then its accepted fibre has cardinality exactly one. -/
theorem uniqueAcceptedSeedWeight_eq_one
    {Seed Output : Type*} [Fintype Seed] [DecidableEq Seed]
    [DecidableEq Output] (sample : Seed → Option Output) (encode : Output → Seed)
    (sampleEncode : ∀ output, sample (encode output) = some output)
    (acceptedUnique : ∀ seed output, sample seed = some output → seed = encode output) :
    ∀ output, rejectionSamplerWeight sample output = 1 := by
  intro output
  have hfibre :
      Finset.univ.filter (fun seed => sample seed = some output) = {encode output} := by
    ext seed
    simp only [Finset.mem_filter, Finset.mem_univ, true_and, Finset.mem_singleton]
    constructor
    · exact acceptedUnique seed output
    · intro hseed
      rw [hseed]
      exact sampleEncode output
  rw [rejectionSamplerWeight, hfibre]
  simp

/-- A one-trial success density of at least `1/cap` bounds the expected number
of independent rejection trials by `cap`. -/
theorem rejectionTrialExpectation_le
    (total accepted cap : ℕ) (acceptedPositive : 0 < accepted)
    (totalLe : total ≤ cap * accepted) :
    (total : ℝ) / (accepted : ℝ) ≤ cap := by
  have acceptedRealPositive : (0 : ℝ) < accepted := by exact_mod_cast acceptedPositive
  rw [div_le_iff₀ acceptedRealPositive]
  exact_mod_cast totalLe

/-! ### Concrete signed-x affine sampler -/

open WeierstrassCurve WeierstrassCurve.Affine

/-- Choose one nonsingular y-coordinate above `x` whenever one exists. -/
noncomputable def affineSamplerRoot
    {K : Type*} [Field K] (W : WeierstrassCurve.Affine K) (x : K) : K := by
  classical
  exact if h : ∃ y, W.Nonsingular x y then Classical.choose h else 0

/-- The chosen root is nonsingular whenever any supplied root is. -/
theorem affineSamplerRoot_spec
    {K : Type*} [Field K] (W : WeierstrassCurve.Affine K) {x y : K}
    (hy : W.Nonsingular x y) : W.Nonsingular x (affineSamplerRoot W x) := by
  classical
  rw [affineSamplerRoot, dite_eq_left ⟨y, hy⟩]
  exact Classical.choose_spec ⟨y, hy⟩

/-- Nonsingular affine coordinate pairs, excluding the point at infinity. -/
abbrev AffineSamplerCoordinates
    {K : Type*} [Field K] (W : WeierstrassCurve.Affine K) :=
  Σ x : K, {y : K // W.Nonsingular x y}

noncomputable instance instFintypeAffineSamplerCoordinates
    {K : Type*} [Field K] [Fintype K] (W : WeierstrassCurve.Affine K) :
    Fintype (AffineSamplerCoordinates W) :=
  Fintype.ofFinite _

/-- Adjoining one `none` value to affine coordinates gives exactly Mathlib's
affine point type, with `none` corresponding to infinity. -/
def affineSamplerCoordinatesEquivPoint
    {K : Type*} [Field K] (W : WeierstrassCurve.Affine K) :
    Option (AffineSamplerCoordinates W) ≃ W.Point where
  toFun
    | none => 0
    | some ⟨x, ⟨y, h⟩⟩ => Point.some x y h
  invFun
    | .zero => none
    | .some x y h => some ⟨x, ⟨y, h⟩⟩
  left_inv value := by cases value <;> rfl
  right_inv point := by cases point <;> rfl

noncomputable instance instFintypeAffineSamplerPoint
    {K : Type*} [Field K] [Fintype K] (W : WeierstrassCurve.Affine K) :
    Fintype W.Point :=
  Fintype.ofEquiv (Option (AffineSamplerCoordinates W))
    (affineSamplerCoordinatesEquivPoint W)

/-- The full point type consists of all affine coordinate pairs plus the point
at infinity. -/
theorem affineSampler_point_card
    {K : Type*} [Field K] [Fintype K] (W : WeierstrassCurve.Affine K) :
    Fintype.card W.Point = Fintype.card (AffineSamplerCoordinates W) + 1 := by
  rw [← Fintype.card_congr (affineSamplerCoordinatesEquivPoint W)]
  simp

/-- One paper-style trial from an x-coordinate and a sign bit.  Soluble x's
use the chosen root and its `negY`; when they coincide, exactly one sign is
accepted. -/
noncomputable def affineSignedTrial
    {K : Type*} [Field K] (W : WeierstrassCurve.Affine K) :
    K × Bool → Option (AffineSamplerCoordinates W) := by
  classical
  exact fun seed =>
    let x := seed.1
    let y := affineSamplerRoot W x
    if h : W.Nonsingular x y then
      if seed.2 then
        if hfixed : W.negY x y = y then none
        else some ⟨x, ⟨W.negY x y, (W.nonsingular_neg x y).2 h⟩⟩
      else some ⟨x, ⟨y, h⟩⟩
    else none

/-- The unique signed-x seed assigned to a nonsingular affine point. -/
noncomputable def affineSignedEncode
    {K : Type*} [Field K] (W : WeierstrassCurve.Affine K) :
    AffineSamplerCoordinates W → K × Bool := by
  classical
  exact fun point =>
    (point.1, if point.2.1 = affineSamplerRoot W point.1 then false else true)

/-- Decoding the assigned signed-x seed recovers the affine point. -/
theorem affineSignedTrial_encode
    {K : Type*} [Field K] (W : WeierstrassCurve.Affine K)
    (point : AffineSamplerCoordinates W) :
    affineSignedTrial W (affineSignedEncode W point) = some point := by
  classical
  obtain ⟨x, y, hy⟩ := point
  have hr := affineSamplerRoot_spec W hy
  by_cases heq : y = affineSamplerRoot W x
  · subst y
    simp [affineSignedEncode, affineSignedTrial, hr]
  · have hneg : y = W.negY x (affineSamplerRoot W x) :=
      (Y_eq_of_X_eq hy.1 hr.1 rfl).resolve_left heq
    have hfixed : W.negY x (affineSamplerRoot W x) ≠ affineSamplerRoot W x := by
      intro h
      exact heq (hneg.trans h)
    have hfixed' :
        -affineSamplerRoot W x - W.a₁ * x - W.a₃ ≠ affineSamplerRoot W x := by
      simpa only [negY] using hfixed
    simp [affineSignedEncode, affineSignedTrial, hr, hfixed', hneg]

/-- Every accepted signed-x seed is the assigned seed of its output. -/
theorem affineSignedTrial_unique
    {K : Type*} [Field K] (W : WeierstrassCurve.Affine K)
    (seed : K × Bool) (point : AffineSamplerCoordinates W)
    (hsample : affineSignedTrial W seed = some point) :
    seed = affineSignedEncode W point := by
  classical
  obtain ⟨x, bit⟩ := seed
  simp only [affineSignedTrial] at hsample
  split at hsample <;> rename_i hr
  · cases bit with
    | false =>
        simp only [Bool.false_eq_true, ↓reduceIte] at hsample
        injection hsample with hp
        subst point
        simp [affineSignedEncode]
    | true =>
        simp only [↓reduceIte] at hsample
        split at hsample <;> rename_i hfixed
        · contradiction
        · injection hsample with hp
          subst point
          have hfixed' :
              -affineSamplerRoot W x - W.a₁ * x - W.a₃ ≠ affineSamplerRoot W x := by
            simpa only [negY] using hfixed
          simp [affineSignedEncode, hfixed']
  · contradiction

/-- Every nonsingular affine point has exactly one accepted signed-x seed. -/
theorem affineSignedTrial_weight_eq_one
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (W : WeierstrassCurve.Affine K) :
    ∀ point, rejectionSamplerWeight (affineSignedTrial W) point = 1 := by
  classical
  exact uniqueAcceptedSeedWeight_eq_one (affineSignedTrial W)
    (affineSignedEncode W) (affineSignedTrial_encode W)
    (affineSignedTrial_unique W)

/-- The successful signed-x seeds are in bijective count with all affine
points. -/
theorem affineSignedTrial_accepted_card
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (W : WeierstrassCurve.Affine K) :
    (rejectionSamplerAcceptedSeeds (affineSignedTrial W)).card =
      Fintype.card (AffineSamplerCoordinates W) := by
  classical
  simpa using uniformAcceptedFibres_card (affineSignedTrial W) 1
    (affineSignedTrial_weight_eq_one W)

/-- Conditioning a signed-x trial on acceptance is exactly uniform for every
pair event on affine points. -/
theorem affineSignedTrial_pairEvent_uniform
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (W : WeierstrassCurve.Affine K)
    (good : AffineSamplerCoordinates W × AffineSamplerCoordinates W → Prop)
    [DecidablePred good] :
    (∑ pair : AffineSamplerCoordinates W × AffineSamplerCoordinates W,
        if good pair then
          rejectionSamplerWeight (affineSignedTrial W) pair.1 *
            rejectionSamplerWeight (affineSignedTrial W) pair.2
        else 0) =
      (Finset.univ.filter good).card := by
  simpa using uniformAcceptedPairEventWeight (affineSignedTrial W) good 1
    (affineSignedTrial_weight_eq_one W)

/-- The repeated signed-x sampler has exact expected-trial tail sum
`2|K| / #affinePoints`. -/
theorem affineSignedTrialTailHasSum
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (W : WeierstrassCurve.Affine K)
    (affinePointPositive : 0 < Fintype.card (AffineSamplerCoordinates W)) :
    HasSum
      (fun failures : ℕ =>
        (1 - (Fintype.card (AffineSamplerCoordinates W) : ℝ) /
          (2 * Fintype.card K : ℕ)) ^ failures)
      ((2 * Fintype.card K : ℕ) /
        (Fintype.card (AffineSamplerCoordinates W) : ℝ)) := by
  have hle : Fintype.card (AffineSamplerCoordinates W) ≤ 2 * Fintype.card K := by
    rw [← affineSignedTrial_accepted_card W]
    calc
      (rejectionSamplerAcceptedSeeds (affineSignedTrial W)).card ≤
          Fintype.card (K × Bool) := Finset.card_le_univ _
      _ = 2 * Fintype.card K := by simp [Nat.mul_comm]
  simpa only [Nat.cast_mul, Nat.cast_ofNat] using
    rejectionTrialTailHasSum (2 * Fintype.card K)
      (Fintype.card (AffineSamplerCoordinates W)) affinePointPositive hle

/-- The paper's `2p` signed-x trial has expected retry count at most four once
the displayed affine-point cardinality inequality is supplied. -/
theorem affineSignedTrialExpectation_le_four
    {K : Type*} [Field K] [Fintype K]
    (W : WeierstrassCurve.Affine K)
    (affinePointPositive : 0 < Fintype.card (AffineSamplerCoordinates W))
    (hasseForm :
      2 * Fintype.card K ≤ 4 * Fintype.card (AffineSamplerCoordinates W)) :
    (2 * Fintype.card K : ℕ) /
        (Fintype.card (AffineSamplerCoordinates W) : ℝ) ≤ 4 := by
  exact rejectionTrialExpectation_le (2 * Fintype.card K)
    (Fintype.card (AffineSamplerCoordinates W)) 4 affinePointPositive hasseForm

/-- A single full-curve rejection trial: one distinguished seed produces the
identity and every other seed runs the signed-x affine trial. -/
noncomputable def affineFullSignedTrial
    {K : Type*} [Field K] (W : WeierstrassCurve.Affine K) :
    Option (K × Bool) → Option (Option (AffineSamplerCoordinates W))
  | none => some none
  | some seed => (affineSignedTrial W seed).map some

/-- The assigned full-curve seed: the distinguished seed for infinity and the
assigned signed-x seed for an affine point. -/
noncomputable def affineFullSignedEncode
    {K : Type*} [Field K] (W : WeierstrassCurve.Affine K) :
    Option (AffineSamplerCoordinates W) → Option (K × Bool)
  | none => none
  | some point => some (affineSignedEncode W point)

/-- The full-curve trial decodes every assigned seed to its output. -/
theorem affineFullSignedTrial_encode
    {K : Type*} [Field K] (W : WeierstrassCurve.Affine K)
    (point : Option (AffineSamplerCoordinates W)) :
    affineFullSignedTrial W (affineFullSignedEncode W point) = some point := by
  cases point with
  | none => rfl
  | some point => simp [affineFullSignedEncode, affineFullSignedTrial,
      affineSignedTrial_encode]

/-- Every accepted full-curve seed is the assigned seed of its output. -/
theorem affineFullSignedTrial_unique
    {K : Type*} [Field K] (W : WeierstrassCurve.Affine K)
    (seed : Option (K × Bool)) (point : Option (AffineSamplerCoordinates W))
    (hsample : affineFullSignedTrial W seed = some point) :
    seed = affineFullSignedEncode W point := by
  cases seed with
  | none =>
      simp [affineFullSignedTrial] at hsample
      subst point
      rfl
  | some seed =>
      cases htrial : affineSignedTrial W seed with
      | none => simp [affineFullSignedTrial, htrial] at hsample
      | some affinePoint =>
          simp [affineFullSignedTrial, htrial] at hsample
          subst point
          simp [affineFullSignedEncode,
            affineSignedTrial_unique W seed affinePoint htrial]

/-- Every full curve point, identity included, has exactly one accepted seed. -/
theorem affineFullSignedTrial_weight_eq_one
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (W : WeierstrassCurve.Affine K) :
    ∀ point, rejectionSamplerWeight (affineFullSignedTrial W) point = 1 := by
  classical
  exact uniqueAcceptedSeedWeight_eq_one (affineFullSignedTrial W)
    (affineFullSignedEncode W) (affineFullSignedTrial_encode W)
    (affineFullSignedTrial_unique W)

/-- Conditioning the full sampler on acceptance is exactly uniform for every
pair event on the whole curve. -/
theorem affineFullSignedTrial_pairEvent_uniform
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (W : WeierstrassCurve.Affine K)
    (good : Option (AffineSamplerCoordinates W) ×
      Option (AffineSamplerCoordinates W) → Prop)
    [DecidablePred good] :
    (∑ pair : Option (AffineSamplerCoordinates W) ×
        Option (AffineSamplerCoordinates W),
      if good pair then
        rejectionSamplerWeight (affineFullSignedTrial W) pair.1 *
          rejectionSamplerWeight (affineFullSignedTrial W) pair.2
      else 0) =
      (Finset.univ.filter good).card := by
  simpa using uniformAcceptedPairEventWeight (affineFullSignedTrial W) good 1
    (affineFullSignedTrial_weight_eq_one W)

/-- The successful full-sampler seeds are counted by the full curve points. -/
theorem affineFullSignedTrial_accepted_card
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (W : WeierstrassCurve.Affine K) :
    (rejectionSamplerAcceptedSeeds (affineFullSignedTrial W)).card =
      Fintype.card W.Point := by
  rw [uniformAcceptedFibres_card (affineFullSignedTrial W) 1
    (affineFullSignedTrial_weight_eq_one W)]
  simpa using Fintype.card_congr (affineSamplerCoordinatesEquivPoint W)

/-- The full sampler's retry tail has exact expected length
`(2|K|+1)/|W.Point|`. -/
theorem affineFullSignedTrialTailHasSum
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (W : WeierstrassCurve.Affine K) :
    HasSum
      (fun failures : ℕ =>
        (1 - (Fintype.card W.Point : ℝ) /
          (2 * Fintype.card K + 1 : ℕ)) ^ failures)
      ((2 * Fintype.card K + 1 : ℕ) / (Fintype.card W.Point : ℝ)) := by
  have pointPositive : 0 < Fintype.card W.Point := Fintype.card_pos
  have hle : Fintype.card W.Point ≤ 2 * Fintype.card K + 1 := by
    rw [← affineFullSignedTrial_accepted_card W]
    calc
      (rejectionSamplerAcceptedSeeds (affineFullSignedTrial W)).card ≤
          Fintype.card (Option (K × Bool)) := Finset.card_le_univ _
      _ = 2 * Fintype.card K + 1 := by simp [Nat.mul_comm]
  simpa only [Nat.cast_add, Nat.cast_mul, Nat.cast_one, Nat.cast_ofNat] using
    rejectionTrialTailHasSum (2 * Fintype.card K + 1)
      (Fintype.card W.Point) pointPositive hle

/-- The full signed-x sampler has expected retry count at most four under the
single displayed curve-cardinality inequality. -/
theorem affineFullSignedTrialExpectation_le_four
    {K : Type*} [Field K] [Fintype K]
    (W : WeierstrassCurve.Affine K)
    (hasseForm : 2 * Fintype.card K + 1 ≤ 4 * Fintype.card W.Point) :
    (2 * Fintype.card K + 1 : ℕ) / (Fintype.card W.Point : ℝ) ≤ 4 := by
  exact rejectionTrialExpectation_le (2 * Fintype.card K + 1)
    (Fintype.card W.Point) 4 Fintype.card_pos hasseForm

/-- For a field-size proxy at least 16, the squared Hasse trace bound already
forces the trace deficit to be at most half the field size. -/
theorem hasseSquareImpliesHalfFieldTrace
    (fieldSize trace : ℕ) (fieldLarge : 16 ≤ fieldSize)
    (hasseSquare : trace ^ 2 ≤ 4 * fieldSize) :
    2 * trace ≤ fieldSize := by
  by_contra traceTooLarge
  have nextField_le_twiceTrace : fieldSize + 1 ≤ 2 * trace := by omega
  have squareGrowth : (fieldSize + 1) ^ 2 ≤ (2 * trace) ^ 2 :=
    Nat.pow_le_pow_left nextField_le_twiceTrace 2
  nlinarith

/-- The standard point-count relation and squared Hasse trace bound imply the
exact cardinality inequality required by the full signed-x sampler. -/
theorem hasseTraceBoundGivesSamplerFour
    (fieldSize pointCount trace : ℕ) (fieldLarge : 16 ≤ fieldSize)
    (pointTrace : fieldSize + 1 ≤ pointCount + trace)
    (hasseSquare : trace ^ 2 ≤ 4 * fieldSize) :
    2 * fieldSize + 1 ≤ 4 * pointCount := by
  have traceHalf : 2 * trace ≤ fieldSize :=
    hasseSquareImpliesHalfFieldTrace fieldSize trace fieldLarge hasseSquare
  omega

/-- Curve-level composition of the Hasse arithmetic bridge with the exact
full-curve rejection-sampler expectation theorem. -/
theorem affineFullSignedTrialExpectation_le_four_of_hasseSquare
    {K : Type*} [Field K] [Fintype K]
    (W : WeierstrassCurve.Affine K) (trace : ℕ)
    (fieldLarge : 16 ≤ Fintype.card K)
    (pointTrace : Fintype.card K + 1 ≤ Fintype.card W.Point + trace)
    (hasseSquare : trace ^ 2 ≤ 4 * Fintype.card K) :
    (2 * Fintype.card K + 1 : ℕ) / (Fintype.card W.Point : ℝ) ≤ 4 := by
  apply affineFullSignedTrialExpectation_le_four W
  exact hasseTraceBoundGivesSamplerFour (Fintype.card K)
    (Fintype.card W.Point) trace fieldLarge pointTrace hasseSquare

/-- The integral squared form of Hasse directly implies the natural
cardinality inequality required by the full signed-x sampler. -/
theorem integerHasseGivesSamplerFour
    (fieldSize pointCount : ℕ) (fieldLarge : 16 ≤ fieldSize)
    (hasseSquare :
      (((fieldSize : ℤ) + 1 - (pointCount : ℤ)) ^ 2 ≤
        4 * (fieldSize : ℤ))) :
    2 * fieldSize + 1 ≤ 4 * pointCount := by
  have fieldLargeInt : (16 : ℤ) ≤ fieldSize := by exact_mod_cast fieldLarge
  have pointCountNonnegative : (0 : ℤ) ≤ pointCount := by positivity
  have targetInt :
      2 * (fieldSize : ℤ) + 1 ≤ 4 * (pointCount : ℤ) := by
    nlinarith [sq_nonneg ((fieldSize : ℤ) - 16)]
  exact_mod_cast targetInt

/-- The source-pinned Atlas Manin proof of Hasse closes the sampler's
four-trial bound for elliptic curves over finite fields of size at least 16. -/
theorem affineFullSignedTrialExpectation_le_four_of_atlasHasse
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (W : WeierstrassCurve.Affine K) [W.IsElliptic]
    (fieldLarge : 16 ≤ Fintype.card K) :
    (2 * Fintype.card K + 1 : ℕ) / (Fintype.card W.Point : ℝ) ≤ 4 := by
  apply affineFullSignedTrialExpectation_le_four W
  apply integerHasseGivesSamplerFour (Fintype.card K) (Fintype.card W.Point)
    fieldLarge
  simpa only [Nat.card_eq_fintype_card] using
    MathlibExt.AlgebraicGeometry.EllipticCurve.HasseBoundWanted.hasse_bound W

/-- For finite fields of cardinality three modulo four, exponentiation by
`(|K|+1)/4` is an executable square-root candidate. -/
def threeModFourSqrt
    {K : Type*} [Field K] [Fintype K] (a : K) : K :=
  a ^ ((Fintype.card K + 1) / 4)

/-- The three-mod-four exponentiation candidate squares to every supplied
quadratic residue, including zero. -/
theorem threeModFourSqrt_sq
    {K : Type*} [Field K] [Fintype K]
    (cardModFour : Fintype.card K % 4 = 3)
    {a : K} (isSquare : ∃ y : K, y ^ 2 = a) :
    threeModFourSqrt a ^ 2 = a := by
  rcases isSquare with ⟨y, rfl⟩
  have exponentIdentity :
      4 * ((Fintype.card K + 1) / 4) = Fintype.card K + 1 := by
    omega
  rw [threeModFourSqrt, ← pow_mul, ← pow_mul]
  rw [show 2 * (((Fintype.card K + 1) / 4) * 2) =
      Fintype.card K + 1 by omega]
  rw [pow_succ, FiniteField.pow_card]
  simp [pow_two]

/-- The right-hand side of a short-form Weierstrass equation. -/
def shortCurveRhs
    {K : Type*} [Field K] (W : WeierstrassCurve.Affine K) (x : K) : K :=
  x ^ 3 + W.a₂ * x ^ 2 + W.a₄ * x + W.a₆

/-- Executable y-root candidate for short-form curves over fields of cardinality
three modulo four. -/
def shortThreeModFourRoot
    {K : Type*} [Field K] [Fintype K]
    (W : WeierstrassCurve.Affine K) (x : K) : K :=
  threeModFourSqrt (shortCurveRhs W x)

/-- On an elliptic short-form curve, the executable three-mod-four candidate
is nonsingular whenever any y-coordinate above the same x is nonsingular. -/
theorem shortThreeModFourRoot_spec
    {K : Type*} [Field K] [Fintype K]
    (W : WeierstrassCurve.Affine K) [W.IsElliptic]
    (a₁_zero : W.a₁ = 0) (a₃_zero : W.a₃ = 0)
    (cardModFour : Fintype.card K % 4 = 3)
    {x y : K} (hy : W.Nonsingular x y) :
    W.Nonsingular x (shortThreeModFourRoot W x) := by
  have rhsIsSquare : ∃ root : K, root ^ 2 = shortCurveRhs W x := by
    refine ⟨y, ?_⟩
    have equation := (W.equation_iff x y).mp hy.1
    simpa [shortCurveRhs, a₁_zero, a₃_zero] using equation
  apply W.equation_iff_nonsingular.mp
  apply (W.equation_iff x (shortThreeModFourRoot W x)).mpr
  have rootSquare := threeModFourSqrt_sq cardModFour rhsIsSquare
  simpa [shortThreeModFourRoot, shortCurveRhs, a₁_zero, a₃_zero] using rootSquare

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

/-- Square an already computed scalar, charging only the new multiplication. -/
def RingCounted.squareCached [Mul R] (x : RingCounted R) : RingCounted R :=
  ⟨x.value * x.value, x.ops + 1⟩

/-- Multiply an already computed scalar by an available scalar, charging only
the new multiplication. -/
def RingCounted.mulCachedRight [Mul R]
    (x : RingCounted R) (y : R) : RingCounted R :=
  ⟨x.value * y, x.ops + 1⟩

/-- Executable right-to-left binary exponentiation with its scalar
multiplications counted. -/
def countedBinaryPow {R : Type*} [Monoid R]
    (a : R) : ℕ → RingCounted R :=
  Nat.binaryRec' (RingCounted.lift 1) fun bit _ _ previous =>
    let squared := RingCounted.squareCached previous
    if bit then RingCounted.mulCachedRight squared a else squared

@[simp] theorem countedBinaryPow_zero
    {R : Type*} [Monoid R] (a : R) :
    countedBinaryPow a 0 = RingCounted.lift 1 := by
  simp [countedBinaryPow]

theorem countedBinaryPow_bit
    {R : Type*} [Monoid R] (a : R) (bit : Bool) (exponent : ℕ)
    (nonzero : exponent = 0 → bit = true) :
    countedBinaryPow a (Nat.bit bit exponent) =
      let squared := RingCounted.squareCached (countedBinaryPow a exponent)
      if bit then RingCounted.mulCachedRight squared a else squared := by
  rw [countedBinaryPow, Nat.binaryRec'_eq _ _ nonzero]
  change (let squared := RingCounted.squareCached (countedBinaryPow a exponent)
    if bit then RingCounted.mulCachedRight squared a else squared) = _
  rfl

/-- Erasing the multiplication counter from binary exponentiation gives the
ordinary monoid power. -/
theorem countedBinaryPow_value
    {R : Type*} [Monoid R] (a : R) (exponent : ℕ) :
    (countedBinaryPow a exponent).value = a ^ exponent := by
  induction exponent using Nat.binaryRec' with
  | zero => simp [RingCounted.lift]
  | bit bit exponent nonzero ih =>
      rw [countedBinaryPow_bit a bit exponent nonzero]
      cases bit <;>
        simp [RingCounted.squareCached, RingCounted.mulCachedRight, ih,
          ← pow_add, two_mul, pow_succ]

/-- Binary exponentiation uses at most two multiplications per exponent bit. -/
theorem countedBinaryPow_ops_le
    {R : Type*} [Monoid R] (a : R) (exponent : ℕ) :
    (countedBinaryPow a exponent).ops ≤ 2 * Nat.size exponent := by
  induction exponent using Nat.binaryRec' with
  | zero => simp [countedBinaryPow, RingCounted.lift, Nat.size_zero]
  | bit bit exponent nonzero ih =>
      have bitNonzero : Nat.bit bit exponent ≠ 0 :=
        Nat.bit_ne_zero_iff.mpr nonzero
      rw [countedBinaryPow_bit a bit exponent nonzero,
        Nat.size_bit bitNonzero]
      cases bit <;>
        simp [RingCounted.squareCached, RingCounted.mulCachedRight] <;> omega

/-- Counted repeated-squaring implementation of the three-mod-four square-root
candidate. -/
def countedThreeModFourSqrt
    {K : Type*} [Field K] [Fintype K] (a : K) : RingCounted K :=
  countedBinaryPow a ((Fintype.card K + 1) / 4)

theorem countedThreeModFourSqrt_value
    {K : Type*} [Field K] [Fintype K] (a : K) :
    (countedThreeModFourSqrt a).value = threeModFourSqrt a := by
  exact countedBinaryPow_value a _

theorem countedThreeModFourSqrt_ops_le
    {K : Type*} [Field K] [Fintype K] (a : K) :
    (countedThreeModFourSqrt a).ops ≤
      2 * Nat.size ((Fintype.card K + 1) / 4) := by
  exact countedBinaryPow_ops_le a _

theorem countedThreeModFourSqrt_ops_le_fieldSize
    {K : Type*} [Field K] [Fintype K] (a : K)
    (cardModFour : Fintype.card K % 4 = 3) :
    (countedThreeModFourSqrt a).ops ≤ 2 * Nat.size (Fintype.card K) := by
  apply (countedThreeModFourSqrt_ops_le a).trans
  apply Nat.mul_le_mul_left 2
  apply Nat.size_le_size
  omega

/-- Abstract bit work for the counted three-mod-four square root. -/
def countedThreeModFourSqrtBitWork
    {K : Type*} [Field K] [Fintype K]
    (a : K) (scalarMulBitCost : ℕ) : ℕ :=
  (countedThreeModFourSqrt a).ops * scalarMulBitCost

/-- Conditional bit-work bound for the executable three-mod-four square root.
The only external arithmetic premise is the supplied scalar multiplication
cost; the number of multiplications is checked above. -/
theorem countedThreeModFourSqrtBitWork_le
    {K : Type*} [Field K] [Fintype K]
    (a : K) (modulusBits scalarMulBitCost constant degree : ℕ)
    (cardModFour : Fintype.card K % 4 = 3)
    (hMul : scalarMulBitCost ≤ constant * (modulusBits + 1) ^ degree) :
    countedThreeModFourSqrtBitWork a scalarMulBitCost ≤
      (2 * Nat.size (Fintype.card K)) *
        (constant * (modulusBits + 1) ^ degree) := by
  exact Nat.mul_le_mul (countedThreeModFourSqrt_ops_le_fieldSize a cardModFour)
    hMul

theorem countedThreeModFourSqrt_sq
    {K : Type*} [Field K] [Fintype K]
    (cardModFour : Fintype.card K % 4 = 3)
    {a : K} (isSquare : ∃ y : K, y ^ 2 = a) :
    (countedThreeModFourSqrt a).value ^ 2 = a := by
  rw [countedThreeModFourSqrt_value]
  exact threeModFourSqrt_sq cardModFour isSquare

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

/-- Scalar-ring budget for the whole Bird inverse formula: one determinant,
all row-update determinants forming the adjugate, and `n²` final scalar
multiplications.  The single scalar inversion is kept separate because its
bit cost is not a ring-operation count. -/
def eagerBirdZModInverseRingOps (n : ℕ) : ℕ :=
  eagerBirdAdjugateRingOps n + eagerBirdDetRingOps n + n * n

/-- The complete Bird inverse formula, apart from its one scalar inversion,
has an explicit algebraic-operation bound. -/
theorem eagerBirdZModInverseRingOps_le (n : ℕ) :
    eagerBirdZModInverseRingOps n ≤
      4 * (n + 1) ^ 6 + 4 * (n + 1) ^ 4 + n * n := by
  exact Nat.add_le_add
    (Nat.add_le_add (eagerBirdAdjugateRingOps_le_sixth n)
      (eagerBirdDetRingOps_le_quartic n))
    le_rfl

/-- Abstract bit work obtained by charging every scalar ring operation the
same supplied cost and charging the one scalar inversion separately. -/
def eagerBirdZModInverseBitWork
    (n scalarRingBitCost scalarInverseBitCost : ℕ) : ℕ :=
  eagerBirdZModInverseRingOps n * scalarRingBitCost + scalarInverseBitCost

/-- Explicit conditional bit bound for the executable Bird inverse.  The two
hypotheses expose the currently external arithmetic facts: a fixed polynomial
bound for one modular ring operation and a fixed polynomial bound for the
extended-GCD scalar inverse.  No runtime claim about Lean's evaluator is used. -/
theorem eagerBirdZModInverseBitWork_le
    (n modulusBits scalarRingBitCost scalarInverseBitCost : ℕ)
    (ringConstant ringDegree inverseConstant inverseDegree : ℕ)
    (hRing : scalarRingBitCost ≤
      ringConstant * (modulusBits + 1) ^ ringDegree)
    (hInverse : scalarInverseBitCost ≤
      inverseConstant * (modulusBits + 1) ^ inverseDegree) :
    eagerBirdZModInverseBitWork n scalarRingBitCost scalarInverseBitCost ≤
      (4 * (n + 1) ^ 6 + 4 * (n + 1) ^ 4 + n * n) *
          (ringConstant * (modulusBits + 1) ^ ringDegree) +
        inverseConstant * (modulusBits + 1) ^ inverseDegree := by
  rw [eagerBirdZModInverseBitWork]
  exact Nat.add_le_add
    (Nat.mul_le_mul (eagerBirdZModInverseRingOps_le n) hRing)
    hInverse

/-! ### Counted dense rectangular postprocessing -/

/-- Structurally recursive dense dot product on finite vectors. -/
def denseDot
    {R : Type*} [CommRing R] : {n : ℕ} → (Fin n → R) → (Fin n → R) → R
  | 0, _, _ => 0
  | _n + 1, left, right =>
      left 0 * right 0 + denseDot (fun k => left k.succ) (fun k => right k.succ)

/-- The recursive dot product is the ordinary finite sum. -/
theorem denseDot_eq_sum
    {R : Type*} [CommRing R] {n : ℕ} (left right : Fin n → R) :
    denseDot left right = ∑ k, left k * right k := by
  induction n with
  | zero => simp [denseDot]
  | succ n ih =>
      rw [denseDot, Fin.sum_univ_succ]
      congr 1
      exact ih _ _

/-- Dot product with scalar-ring operations instrumented. -/
def countedDenseDot
    {R : Type*} [CommRing R] :
    {n : ℕ} → (Fin n → R) → (Fin n → R) → RingCounted R
  | 0, _, _ => RingCounted.lift 0
  | _n + 1, left, right =>
      RingCounted.add
        (RingCounted.mul (RingCounted.lift (left 0))
          (RingCounted.lift (right 0)))
        (countedDenseDot (fun k => left k.succ) (fun k => right k.succ))

/-- Erasing the dense-dot counter recovers the recursive value. -/
theorem countedDenseDot_value
    {R : Type*} [CommRing R] {n : ℕ} (left right : Fin n → R) :
    (countedDenseDot left right).value = denseDot left right := by
  induction n with
  | zero => rfl
  | succ n ih =>
      simp only [countedDenseDot, denseDot, RingCounted.add, RingCounted.mul,
        RingCounted.lift]
      rw [ih]

/-- A dense dot product of length `n` uses exactly `n` multiplications and
`n` additions, counting addition into the zero accumulator. -/
theorem countedDenseDot_ops
    {R : Type*} [CommRing R] {n : ℕ} (left right : Fin n → R) :
    (countedDenseDot left right).ops = 2 * n := by
  induction n with
  | zero => rfl
  | succ n ih =>
      simp only [countedDenseDot, RingCounted.add, RingCounted.mul,
        RingCounted.lift]
      rw [ih]
      omega

/-- Dense rectangular multiplication through the counted dot-product value
formula. -/
def eagerDenseMatrixMul
    {R : Type*} [CommRing R] {rows inner cols : ℕ}
    (left : Matrix (Fin rows) (Fin inner) R)
    (right : Matrix (Fin inner) (Fin cols) R) :
    Matrix (Fin rows) (Fin cols) R :=
  fun i j => denseDot (left i) (fun k => right k j)

/-- The recursive dense multiplication equals Mathlib matrix multiplication. -/
theorem eagerDenseMatrixMul_eq_mul
    {R : Type*} [CommRing R] {rows inner cols : ℕ}
    (left : Matrix (Fin rows) (Fin inner) R)
    (right : Matrix (Fin inner) (Fin cols) R) :
    eagerDenseMatrixMul left right = left * right := by
  ext i j
  rw [eagerDenseMatrixMul, Matrix.mul_apply, denseDot_eq_sum]

/-- Total scalar-ring operations for eagerly materializing a dense
`rows × inner` by `inner × cols` product. -/
def eagerDenseMatrixMulRingOps (rows inner cols : ℕ) : ℕ :=
  rows * cols * (2 * inner)

/-- Summing the exact entry counters over every materialized output entry
recovers the advertised dense-multiplication budget. -/
theorem eagerDenseMatrixMulRingOps_eq
    {R : Type*} [CommRing R] {rows inner cols : ℕ}
    (left : Matrix (Fin rows) (Fin inner) R)
    (right : Matrix (Fin inner) (Fin cols) R) :
    (∑ i : Fin rows, ∑ j : Fin cols,
      (countedDenseDot (left i) (fun k => right k j)).ops) =
        eagerDenseMatrixMulRingOps rows inner cols := by
  simp [countedDenseDot_ops, eagerDenseMatrixMulRingOps, mul_assoc]

/-- Scalar-ring budget for the complete executable planted kernel: the Bird
inverse, the dense `R * B⁻¹` product, and one negation per output entry. -/
def eagerBirdPlantedKernelRingOps (f s : ℕ) : ℕ :=
  eagerBirdZModInverseRingOps f + eagerDenseMatrixMulRingOps s f f + s * f

/-- Explicit algebraic-operation bound for the complete planted kernel. -/
theorem eagerBirdPlantedKernelRingOps_le (f s : ℕ) :
    eagerBirdPlantedKernelRingOps f s ≤
      4 * (f + 1) ^ 6 + 4 * (f + 1) ^ 4 + f * f +
        s * f * (2 * f) + s * f := by
  exact Nat.add_le_add
    (Nat.add_le_add (eagerBirdZModInverseRingOps_le f) le_rfl) le_rfl

/-- Abstract bit work for the complete planted kernel. -/
def eagerBirdPlantedKernelBitWork
    (f s scalarRingBitCost scalarInverseBitCost : ℕ) : ℕ :=
  eagerBirdPlantedKernelRingOps f s * scalarRingBitCost + scalarInverseBitCost

/-- Conditional polynomial bit bound for inverse construction and rectangular
postprocessing together. -/
theorem eagerBirdPlantedKernelBitWork_le
    (f s modulusBits scalarRingBitCost scalarInverseBitCost : ℕ)
    (ringConstant ringDegree inverseConstant inverseDegree : ℕ)
    (hRing : scalarRingBitCost ≤
      ringConstant * (modulusBits + 1) ^ ringDegree)
    (hInverse : scalarInverseBitCost ≤
      inverseConstant * (modulusBits + 1) ^ inverseDegree) :
    eagerBirdPlantedKernelBitWork f s scalarRingBitCost scalarInverseBitCost ≤
      (4 * (f + 1) ^ 6 + 4 * (f + 1) ^ 4 + f * f +
          s * f * (2 * f) + s * f) *
          (ringConstant * (modulusBits + 1) ^ ringDegree) +
        inverseConstant * (modulusBits + 1) ^ inverseDegree := by
  rw [eagerBirdPlantedKernelBitWork]
  exact Nat.add_le_add
    (Nat.mul_le_mul (eagerBirdPlantedKernelRingOps_le f s) hRing)
    hInverse

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

/-- Executable replacement for the exact integer-block interface of upstream
`L114_Phase.kerMat`. -/
def executableBirdKerMat
    (q : ℕ) {f s : ℕ}
    (Ttop : Matrix (Fin f) (Fin f) ℤ)
    (Rint : Matrix (Fin s) (Fin f) ℤ)
    (nu : ℤ) : Matrix (Fin s) (Fin f ⊕ Fin s) (ZMod q) :=
  executableBirdPlantedKernel
    ((Ttop - Matrix.scalar (Fin f) nu).map (Int.castRingHom (ZMod q)))
    (Rint.map (Int.castRingHom (ZMod q)))

/-- The executable upstream-shaped kernel equals the original canonical block
formula whenever its planted block has unit determinant. -/
theorem executableBirdKerMat_eq
    (q : ℕ) {f s : ℕ}
    (Ttop : Matrix (Fin f) (Fin f) ℤ)
    (Rint : Matrix (Fin s) (Fin f) ℤ)
    (nu : ℤ)
    (hB : IsUnit (((Ttop - Matrix.scalar (Fin f) nu).map
      (Int.castRingHom (ZMod q))).det)) :
    executableBirdKerMat q Ttop Rint nu =
      Matrix.fromCols (-((Rint.map (Int.castRingHom (ZMod q))) *
        ((Ttop - Matrix.scalar (Fin f) nu).map
          (Int.castRingHom (ZMod q)))⁻¹)) 1 := by
  exact executableBirdPlantedKernel_eq _ _ hB

/-- Exact specialization of the executable planted-kernel laws to upstream
`Ttop`, `Rint`, `nu`, and `ZMod q` data. -/
theorem executableBirdKerMat_spec
    (q : ℕ) {f s : ℕ}
    (Ttop : Matrix (Fin f) (Fin f) ℤ)
    (Rint : Matrix (Fin s) (Fin f) ℤ)
    (nu : ℤ)
    (hB : IsUnit (((Ttop - Matrix.scalar (Fin f) nu).map
      (Int.castRingHom (ZMod q))).det)) :
    executableBirdKerMat q Ttop Rint nu *
        Matrix.fromRows
          ((Ttop - Matrix.scalar (Fin f) nu).map
            (Int.castRingHom (ZMod q)))
          (Rint.map (Int.castRingHom (ZMod q))) = 0 ∧
      executableBirdKerMat q Ttop Rint nu *
        Matrix.fromRows (0 : Matrix (Fin f) (Fin s) (ZMod q))
          (1 : Matrix (Fin s) (Fin s) (ZMod q)) = 1 := by
  exact executableBirdPlantedKernel_spec _ _ hB

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

/-! ## Hasse bounds for the first-moment character sums -/

open Finset

/-- The quadratic-character sum of a general monic short Weierstrass cubic. -/
def shortCubicCharacterSum
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (a₂ a₄ a₆ : K) : ℤ :=
  ∑ x : K, quadraticChar K (x ^ 3 + a₂ * x ^ 2 + a₄ * x + a₆)

/-- The short Weierstrass model whose affine equation is
`Y² = X³ + a₂X² + a₄X + a₆`. -/
def shortCharacterCurve
    {K : Type*} [Field K] (a₂ a₄ a₆ : K) : WeierstrassCurve K :=
  ⟨0, a₂, 0, a₄, a₆⟩

theorem shortCharacterCurve_equation_iff
    {K : Type*} [Field K] (a₂ a₄ a₆ x y : K) :
    (shortCharacterCurve a₂ a₄ a₆).toAffine.Equation x y ↔
      y ^ 2 = x ^ 3 + a₂ * x ^ 2 + a₄ * x + a₆ := by
  rw [WeierstrassCurve.Affine.equation_iff]
  simp only [shortCharacterCurve, zero_mul, zero_add]
  constructor <;> intro h
  · linear_combination h
  · linear_combination h

/-- Affine solutions of the short equation, rearranged into `X`-fibres. -/
def shortAffineSolutionEquivSigma
    {K : Type*} [Field K] (a₂ a₄ a₆ : K) :
    {xy : K × K // xy.2 ^ 2 =
      xy.1 ^ 3 + a₂ * xy.1 ^ 2 + a₄ * xy.1 + a₆} ≃
      Σ x : K, {y : K // y ^ 2 = x ^ 3 + a₂ * x ^ 2 + a₄ * x + a₆} where
  toFun xy := ⟨xy.1.1, xy.1.2, xy.2⟩
  invFun xy := ⟨(xy.1, xy.2.1), xy.2.2⟩
  left_inv _ := rfl
  right_inv _ := rfl

/-- The Mathlib affine equation subtype is exactly the displayed short
equation subtype. -/
def shortAffineEquationEquiv
    {K : Type*} [Field K] (a₂ a₄ a₆ : K) :
    {xy : K × K //
      (shortCharacterCurve a₂ a₄ a₆).toAffine.Equation xy.1 xy.2} ≃
      {xy : K × K // xy.2 ^ 2 =
        xy.1 ^ 3 + a₂ * xy.1 ^ 2 + a₄ * xy.1 + a₆} where
  toFun xy := ⟨xy.1,
    (shortCharacterCurve_equation_iff a₂ a₄ a₆ xy.1.1 xy.1.2).mp xy.2⟩
  invFun xy := ⟨xy.1,
    (shortCharacterCurve_equation_iff a₂ a₄ a₆ xy.1.1 xy.1.2).mpr xy.2⟩
  left_inv _ := rfl
  right_inv _ := rfl

/-- Counting the short equation by `X`-fibres gives `|K|` plus its quadratic
character sum. -/
theorem shortAffineSolution_card_eq_card_add_characterSum
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (hK : ringChar K ≠ 2) (a₂ a₄ a₆ : K) :
    (Fintype.card {xy : K × K // xy.2 ^ 2 =
      xy.1 ^ 3 + a₂ * xy.1 ^ 2 + a₄ * xy.1 + a₆} : ℤ) =
      Fintype.card K + shortCubicCharacterSum a₂ a₄ a₆ := by
  rw [Fintype.card_congr (shortAffineSolutionEquivSigma a₂ a₄ a₆),
    Fintype.card_sigma]
  push_cast
  calc
    ∑ x : K, (Fintype.card {y : K //
          y ^ 2 = x ^ 3 + a₂ * x ^ 2 + a₄ * x + a₆} : ℤ) =
        ∑ x : K, (quadraticChar K
          (x ^ 3 + a₂ * x ^ 2 + a₄ * x + a₆) + 1) := by
      apply sum_congr rfl
      intro x _
      calc
        (Fintype.card {y : K //
            y ^ 2 = x ^ 3 + a₂ * x ^ 2 + a₄ * x + a₆} : ℤ) =
            ((({y : K | y ^ 2 =
              x ^ 3 + a₂ * x ^ 2 + a₄ * x + a₆} : Set K).toFinset.card : ℕ) : ℤ) := by
              rw [Set.toFinset_card]
              exact_mod_cast Fintype.card_congr
                (Equiv.refl {y : K //
                  y ^ 2 = x ^ 3 + a₂ * x ^ 2 + a₄ * x + a₆})
        _ = quadraticChar K
              (x ^ 3 + a₂ * x ^ 2 + a₄ * x + a₆) + 1 :=
          quadraticChar_card_sqrts hK
            (x ^ 3 + a₂ * x ^ 2 + a₄ * x + a₆)
    _ = Fintype.card K + shortCubicCharacterSum a₂ a₄ a₆ := by
      simp [shortCubicCharacterSum, sum_add_distrib, add_comm]

/-- Exact point-count identity behind the short cubic character sum. -/
theorem shortCharacterCurve_point_card_eq_card_add_one_add_characterSum
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (hK : ringChar K ≠ 2) (a₂ a₄ a₆ : K)
    [(shortCharacterCurve a₂ a₄ a₆).IsElliptic] :
    (Nat.card (shortCharacterCurve a₂ a₄ a₆).toAffine.Point : ℤ) =
      Fintype.card K + 1 + shortCubicCharacterSum a₂ a₄ a₆ := by
  let W := shortCharacterCurve a₂ a₄ a₆
  calc
    (Nat.card W.toAffine.Point : ℤ) =
        Nat.card (WithZero {xy : K × K // W.toAffine.Equation xy.1 xy.2}) := by
      exact_mod_cast Nat.card_congr W.toAffine.pointEquiv
    _ = Nat.card {xy : K × K // W.toAffine.Equation xy.1 xy.2} + 1 := by
      change (Nat.card (Option {xy : K × K // W.toAffine.Equation xy.1 xy.2}) : ℤ) = _
      rw [Finite.card_option]
      norm_num
    _ = Nat.card {xy : K × K // xy.2 ^ 2 =
          xy.1 ^ 3 + a₂ * xy.1 ^ 2 + a₄ * xy.1 + a₆} + 1 := by
      exact_mod_cast congrArg (fun n : ℕ => n + 1)
        (Nat.card_congr (shortAffineEquationEquiv a₂ a₄ a₆))
    _ = Fintype.card {xy : K × K // xy.2 ^ 2 =
          xy.1 ^ 3 + a₂ * xy.1 ^ 2 + a₄ * xy.1 + a₆} + 1 := by
      rw [Nat.card_eq_fintype_card]
    _ = Fintype.card K + shortCubicCharacterSum a₂ a₄ a₆ + 1 := by
      rw [shortAffineSolution_card_eq_card_add_characterSum hK a₂ a₄ a₆]
    _ = Fintype.card K + 1 + shortCubicCharacterSum a₂ a₄ a₆ := by ring

/-- Atlas Hasse, expressed directly as the square bound for a monic short
cubic quadratic-character sum. -/
theorem shortCubicCharacterSum_sq_le
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (hK : ringChar K ≠ 2) (a₂ a₄ a₆ : K)
    [(shortCharacterCurve a₂ a₄ a₆).IsElliptic] :
    shortCubicCharacterSum a₂ a₄ a₆ ^ 2 ≤
      4 * (Fintype.card K : ℤ) := by
  have hcount :=
    shortCharacterCurve_point_card_eq_card_add_one_add_characterSum
      hK a₂ a₄ a₆
  have hhasse :=
    MathlibExt.AlgebraicGeometry.EllipticCurve.HasseBoundWanted.hasse_bound
      (shortCharacterCurve a₂ a₄ a₆)
  rw [Nat.card_eq_fintype_card] at hhasse
  nlinarith

/-- The discriminant of the original short curve `Y² = X³ + AX + B`. -/
theorem originalShortCurve_discriminant
    {K : Type*} [Field K] (A B : K) :
    (shortCharacterCurve 0 A B).Δ = -(16 : K) * (4 * A ^ 3 + 27 * B ^ 2) := by
  simp only [shortCharacterCurve, WeierstrassCurve.Δ, WeierstrassCurve.b₂,
    WeierstrassCurve.b₄, WeierstrassCurve.b₆, WeierstrassCurve.b₈]
  ring

/-- After inversion and scaling, the quartic sum is attached to
`Y² = X³ + AX² + B²`; its discriminant is `B²` times that of the original
short curve. -/
theorem transformedShortCurve_discriminant
    {K : Type*} [Field K] (A B : K) :
    (shortCharacterCurve A 0 (B ^ 2)).Δ =
      B ^ 2 * (shortCharacterCurve 0 A B).Δ := by
  simp only [shortCharacterCurve, WeierstrassCurve.Δ, WeierstrassCurve.b₂,
    WeierstrassCurve.b₄, WeierstrassCurve.b₆, WeierstrassCurve.b₈]
  ring

/-- Nonsingularity of the original short curve and `B ≠ 0` imply
nonsingularity of the transformed cubic used for the quartic sum. -/
theorem transformedShortCurveIsElliptic
    {K : Type*} [Field K] (A B : K)
    (hOriginal : (shortCharacterCurve 0 A B).Δ ≠ 0) (hB : B ≠ 0) :
    (shortCharacterCurve A 0 (B ^ 2)).IsElliptic := by
  constructor
  rw [isUnit_iff_ne_zero, transformedShortCurve_discriminant]
  exact mul_ne_zero (pow_ne_zero 2 hB) hOriginal

/-- The nonzero arguments of the paper's quartic character sum. -/
abbrev NonzeroField (K : Type*) [Zero K] := {x : K // x ≠ 0}

/-- On nonzero field elements, `x ↦ B/x` is an involution when `B ≠ 0`. -/
def nonzeroBOverEquiv
    {K : Type*} [Field K] (B : K) (hB : B ≠ 0) :
    NonzeroField K ≃ NonzeroField K where
  toFun x := ⟨B / x.1, div_ne_zero hB x.2⟩
  invFun x := ⟨B / x.1, div_ne_zero hB x.2⟩
  left_inv x := by
    apply Subtype.ext
    field_simp
  right_inv x := by
    apply Subtype.ext
    field_simp

/-- The character sum of `x(x³+Ax+B)` over nonzero `x`. -/
def quarticCharacterSum
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (A B : K) : ℤ :=
  ∑ x : NonzeroField K,
    quadraticChar K (x.1 * (x.1 ^ 3 + A * x.1 + B))

/-- Inverting `x` and scaling by `B` turns the nonzero quartic sum into the
nonzero part of the transformed cubic sum. -/
theorem quarticCharacterSum_eq_transformedCubic_sub_one
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (A B : K) (hB : B ≠ 0) :
    quarticCharacterSum A B =
      shortCubicCharacterSum A 0 (B ^ 2) - 1 := by
  let e := nonzeroBOverEquiv B hB
  have hterm : ∀ x : NonzeroField K,
      quadraticChar K
          ((e x).1 ^ 3 + A * (e x).1 ^ 2 + 0 * (e x).1 + B ^ 2) =
        quadraticChar K (x.1 * (x.1 ^ 3 + A * x.1 + B)) := by
    intro x
    have hfactor : B / x.1 ^ 2 ≠ 0 := div_ne_zero hB (pow_ne_zero 2 x.2)
    have harg :
        (e x).1 ^ 3 + A * (e x).1 ^ 2 + 0 * (e x).1 + B ^ 2 =
          (B / x.1 ^ 2) ^ 2 * (x.1 * (x.1 ^ 3 + A * x.1 + B)) := by
      dsimp [e, nonzeroBOverEquiv]
      field_simp [x.2]
      ring
    rw [harg, map_mul, quadraticChar_sq_one' hfactor, one_mul]
  have hreindex :
      (∑ x : NonzeroField K,
        quadraticChar K
          (x.1 ^ 3 + A * x.1 ^ 2 + 0 * x.1 + B ^ 2)) =
        quarticCharacterSum A B := by
    rw [← Equiv.sum_comp e]
    exact Fintype.sum_congr _ _ hterm
  have hsplit := Fintype.sum_eq_add_sum_subtype_ne
    (fun x : K => quadraticChar K (x ^ 3 + A * x ^ 2 + 0 * x + B ^ 2)) 0
  rw [shortCubicCharacterSum]
  rw [hsplit]
  have hzero :
      quadraticChar K ((0 : K) ^ 3 + A * 0 ^ 2 + 0 * 0 + B ^ 2) = 1 := by
    rw [show (0 : K) ^ 3 + A * 0 ^ 2 + 0 * 0 + B ^ 2 = B ^ 2 by ring]
    exact quadraticChar_sq_one' hB
  have hreindex' :
      (∑ x : NonzeroField K,
        quadraticChar K (x.1 ^ 3 + A * x.1 ^ 2 + 0 * x.1 + B ^ 2)) =
          quarticCharacterSum A B := by
    exact hreindex
  rw [hzero, hreindex']
  omega

/-- Atlas Hasse supplies the quartic-sum square bound after the explicit
inversion/scaling reduction. -/
theorem quarticCharacterSum_add_one_sq_le
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (hK : ringChar K ≠ 2) (A B : K)
    (hOriginal : (shortCharacterCurve 0 A B).Δ ≠ 0) (hB : B ≠ 0) :
    (quarticCharacterSum A B + 1) ^ 2 ≤ 4 * (Fintype.card K : ℤ) := by
  let _ : (shortCharacterCurve A 0 (B ^ 2)).IsElliptic :=
    transformedShortCurveIsElliptic A B hOriginal hB
  rw [quarticCharacterSum_eq_transformedCubic_sub_one A B hB]
  convert shortCubicCharacterSum_sq_le hK A 0 (B ^ 2) using 1 <;> ring

/-- The complete quadratic sum used by the `B = 0` squarefree-part case. -/
def quadraticTranslateCharacterSum
    {K : Type*} [Field K] [Fintype K] [DecidableEq K] (A : K) : ℤ :=
  ∑ x : K, quadraticChar K (x ^ 2 + A)

/-- Factoring `Y²-X²=A` as `(Y-X)(Y+X)=A` parametrizes all its solutions by
one nonzero field element. -/
def quadraticEquationSolutionEquivNonzero
    {K : Type*} [Field K] (hK : ringChar K ≠ 2) (A : K) (hA : A ≠ 0) :
    {xy : K × K // xy.2 ^ 2 = xy.1 ^ 2 + A} ≃ NonzeroField K where
  toFun xy := ⟨xy.1.2 - xy.1.1, by
    intro hzero
    have hyx : xy.1.2 = xy.1.1 := sub_eq_zero.mp hzero
    have heq := xy.2
    rw [hyx] at heq
    apply hA
    linear_combination -heq⟩
  invFun u :=
    ⟨((A / u.1 - u.1) / 2, (A / u.1 + u.1) / 2), by
      field_simp [Ring.two_ne_zero hK, u.2]
      ring⟩
  left_inv xy := by
    apply Subtype.ext
    apply Prod.ext
    · have hu : xy.1.2 - xy.1.1 ≠ 0 := by
        intro hzero
        have hyx : xy.1.2 = xy.1.1 := sub_eq_zero.mp hzero
        have heq := xy.2
        rw [hyx] at heq
        apply hA
        linear_combination -heq
      dsimp
      field_simp [Ring.two_ne_zero hK, hu]
      have heq := xy.2
      ring_nf
      rw [heq]
      ring
    · have hu : xy.1.2 - xy.1.1 ≠ 0 := by
        intro hzero
        have hyx : xy.1.2 = xy.1.1 := sub_eq_zero.mp hzero
        have heq := xy.2
        rw [hyx] at heq
        apply hA
        linear_combination -heq
      dsimp
      field_simp [Ring.two_ne_zero hK, hu]
      have heq := xy.2
      ring_nf
      rw [heq]
      ring
  right_inv u := by
    apply Subtype.ext
    dsimp
    field_simp [Ring.two_ne_zero hK, u.2]
    ring

/-- Counting `Y²=X²+A` by `X`-fibres gives `|K|` plus the complete
quadratic character sum. -/
theorem quadraticEquation_card_eq_card_add_characterSum
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (hK : ringChar K ≠ 2) (A : K) :
    (Fintype.card {xy : K × K // xy.2 ^ 2 = xy.1 ^ 2 + A} : ℤ) =
      Fintype.card K + quadraticTranslateCharacterSum A := by
  let e : {xy : K × K // xy.2 ^ 2 = xy.1 ^ 2 + A} ≃
      Σ x : K, {y : K // y ^ 2 = x ^ 2 + A} :=
    { toFun := fun xy => ⟨xy.1.1, xy.1.2, xy.2⟩
      invFun := fun xy => ⟨(xy.1, xy.2.1), xy.2.2⟩
      left_inv := fun _ => rfl
      right_inv := fun _ => rfl }
  rw [Fintype.card_congr e, Fintype.card_sigma]
  push_cast
  calc
    ∑ x : K, (Fintype.card {y : K // y ^ 2 = x ^ 2 + A} : ℤ) =
        ∑ x : K, (quadraticChar K (x ^ 2 + A) + 1) := by
      apply sum_congr rfl
      intro x _
      calc
        (Fintype.card {y : K // y ^ 2 = x ^ 2 + A} : ℤ) =
            ((({y : K | y ^ 2 = x ^ 2 + A} : Set K).toFinset.card : ℕ) : ℤ) := by
              rw [Set.toFinset_card]
              exact_mod_cast Fintype.card_congr
                (Equiv.refl {y : K // y ^ 2 = x ^ 2 + A})
        _ = quadraticChar K (x ^ 2 + A) + 1 :=
          quadraticChar_card_sqrts hK (x ^ 2 + A)
    _ = Fintype.card K + quadraticTranslateCharacterSum A := by
      simp [quadraticTranslateCharacterSum, sum_add_distrib, add_comm]

/-- The classical complete quadratic-character identity
`Σₓ χ(x²+A) = -1` for `A ≠ 0`, proved here by the explicit hyperbola
parametrization rather than assumed as a Weil estimate. -/
theorem quadraticTranslateCharacterSum_eq_neg_one
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (hK : ringChar K ≠ 2) (A : K) (hA : A ≠ 0) :
    quadraticTranslateCharacterSum A = -1 := by
  have hsolutions := quadraticEquation_card_eq_card_add_characterSum hK A
  have hequivCard :
      Fintype.card {xy : K × K // xy.2 ^ 2 = xy.1 ^ 2 + A} =
        Fintype.card (NonzeroField K) :=
    Fintype.card_congr (quadraticEquationSolutionEquivNonzero hK A hA)
  have hnonzeroCard :
      Fintype.card K = Fintype.card (NonzeroField K) + 1 := by
    calc
      Fintype.card K = Fintype.card Kˣ + 1 :=
        Fintype.card_eq_card_units_add_one K
      _ = Fintype.card (NonzeroField K) + 1 := by
        rw [Fintype.card_congr unitsEquivNeZero]
  rw [hequivCard] at hsolutions
  have hnonzeroCardInt :
      (Fintype.card K : ℤ) = Fintype.card (NonzeroField K) + 1 := by
    exact_mod_cast hnonzeroCard
  rw [hnonzeroCardInt] at hsolutions
  linarith

/-- When `B=0`, the nonzero quartic sum is the complete quadratic sum with
its `x=0` term removed. -/
theorem quarticCharacterSum_zero_eq
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (hK : ringChar K ≠ 2) (A : K) (hA : A ≠ 0) :
    quarticCharacterSum A 0 = -1 - quadraticChar K A := by
  have hterm : ∀ x : NonzeroField K,
      quadraticChar K (x.1 * (x.1 ^ 3 + A * x.1 + 0)) =
        quadraticChar K (x.1 ^ 2 + A) := by
    intro x
    rw [show x.1 * (x.1 ^ 3 + A * x.1 + 0) =
      x.1 ^ 2 * (x.1 ^ 2 + A) by ring]
    rw [map_mul, quadraticChar_sq_one' x.2, one_mul]
  have hquartic : quarticCharacterSum A 0 =
      ∑ x : NonzeroField K, quadraticChar K (x.1 ^ 2 + A) := by
    rw [quarticCharacterSum]
    exact Fintype.sum_congr _ _ hterm
  have hsplit := Fintype.sum_eq_add_sum_subtype_ne
    (fun x : K => quadraticChar K (x ^ 2 + A)) 0
  have hcomplete := quadraticTranslateCharacterSum_eq_neg_one hK A hA
  rw [quadraticTranslateCharacterSum, hsplit] at hcomplete
  have hzero : quadraticChar K ((0 : K) ^ 2 + A) = quadraticChar K A := by
    congr 1
    ring
  rw [hzero] at hcomplete
  rw [hquartic]
  linarith

/-- The exceptional `B=0` quartic sum has an elementary constant square
bound, stronger than the Hasse-scale bound needed by the first moment. -/
theorem quarticCharacterSum_zero_add_one_sq_le
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (hK : ringChar K ≠ 2) (A : K) (hA : A ≠ 0) :
    (quarticCharacterSum A 0 + 1) ^ 2 ≤ 4 * (Fintype.card K : ℤ) := by
  rw [quarticCharacterSum_zero_eq hK A hA]
  have hchar : quadraticChar K A ^ 2 = 1 := quadraticChar_sq_one hA
  have hcard : (1 : ℤ) ≤ 4 * Fintype.card K := by
    have : 0 < Fintype.card K := Fintype.card_pos
    exact_mod_cast (show 1 ≤ 4 * Fintype.card K by omega)
  nlinarith

/-- Unified quartic character-sum bound for every nonsingular short curve in
odd characteristic. The `B ≠ 0` branch is Hasse on an explicit transformed
elliptic curve; the `B = 0` branch is the elementary hyperbola count above. -/
theorem quarticCharacterSum_add_one_sq_le_of_nonsingular
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (hK : ringChar K ≠ 2) (A B : K)
    (hOriginal : (shortCharacterCurve 0 A B).Δ ≠ 0) :
    (quarticCharacterSum A B + 1) ^ 2 ≤ 4 * (Fintype.card K : ℤ) := by
  by_cases hB : B = 0
  · subst B
    have hA : A ≠ 0 := by
      intro hAzero
      subst A
      apply hOriginal
      rw [originalShortCurve_discriminant]
      ring
    exact quarticCharacterSum_zero_add_one_sq_le hK A hA
  · exact quarticCharacterSum_add_one_sq_le hK A B hOriginal hB

/-- Pushing an integer-valued function through the squaring map weights each
field element by `1 + χ`. This is the exact finite identity behind the
two-to-one square-class substitution in the dilation average. -/
theorem sum_square_eq_characterWeighted
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (hK : ringChar K ≠ 2) (value : K → ℤ) :
    (∑ u : K, value (u ^ 2)) =
      ∑ s : K, (quadraticChar K s + 1) * value s := by
  rw [← Fintype.sum_fiberwise' (fun u : K => u ^ 2) value]
  apply Fintype.sum_congr
  intro s
  have hcard :
      (Fintype.card {u : K // u ^ 2 = s} : ℤ) = quadraticChar K s + 1 := by
    calc
      (Fintype.card {u : K // u ^ 2 = s} : ℤ) =
          ((({u : K | u ^ 2 = s} : Set K).toFinset.card : ℕ) : ℤ) := by
            rw [Set.toFinset_card]
            exact_mod_cast Fintype.card_congr (Equiv.refl {u : K // u ^ 2 = s})
      _ = quadraticChar K s + 1 := quadraticChar_card_sqrts hK s
  simp only [Finset.sum_const, nsmul_eq_mul]
  rw [Finset.card_univ]
  exact congrArg (fun count : ℤ => count * value s) hcard

/-- Inversion is a permutation of the nonzero field elements. -/
def nonzeroInvEquiv
    {K : Type*} [Field K] : NonzeroField K ≃ NonzeroField K where
  toFun x := ⟨x.1⁻¹, inv_ne_zero x.2⟩
  invFun x := ⟨x.1⁻¹, inv_ne_zero x.2⟩
  left_inv x := by
    apply Subtype.ext
    exact inv_inv x.1
  right_inv x := by
    apply Subtype.ext
    exact inv_inv x.1

/-- Removing the zero term and reindexing by inversion gives the exact
nonzero inverse-square identity used by the dilation family. -/
theorem sum_nonzero_inverseSquare_eq_characterWeighted
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (hK : ringChar K ≠ 2) (value : K → ℤ) :
    (∑ u : NonzeroField K, value (u.1⁻¹ ^ 2)) =
      ∑ s : NonzeroField K,
        (quadraticChar K s.1 + 1) * value s.1 := by
  have hfull := sum_square_eq_characterWeighted hK value
  have hleft := Fintype.sum_eq_add_sum_subtype_ne
    (fun u : K => value (u ^ 2)) 0
  have hright := Fintype.sum_eq_add_sum_subtype_ne
    (fun s : K => (quadraticChar K s + 1) * value s) 0
  have hzero : (quadraticChar K (0 : K) + 1) * value 0 = value 0 := by
    rw [quadraticChar_zero]
    ring
  rw [hleft, hright, hzero] at hfull
  have hsquare :
      (∑ u : NonzeroField K, value (u.1 ^ 2)) =
        ∑ s : NonzeroField K,
          (quadraticChar K s.1 + 1) * value s.1 := by
    simpa using hfull
  let e : NonzeroField K ≃ NonzeroField K := nonzeroInvEquiv
  calc
    (∑ u : NonzeroField K, value (u.1⁻¹ ^ 2)) =
        ∑ u : NonzeroField K, value ((e u).1 ^ 2) := by rfl
    _ = ∑ u : NonzeroField K, value (u.1 ^ 2) :=
      Equiv.sum_comp e (fun u : NonzeroField K => value (u.1 ^ 2))
    _ = _ := hsquare

/-- The first complete cubic sum with its zero argument removed. -/
def cubicNonzeroCharacterSum
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (A B : K) : ℤ :=
  ∑ x : NonzeroField K, quadraticChar K (x.1 ^ 3 + A * x.1 + B)

/-- Removing the zero argument from the cubic sum costs exactly `χ(B)`. -/
theorem shortCubicCharacterSum_eq_zero_add_nonzero
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (A B : K) :
    shortCubicCharacterSum 0 A B =
      quadraticChar K B + cubicNonzeroCharacterSum A B := by
  have hsplit := Fintype.sum_eq_add_sum_subtype_ne
    (fun x : K => quadraticChar K (x ^ 3 + 0 * x ^ 2 + A * x + B)) 0
  rw [shortCubicCharacterSum, hsplit, cubicNonzeroCharacterSum]
  congr 1
  · congr 1
    ring
  · apply Fintype.sum_congr
    intro x
    congr 1
    ring

/-- Atlas Hasse in the exact shifted form needed after excluding `x=0`. -/
theorem cubicNonzeroCharacterSum_add_zero_sq_le
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (hK : ringChar K ≠ 2) (A B : K)
    [(shortCharacterCurve 0 A B).IsElliptic] :
    (cubicNonzeroCharacterSum A B + quadraticChar K B) ^ 2 ≤
      4 * (Fintype.card K : ℤ) := by
  rw [add_comm, ← shortCubicCharacterSum_eq_zero_add_nonzero A B]
  exact shortCubicCharacterSum_sq_le hK 0 A B

/-- Multiplication by a nonzero scalar permutes the nonzero field elements. -/
def nonzeroScaleEquiv
    {K : Type*} [Field K] (t : K) (ht : t ≠ 0) :
    NonzeroField K ≃ NonzeroField K where
  toFun x := ⟨t * x.1, mul_ne_zero ht x.2⟩
  invFun x := ⟨x.1 / t, div_ne_zero x.2 ht⟩
  left_inv x := by
    apply Subtype.ext
    dsimp
    field_simp [ht]
  right_inv x := by
    apply Subtype.ext
    dsimp
    field_simp [ht]

/-- Total quadratic-character contribution of one nonzero abscissa across all
inverse-square dilations. -/
def dilationCharacterTotal
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (A B t : K) : ℤ :=
  ∑ u : NonzeroField K,
    quadraticChar K
      ((t * u.1⁻¹ ^ 2) ^ 3 + A * (t * u.1⁻¹ ^ 2) + B)

/-- Exact source identity for the first moment: the dilation total at `t` is
the nonzero cubic sum plus `χ(t)` times the nonzero quartic sum. -/
theorem dilationCharacterTotal_eq_cubic_add_quartic
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (hK : ringChar K ≠ 2) (A B t : K) (ht : t ≠ 0) :
    dilationCharacterTotal A B t =
      cubicNonzeroCharacterSum A B +
        quadraticChar K t * quarticCharacterSum A B := by
  let value : K → ℤ := fun s =>
    quadraticChar K ((t * s) ^ 3 + A * (t * s) + B)
  have hsquare := sum_nonzero_inverseSquare_eq_characterWeighted hK value
  have hdilation : dilationCharacterTotal A B t =
      ∑ s : NonzeroField K,
        (quadraticChar K s.1 + 1) * value s.1 := by
    rw [dilationCharacterTotal]
    exact hsquare
  have hcubic : (∑ s : NonzeroField K, value s.1) =
      cubicNonzeroCharacterSum A B := by
    let e := nonzeroScaleEquiv t ht
    simpa [value, cubicNonzeroCharacterSum, e, nonzeroScaleEquiv] using
      (Equiv.sum_comp e
        (fun x : NonzeroField K =>
          quadraticChar K (x.1 ^ 3 + A * x.1 + B)))
  have hweightedTerm : ∀ s : NonzeroField K,
      quadraticChar K s.1 * value s.1 =
        quadraticChar K t *
          quadraticChar K
            ((t * s.1) * ((t * s.1) ^ 3 + A * (t * s.1) + B)) := by
    intro s
    dsimp [value]
    rw [map_mul, map_mul]
    have htchar : quadraticChar K t ^ 2 = 1 := quadraticChar_sq_one ht
    calc
      quadraticChar K s.1 *
          quadraticChar K ((t * s.1) ^ 3 + A * (t * s.1) + B) =
        1 * (quadraticChar K s.1 *
          quadraticChar K ((t * s.1) ^ 3 + A * (t * s.1) + B)) := by ring
      _ = quadraticChar K t ^ 2 * (quadraticChar K s.1 *
          quadraticChar K ((t * s.1) ^ 3 + A * (t * s.1) + B)) := by
        rw [htchar]
      _ = quadraticChar K t *
          (quadraticChar K t * quadraticChar K s.1 *
            quadraticChar K ((t * s.1) ^ 3 + A * (t * s.1) + B)) := by ring
  have hquarticReindex :
      (∑ s : NonzeroField K,
        quadraticChar K
          ((t * s.1) * ((t * s.1) ^ 3 + A * (t * s.1) + B))) =
        quarticCharacterSum A B := by
    let e := nonzeroScaleEquiv t ht
    simpa [quarticCharacterSum, e, nonzeroScaleEquiv] using
      (Equiv.sum_comp e
        (fun x : NonzeroField K =>
          quadraticChar K (x.1 * (x.1 ^ 3 + A * x.1 + B))))
  rw [hdilation]
  simp_rw [add_mul, one_mul]
  rw [sum_add_distrib, hcubic, add_comm]
  calc
    cubicNonzeroCharacterSum A B +
        ∑ s : NonzeroField K, quadraticChar K s.1 * value s.1 =
      cubicNonzeroCharacterSum A B +
        ∑ s : NonzeroField K,
          quadraticChar K t * quadraticChar K
            ((t * s.1) * ((t * s.1) ^ 3 + A * (t * s.1) + B)) := by
        congr 1
        exact Fintype.sum_congr _ _ hweightedTerm
    _ = cubicNonzeroCharacterSum A B +
        quadraticChar K t * quarticCharacterSum A B := by
      rw [← Finset.mul_sum, hquarticReindex]

/-- An integral square bound against a nonnegative radius gives the usual
two-sided interval bound, without introducing real square roots. -/
theorem intBounds_of_sq_le_sq (value radius : ℤ) (radiusNonnegative : 0 ≤ radius)
    (squareBound : value ^ 2 ≤ radius ^ 2) :
    -radius ≤ value ∧ value ≤ radius := by
  constructor <;> nlinarith [sq_nonneg (value - radius), sq_nonneg (value + radius)]

/-- The finite-field quadratic character always lies in `[-1,1]`. -/
theorem quadraticChar_bounds
    {K : Type*} [Field K] [Fintype K] [DecidableEq K] (x : K) :
    (-1 : ℤ) ≤ quadraticChar K x ∧ quadraticChar K x ≤ 1 := by
  by_cases hx : x = 0
  · subst x
    rw [quadraticChar_zero]
    omega
  · rcases quadraticChar_dichotomy hx with h | h <;> rw [h] <;> omega

/-- A radius dominating `2√|K|` gives an explicit lower bound for the exact
character contribution of every nonzero abscissa across all dilations. -/
theorem dilationCharacterTotal_lower
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (hK : ringChar K ≠ 2) (A B t : K) (ht : t ≠ 0)
    (hOriginal : (shortCharacterCurve 0 A B).Δ ≠ 0)
    (radius : ℤ) (radiusNonnegative : 0 ≤ radius)
    (radiusSquare : 4 * (Fintype.card K : ℤ) ≤ radius ^ 2) :
    -(2 * radius + 2) ≤ dilationCharacterTotal A B t := by
  let _ : (shortCharacterCurve 0 A B).IsElliptic :=
    ⟨isUnit_iff_ne_zero.mpr hOriginal⟩
  have hcubicSq := cubicNonzeroCharacterSum_add_zero_sq_le hK A B
  have hquarticSq :=
    quarticCharacterSum_add_one_sq_le_of_nonsingular hK A B hOriginal
  have hcubicBounds := intBounds_of_sq_le_sq
    (cubicNonzeroCharacterSum A B + quadraticChar K B) radius
    radiusNonnegative (hcubicSq.trans radiusSquare)
  have hquarticBounds := intBounds_of_sq_le_sq
    (quarticCharacterSum A B + 1) radius radiusNonnegative
    (hquarticSq.trans radiusSquare)
  have hBchar := quadraticChar_bounds (K := K) B
  have hcubicLower : -radius - 1 ≤ cubicNonzeroCharacterSum A B := by
    linarith
  have hweightedLower :
      -radius - 1 ≤ quadraticChar K t * quarticCharacterSum A B := by
    rcases quadraticChar_dichotomy ht with htchar | htchar
    · rw [htchar, one_mul]
      linarith
    · rw [htchar, neg_one_mul]
      linarith
  rw [dilationCharacterTotal_eq_cubic_add_quartic hK A B t ht]
  linarith

/-- The actual number of `Y`-coordinates above one dilated abscissa. -/
def dilationFiberCard
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (A B t : K) (u : NonzeroField K) : ℕ :=
  Fintype.card {y : K // y ^ 2 =
    (t * u.1⁻¹ ^ 2) ^ 3 + A * (t * u.1⁻¹ ^ 2) + B}

/-- The signed factor-base size on one dilation, counted as the sum of all
`Y`-fibres above the supplied abscissa set. -/
def dilationFactorBaseSize
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (A B : K) (T : Finset K) (u : NonzeroField K) : ℕ :=
  ∑ t ∈ T, dilationFiberCard A B t u

/-- Each concrete `Y`-fibre is exactly `1 + χ(rhs)`. -/
theorem dilationFiberCard_cast
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (hK : ringChar K ≠ 2) (A B t : K) (u : NonzeroField K) :
    (dilationFiberCard A B t u : ℤ) =
      quadraticChar K
        ((t * u.1⁻¹ ^ 2) ^ 3 + A * (t * u.1⁻¹ ^ 2) + B) + 1 := by
  rw [dilationFiberCard]
  calc
    (Fintype.card {y : K // y ^ 2 =
        (t * u.1⁻¹ ^ 2) ^ 3 + A * (t * u.1⁻¹ ^ 2) + B} : ℤ) =
        ((({y : K | y ^ 2 =
          (t * u.1⁻¹ ^ 2) ^ 3 + A * (t * u.1⁻¹ ^ 2) + B} : Set K).toFinset.card : ℕ) : ℤ) := by
      rw [Set.toFinset_card]
      exact_mod_cast Fintype.card_congr
        (Equiv.refl {y : K // y ^ 2 =
          (t * u.1⁻¹ ^ 2) ^ 3 + A * (t * u.1⁻¹ ^ 2) + B})
    _ = _ := quadraticChar_card_sqrts hK
      ((t * u.1⁻¹ ^ 2) ^ 3 + A * (t * u.1⁻¹ ^ 2) + B)

/-- Integer form of one concrete factor-base size: one main term per
abscissa plus the character contributions. -/
theorem dilationFactorBaseSize_cast
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (hK : ringChar K ≠ 2) (A B : K) (T : Finset K)
    (u : NonzeroField K) :
    (dilationFactorBaseSize A B T u : ℤ) =
      T.card + ∑ t ∈ T, quadraticChar K
        ((t * u.1⁻¹ ^ 2) ^ 3 + A * (t * u.1⁻¹ ^ 2) + B) := by
  rw [dilationFactorBaseSize]
  push_cast
  simp_rw [dilationFiberCard_cast hK]
  rw [Finset.sum_add_distrib]
  simp
  ring

/-- Exact total of the concrete factor-base sizes across every nonzero
dilation. -/
theorem dilationFactorBaseSize_total
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (hK : ringChar K ≠ 2) (A B : K) (T : Finset K) :
    (∑ u : NonzeroField K, (dilationFactorBaseSize A B T u : ℤ)) =
      Fintype.card (NonzeroField K) * T.card +
        ∑ t ∈ T, dilationCharacterTotal A B t := by
  simp_rw [dilationFactorBaseSize_cast hK A B T]
  rw [Finset.sum_add_distrib]
  simp only [Finset.sum_const, nsmul_eq_mul]
  rw [Finset.sum_comm]
  rfl

/-- Summing the per-abscissa character bound gives a lower bound for the
actual natural-valued factor-base total. -/
theorem dilationFactorBaseSize_total_lower
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (hK : ringChar K ≠ 2) (A B : K) (T : Finset K)
    (TNonzero : ∀ t ∈ T, t ≠ 0)
    (hOriginal : (shortCharacterCurve 0 A B).Δ ≠ 0)
    (radius : ℤ) (radiusNonnegative : 0 ≤ radius)
    (radiusSquare : 4 * (Fintype.card K : ℤ) ≤ radius ^ 2) :
    (T.card : ℤ) *
        ((Fintype.card K : ℤ) - 1 - (2 * radius + 2)) ≤
      ∑ u : NonzeroField K, (dilationFactorBaseSize A B T u : ℤ) := by
  have hcharacters :
      (∑ t ∈ T, (-(2 * radius + 2) : ℤ)) ≤
        ∑ t ∈ T, dilationCharacterTotal A B t := by
    apply Finset.sum_le_sum
    intro t ht
    exact dilationCharacterTotal_lower hK A B t (TNonzero t ht)
      hOriginal radius radiusNonnegative radiusSquare
  have hcard :
      (Fintype.card K : ℤ) = Fintype.card (NonzeroField K) + 1 := by
    calc
      (Fintype.card K : ℤ) = Fintype.card Kˣ + 1 := by
        exact_mod_cast Fintype.card_eq_card_units_add_one K
      _ = Fintype.card (NonzeroField K) + 1 := by
        norm_cast
        rw [Fintype.card_congr unitsEquivNeZero]
  rw [dilationFactorBaseSize_total hK A B T]
  simp only [Finset.sum_const, nsmul_eq_mul] at hcharacters
  rw [hcard]
  nlinarith

/-- The exact `3/4` first-moment inequality consumed by
`firstMomentForcesLargeValueSeeds`, under a division-free radius condition. -/
theorem dilationFactorBaseFirstMoment
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (hK : ringChar K ≠ 2) (A B : K) (T : Finset K)
    (TNonzero : ∀ t ∈ T, t ≠ 0)
    (hOriginal : (shortCharacterCurve 0 A B).Δ ≠ 0)
    (radius : ℕ)
    (radiusSquare : 4 * Fintype.card K ≤ radius ^ 2)
    (density : 4 * (2 * radius + 2) ≤ Fintype.card K - 1) :
    3 * T.card * Fintype.card (NonzeroField K) ≤
      4 * ∑ u : NonzeroField K, dilationFactorBaseSize A B T u := by
  have hlower := dilationFactorBaseSize_total_lower hK A B T TNonzero
    hOriginal (radius : ℤ) (by positivity) (by exact_mod_cast radiusSquare)
  have hcardNat :
      Fintype.card K = Fintype.card (NonzeroField K) + 1 := by
    calc
      Fintype.card K = Fintype.card Kˣ + 1 :=
        Fintype.card_eq_card_units_add_one K
      _ = Fintype.card (NonzeroField K) + 1 := by
        rw [Fintype.card_congr unitsEquivNeZero]
  have hdensityInt :
      4 * (2 * (radius : ℤ) + 2) ≤ (Fintype.card K : ℤ) - 1 := by
    have hle : 4 * (2 * radius + 2) ≤ Fintype.card K :=
      density.trans (Nat.sub_le _ _)
    have hpositive : 0 < Fintype.card K := Fintype.card_pos
    omega
  have htargetInt :
      3 * (T.card : ℤ) * Fintype.card (NonzeroField K) ≤
        4 * ∑ u : NonzeroField K, (dilationFactorBaseSize A B T u : ℤ) := by
    have hTnonnegative : (0 : ℤ) ≤ T.card := by positivity
    have hnonzeroCardInt :
        (Fintype.card K : ℤ) - 1 = Fintype.card (NonzeroField K) := by
      have hcardCast :
          (Fintype.card K : ℤ) = Fintype.card (NonzeroField K) + 1 := by
        exact_mod_cast hcardNat
      omega
    rw [← hnonzeroCardInt]
    nlinarith
  exact_mod_cast htargetInt

/-- Every quadratic `Y`-fibre has at most two points. -/
theorem dilationFiberCard_le_two
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (hK : ringChar K ≠ 2) (A B t : K) (u : NonzeroField K) :
    dilationFiberCard A B t u ≤ 2 := by
  have hcast := dilationFiberCard_cast hK A B t u
  have hchar := quadraticChar_bounds (K := K)
    ((t * u.1⁻¹ ^ 2) ^ 3 + A * (t * u.1⁻¹ ^ 2) + B)
  exact_mod_cast (show (dilationFiberCard A B t u : ℤ) ≤ 2 by linarith)

/-- Consequently a factor base over `X` supplied abscissas has at most `2X`
signed points on every dilation. -/
theorem dilationFactorBaseSize_le_twice
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (hK : ringChar K ≠ 2) (A B : K) (T : Finset K)
    (u : NonzeroField K) :
    dilationFactorBaseSize A B T u ≤ 2 * T.card := by
  rw [dilationFactorBaseSize]
  calc
    ∑ t ∈ T, dilationFiberCard A B t u ≤ ∑ _t ∈ T, 2 := by
      exact Finset.sum_le_sum fun t _ => dilationFiberCard_le_two hK A B t u
    _ = 2 * T.card := by simp [Nat.mul_comm]

/-! ## First-moment coverage repair

The following finite counting lemmas isolate the arithmetic in the corrected
fixed-arity coverage argument.  The preceding declarations now discharge the
curve-specific character-sum estimate under explicit radius hypotheses; the
total-energy estimate remains an input to the eventual specialization.
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

/-- Concrete first-moment coverage event for the actual finite-field
factor-base fibres. Atlas Hasse and the elementary `B=0` conic argument supply
the mean bound; no character-sum premise remains. -/
theorem dilationLargeFactorBaseSeeds
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (hK : ringChar K ≠ 2) (A B : K) (T : Finset K)
    (TPositive : 0 < T.card) (TNonzero : ∀ t ∈ T, t ≠ 0)
    (hOriginal : (shortCharacterCurve 0 A B).Δ ≠ 0)
    (radius : ℕ)
    (radiusSquare : 4 * Fintype.card K ≤ radius ^ 2)
    (density : 4 * (2 * radius + 2) ≤ Fintype.card K - 1) :
    Fintype.card (NonzeroField K) ≤
      6 * (Finset.univ.filter fun u : NonzeroField K =>
        T.card ≤ 2 * dilationFactorBaseSize A B T u).card := by
  apply firstMomentForcesLargeValueSeeds
    (fun u : NonzeroField K => dilationFactorBaseSize A B T u) T.card
    TPositive
  · exact dilationFactorBaseSize_le_twice hK A B T
  · exact dilationFactorBaseFirstMoment hK A B T TNonzero hOriginal
      radius radiusSquare density

/-- Twice the successor of the natural square root is an integral radius
whose square dominates `4q`. -/
theorem twiceSuccSqrt_radiusSquare (q : ℕ) :
    4 * q ≤ (2 * (Nat.sqrt q + 1)) ^ 2 := by
  have hsqrt := Nat.succ_le_succ_sqrt' q
  nlinarith

/-- From `q ≥ 324`, the same explicit radius satisfies the density inequality
required by the `3/4` first moment. -/
theorem twiceSuccSqrt_density (q : ℕ) (fieldLarge : 324 ≤ q) :
    4 * (2 * (2 * (Nat.sqrt q + 1)) + 2) ≤ q - 1 := by
  let s := Nat.sqrt q
  have hsLower : 18 ≤ s := by
    rw [Nat.le_sqrt']
    exact fieldLarge
  have hsSquare : s ^ 2 ≤ q := Nat.sqrt_le' q
  have hleft : (0 : ℤ) ≤ (s : ℤ) - 18 := by omega
  have hright : (0 : ℤ) ≤ (s : ℤ) + 2 := by positivity
  have hproduct := mul_nonneg hleft hright
  have htargetInt : (16 : ℤ) * s + 25 ≤ q := by
    have hsSquareInt : (s : ℤ) ^ 2 ≤ q := by
      exact_mod_cast hsSquare
    nlinarith
  have htargetNat : 16 * s + 25 ≤ q := by
    exact_mod_cast htargetInt
  dsimp [s] at htargetNat
  omega

/-- Clean field-size specialization of the concrete first-moment event. -/
theorem dilationLargeFactorBaseSeeds_of_card_ge_324
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (hK : ringChar K ≠ 2) (A B : K) (T : Finset K)
    (TPositive : 0 < T.card) (TNonzero : ∀ t ∈ T, t ≠ 0)
    (hOriginal : (shortCharacterCurve 0 A B).Δ ≠ 0)
    (fieldLarge : 324 ≤ Fintype.card K) :
    Fintype.card (NonzeroField K) ≤
      6 * (Finset.univ.filter fun u : NonzeroField K =>
        T.card ≤ 2 * dilationFactorBaseSize A B T u).card := by
  apply dilationLargeFactorBaseSeeds hK A B T TPositive TNonzero hOriginal
    (2 * (Nat.sqrt (Fintype.card K) + 1))
  · exact twiceSuccSqrt_radiusSquare (Fintype.card K)
  · exact twiceSuccSqrt_density (Fintype.card K) fieldLarge

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

/-- Concrete one-twelfth coverage endpoint for the dilated short-curve factor
bases.  For odd finite fields of cardinality at least `324`, the character-sum
and first-moment inputs have been discharged.  The only remaining premise is
the displayed total-energy estimate for the caller-supplied natural-valued
energy function. -/
theorem dilationAndEnergyGiveOneTwelfthGoodSeeds_of_card_ge_324
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (hK : ringChar K ≠ 2) (A B : K) (T : Finset K)
    (TPositive : 0 < T.card) (TNonzero : ∀ t ∈ T, t ≠ 0)
    (hOriginal : (shortCharacterCurve 0 A B).Δ ≠ 0)
    (fieldLarge : 324 ≤ Fintype.card K)
    (energy : NonzeroField K → ℕ) (meanBound : ℕ)
    (meanPositive : 0 < meanBound)
    (energyTotalUpper :
      (∑ u, energy u) ≤ meanBound * Fintype.card (NonzeroField K)) :
    Fintype.card (NonzeroField K) ≤ 12 *
      ((Finset.univ.filter fun u : NonzeroField K =>
          T.card ≤ 2 * dilationFactorBaseSize A B T u) \
        (Finset.univ.filter fun u : NonzeroField K =>
          12 * meanBound ≤ energy u)).card := by
  apply oneSixthEventSurvivesOneTwelfthBadSet
  · exact dilationLargeFactorBaseSeeds_of_card_ge_324 hK A B T TPositive
      TNonzero hOriginal fieldLarge
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

/-! ## Cubic scaling symmetry for the Semaev/Kummer step -/

/-- A base-ring element that becomes a square in its fraction field is a unit
times a square in the base unique-factorization domain. -/
theorem unitMulSquare_of_isSquare_algebraMap_fractionRing
    {A Fraction : Type*} [CommRing A] [IsDomain A]
    [UniqueFactorizationMonoid A] [Field Fraction]
    [Algebra A Fraction] [IsFractionRing A Fraction]
    (value : A) (squareInFraction : IsSquare (algebraMap A Fraction value)) :
    ∃ unitFactor square : A,
      IsUnit unitFactor ∧ value = unitFactor * square ^ 2 := by
  obtain ⟨fractionSquare, squareInFraction⟩ := squareInFraction
  let numerator : A := IsFractionRing.num A fractionSquare
  let denominator : nonZeroDivisors A := IsFractionRing.den A fractionSquare
  have fractionSquareEq :
      IsLocalization.mk' Fraction numerator denominator = fractionSquare :=
    IsFractionRing.mk'_num_den A fractionSquare
  have fractionRepresentation :
      algebraMap A Fraction value =
        IsLocalization.mk' Fraction (numerator ^ 2) (denominator ^ 2) := by
    calc
      algebraMap A Fraction value = fractionSquare * fractionSquare := squareInFraction
      _ = IsLocalization.mk' Fraction numerator denominator *
          IsLocalization.mk' Fraction numerator denominator := by rw [fractionSquareEq]
      _ = IsLocalization.mk' Fraction (numerator * numerator)
          (denominator * denominator) :=
        (IsLocalization.mk'_mul (S := Fraction) numerator numerator denominator denominator).symm
      _ = IsLocalization.mk' Fraction (numerator ^ 2) (denominator ^ 2) := by
        simp only [pow_two]
  have clearedInFraction :
      algebraMap A Fraction
          (value * ((denominator ^ 2 : nonZeroDivisors A) : A)) =
        algebraMap A Fraction (numerator ^ 2) := by
    have := IsLocalization.eq_mk'_iff_mul_eq.mp fractionRepresentation
    simpa only [map_mul] using this
  have clearedProduct :
      value * ((denominator ^ 2 : nonZeroDivisors A) : A) = numerator ^ 2 :=
    (IsFractionRing.injective A Fraction) clearedInFraction
  have cleared : value * (denominator : A) ^ 2 = numerator ^ 2 := by
    simpa [pow_two] using clearedProduct
  have denominatorDvdNumeratorSquare : (denominator : A) ∣ numerator ^ 2 := by
    rw [← cleared]
    exact (dvd_pow_self (denominator : A) two_ne_zero).mul_left value
  have denominatorUnit : IsUnit (denominator : A) :=
    (IsFractionRing.num_den_reduced A fractionSquare).pow_left
      denominatorDvdNumeratorSquare dvd_rfl
  have denominatorSquareUnit : IsUnit ((denominator : A) ^ 2) :=
    denominatorUnit.pow 2
  obtain ⟨inverseDenominatorSquare, inverseMul⟩ :=
    isUnit_iff_exists_inv'.mp denominatorSquareUnit
  have inverseUnit : IsUnit inverseDenominatorSquare :=
    IsUnit.of_mul_eq_one ((denominator : A) ^ 2) inverseMul
  refine ⟨inverseDenominatorSquare, numerator, inverseUnit, ?_⟩
  calc
    value = 1 * value := by rw [one_mul]
    _ = (inverseDenominatorSquare * (denominator : A) ^ 2) * value := by
      rw [inverseMul]
    _ = inverseDenominatorSquare * (value * (denominator : A) ^ 2) := by ring
    _ = inverseDenominatorSquare * numerator ^ 2 := by rw [cleared]

/-- Polynomial specialization of the fraction-ring square-class conversion:
a polynomial square in the rational function field is a nonzero constant
times a polynomial square. -/
theorem polynomial_constantMulSquare_of_isSquare_fractionRing
    {K : Type*} [Field K] (polynomial : Polynomial K)
    (squareInFraction :
      IsSquare (algebraMap (Polynomial K) (FractionRing (Polynomial K)) polynomial)) :
    ∃ constant : K, ∃ square : Polynomial K,
      constant ≠ 0 ∧ polynomial = Polynomial.C constant * square ^ 2 := by
  obtain ⟨unitFactor, square, unitFactorUnit, polynomialEq⟩ :=
    unitMulSquare_of_isSquare_algebraMap_fractionRing polynomial squareInFraction
  obtain ⟨constant, constantUnit, constantEq⟩ :=
    Polynomial.isUnit_iff.mp unitFactorUnit
  refine ⟨constant, square, isUnit_iff_ne_zero.mp constantUnit, ?_⟩
  simpa [constantEq] using polynomialEq

/-- The monic short cubic `X³ + A X + B` used in Lemma 5.4. -/
noncomputable def shortCubicPolynomial
    {K : Type*} [CommRing K] (A B : K) : Polynomial K :=
  Polynomial.X ^ 3 + Polynomial.C A * Polynomial.X + Polynomial.C B

/-- The short cubic is monic of degree three. -/
theorem shortCubicPolynomial_monic
    {K : Type*} [Field K] (A B : K) :
    (shortCubicPolynomial A B).Monic := by
  rw [Polynomial.Monic]
  simpa [shortCubicPolynomial] using
    (Polynomial.leadingCoeff_cubic (R := K) (a := (1 : K)) (b := 0) (c := A) (d := B)
      one_ne_zero)

/-- The short cubic has natural degree three. -/
theorem shortCubicPolynomial_natDegree
    {K : Type*} [Field K] (A B : K) :
    (shortCubicPolynomial A B).natDegree = 3 := by
  simpa [shortCubicPolynomial] using
    (Polynomial.natDegree_cubic (R := K) (a := (1 : K)) (b := 0) (c := A) (d := B)
      one_ne_zero)

/-- A squarefree polynomial on the left of a product that is a unit times a
square divides the other factor.  This is the divisibility step behind the
root-set argument in Lemma 5.4, without passing to a splitting field. -/
theorem squarefreePolynomial_dvd_right_of_mul_eq_unit_mul_square
    {K : Type*} [Field K]
    {left right coefficient square : Polynomial K}
    (leftSquarefree : Squarefree left) (coefficientUnit : IsUnit coefficient)
    (relation : left * right = coefficient * square ^ 2) :
    left ∣ right := by
  have leftDvdSquareSquared : left ∣ square ^ 2 := by
    apply coefficientUnit.dvd_mul_left.mp
    rw [← relation]
    exact dvd_mul_right left right
  have leftDvdSquare : left ∣ square :=
    (leftSquarefree.dvd_pow_iff_dvd two_ne_zero).mp leftDvdSquareSquared
  obtain ⟨factor, squareEq⟩ := leftDvdSquare
  refine ⟨coefficient * factor ^ 2, ?_⟩
  apply mul_left_cancel₀ leftSquarefree.ne_zero
  calc
    left * right = coefficient * square ^ 2 := relation
    _ = left * (left * (coefficient * factor ^ 2)) := by rw [squareEq]; ring

/-- Nonsingularity makes the short cubic squarefree.  The proof uses the
explicit Bezout identity

`(-18AX + 27B)ℓ + (6AX² - 9BX + 4A²)ℓ' = 4A³ + 27B²`.
-/
theorem shortCubicPolynomial_squarefree
    {K : Type*} [Field K] (A B : K)
    (nonsingular : (4 : K) * A ^ 3 + 27 * B ^ 2 ≠ 0) :
    Squarefree (shortCubicPolynomial A B) := by
  apply Polynomial.Separable.squarefree
  rw [Polynomial.separable_def]
  let discriminant : K := 4 * A ^ 3 + 27 * B ^ 2
  let leftCoefficient : Polynomial K :=
    Polynomial.C (-18 * A) * Polynomial.X + Polynomial.C (27 * B)
  let rightCoefficient : Polynomial K :=
    Polynomial.C (6 * A) * Polynomial.X ^ 2 +
      Polynomial.C (-9 * B) * Polynomial.X + Polynomial.C (4 * A ^ 2)
  have bezout :
      leftCoefficient * shortCubicPolynomial A B +
          rightCoefficient * (shortCubicPolynomial A B).derivative =
        Polynomial.C discriminant := by
    dsimp [leftCoefficient, rightCoefficient, discriminant, shortCubicPolynomial]
    simp only [Polynomial.derivative_add, Polynomial.derivative_mul,
      Polynomial.derivative_pow, Polynomial.derivative_X, Polynomial.derivative_C,
      zero_mul, add_zero, zero_add, mul_one, map_ofNat,
      Polynomial.C_eq_natCast, Polynomial.C_neg,
      Polynomial.C_add, Polynomial.C_mul, Polynomial.C_pow]
    ring
  refine ⟨Polynomial.C discriminant⁻¹ * leftCoefficient,
    Polynomial.C discriminant⁻¹ * rightCoefficient, ?_⟩
  calc
    (Polynomial.C discriminant⁻¹ * leftCoefficient) * shortCubicPolynomial A B +
          (Polynomial.C discriminant⁻¹ * rightCoefficient) *
            (shortCubicPolynomial A B).derivative =
        Polynomial.C discriminant⁻¹ *
          (leftCoefficient * shortCubicPolynomial A B +
            rightCoefficient * (shortCubicPolynomial A B).derivative) := by ring
    _ = Polynomial.C discriminant⁻¹ * Polynomial.C discriminant := by rw [bezout]
    _ = 1 := by
      rw [← Polynomial.C_mul]
      simp [discriminant, nonsingular]

/-- Expanding the short cubic after the scaling `X ↦ rX`. -/
theorem shortCubicPolynomial_comp_scale
    {K : Type*} [CommRing K] (A B r : K) :
    (shortCubicPolynomial A B).comp (Polynomial.C r * Polynomial.X) =
      Polynomial.C (r ^ 3) * Polynomial.X ^ 3 +
        Polynomial.C (A * r) * Polynomial.X + Polynomial.C B := by
  simp only [shortCubicPolynomial, Polynomial.add_comp, Polynomial.mul_comp,
    Polynomial.pow_comp, Polynomial.X_comp, Polynomial.C_comp, map_mul,
    map_pow]
  ring

/-- The missing square-product implication in Lemma 5.4.  For a nonsingular
short cubic and nonzero scale, if the product of the cubic and its scaled
copy is a nonzero constant times a polynomial square, then the scaled copy is
exactly `r³` times the original cubic. -/
theorem shortCubicPolynomial_scale_of_constant_mul_square
    {K : Type*} [Field K] (A B r constant : K)
    (nonsingular : (4 : K) * A ^ 3 + 27 * B ^ 2 ≠ 0)
    (rNonzero : r ≠ 0) (constantNonzero : constant ≠ 0)
    (squareRelation : ∃ square : Polynomial K,
      shortCubicPolynomial A B *
          (shortCubicPolynomial A B).comp (Polynomial.C r * Polynomial.X) =
        Polynomial.C constant * square ^ 2) :
    (shortCubicPolynomial A B).comp (Polynomial.C r * Polynomial.X) =
      Polynomial.C (r ^ 3) * shortCubicPolynomial A B := by
  obtain ⟨square, squareRelation⟩ := squareRelation
  have divides : shortCubicPolynomial A B ∣
      (shortCubicPolynomial A B).comp (Polynomial.C r * Polynomial.X) :=
    squarefreePolynomial_dvd_right_of_mul_eq_unit_mul_square
      (shortCubicPolynomial_squarefree A B nonsingular)
      (Polynomial.isUnit_C.mpr (isUnit_iff_ne_zero.mpr constantNonzero))
      squareRelation
  have scaledDegree :
      ((shortCubicPolynomial A B).comp
        (Polynomial.C r * Polynomial.X)).natDegree = 3 := by
    rw [shortCubicPolynomial_comp_scale]
    simpa [add_assoc] using
      (Polynomial.natDegree_cubic (R := K) (a := r ^ 3) (b := 0)
        (c := A * r) (d := B) (pow_ne_zero 3 rNonzero))
  have degreeLe :
      ((shortCubicPolynomial A B).comp
          (Polynomial.C r * Polynomial.X)).natDegree ≤
        (shortCubicPolynomial A B).natDegree := by
    rw [scaledDegree, shortCubicPolynomial_natDegree]
  have scaledLeadingCoefficient :
      ((shortCubicPolynomial A B).comp
        (Polynomial.C r * Polynomial.X)).leadingCoeff = r ^ 3 := by
    rw [shortCubicPolynomial_comp_scale]
    simpa [add_assoc] using
      (Polynomial.leadingCoeff_cubic (R := K) (a := r ^ 3) (b := 0)
        (c := A * r) (d := B) (pow_ne_zero 3 rNonzero))
  have exactAssociate :=
    Polynomial.eq_leadingCoeff_mul_of_monic_of_dvd_of_natDegree_le
      (shortCubicPolynomial_monic A B) divides degreeLe
  simpa [scaledLeadingCoefficient] using exactAssociate

/-- Expanding the scalar multiple `r³ (X³ + AX + B)`. -/
theorem shortCubicPolynomial_scale_mul
    {K : Type*} [CommRing K] (A B r : K) :
    Polynomial.C (r ^ 3) * shortCubicPolynomial A B =
      Polynomial.C (r ^ 3) * Polynomial.X ^ 3 +
        Polynomial.C (A * r ^ 3) * Polynomial.X +
          Polynomial.C (B * r ^ 3) := by
  simp only [shortCubicPolynomial, mul_add, Polynomial.C_mul]
  ring

/-- Coefficient form of the cubic scaling symmetry in Lemma 5.4. -/
theorem shortCubicPolynomial_scale_eq_iff
    {K : Type*} [CommRing K] (A B r : K) :
    (shortCubicPolynomial A B).comp (Polynomial.C r * Polynomial.X) =
        Polynomial.C (r ^ 3) * shortCubicPolynomial A B ↔
      A * r = A * r ^ 3 ∧ B = B * r ^ 3 := by
  rw [shortCubicPolynomial_comp_scale, shortCubicPolynomial_scale_mul]
  constructor
  · intro equality
    constructor
    · have coefficient := congrArg (fun p : Polynomial K => p.coeff 1) equality
      simp only [Polynomial.coeff_add,
        Polynomial.coeff_C_mul_X_pow, Polynomial.coeff_C_mul_X,
        Polynomial.coeff_C] at coefficient
      norm_num at coefficient
      exact coefficient
    · have coefficient := congrArg (fun p : Polynomial K => p.coeff 0) equality
      simp only [Polynomial.coeff_add,
        Polynomial.coeff_C_mul_X_pow, Polynomial.coeff_C_mul_X,
        Polynomial.coeff_C] at coefficient
      norm_num at coefficient
      exact coefficient
  · rintro ⟨hA, hB⟩
    congr 1
    · congr 1
      exact congrArg (fun coefficient => Polynomial.C coefficient * Polynomial.X) hA
    · exact congrArg Polynomial.C hB

/-- Classification of nontrivial scaling symmetries of a nonsingular short
cubic, in the coefficient form used by Lemma 5.4.  If `A ≠ 0`, a nonidentity
nonzero symmetry forces `r = -1` and `B = 0`; if `A = 0` and `B ≠ 0`, it
forces `r³ = 1`. -/
theorem shortCubicPolynomial_scale_classification
    {K : Type*} [Field K] (hK : ringChar K ≠ 2)
    (A B r : K) (rNonzero : r ≠ 0) (rNonidentity : r ≠ 1)
    (symmetry :
      (shortCubicPolynomial A B).comp (Polynomial.C r * Polynomial.X) =
        Polynomial.C (r ^ 3) * shortCubicPolynomial A B) :
    (A ≠ 0 → B = 0 ∧ r = -1) ∧ (A = 0 → B ≠ 0 → r ^ 3 = 1) := by
  obtain ⟨hAcoeff, hBcoeff⟩ :=
    (shortCubicPolynomial_scale_eq_iff A B r).mp symmetry
  constructor
  · intro ANonzero
    have rEqCube : r = r ^ 3 := by
      exact mul_left_cancel₀ ANonzero hAcoeff
    have oneEqSquare : 1 = r ^ 2 := by
      apply mul_left_cancel₀ rNonzero
      calc
        r * 1 = r := mul_one r
        _ = r ^ 3 := rEqCube
        _ = r * r ^ 2 := by ring
    have factorZero : (r - 1) * (r + 1) = 0 := by
      calc
        (r - 1) * (r + 1) = r ^ 2 - 1 := by ring
        _ = 0 := by rw [← oneEqSquare, sub_self]
    have rNegOne : r = -1 := by
      rcases mul_eq_zero.mp factorZero with minusZero | plusZero
      · exact absurd (sub_eq_zero.mp minusZero) rNonidentity
      · exact eq_neg_of_add_eq_zero_left plusZero
    refine ⟨?_, rNegOne⟩
    rw [rNegOne] at hBcoeff
    norm_num at hBcoeff
    have twiceB : (2 : K) * B = 0 := by
      linear_combination hBcoeff
    exact (mul_eq_zero.mp twiceB).resolve_left (Ring.two_ne_zero hK)
  · intro _ BNonzero
    have oneEqCube : 1 = r ^ 3 := by
      apply mul_left_cancel₀ BNonzero
      simpa using hBcoeff
    exact oneEqCube.symm

/-- Lemma 5.4's forward classification starting from the actual
constant-times-a-square hypothesis, rather than an assumed scaling symmetry. -/
theorem shortCubicPolynomial_square_product_classification
    {K : Type*} [Field K] (hK : ringChar K ≠ 2)
    (A B r constant : K)
    (nonsingular : (4 : K) * A ^ 3 + 27 * B ^ 2 ≠ 0)
    (rNonzero : r ≠ 0) (rNonidentity : r ≠ 1)
    (constantNonzero : constant ≠ 0)
    (squareRelation : ∃ square : Polynomial K,
      shortCubicPolynomial A B *
          (shortCubicPolynomial A B).comp (Polynomial.C r * Polynomial.X) =
        Polynomial.C constant * square ^ 2) :
    (A ≠ 0 → B = 0 ∧ r = -1) ∧ (A = 0 → B ≠ 0 → r ^ 3 = 1) := by
  apply shortCubicPolynomial_scale_classification hK A B r rNonzero rNonidentity
  exact shortCubicPolynomial_scale_of_constant_mul_square A B r constant
    nonsingular rNonzero constantNonzero squareRelation

/-- Lemma 5.4's forward classification from a square in the actual rational
function field.  The fraction-ring conversion produces the nonzero constant
and polynomial square required by
`shortCubicPolynomial_square_product_classification`. -/
theorem shortCubicPolynomial_fraction_square_product_classification
    {K : Type*} [Field K] (hK : ringChar K ≠ 2)
    (A B r : K)
    (nonsingular : (4 : K) * A ^ 3 + 27 * B ^ 2 ≠ 0)
    (rNonzero : r ≠ 0) (rNonidentity : r ≠ 1)
    (squareInFunctionField :
      IsSquare (algebraMap (Polynomial K) (FractionRing (Polynomial K))
        (shortCubicPolynomial A B *
          (shortCubicPolynomial A B).comp (Polynomial.C r * Polynomial.X)))) :
    (A ≠ 0 → B = 0 ∧ r = -1) ∧ (A = 0 → B ≠ 0 → r ^ 3 = 1) := by
  obtain ⟨constant, square, constantNonzero, squareRelation⟩ :=
    polynomial_constantMulSquare_of_isSquare_fractionRing _ squareInFunctionField
  exact shortCubicPolynomial_square_product_classification hK A B r constant
    nonsingular rNonzero rNonidentity constantNonzero ⟨square, squareRelation⟩

/-- Two independently scaled copies of a nonsingular short cubic reduce to
the one-parameter square-product classification after normalizing by the
left scale.  This is the function-field form needed for the radicands
`ℓ(s * tᵢ)` and `ℓ(s * tⱼ)` in Lemma N'. -/
theorem shortCubicPolynomial_scaled_pair_fraction_square_classification
    {K : Type*} [Field K] (hK : ringChar K ≠ 2)
    (A B left right : K)
    (nonsingular : (4 : K) * A ^ 3 + 27 * B ^ 2 ≠ 0)
    (leftNonzero : left ≠ 0) (rightNonzero : right ≠ 0)
    (distinct : left ≠ right)
    (squareInFunctionField :
      IsSquare (algebraMap (Polynomial K) (FractionRing (Polynomial K))
        ((shortCubicPolynomial A B).comp (Polynomial.C left * Polynomial.X) *
          (shortCubicPolynomial A B).comp
            (Polynomial.C right * Polynomial.X)))) :
    (A ≠ 0 → B = 0 ∧ right / left = -1) ∧
      (A = 0 → B ≠ 0 → (right / left) ^ 3 = 1) := by
  let normalizedA : K := A / left ^ 2
  let normalizedB : K := B / left ^ 3
  let ratio : K := right / left
  have leftScale :
      (shortCubicPolynomial A B).comp (Polynomial.C left * Polynomial.X) =
        Polynomial.C (left ^ 3) *
          shortCubicPolynomial normalizedA normalizedB := by
    rw [shortCubicPolynomial_comp_scale,
      shortCubicPolynomial_scale_mul]
    dsimp [normalizedA, normalizedB]
    congr 1 <;> field_simp
  have rightScale :
      (shortCubicPolynomial A B).comp (Polynomial.C right * Polynomial.X) =
        Polynomial.C (left ^ 3) *
          (shortCubicPolynomial normalizedA normalizedB).comp
            (Polynomial.C ratio * Polynomial.X) := by
    have cubeCoefficient : Polynomial.C (right ^ 3) =
        Polynomial.C (left ^ 3) * Polynomial.C (ratio ^ 3) := by
      rw [← Polynomial.C_mul]
      congr 1
      dsimp [ratio]
      field_simp
    have linearCoefficient : Polynomial.C (A * right) =
        Polynomial.C (left ^ 3) *
          Polynomial.C (normalizedA * ratio) := by
      rw [← Polynomial.C_mul]
      congr 1
      dsimp [normalizedA, ratio]
      field_simp
    have constantCoefficient : Polynomial.C B =
        Polynomial.C (left ^ 3) * Polynomial.C normalizedB := by
      rw [← Polynomial.C_mul]
      congr 1
      dsimp [normalizedB]
      field_simp
    rw [shortCubicPolynomial_comp_scale,
      shortCubicPolynomial_comp_scale]
    rw [cubeCoefficient, linearCoefficient, constantCoefficient]
    ring
  obtain ⟨constant, square, constantNonzero, squareRelation⟩ :=
    polynomial_constantMulSquare_of_isSquare_fractionRing _
      squareInFunctionField
  have leftSixNonzero : left ^ 6 ≠ 0 := pow_ne_zero 6 leftNonzero
  have normalizedSquareRelation :
      shortCubicPolynomial normalizedA normalizedB *
          (shortCubicPolynomial normalizedA normalizedB).comp
            (Polynomial.C ratio * Polynomial.X) =
        Polynomial.C ((left ^ 6)⁻¹ * constant) * square ^ 2 := by
    apply mul_left_cancel₀ (Polynomial.C_ne_zero.mpr leftSixNonzero)
    calc
      Polynomial.C (left ^ 6) *
          (shortCubicPolynomial normalizedA normalizedB *
            (shortCubicPolynomial normalizedA normalizedB).comp
              (Polynomial.C ratio * Polynomial.X)) =
        (shortCubicPolynomial A B).comp
            (Polynomial.C left * Polynomial.X) *
          (shortCubicPolynomial A B).comp
            (Polynomial.C right * Polynomial.X) := by
              rw [leftScale, rightScale]
              have constantPower : Polynomial.C (left ^ 6) =
                  Polynomial.C (left ^ 3) * Polynomial.C (left ^ 3) := by
                rw [← Polynomial.C_mul]
                congr 1
                ring
              rw [constantPower]
              ring
      _ = Polynomial.C constant * square ^ 2 := squareRelation
      _ = Polynomial.C
          (left ^ 6 * ((left ^ 6)⁻¹ * constant)) * square ^ 2 := by
            have scalarCancel :
                left ^ 6 * ((left ^ 6)⁻¹ * constant) = constant := by
              field_simp
            rw [scalarCancel]
      _ = Polynomial.C (left ^ 6) *
          (Polynomial.C ((left ^ 6)⁻¹ * constant) * square ^ 2) := by
            rw [Polynomial.C_mul]
            ring
  have normalizedNonsingular :
      (4 : K) * normalizedA ^ 3 + 27 * normalizedB ^ 2 ≠ 0 := by
    intro normalizedZero
    apply nonsingular
    dsimp [normalizedA, normalizedB] at normalizedZero
    field_simp at normalizedZero
    simpa using normalizedZero
  have ratioNonzero : ratio ≠ 0 := div_ne_zero rightNonzero leftNonzero
  have ratioNonidentity : ratio ≠ 1 := by
    intro ratioOne
    apply distinct
    exact ((div_eq_one_iff_eq leftNonzero).mp ratioOne).symm
  have classified := shortCubicPolynomial_square_product_classification hK
    normalizedA normalizedB ratio ((left ^ 6)⁻¹ * constant)
    normalizedNonsingular ratioNonzero ratioNonidentity
    (mul_ne_zero (inv_ne_zero leftSixNonzero) constantNonzero)
    ⟨square, normalizedSquareRelation⟩
  dsimp [normalizedA, normalizedB, ratio] at classified ⊢
  constructor
  · intro ANonzero
    obtain ⟨normalizedBZero, ratioNegOne⟩ :=
      classified.1 (div_ne_zero ANonzero (pow_ne_zero 2 leftNonzero))
    exact ⟨(div_eq_zero_iff.mp normalizedBZero).resolve_right
      (pow_ne_zero 3 leftNonzero), ratioNegOne⟩
  · intro AZero BNonzero
    exact classified.2 (by simp [AZero])
      (div_ne_zero BNonzero (pow_ne_zero 3 leftNonzero))

/-- In the `A ≠ 0` branch, the two scaled short-cubic radicands have
nonsquare product in the rational function field as soon as their scale ratio
is not `-1`. -/
theorem shortCubicPolynomial_scaled_pair_not_isSquare_of_A_ne_zero
    {K : Type*} [Field K] (hK : ringChar K ≠ 2)
    (A B left right : K)
    (nonsingular : (4 : K) * A ^ 3 + 27 * B ^ 2 ≠ 0)
    (ANonzero : A ≠ 0) (leftNonzero : left ≠ 0)
    (rightNonzero : right ≠ 0) (distinct : left ≠ right)
    (ratioNotNegOne : right / left ≠ -1) :
    ¬ IsSquare (algebraMap (Polynomial K) (FractionRing (Polynomial K))
      ((shortCubicPolynomial A B).comp (Polynomial.C left * Polynomial.X) *
        (shortCubicPolynomial A B).comp
          (Polynomial.C right * Polynomial.X))) := by
  intro squareInFunctionField
  exact ratioNotNegOne
    ((shortCubicPolynomial_scaled_pair_fraction_square_classification hK
      A B left right nonsingular leftNonzero rightNonzero distinct
      squareInFunctionField).1 ANonzero).2

/-- The short-cubic radicand `ℓ(tX)` as an element of the rational
function field. -/
noncomputable def shortCubicFunctionRadicand
    {K : Type*} [Field K] (A B scale : K) :
    FractionRing (Polynomial K) :=
  algebraMap (Polynomial K) (FractionRing (Polynomial K))
    ((shortCubicPolynomial A B).comp
      (Polynomial.C scale * Polynomial.X))

/-- A nonzero scale makes the cubic function-field radicand nonzero. -/
theorem shortCubicFunctionRadicand_ne_zero
    {K : Type*} [Field K] (A B scale : K) (scaleNonzero : scale ≠ 0) :
    shortCubicFunctionRadicand A B scale ≠ 0 := by
  intro radicandZero
  have polynomialZero :
      (shortCubicPolynomial A B).comp
        (Polynomial.C scale * Polynomial.X) = 0 := by
    apply IsFractionRing.injective (Polynomial K)
      (FractionRing (Polynomial K))
    simpa only [shortCubicFunctionRadicand, map_zero] using radicandZero
  have leadingCoefficient := congrArg
    (fun polynomial : Polynomial K ↦ polynomial.coeff 3) polynomialZero
  rw [shortCubicPolynomial_comp_scale] at leadingCoefficient
  simp only [Polynomial.coeff_add, Polynomial.coeff_C_mul_X_pow,
    Polynomial.coeff_C_mul_X, Polynomial.coeff_C, Polynomial.coeff_zero]
    at leadingCoefficient
  norm_num at leadingCoefficient
  exact scaleNonzero leadingCoefficient

/-- On an elliptic short curve, the displayed square-root equation already
supplies the nonsingularity proof required by the affine point constructor. -/
theorem shortCharacterCurve_nonsingular_of_sq
    {K : Type*} [Field K] (A B x y : K)
    [(shortCharacterCurve 0 A B).IsElliptic]
    (square : y ^ 2 = x ^ 3 + A * x + B) :
    (shortCharacterCurve 0 A B).toAffine.Nonsingular x y := by
  apply (WeierstrassCurve.Affine.equation_iff_nonsingular).mp
  exact (shortCharacterCurve_equation_iff 0 A B x y).mpr (by
    simpa using square)

/-- Function-field packaging of the `A ≠ 0` scaled-radicand
nonsquareness result. -/
theorem shortCubicFunctionRadicand_mul_not_isSquare_of_A_ne_zero
    {K : Type*} [Field K] (hK : ringChar K ≠ 2)
    (A B left right : K)
    (nonsingular : (4 : K) * A ^ 3 + 27 * B ^ 2 ≠ 0)
    (ANonzero : A ≠ 0) (leftNonzero : left ≠ 0)
    (rightNonzero : right ≠ 0) (distinct : left ≠ right)
    (ratioNotNegOne : right / left ≠ -1) :
    ¬ IsSquare
      (shortCubicFunctionRadicand A B left *
        shortCubicFunctionRadicand A B right) := by
  simpa only [shortCubicFunctionRadicand, ← map_mul] using
    shortCubicPolynomial_scaled_pair_not_isSquare_of_A_ne_zero
      hK A B left right nonsingular ANonzero leftNonzero rightNonzero
        distinct ratioNotNegOne

/-- Two positive natural representatives whose sum is below the modulus
cannot have quotient `-1` in the prime residue field. -/
theorem zmodPositivePairRatio_ne_neg_one
    (modulus left right : ℕ) [Fact modulus.Prime]
    (leftPositive : 0 < left) (rightPositive : 0 < right)
    (sumBelow : left + right < modulus) :
    (right : ZMod modulus) / (left : ZMod modulus) ≠ -1 := by
  have leftBelow : left < modulus := by omega
  have rightBelow : right < modulus := by omega
  have leftNonzero : (left : ZMod modulus) ≠ 0 := by
    intro leftZero
    have values := congrArg ZMod.val leftZero
    rw [ZMod.val_natCast_of_lt leftBelow, ZMod.val_zero] at values
    omega
  have rightNonzero : (right : ZMod modulus) ≠ 0 := by
    intro rightZero
    have values := congrArg ZMod.val rightZero
    rw [ZMod.val_natCast_of_lt rightBelow, ZMod.val_zero] at values
    omega
  intro ratioNegOne
  have rightEq : (right : ZMod modulus) = -(left : ZMod modulus) := by
    have multiplied := (div_eq_iff leftNonzero).mp ratioNegOne
    simpa using multiplied
  have sumZero : ((left + right : ℕ) : ZMod modulus) = 0 := by
    rw [Nat.cast_add, rightEq, add_neg_cancel]
  have values := congrArg ZMod.val sumZero
  rw [ZMod.val_natCast_of_lt sumBelow, ZMod.val_zero] at values
  omega

/-- Prime-field interval specialization of the `A ≠ 0` radicand
nonsquareness theorem.  Positive representatives with sum below the modulus
automatically exclude the only exceptional scale ratio. -/
theorem zmodShortCubicPolynomial_scaled_pair_not_isSquare_of_A_ne_zero
    (modulus left right : ℕ) [Fact modulus.Prime]
    (hChar : ringChar (ZMod modulus) ≠ 2)
    (A B : ZMod modulus)
    (nonsingular : (4 : ZMod modulus) * A ^ 3 + 27 * B ^ 2 ≠ 0)
    (ANonzero : A ≠ 0)
    (leftPositive : 0 < left) (rightPositive : 0 < right)
    (distinct : left ≠ right) (sumBelow : left + right < modulus) :
    ¬ IsSquare
      (algebraMap (Polynomial (ZMod modulus))
        (FractionRing (Polynomial (ZMod modulus)))
        ((shortCubicPolynomial A B).comp
            (Polynomial.C (left : ZMod modulus) * Polynomial.X) *
          (shortCubicPolynomial A B).comp
            (Polynomial.C (right : ZMod modulus) * Polynomial.X))) := by
  have leftBelow : left < modulus := by omega
  have rightBelow : right < modulus := by omega
  have leftNonzero : (left : ZMod modulus) ≠ 0 := by
    intro leftZero
    have values := congrArg ZMod.val leftZero
    rw [ZMod.val_natCast_of_lt leftBelow, ZMod.val_zero] at values
    omega
  have rightNonzero : (right : ZMod modulus) ≠ 0 := by
    intro rightZero
    have values := congrArg ZMod.val rightZero
    rw [ZMod.val_natCast_of_lt rightBelow, ZMod.val_zero] at values
    omega
  have castDistinct : (left : ZMod modulus) ≠ (right : ZMod modulus) := by
    intro castEqual
    have values := congrArg ZMod.val castEqual
    rw [ZMod.val_natCast_of_lt leftBelow,
      ZMod.val_natCast_of_lt rightBelow] at values
    exact distinct values
  exact shortCubicPolynomial_scaled_pair_not_isSquare_of_A_ne_zero
    hChar A B (left : ZMod modulus) (right : ZMod modulus)
    nonsingular ANonzero leftNonzero rightNonzero castDistinct
    (zmodPositivePairRatio_ne_neg_one modulus left right
      leftPositive rightPositive sumBelow)

/-- Indexed prime-field interval radicands satisfy exactly the pairwise
nonsquare premise of the quadratic splitting-field projector in the
`A ≠ 0` branch. -/
theorem zmodShortCubicFunctionRadicands_pairwise_not_isSquare_of_A_ne_zero
    {Index : Type*} [Fintype Index]
    (modulus : ℕ) [Fact modulus.Prime]
    (hChar : ringChar (ZMod modulus) ≠ 2)
    (A B : ZMod modulus)
    (nonsingular : (4 : ZMod modulus) * A ^ 3 + 27 * B ^ 2 ≠ 0)
    (ANonzero : A ≠ 0) (scale : Index → ℕ)
    (scalePositive : ∀ index, 0 < scale index)
    (scaleInjective : Function.Injective scale)
    (pairSumBelow : ∀ left right, left ≠ right →
      scale left + scale right < modulus) :
    ∀ left right, left ≠ right →
      ¬ IsSquare
        (shortCubicFunctionRadicand A B (scale left : ZMod modulus) *
          shortCubicFunctionRadicand A B (scale right : ZMod modulus)) := by
  intro left right distinct
  simpa only [shortCubicFunctionRadicand, ← map_mul] using
    zmodShortCubicPolynomial_scaled_pair_not_isSquare_of_A_ne_zero
      modulus (scale left) (scale right) hChar A B nonsingular ANonzero
      (scalePositive left) (scalePositive right)
      (fun equal ↦ distinct (scaleInjective equal))
      (pairSumBelow left right distinct)

/-- A positive triple of natural representatives with sum below the modulus
cannot sum to zero in `ZMod modulus`. -/
theorem zmodPositiveTripleSum_ne_zero
    (modulus a b c : ℕ) (aPositive : 0 < a)
    (sumBelow : a + b + c < modulus) :
    (a : ZMod modulus) + (b : ZMod modulus) + (c : ZMod modulus) ≠ 0 := by
  intro sumZero
  have castZero : ((a + b + c : ℕ) : ZMod modulus) = 0 := by
    simpa [Nat.cast_add] using sumZero
  have valZero := congrArg ZMod.val castZero
  rw [ZMod.val_natCast_of_lt sumBelow] at valZero
  simp only [ZMod.val_zero] at valZero
  omega

/-- The `A = 0` interval repair from Lemma N': a full orbit
`a, ωa, ω²a` cannot have three positive representatives at most `X` when
`3X < p`.  Hence every cubic-character collision class in the interval has
size at most two. -/
theorem zmodNoFullCubicOrbitInShortInterval
    (modulus X a b c : ℕ) (omega : ZMod modulus)
    (aPositive : 0 < a) (aBound : a ≤ X) (bBound : b ≤ X) (cBound : c ≤ X)
    (intervalShort : 3 * X < modulus)
    (omegaRelation : omega ^ 2 + omega + 1 = 0)
    (bOrbit : (b : ZMod modulus) = omega * (a : ZMod modulus))
    (cOrbit : (c : ZMod modulus) = omega * (b : ZMod modulus)) : False := by
  have sumBelow : a + b + c < modulus := by omega
  apply zmodPositiveTripleSum_ne_zero modulus a b c aPositive sumBelow
  rw [bOrbit, cOrbit, bOrbit]
  linear_combination (a : ZMod modulus) * omegaRelation

/-- Multiplying a transcendental element by a nonzero constant preserves
transcendence.  This is the corrected final contradiction in Lemma N': a
coordinate algebraic over the constant field cannot equal `t * s`. -/
theorem transcendental_const_mul
    {K L : Type*} [Field K] [Field L] [Algebra K L]
    (s : L) (sTranscendental : Transcendental K s)
    (t : K) (tNonzero : t ≠ 0) :
    Transcendental K (algebraMap K L t * s) := by
  let polynomial : Polynomial K := Polynomial.C t * Polynomial.X
  have degreeNonzero : polynomial.natDegree ≠ 0 := by
    simp [polynomial, Polynomial.natDegree_C_mul_X t tNonzero]
  have leadingNonzerodivisor :
      polynomial.leadingCoeff ∈ nonZeroDivisors K := by
    rw [mem_nonZeroDivisors_iff_ne_zero]
    simpa [polynomial, Polynomial.leadingCoeff_C_mul_X] using tNonzero
  simpa [polynomial] using
    sTranscendental.aeval polynomial degreeNonzero leadingNonzerodivisor

/-- An algebraic element cannot be a nonzero constant multiple of a
transcendental element. -/
theorem algebraic_ne_const_mul_transcendental
    {K L : Type*} [Field K] [Field L] [Algebra K L]
    (x s : L) (xAlgebraic : IsAlgebraic K x)
    (sTranscendental : Transcendental K s)
    (t : K) (tNonzero : t ≠ 0) :
    x ≠ algebraMap K L t * s := by
  intro equality
  have productTranscendental :=
    transcendental_const_mul s sTranscendental t tNonzero
  apply productTranscendental
  rw [← equality]
  exact xAlgebraic

/-- A root in an extension field of the auxiliary polynomial `preΨ` whose
index is nonzero in the constant field is algebraic over that constant
field. For even indices, `preΨ` omits the two-torsion factor; the parity-safe
torsion endpoint below therefore uses `ΨSq` instead. -/
theorem divisionPolynomialRoot_isAlgebraic
    {K L : Type*} [Field K] [Field L] [Algebra K L]
    (curve : WeierstrassCurve K) (index : ℤ) (x : L)
    (indexNonzero : (index : K) ≠ 0)
    (root : Polynomial.aeval x (curve.preΨ index) = 0) :
    IsAlgebraic K x := by
  exact ⟨curve.preΨ index, curve.preΨ_ne_zero indexNonzero, root⟩

/-- The auxiliary polynomial `preΨ` cannot encode general even torsion:
already at index two it is the constant polynomial one and has no root. -/
theorem preDivisionPolynomial_two_aeval_ne_zero
    {K L : Type*} [Field K] [Field L] [Algebra K L]
    (curve : WeierstrassCurve K) (x : L) :
    Polynomial.aeval x (curve.preΨ 2) ≠ 0 := by
  simp

/-- The parity-correct squared division polynomial does detect affine
two-torsion, in contrast to `preΨ`. -/
theorem divisionPolynomialSq_two_vanishes_of_add_self
    {K : Type*} [Field K] [DecidableEq K]
    (curve : WeierstrassCurve K) {x y : K}
    (nonsingular : curve.toAffine.Nonsingular x y)
    (torsion :
      WeierstrassCurve.Affine.Point.some x y nonsingular +
        WeierstrassCurve.Affine.Point.some x y nonsingular = 0) :
    Polynomial.eval x (curve.ΨSq 2) = 0 := by
  rw [curve.ΨSq_two, curve.Ψ₂Sq_eq]
  exact WeierstrassCurve.Affine.Point.isRoot_twoTorsionPolynomial_of_add_self
    nonsingular torsion

open WeierstrassCurve.Jacobian in
/-- A torsion relation at a nonsingular affine point forces the corresponding
full squared division polynomial to vanish at its x-coordinate.  This is the
parity-safe bridge: unlike `preΨ`, `ΨSq` retains the two-torsion factor. -/
theorem divisionPolynomialSq_eval_eq_zero_of_zsmul_eq_zero
    {F : Type*} [Field F] (curve : WeierstrassCurve F)
    {x y : F} (nonsingular : curve.toAffine.Nonsingular x y) (index : ℤ)
    (torsion : index • WeierstrassCurve.Jacobian.Point.fromAffine
      (WeierstrassCurve.Affine.Point.some _ _ nonsingular) = 0) :
    (curve.ΨSq index).eval x = 0 := by
  have pointFormula := WeierstrassCurve.zsmul_eq_smulEval
    curve nonsingular index
  rw [torsion, WeierstrassCurve.Jacobian.Point.zero_point] at pointFormula
  have equivalent : (![1, 1, 0] : Fin 3 → F) ≈
      WeierstrassCurve.smulEval curve x y index :=
    Quotient.eq.mp pointFormula
  have psiZero : Polynomial.evalEval x y (curve.ψ index) = 0 := by
    have zZero : (WeierstrassCurve.smulEval curve x y index) (2 : Fin 3) = 0 :=
      (WeierstrassCurve.Jacobian.Z_eq_zero_of_equiv equivalent).mp rfl
    simpa [WeierstrassCurve.smulEval] using zZero
  let evaluation : curve.toAffine.CoordinateRing →+* F :=
    AdjoinRoot.evalEval nonsingular.1
  have psiEqualsPsi :
      Polynomial.evalEval x y (curve.ψ index) =
        Polynomial.evalEval x y (curve.Ψ index) := by
    calc
      _ = evaluation (WeierstrassCurve.Affine.CoordinateRing.mk curve
          (curve.ψ index)) :=
        (AdjoinRoot.evalEval_mk nonsingular.1 _).symm
      _ = evaluation (WeierstrassCurve.Affine.CoordinateRing.mk curve
          (curve.Ψ index)) :=
        congrArg evaluation
          (WeierstrassCurve.Affine.CoordinateRing.mk_ψ curve index)
      _ = _ := AdjoinRoot.evalEval_mk nonsingular.1 _
  have fullPsiZero : Polynomial.evalEval x y (curve.Ψ index) = 0 :=
    psiEqualsPsi.symm.trans psiZero
  have squareEvaluation :
      Polynomial.evalEval x y (curve.Ψ index) ^ 2 =
        (curve.ΨSq index).eval x := by
    calc
      _ = (evaluation (WeierstrassCurve.Affine.CoordinateRing.mk curve
          (curve.Ψ index))) ^ 2 :=
        congrArg (fun value : F => value ^ 2)
          (AdjoinRoot.evalEval_mk nonsingular.1 _).symm
      _ = evaluation ((WeierstrassCurve.Affine.CoordinateRing.mk curve
          (curve.Ψ index)) ^ 2) := (map_pow evaluation _ 2).symm
      _ = evaluation (WeierstrassCurve.Affine.CoordinateRing.mk curve
          (Polynomial.C <| curve.ΨSq index)) :=
        congrArg evaluation
          (WeierstrassCurve.Affine.CoordinateRing.mk_Ψ_sq curve index)
      _ = Polynomial.evalEval x y (Polynomial.C <| curve.ΨSq index) :=
        AdjoinRoot.evalEval_mk nonsingular.1 _
      _ = _ := Polynomial.evalEval_C x y _
  rw [fullPsiZero] at squareEvaluation
  simpa using squareEvaluation.symm

/-- Affine form of the torsion-to-division-polynomial implication, obtained
by transporting the scalar relation through the Jacobian/affine additive
equivalence. -/
theorem divisionPolynomialSq_eval_eq_zero_of_affine_zsmul_eq_zero
    {F : Type*} [Field F] [DecidableEq F]
    (curve : WeierstrassCurve F) {x y : F}
    (nonsingular : curve.toAffine.Nonsingular x y) (index : ℤ)
    (torsion : index • WeierstrassCurve.Affine.Point.some
      x y nonsingular = 0) :
    (curve.ΨSq index).eval x = 0 := by
  apply divisionPolynomialSq_eval_eq_zero_of_zsmul_eq_zero
    curve nonsingular index
  let equivalence := WeierstrassCurve.Jacobian.Point.toAffineAddEquiv curve
  calc
    index • WeierstrassCurve.Jacobian.Point.fromAffine
        (WeierstrassCurve.Affine.Point.some _ _ nonsingular) =
        equivalence.symm
          (index • WeierstrassCurve.Affine.Point.some _ _ nonsingular) := by
            rw [map_zsmul]
            rfl
    _ = equivalence.symm 0 := congrArg equivalence.symm torsion
    _ = 0 := map_zero equivalence.symm

/-- A root of the full univariate squared division polynomial is algebraic.
Unlike `preΨ`, `ΨSq` retains the two-torsion factor at even indices, so this
is the polynomial appropriate for a general scalar-torsion implication. -/
theorem divisionPolynomialSqRoot_isAlgebraic
    {K L : Type*} [Field K] [Field L] [Algebra K L]
    (curve : WeierstrassCurve K) (index : ℤ) (x : L)
    (indexNonzero : (index : K) ≠ 0)
    (root : Polynomial.aeval x (curve.ΨSq index) = 0) :
    IsAlgebraic K x := by
  exact ⟨curve.ΨSq index, curve.ΨSq_ne_zero indexNonzero, root⟩

/-- Consequently, a division polynomial with nonzero index cannot vanish at
a nonzero constant multiple of a transcendental parameter. -/
theorem divisionPolynomial_ne_zero_at_const_mul_transcendental
    {K L : Type*} [Field K] [Field L] [Algebra K L]
    (curve : WeierstrassCurve K) (index : ℤ)
    (indexNonzero : (index : K) ≠ 0)
    (s : L) (sTranscendental : Transcendental K s)
    (t : K) (tNonzero : t ≠ 0) :
    Polynomial.aeval (algebraMap K L t * s) (curve.preΨ index) ≠ 0 := by
  intro root
  have algebraic : IsAlgebraic K (algebraMap K L t * s) :=
    divisionPolynomialRoot_isAlgebraic curve index
      (algebraMap K L t * s) indexNonzero root
  exact (transcendental_const_mul s sTranscendental t tNonzero) algebraic

/-- The full squared division polynomial cannot vanish at a nonzero constant
multiple of a transcendental element. This is the parity-correct endpoint for
the eventual torsion-coordinate contradiction. -/
theorem divisionPolynomialSq_ne_zero_at_const_mul_transcendental
    {K L : Type*} [Field K] [Field L] [Algebra K L]
    (curve : WeierstrassCurve K) (index : ℤ)
    (indexNonzero : (index : K) ≠ 0)
    (s : L) (sTranscendental : Transcendental K s)
    (t : K) (tNonzero : t ≠ 0) :
    Polynomial.aeval (algebraMap K L t * s) (curve.ΨSq index) ≠ 0 := by
  intro root
  have algebraic : IsAlgebraic K (algebraMap K L t * s) :=
    divisionPolynomialSqRoot_isAlgebraic curve index
      (algebraMap K L t * s) indexNonzero root
  exact (transcendental_const_mul s sTranscendental t tNonzero) algebraic

/-- A nonsingular affine point over an extension field whose x-coordinate is
a nonzero constant multiple of a transcendental element cannot be killed by
an integer that remains nonzero in the constant field.  This combines the
integer-multiplication formula with the parity-correct squared division
polynomial, so it includes even-order torsion. -/
theorem no_zsmul_torsion_at_const_mul_transcendental
    {K L : Type*} [Field K] [Field L] [Algebra K L]
    (curve : WeierstrassCurve K) (index : ℤ)
    (indexNonzero : (index : K) ≠ 0)
    (s : L) (sTranscendental : Transcendental K s)
    (t : K) (tNonzero : t ≠ 0) (y : L)
    (nonsingular : (curve.map (algebraMap K L)).toAffine.Nonsingular
      (algebraMap K L t * s) y)
    (torsion : index • WeierstrassCurve.Jacobian.Point.fromAffine
      (WeierstrassCurve.Affine.Point.some _ _ nonsingular) = 0) :
    False := by
  have root := divisionPolynomialSq_eval_eq_zero_of_zsmul_eq_zero
    (curve.map (algebraMap K L)) nonsingular index torsion
  apply divisionPolynomialSq_ne_zero_at_const_mul_transcendental
    curve index indexNonzero s sTranscendental t tNonzero
  simpa [Polynomial.aeval_def, Polynomial.eval_map] using root

/-- Affine-coordinate form of
`no_zsmul_torsion_at_const_mul_transcendental`.  The Jacobian/affine group
equivalence transports the asserted torsion relation to the division
polynomial multiplication formula. -/
theorem no_affine_zsmul_torsion_at_const_mul_transcendental
    {K L : Type*} [Field K] [Field L] [Algebra K L] [DecidableEq L]
    (curve : WeierstrassCurve K) (index : ℤ)
    (indexNonzero : (index : K) ≠ 0)
    (s : L) (sTranscendental : Transcendental K s)
    (t : K) (tNonzero : t ≠ 0) (y : L)
    (nonsingular : (curve.map (algebraMap K L)).toAffine.Nonsingular
      (algebraMap K L t * s) y)
    (torsion : index • WeierstrassCurve.Affine.Point.some
      (algebraMap K L t * s) y nonsingular = 0) :
    False := by
  apply no_zsmul_torsion_at_const_mul_transcendental
    curve index indexNonzero s sTranscendental t tNonzero y nonsingular
  let equivalence := WeierstrassCurve.Jacobian.Point.toAffineAddEquiv
    (curve.map (algebraMap K L))
  calc
    index • WeierstrassCurve.Jacobian.Point.fromAffine
        (WeierstrassCurve.Affine.Point.some _ _ nonsingular) =
        equivalence.symm
          (index • WeierstrassCurve.Affine.Point.some _ _ nonsingular) := by
            rw [map_zsmul]
            rfl
    _ = equivalence.symm 0 := congrArg equivalence.symm torsion
    _ = 0 := map_zero equivalence.symm

/-- Product of the indexed quadratic polynomials whose splitting field is the
multiquadratic extension used by the Kummer-sign argument. -/
noncomputable def quadraticRadicandPolynomial
    {K Index : Type*} [Field K] [Fintype Index]
    (radicand : Index → K) : Polynomial K :=
  ∏ index, (Polynomial.X ^ 2 - Polynomial.C (radicand index))

/-- Every quadratic factor of `quadraticRadicandPolynomial` is nonzero. -/
theorem quadraticRadicandFactor_ne_zero
    {K : Type*} [Field K] (value : K) :
    Polynomial.X ^ 2 - Polynomial.C value ≠ 0 :=
  (Polynomial.monic_X_pow_sub_C value (by norm_num : 2 ≠ 0)).ne_zero

/-- The product of the indexed monic quadratic factors is nonzero. -/
theorem quadraticRadicandPolynomial_ne_zero
    {K Index : Type*} [Field K] [Fintype Index]
    (radicand : Index → K) :
    quadraticRadicandPolynomial radicand ≠ 0 := by
  classical
  rw [quadraticRadicandPolynomial]
  exact Finset.prod_ne_zero_iff.mpr fun index _ ↦
    quadraticRadicandFactor_ne_zero (radicand index)

/-- Distinct constant terms give coprime quadratic factors. -/
theorem quadraticRadicandFactors_isCoprime
    {K : Type*} [Field K] {left right : K} (distinct : left ≠ right) :
    IsCoprime (Polynomial.X ^ 2 - Polynomial.C left)
      (Polynomial.X ^ 2 - Polynomial.C right) := by
  have differenceNonzero : right - left ≠ 0 := sub_ne_zero.mpr (Ne.symm distinct)
  refine ⟨Polynomial.C (right - left)⁻¹,
    -Polynomial.C (right - left)⁻¹, ?_⟩
  calc
    Polynomial.C (right - left)⁻¹ *
          (Polynomial.X ^ 2 - Polynomial.C left) +
        -Polynomial.C (right - left)⁻¹ *
          (Polynomial.X ^ 2 - Polynomial.C right) =
      Polynomial.C (right - left)⁻¹ * Polynomial.C (right - left) := by
      ext degree
      simp
      ring
    _ = 1 := by
      rw [← Polynomial.C_mul, inv_mul_cancel₀ differenceNonzero]
      simp

/-- Pairwise nonsquare products force the radicands to be injectively
indexed: equal radicands would have a square product. -/
theorem radicand_injective_of_pairwise_not_isSquare_mul
    {K Index : Type*} [Field K] (radicand : Index → K)
    (pairwiseNonsquare : ∀ left right, left ≠ right →
      ¬ IsSquare (radicand left * radicand right)) :
    Function.Injective radicand := by
  intro left right equal
  by_contra distinct
  apply pairwiseNonsquare left right distinct
  refine ⟨radicand left, ?_⟩
  rw [equal]

/-- In odd characteristic, nonzero injectively indexed radicands make the
product of their quadratic factors separable. -/
theorem quadraticRadicandPolynomial_separable
    {K Index : Type*} [Field K] [Fintype Index]
    (radicand : Index → K) (twoNonzero : (2 : K) ≠ 0)
    (radicandNonzero : ∀ index, radicand index ≠ 0)
    (radicandInjective : Function.Injective radicand) :
    Polynomial.Separable (quadraticRadicandPolynomial radicand) := by
  classical
  rw [quadraticRadicandPolynomial]
  apply Polynomial.separable_prod
  · intro left right distinct
    exact quadraticRadicandFactors_isCoprime
      (fun equal ↦ distinct (radicandInjective equal))
  · intro index
    exact Polynomial.separable_X_pow_sub_C
      (radicand index) twoNonzero (radicandNonzero index)

/-- The canonical splitting field of the indexed quadratic product is a
finite Galois extension once the radicands are nonzero and pairwise distinct. -/
theorem quadraticRadicandSplittingField_isGalois
    {K Index : Type*} [Field K] [Fintype Index]
    (radicand : Index → K) (twoNonzero : (2 : K) ≠ 0)
    (radicandNonzero : ∀ index, radicand index ≠ 0)
    (radicandInjective : Function.Injective radicand) :
    IsGalois K (quadraticRadicandPolynomial radicand).SplittingField :=
  IsGalois.of_separable_splitting_field
    (quadraticRadicandPolynomial_separable radicand twoNonzero
      radicandNonzero radicandInjective)

/-- Every indexed quadratic factor has a root in the canonical splitting
field of their product. -/
theorem exists_quadraticRadicandRoot
    {K Index : Type*} [Field K] [Fintype Index]
    (radicand : Index → K) (index : Index) :
    ∃ root : (quadraticRadicandPolynomial radicand).SplittingField,
      root ^ 2 = algebraMap K _ (radicand index) := by
  classical
  let factor : Polynomial K :=
    Polynomial.X ^ 2 - Polynomial.C (radicand index)
  have factorDvd : factor ∣ quadraticRadicandPolynomial radicand := by
    rw [quadraticRadicandPolynomial]
    exact Finset.dvd_prod_of_mem
      (fun i ↦ Polynomial.X ^ 2 - Polynomial.C (radicand i))
      (Finset.mem_univ index)
  have mappedFactorDvd :
      factor.map (algebraMap K
        (quadraticRadicandPolynomial radicand).SplittingField) ∣
      (quadraticRadicandPolynomial radicand).map
        (algebraMap K (quadraticRadicandPolynomial radicand).SplittingField) :=
    Polynomial.map_dvd _ factorDvd
  have factorSplits :
      (factor.map (algebraMap K
        (quadraticRadicandPolynomial radicand).SplittingField)).Splits :=
    (Polynomial.SplittingField.splits (quadraticRadicandPolynomial radicand)).of_dvd
      (Polynomial.map_ne_zero
        (quadraticRadicandPolynomial_ne_zero radicand)) mappedFactorDvd
  obtain ⟨root, rootEvaluation⟩ := factorSplits.exists_eval_eq_zero (by
    dsimp [factor]
    simp only [Polynomial.map_sub, Polynomial.map_pow,
      Polynomial.map_X, Polynomial.map_C]
    rw [Polynomial.degree_X_pow_sub_C (by norm_num : 0 < 2)]
    norm_num)
  refine ⟨root, ?_⟩
  have : root ^ 2 - algebraMap K
      (quadraticRadicandPolynomial radicand).SplittingField
        (radicand index) = 0 := by
    simpa [factor] using rootEvaluation
  exact sub_eq_zero.mp this

/-- A canonical selected root for each quadratic factor in the splitting
field. -/
noncomputable def quadraticRadicandRoot
    {K Index : Type*} [Field K] [Fintype Index]
    (radicand : Index → K) (index : Index) :
    (quadraticRadicandPolynomial radicand).SplittingField :=
  Classical.choose (exists_quadraticRadicandRoot radicand index)

/-- The canonical selected root squares to its indexed radicand. -/
theorem quadraticRadicandRoot_sq
    {K Index : Type*} [Field K] [Fintype Index]
    (radicand : Index → K) (index : Index) :
    quadraticRadicandRoot radicand index ^ 2 =
      algebraMap K _ (radicand index) :=
  Classical.choose_spec (exists_quadraticRadicandRoot radicand index)

/-- A selected root of a nonzero radicand is nonzero. -/
theorem quadraticRadicandRoot_ne_zero
    {K Index : Type*} [Field K] [Fintype Index]
    (radicand : Index → K) (index : Index)
    (radicandNonzero : radicand index ≠ 0) :
    quadraticRadicandRoot radicand index ≠ 0 := by
  intro rootZero
  have rootSquare := quadraticRadicandRoot_sq radicand index
  rw [rootZero, zero_pow (by norm_num : 2 ≠ 0)] at rootSquare
  exact ((map_ne_zero (algebraMap K
    (quadraticRadicandPolynomial radicand).SplittingField)).2
      radicandNonzero) rootSquare.symm

/-- Two square roots of base-field elements in a finite Galois extension
cannot transform with the same ratio under every automorphism unless the
product of their radicands is already a square in the base field.  This is
the fixed-field bridge needed to obtain pairwise separation of Kummer sign
characters without assuming a character table. -/
theorem exists_galoisAut_sqrt_ratio_ne_of_not_isSquare_mul
    {K L : Type*} [Field K] [Field L] [Algebra K L]
    [FiniteDimensional K L] [IsGalois K L]
    {a b : K} {x y : L}
    (xNonzero : x ≠ 0) (yNonzero : y ≠ 0)
    (xSquare : x ^ 2 = algebraMap K L a)
    (ySquare : y ^ 2 = algebraMap K L b)
    (productNonsquare : ¬ IsSquare (a * b)) :
    ∃ sigma : L ≃ₐ[K] L, sigma x / x ≠ sigma y / y := by
  by_contra noSeparator
  push Not at noSeparator
  have productFixed : ∀ sigma : L ≃ₐ[K] L, sigma (x * y) = x * y := by
    intro sigma
    have crossEquality : sigma x * y = sigma y * x :=
      (div_eq_div_iff xNonzero yNonzero).mp (noSeparator sigma)
    have sigmaYSquare : (sigma y) ^ 2 = y ^ 2 := by
      calc
        (sigma y) ^ 2 = sigma (y ^ 2) := by rw [map_pow]
        _ = sigma (algebraMap K L b) := by rw [ySquare]
        _ = algebraMap K L b := sigma.commutes b
        _ = y ^ 2 := ySquare.symm
    have sigmaX : sigma x = sigma y * x / y :=
      (eq_div_iff yNonzero).2 crossEquality
    calc
      sigma (x * y) = sigma x * sigma y := map_mul sigma x y
      _ = (sigma y * x / y) * sigma y := by rw [sigmaX]
      _ = (sigma y) ^ 2 * x / y := by ring
      _ = y ^ 2 * x / y := by rw [sigmaYSquare]
      _ = x * y := by field_simp [yNonzero]
  obtain ⟨z, zMaps⟩ :=
    (IsGalois.mem_range_algebraMap_iff_fixed (F := K) (E := L) (x * y)).2
      productFixed
  apply productNonsquare
  refine ⟨z, ?_⟩
  apply (algebraMap K L).injective
  calc
    algebraMap K L (a * b) = algebraMap K L a * algebraMap K L b :=
      map_mul (algebraMap K L) a b
    _ = x ^ 2 * y ^ 2 := by rw [xSquare, ySquare]
    _ = (x * y) * (x * y) := by ring
    _ = algebraMap K L z * algebraMap K L z := by rw [zMaps]
    _ = algebraMap K L (z * z) := (map_mul (algebraMap K L) z z).symm

/-- The integer sign with which a Galois automorphism transforms a chosen
square root. -/
noncomputable def galoisSqrtSign
    {K L : Type*} [Field K] [Field L] [Algebra K L]
    (x : L) (sigma : L ≃ₐ[K] L) : ℤ :=
  by
    classical
    exact if sigma x = x then 1 else -1

/-- A Galois automorphism acts on a square root by its selected sign. -/
theorem galoisSqrtSign_action
    {K L : Type*} [Field K] [Field L] [Algebra K L]
    {a : K} {x : L} (xSquare : x ^ 2 = algebraMap K L a)
    (sigma : L ≃ₐ[K] L) :
    sigma x = galoisSqrtSign x sigma • x := by
  classical
  have imageSquare : (sigma x) ^ 2 = x ^ 2 := by
    calc
      (sigma x) ^ 2 = sigma (x ^ 2) := by rw [map_pow]
      _ = sigma (algebraMap K L a) := by rw [xSquare]
      _ = algebraMap K L a := sigma.commutes a
      _ = x ^ 2 := xSquare.symm
  by_cases fixed : sigma x = x
  · simp [galoisSqrtSign, fixed]
  · have negated : sigma x = -x :=
      (sq_eq_sq_iff_eq_or_eq_neg.mp imageSquare).resolve_left fixed
    simp only [galoisSqrtSign]
    simp [negated]

/-- The Galois square-root sign is multiplicative.  Characteristic two is
excluded so that the two integer signs remain distinguishable on a nonzero
root. -/
theorem galoisSqrtSign_multiplicative
    {K L : Type*} [Field K] [Field L] [Algebra K L]
    {a : K} {x : L} (xNonzero : x ≠ 0) (twoNonzero : (2 : L) ≠ 0)
    (xSquare : x ^ 2 = algebraMap K L a)
    (left right : L ≃ₐ[K] L) :
    galoisSqrtSign x (left * right) =
      galoisSqrtSign x left * galoisSqrtSign x right := by
  classical
  have negXNe : -x ≠ x := by
    intro negEqual
    have addSelf : x + x = 0 := by
      calc
        x + x = -x + x := by rw [negEqual]
        _ = 0 := neg_add_cancel x
    have twoMul : (2 : L) * x = 0 := by
      simpa [two_mul] using addSelf
    exact (mul_ne_zero twoNonzero xNonzero) twoMul
  by_cases leftFixed : left x = x
  · by_cases rightFixed : right x = x
    · have productFixed : (left * right) x = x := by
        rw [AlgEquiv.mul_apply, rightFixed, leftFixed]
      simp only [galoisSqrtSign]
      simp [productFixed, leftFixed, rightFixed]
    · have rightNeg : right x = -x := by
        simpa [galoisSqrtSign, rightFixed] using
          galoisSqrtSign_action xSquare right
      have productNotFixed : (left * right) x ≠ x := by
        rw [AlgEquiv.mul_apply, rightNeg, map_neg, leftFixed]
        exact negXNe
      simp only [galoisSqrtSign]
      simp [rightNeg, leftFixed, negXNe]
  · have leftNeg : left x = -x := by
      simpa [galoisSqrtSign, leftFixed] using
        galoisSqrtSign_action xSquare left
    by_cases rightFixed : right x = x
    · simp only [galoisSqrtSign]
      simp [leftFixed, rightFixed]
    · have rightNeg : right x = -x := by
        simpa [galoisSqrtSign, rightFixed] using
          galoisSqrtSign_action xSquare right
      have productFixed : (left * right) x = x := by
        rw [AlgEquiv.mul_apply, rightNeg, map_neg, leftNeg, neg_neg]
      simp only [galoisSqrtSign]
      simp [productFixed, leftFixed, rightFixed]

/-- Every Galois square-root sign has square one. -/
theorem galoisSqrtSign_squareOne
    {K L : Type*} [Field K] [Field L] [Algebra K L]
    (x : L) (sigma : L ≃ₐ[K] L) :
    galoisSqrtSign x sigma * galoisSqrtSign x sigma = 1 := by
  classical
  by_cases fixed : sigma x = x <;> simp [galoisSqrtSign, fixed]

/-- A nonsquare product of two radicands gives an automorphism on which their
integer-valued Galois signs have product `-1`. -/
theorem galoisSqrtSigns_separate_of_not_isSquare_mul
    {K L : Type*} [Field K] [Field L] [Algebra K L]
    [FiniteDimensional K L] [IsGalois K L]
    {a b : K} {x y : L}
    (xNonzero : x ≠ 0) (yNonzero : y ≠ 0)
    (xSquare : x ^ 2 = algebraMap K L a)
    (ySquare : y ^ 2 = algebraMap K L b)
    (productNonsquare : ¬ IsSquare (a * b)) :
    ∃ sigma : L ≃ₐ[K] L,
      galoisSqrtSign x sigma * galoisSqrtSign y sigma = -1 := by
  obtain ⟨sigma, ratioDifferent⟩ :=
    exists_galoisAut_sqrt_ratio_ne_of_not_isSquare_mul
      xNonzero yNonzero xSquare ySquare productNonsquare
  by_cases xFixed : sigma x = x
  · have yNotFixed : sigma y ≠ y := by
      intro yFixed
      apply ratioDifferent
      rw [xFixed, yFixed]
      simp [xNonzero, yNonzero]
    refine ⟨sigma, ?_⟩
    simp [galoisSqrtSign, xFixed, yNotFixed]
  · have xNeg : sigma x = -x := by
      simpa [galoisSqrtSign, xFixed] using
        galoisSqrtSign_action xSquare sigma
    have yFixed : sigma y = y := by
      by_contra yNotFixed
      have yNeg : sigma y = -y := by
        simpa [galoisSqrtSign, yNotFixed] using
          galoisSqrtSign_action ySquare sigma
      apply ratioDifferent
      rw [xNeg, yNeg]
      simp [xNonzero, yNonzero]
    refine ⟨sigma, ?_⟩
    simp [galoisSqrtSign, xFixed, yFixed]

/-- Applying an additive equivalence to one point relation produces the
corresponding character-twisted relation when each indexed point is an
eigenvector with the supplied integer character. -/
theorem transformedRelations_of_addEquiv_eigenvectors
    {Index Gamma Point : Type*}
    [Fintype Index] [AddCommGroup Point]
    (character : Index → Gamma → ℤ) (coefficient : Index → ℤ)
    (point : Index → Point) (action : Gamma → Point ≃+ Point)
    (pointAction : ∀ gamma index,
      action gamma (point index) = character index gamma • point index)
    (relation : ∑ index, coefficient index • point index = 0) :
    ∀ gamma, ∑ index,
      (character index gamma * coefficient index) • point index = 0 := by
  intro gamma
  have mapped := congrArg (action gamma) relation
  have mappedRelation :
      ∑ index, coefficient index •
        (character index gamma • point index) = 0 := by
    simpa [map_sum, pointAction] using mapped
  simpa only [smul_smul, mul_comm] using mappedRelation

/-- A base-field algebra automorphism acts as an additive equivalence on the
points of a base-changed affine Weierstrass curve. -/
noncomputable def affinePointGaloisAddEquiv
    {K L : Type*} [Field K] [Field L] [Algebra K L] [DecidableEq L]
    (curve : WeierstrassCurve.Affine K) [curve.IsElliptic]
    (sigma : L ≃ₐ[K] L) :
    (curve.baseChange L).Point ≃+ (curve.baseChange L).Point where
  toFun := WeierstrassCurve.Affine.Point.map
    (W' := curve) sigma.toAlgHom
  invFun := WeierstrassCurve.Affine.Point.map
    (W' := curve) sigma.symm.toAlgHom
  left_inv point := by
    rw [WeierstrassCurve.Affine.Point.map_map]
    have composition :
        sigma.symm.toAlgHom.comp sigma.toAlgHom = AlgHom.id K L := by
      ext value
      simp
    rw [composition]
    cases point <;> rfl
  right_inv point := by
    rw [WeierstrassCurve.Affine.Point.map_map]
    have composition :
        sigma.toAlgHom.comp sigma.symm.toAlgHom = AlgHom.id K L := by
      ext value
      simp
    rw [composition]
    cases point <;> rfl
  map_add' left right := by
    exact (WeierstrassCurve.Affine.Point.map
      (W' := curve) sigma.toAlgHom).map_add left right

/-- On affine coordinates, the Galois point action applies the field
automorphism to both coordinates. -/
theorem affinePointGaloisAddEquiv_some
    {K L : Type*} [Field K] [Field L] [Algebra K L] [DecidableEq L]
    (curve : WeierstrassCurve.Affine K) [curve.IsElliptic]
    (sigma : L ≃ₐ[K] L)
    {x y : L} (nonsingular : (curve.baseChange L).Nonsingular x y) :
    affinePointGaloisAddEquiv curve sigma
        (WeierstrassCurve.Affine.Point.some x y nonsingular) =
      WeierstrassCurve.Affine.Point.some
        (sigma.toAlgHom x) (sigma.toAlgHom y)
        ((WeierstrassCurve.Affine.baseChange_nonsingular
          (W := curve) (f := sigma.toAlgHom) sigma.toAlgHom.injective x y).mpr
            nonsingular) :=
  rfl

/-- On a short-form base-changed curve, a point whose `Y`-coordinate is a
chosen square root is an eigenvector for the Galois point action with the
corresponding integer square-root sign. -/
theorem affinePointGaloisAddEquiv_eigenvector
    {K L : Type*} [Field K] [Field L] [Algebra K L] [DecidableEq L]
    (curve : WeierstrassCurve.Affine K) [curve.IsElliptic]
    (a₁Zero : curve.a₁ = 0) (a₃Zero : curve.a₃ = 0)
    {x radicand : K} {y : L}
    (ySquare : y ^ 2 = algebraMap K L radicand)
    (nonsingular : (curve.baseChange L).Nonsingular (algebraMap K L x) y)
    (sigma : L ≃ₐ[K] L) :
    affinePointGaloisAddEquiv curve sigma
        (WeierstrassCurve.Affine.Point.some
          (algebraMap K L x) y nonsingular) =
      galoisSqrtSign y sigma •
        WeierstrassCurve.Affine.Point.some
          (algebraMap K L x) y nonsingular := by
  classical
  by_cases fixed : sigma y = y
  · simp only [galoisSqrtSign, if_pos fixed, one_zsmul]
    rw [affinePointGaloisAddEquiv_some]
    rw [WeierstrassCurve.Affine.Point.some.injEq]
    exact ⟨sigma.commutes x, fixed⟩
  · have negated : sigma y = -y := by
      simpa [galoisSqrtSign, fixed] using
        galoisSqrtSign_action ySquare sigma
    have negatedAlgHom : sigma.toAlgHom y = -y := negated
    simp only [galoisSqrtSign, if_neg fixed, neg_one_zsmul]
    rw [affinePointGaloisAddEquiv_some,
      WeierstrassCurve.Affine.Point.neg_some]
    rw [WeierstrassCurve.Affine.Point.some.injEq]
    refine ⟨sigma.commutes x, ?_⟩
    rw [negatedAlgHom]
    simp [WeierstrassCurve.Affine.negY, a₁Zero, a₃Zero]

/-- Abstract Kummer-character projector. Orthogonality of the sign characters
and the transformed zero-sum relations isolate one term and force
`|Gamma| * n_i` times that point to vanish. -/
theorem characterProjector_isolates_term
    {Index Gamma Point : Type*}
    [Fintype Index] [DecidableEq Index]
    [Fintype Gamma] [DecidableEq Gamma] [AddCommGroup Point]
    (character : Index → Gamma → ℤ) (coefficient : Index → ℤ)
    (point : Index → Point) (chosen : Index)
    (orthogonality : ∀ index,
      (∑ gamma, character chosen gamma * character index gamma) =
        if index = chosen then Fintype.card Gamma else 0)
    (transformedRelation : ∀ gamma,
      ∑ index, (character index gamma * coefficient index) • point index = 0) :
    ((Fintype.card Gamma : ℤ) * coefficient chosen) • point chosen = 0 := by
  have projectedZero :
      ∑ gamma, character chosen gamma •
        (∑ index, (character index gamma * coefficient index) • point index) = 0 := by
    simp [transformedRelation]
  have rearranged :
      (∑ gamma, character chosen gamma •
        (∑ index, (character index gamma * coefficient index) • point index)) =
      ∑ index,
        (((∑ gamma, character chosen gamma * character index gamma) *
          coefficient index) • point index) := by
    simp_rw [smul_sum, ← mul_smul]
    rw [Finset.sum_comm]
    apply Finset.sum_congr rfl
    intro index _
    rw [← Finset.sum_smul, Finset.sum_mul]
    apply congrArg fun scalar : ℤ => scalar • point index
    apply Finset.sum_congr rfl
    intro gamma _
    ring
  rw [rearranged] at projectedZero
  simpa [orthogonality] using projectedZero

/-- Orthogonality of a finite family of multiplicative sign characters.
Distinct characters need only be separated by one group element on which
their product is `-1`; translation by that element pairs every summand with
its negative. -/
theorem separatingSignCharacters_orthogonality
    {Index Gamma : Type*}
    [DecidableEq Index] [Fintype Gamma] [DecidableEq Gamma] [Group Gamma]
    (character : Index → Gamma → ℤ)
    (multiplicative : ∀ index left right,
      character index (left * right) =
        character index left * character index right)
    (squareOne : ∀ index gamma,
      character index gamma * character index gamma = 1)
    (separates : ∀ left right, left ≠ right →
      ∃ delta, character left delta * character right delta = -1) :
    ∀ left right,
      (∑ gamma, character left gamma * character right gamma) =
        if right = left then Fintype.card Gamma else 0 := by
  intro left right
  by_cases equal : right = left
  · subst right
    simp [squareOne]
  · obtain ⟨delta, separatesAtDelta⟩ := separates left right (Ne.symm equal)
    let productCharacter : Gamma → ℤ := fun gamma =>
      character left gamma * character right gamma
    have translated :
        (∑ gamma, productCharacter (delta * gamma)) =
          ∑ gamma, productCharacter gamma := by
      exact Equiv.sum_comp (Equiv.mulLeft delta) productCharacter
    have translatedNeg :
        (∑ gamma, productCharacter (delta * gamma)) =
          -∑ gamma, productCharacter gamma := by
      calc
        (∑ gamma, productCharacter (delta * gamma)) =
            ∑ gamma, -productCharacter gamma := by
          apply Fintype.sum_congr
          intro gamma
          simp only [productCharacter, multiplicative]
          rw [← neg_one_mul (productCharacter gamma), ← separatesAtDelta]
          ring
        _ = -∑ gamma, productCharacter gamma := by simp
    have sumNeg :
        (∑ gamma, productCharacter gamma) =
          -∑ gamma, productCharacter gamma :=
      translated.symm.trans translatedNeg
    have sumZero : (∑ gamma, productCharacter gamma) = 0 := by omega
    simpa [equal, productCharacter] using sumZero

/-- Kummer-projector endpoint with character orthogonality discharged from
multiplicativity, sign values, and pairwise separation. -/
theorem separatingSignCharacterProjector_isolates_term
    {Index Gamma Point : Type*}
    [Fintype Index] [DecidableEq Index]
    [Fintype Gamma] [DecidableEq Gamma] [Group Gamma] [AddCommGroup Point]
    (character : Index → Gamma → ℤ) (coefficient : Index → ℤ)
    (point : Index → Point) (chosen : Index)
    (multiplicative : ∀ index left right,
      character index (left * right) =
        character index left * character index right)
    (squareOne : ∀ index gamma,
      character index gamma * character index gamma = 1)
    (separates : ∀ left right, left ≠ right →
      ∃ delta, character left delta * character right delta = -1)
    (transformedRelation : ∀ gamma,
      ∑ index, (character index gamma * coefficient index) • point index = 0) :
    ((Fintype.card Gamma : ℤ) * coefficient chosen) • point chosen = 0 := by
  apply characterProjector_isolates_term character coefficient point chosen
  · intro index
    simpa [eq_comm] using
      separatingSignCharacters_orthogonality character multiplicative squareOne
        separates chosen index
  · exact transformedRelation

/-- Concrete Galois-sign specialization of the Kummer projector.  Once a
finite Galois extension containing the selected square roots is supplied,
pairwise nonsquareness of the radicand products constructs the separating
integer sign characters used by the projector. -/
theorem galoisSignCharacterProjector_isolates_term
    {K L Index Point : Type*}
    [Field K] [Field L] [Algebra K L]
    [FiniteDimensional K L] [IsGalois K L]
    [Fintype Index] [DecidableEq Index] [AddCommGroup Point]
    (radicand : Index → K) (root : Index → L)
    (coefficient : Index → ℤ) (point : Index → Point) (chosen : Index)
    (rootNonzero : ∀ index, root index ≠ 0)
    (twoNonzero : (2 : L) ≠ 0)
    (rootSquare : ∀ index,
      root index ^ 2 = algebraMap K L (radicand index))
    (pairwiseNonsquare : ∀ left right, left ≠ right →
      ¬ IsSquare (radicand left * radicand right))
    (transformedRelation : ∀ sigma : L ≃ₐ[K] L,
      ∑ index, (galoisSqrtSign (root index) sigma * coefficient index) •
        point index = 0) :
    ((Fintype.card (L ≃ₐ[K] L) : ℤ) * coefficient chosen) •
      point chosen = 0 := by
  classical
  apply separatingSignCharacterProjector_isolates_term
    (fun index sigma ↦ galoisSqrtSign (root index) sigma)
    coefficient point chosen
  · intro index left right
    exact galoisSqrtSign_multiplicative
      (rootNonzero index) twoNonzero (rootSquare index) left right
  · intro index sigma
    exact galoisSqrtSign_squareOne (root index) sigma
  · intro left right distinct
    exact galoisSqrtSigns_separate_of_not_isSquare_mul
      (rootNonzero left) (rootNonzero right)
      (rootSquare left) (rootSquare right)
      (pairwiseNonsquare left right distinct)
  · exact transformedRelation

/-- Splitting-field specialization of the Galois-sign projector.  The finite
Galois extension and indexed square roots are constructed canonically from
the product of the quadratic radicand polynomials, so the remaining supplied
input is only the family of transformed point relations. -/
theorem quadraticSplittingFieldSignProjector_isolates_term
    {K Index Point : Type*}
    [Field K] [Fintype Index] [DecidableEq Index] [AddCommGroup Point]
    (radicand : Index → K) (coefficient : Index → ℤ)
    (point : Index → Point) (chosen : Index)
    (twoNonzero : (2 : K) ≠ 0)
    (radicandNonzero : ∀ index, radicand index ≠ 0)
    (pairwiseNonsquare : ∀ left right, left ≠ right →
      ¬ IsSquare (radicand left * radicand right))
    (transformedRelation : ∀ sigma :
      (quadraticRadicandPolynomial radicand).SplittingField ≃ₐ[K]
        (quadraticRadicandPolynomial radicand).SplittingField,
      ∑ index,
        (galoisSqrtSign (quadraticRadicandRoot radicand index) sigma *
          coefficient index) • point index = 0) :
    ((Fintype.card
      ((quadraticRadicandPolynomial radicand).SplittingField ≃ₐ[K]
        (quadraticRadicandPolynomial radicand).SplittingField) : ℤ) *
      coefficient chosen) • point chosen = 0 := by
  letI : IsGalois K
      (quadraticRadicandPolynomial radicand).SplittingField :=
    quadraticRadicandSplittingField_isGalois radicand twoNonzero
      radicandNonzero
      (radicand_injective_of_pairwise_not_isSquare_mul radicand
        pairwiseNonsquare)
  have extensionTwoNonzero :
      (2 : (quadraticRadicandPolynomial radicand).SplittingField) ≠ 0 := by
    intro twoZero
    apply twoNonzero
    apply (algebraMap K
      (quadraticRadicandPolynomial radicand).SplittingField).injective
    calc
      algebraMap K (quadraticRadicandPolynomial radicand).SplittingField
          (2 : K) = 2 := map_ofNat _ 2
      _ = 0 := twoZero
      _ = algebraMap K
          (quadraticRadicandPolynomial radicand).SplittingField 0 :=
        (map_zero _).symm
  apply galoisSignCharacterProjector_isolates_term
    radicand (quadraticRadicandRoot radicand) coefficient point chosen
  · intro index
    exact quadraticRadicandRoot_ne_zero radicand index
      (radicandNonzero index)
  · exact extensionTwoNonzero
  · intro index
    exact quadraticRadicandRoot_sq radicand index
  · exact pairwiseNonsquare
  · exact transformedRelation

/-- Equivariant splitting-field specialization.  A single original point
relation and the additive Galois action on its indexed summands generate all
twisted relations required by the sign projector. -/
theorem quadraticSplittingFieldSignProjector_of_equivariant_relation
    {K Index Point : Type*}
    [Field K] [Fintype Index] [DecidableEq Index] [AddCommGroup Point]
    (radicand : Index → K) (coefficient : Index → ℤ)
    (point : Index → Point) (chosen : Index)
    (twoNonzero : (2 : K) ≠ 0)
    (radicandNonzero : ∀ index, radicand index ≠ 0)
    (pairwiseNonsquare : ∀ left right, left ≠ right →
      ¬ IsSquare (radicand left * radicand right))
    (action :
      ((quadraticRadicandPolynomial radicand).SplittingField ≃ₐ[K]
        (quadraticRadicandPolynomial radicand).SplittingField) →
      Point ≃+ Point)
    (pointAction : ∀ sigma index,
      action sigma (point index) =
        galoisSqrtSign (quadraticRadicandRoot radicand index) sigma •
          point index)
    (relation : ∑ index, coefficient index • point index = 0) :
    ((Fintype.card
      ((quadraticRadicandPolynomial radicand).SplittingField ≃ₐ[K]
        (quadraticRadicandPolynomial radicand).SplittingField) : ℤ) *
      coefficient chosen) • point chosen = 0 := by
  apply quadraticSplittingFieldSignProjector_isolates_term
    radicand coefficient point chosen twoNonzero radicandNonzero
      pairwiseNonsquare
  exact transformedRelations_of_addEquiv_eigenvectors
    (fun index sigma ↦
      galoisSqrtSign (quadraticRadicandRoot radicand index) sigma)
    coefficient point action pointAction relation

/-- Concrete short-curve splitting-field projector.  The extension, roots,
Galois action, eigenvector identities, and all transformed relations are
constructed internally from the indexed radicands and one original point
relation. -/
theorem quadraticSplittingFieldAffinePointProjector_isolates_term
    {K Index : Type*} [Field K] [Fintype Index] [DecidableEq Index]
    (curve : WeierstrassCurve.Affine K) [curve.IsElliptic]
    (a₁Zero : curve.a₁ = 0) (a₃Zero : curve.a₃ = 0)
    (radicand xCoordinate : Index → K)
    [DecidableEq (quadraticRadicandPolynomial radicand).SplittingField]
    (coefficient : Index → ℤ) (chosen : Index)
    (twoNonzero : (2 : K) ≠ 0)
    (radicandNonzero : ∀ index, radicand index ≠ 0)
    (pairwiseNonsquare : ∀ left right, left ≠ right →
      ¬ IsSquare (radicand left * radicand right))
    (nonsingular : ∀ index,
      (curve.baseChange
        (quadraticRadicandPolynomial radicand).SplittingField).Nonsingular
          (algebraMap K _ (xCoordinate index))
          (quadraticRadicandRoot radicand index))
    (relation :
      ∑ index, coefficient index •
        WeierstrassCurve.Affine.Point.some
          (algebraMap K
            (quadraticRadicandPolynomial radicand).SplittingField
              (xCoordinate index))
          (quadraticRadicandRoot radicand index) (nonsingular index) = 0) :
    ((Fintype.card
      ((quadraticRadicandPolynomial radicand).SplittingField ≃ₐ[K]
        (quadraticRadicandPolynomial radicand).SplittingField) : ℤ) *
      coefficient chosen) •
        WeierstrassCurve.Affine.Point.some
          (algebraMap K
            (quadraticRadicandPolynomial radicand).SplittingField
              (xCoordinate chosen))
          (quadraticRadicandRoot radicand chosen) (nonsingular chosen) = 0 := by
  apply quadraticSplittingFieldSignProjector_of_equivariant_relation
    radicand coefficient
    (fun index ↦ WeierstrassCurve.Affine.Point.some
      (algebraMap K (quadraticRadicandPolynomial radicand).SplittingField
        (xCoordinate index))
      (quadraticRadicandRoot radicand index) (nonsingular index))
    chosen twoNonzero radicandNonzero pairwiseNonsquare
    (fun sigma ↦ affinePointGaloisAddEquiv curve sigma)
  · intro sigma index
    exact affinePointGaloisAddEquiv_eigenvector curve a₁Zero a₃Zero
      (quadraticRadicandRoot_sq radicand index) (nonsingular index) sigma
  · exact relation

/-- A finite bad set pulled back through a map with fibres of size at most
`multiplicity` has size at most `multiplicity * degree` when every bad point
maps to a root of a nonzero polynomial of degree at most `degree`. -/
theorem polynomialPullbackBadSetCardLe
    {K Seed : Type*} [Field K] [DecidableEq K]
    [Fintype Seed] [DecidableEq Seed]
    (polynomial : Polynomial K) (parameter : Seed → K)
    (bad : Finset Seed) (multiplicity degree : ℕ)
    (fiberBound : ∀ value,
      (bad.filter fun seed => parameter seed = value).card ≤ multiplicity)
    (badRoot : ∀ seed ∈ bad, polynomial.eval (parameter seed) = 0)
    (nonzero : polynomial ≠ 0) (degreeBound : polynomial.natDegree ≤ degree) :
    bad.card ≤ multiplicity * degree := by
  have imageRoot : ∀ value ∈ bad.image parameter,
      polynomial.eval value = 0 := by
    intro value valueMem
    obtain ⟨seed, seedMem, rfl⟩ := Finset.mem_image.mp valueMem
    exact badRoot seed seedMem
  have imageDegree : (bad.image parameter).card ≤ polynomial.natDegree := by
    have filtered :
        (bad.image parameter).filter (fun value => polynomial.eval value = 0) =
          bad.image parameter := by
      exact Finset.filter_eq_self.mpr imageRoot
    rw [← filtered]
    exact polynomialBadValuesCardLeDegree polynomial (bad.image parameter) nonzero
  rw [Finset.card_eq_sum_card_image parameter bad]
  calc
    ∑ value ∈ bad.image parameter,
        (bad.filter fun seed => parameter seed = value).card ≤
        ∑ _value ∈ bad.image parameter, multiplicity := by
      exact Finset.sum_le_sum fun value _ => fiberBound value
    _ = multiplicity * (bad.image parameter).card := by
      simp [Nat.mul_comm]
    _ ≤ multiplicity * polynomial.natDegree :=
      Nat.mul_le_mul_left multiplicity imageDegree
    _ ≤ multiplicity * degree :=
      Nat.mul_le_mul_left multiplicity degreeBound

/-- The inverse-square parameter map on nonzero field elements is at most
two-to-one.  This is the elementary multiplicity factor in the dilation
root-count argument. -/
theorem inverseSquareFiberCard_le_two
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (value : K) :
    ((Finset.univ : Finset (NonzeroField K)).filter fun u =>
      u.1⁻¹ ^ 2 = value).card ≤ 2 := by
  let fiber := (Finset.univ : Finset (NonzeroField K)).filter fun u =>
    u.1⁻¹ ^ 2 = value
  let roots := fiber.image fun u => u.1
  let polynomial : Polynomial K :=
    Polynomial.C value * Polynomial.X ^ 2 - 1
  have polynomialNonzero : polynomial ≠ 0 := by
    intro polynomialZero
    have evaluated := congrArg (Polynomial.eval (0 : K)) polynomialZero
    simp [polynomial] at evaluated
  have rootsAreRoots :
      roots.filter (fun x => polynomial.eval x = 0) = roots := by
    apply Finset.filter_eq_self.mpr
    intro x xMem
    obtain ⟨u, uMem, rfl⟩ := Finset.mem_image.mp xMem
    have inverseSquare := (Finset.mem_filter.mp uMem).2
    simp only [polynomial, Polynomial.eval_sub, Polynomial.eval_mul,
      Polynomial.eval_C, Polynomial.eval_pow, Polynomial.eval_X,
      Polynomial.eval_one]
    rw [← inverseSquare]
    field_simp
    rw [div_self u.2, sub_self]
  have rootsDegree : roots.card ≤ polynomial.natDegree := by
    rw [← rootsAreRoots]
    exact polynomialBadValuesCardLeDegree polynomial roots polynomialNonzero
  have polynomialDegree : polynomial.natDegree ≤ 2 := by
    calc
      polynomial.natDegree ≤
          max (Polynomial.C value * Polynomial.X ^ 2).natDegree
            (1 : Polynomial K).natDegree := by
        exact Polynomial.natDegree_sub_le _ _
      _ ≤ 2 := by
        apply max_le
        · exact Polynomial.natDegree_C_mul_X_pow_le value 2
        · simp
  calc
    fiber.card = roots.card := by
      exact (Finset.card_image_of_injective fiber Subtype.val_injective).symm
    _ ≤ polynomial.natDegree := rootsDegree
    _ ≤ 2 := polynomialDegree

/-- Exact root-count bridge used by the dilation nondegeneracy obligation.
If a nonzero degree-`degree` polynomial in `s = u⁻²` vanishes for every
bad nonzero dilation, then there are at most `2 * degree` bad dilations. -/
theorem inverseSquarePolynomialBadSetCardLe
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (polynomial : Polynomial K) (bad : Finset (NonzeroField K)) (degree : ℕ)
    (badRoot : ∀ u ∈ bad, polynomial.eval (u.1⁻¹ ^ 2) = 0)
    (nonzero : polynomial ≠ 0) (degreeBound : polynomial.natDegree ≤ degree) :
    bad.card ≤ 2 * degree := by
  apply polynomialPullbackBadSetCardLe polynomial
    (fun u : NonzeroField K => u.1⁻¹ ^ 2) bad 2 degree
  · intro value
    calc
      (bad.filter fun u => u.1⁻¹ ^ 2 = value).card ≤
          ((Finset.univ : Finset (NonzeroField K)).filter fun u =>
            u.1⁻¹ ^ 2 = value).card := by
        apply Finset.card_le_card
        intro u uMem
        simp only [Finset.mem_filter, Finset.mem_univ, true_and]
        exact (Finset.mem_filter.mp uMem).2
      _ ≤ 2 := inverseSquareFiberCard_le_two value
  · exact badRoot
  · exact nonzero
  · exact degreeBound

/-- Uniform certificate interface for dilation nondegeneracy.  For every
tuple, it supplies one nonzero polynomial of bounded degree whose roots contain
all bad inverse-square dilation parameters. -/
def InverseSquarePolynomialCertificate
    {K Tuple : Type*} [Field K] [Fintype K]
    (bad : Tuple → NonzeroField K → Prop) (degree : ℕ) : Prop :=
  ∀ tuple, ∃ polynomial : Polynomial K,
    polynomial ≠ 0 ∧ polynomial.natDegree ≤ degree ∧
      ∀ u, bad tuple u → polynomial.eval (u.1⁻¹ ^ 2) = 0

/-- A uniform inverse-square polynomial certificate implies the exact
`2 * degree` bad-dilation bound for every tuple.  Thus, after this theorem,
the remaining curve-specific work is precisely construction, degree,
vanishing, and nonvanishing of the specialized polynomial family. -/
theorem inverseSquarePolynomialCertificate_card_le
    {K Tuple : Type*} [Field K] [Fintype K] [DecidableEq K]
    (bad : Tuple → NonzeroField K → Prop) [DecidableRel bad] (degree : ℕ)
    (certificate : InverseSquarePolynomialCertificate bad degree)
    (tuple : Tuple) :
    ((Finset.univ : Finset (NonzeroField K)).filter fun u =>
      bad tuple u).card ≤ 2 * degree := by
  classical
  obtain ⟨polynomial, nonzero, degreeBound, badRoot⟩ := certificate tuple
  apply inverseSquarePolynomialBadSetCardLe polynomial
    ((Finset.univ : Finset (NonzeroField K)).filter fun u => bad tuple u)
    degree
  · intro u uMem
    exact badRoot u (Finset.mem_filter.mp uMem).2
  · exact nonzero
  · exact degreeBound

/-! ## Exact bridge to the upstream dilation hypothesis

The upstream ECDLP development indexes dilations by the unit group `Kˣ`,
rather than by `NonzeroField K`.  The following variants close that purely
representational gap and then instantiate its exact `DilationNondeg`
predicate.
-/

/-- The inverse-square map on field units is at most two-to-one. -/
theorem unitsInverseSquareFiberCard_le_two
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (value : K) :
    ((Finset.univ : Finset Kˣ).filter fun u : Kˣ =>
      ((u : K)⁻¹) ^ 2 = value).card ≤ 2 := by
  let fiber : Finset Kˣ := (Finset.univ : Finset Kˣ).filter fun u : Kˣ =>
    ((u : K)⁻¹) ^ 2 = value
  let roots : Finset K := fiber.image fun u : Kˣ => (u : K)
  let polynomial : Polynomial K :=
    Polynomial.C value * Polynomial.X ^ 2 - 1
  have polynomialNonzero : polynomial ≠ 0 := by
    intro polynomialZero
    have evaluated := congrArg (Polynomial.eval (0 : K)) polynomialZero
    simp [polynomial] at evaluated
  have rootsAreRoots :
      roots.filter (fun x => polynomial.eval x = 0) = roots := by
    apply Finset.filter_eq_self.mpr
    intro x xMem
    obtain ⟨u, uMem, rfl⟩ := Finset.mem_image.mp xMem
    have inverseSquare := (Finset.mem_filter.mp uMem).2
    simp only [polynomial, Polynomial.eval_sub, Polynomial.eval_mul,
      Polynomial.eval_C, Polynomial.eval_pow, Polynomial.eval_X,
      Polynomial.eval_one]
    rw [← inverseSquare]
    field_simp
    simp
  have rootsDegree : roots.card ≤ polynomial.natDegree := by
    rw [← rootsAreRoots]
    exact polynomialBadValuesCardLeDegree polynomial roots polynomialNonzero
  have polynomialDegree : polynomial.natDegree ≤ 2 := by
    calc
      polynomial.natDegree ≤
          max (Polynomial.C value * Polynomial.X ^ 2).natDegree
            (1 : Polynomial K).natDegree := by
        exact Polynomial.natDegree_sub_le _ _
      _ ≤ 2 := by
        apply max_le
        · exact Polynomial.natDegree_C_mul_X_pow_le value 2
        · simp
  calc
    fiber.card = roots.card := by
      exact (Finset.card_image_of_injective fiber Units.val_injective).symm
    _ ≤ polynomial.natDegree := rootsDegree
    _ ≤ 2 := polynomialDegree

/-- Unit-group form of the inverse-square polynomial root bound. -/
theorem unitsInverseSquarePolynomialBadSetCardLe
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (polynomial : Polynomial K) (bad : Finset Kˣ) (degree : ℕ)
    (badRoot : ∀ u ∈ bad, polynomial.eval (((u : K)⁻¹) ^ 2) = 0)
    (nonzero : polynomial ≠ 0) (degreeBound : polynomial.natDegree ≤ degree) :
    bad.card ≤ 2 * degree := by
  apply polynomialPullbackBadSetCardLe polynomial
    (fun u : Kˣ => ((u : K)⁻¹) ^ 2) bad 2 degree
  · intro value
    calc
      (bad.filter fun u : Kˣ => ((u : K)⁻¹) ^ 2 = value).card ≤
          ((Finset.univ : Finset Kˣ).filter fun u : Kˣ =>
            ((u : K)⁻¹) ^ 2 = value).card := by
        apply Finset.card_le_card
        intro u uMem
        simp only [Finset.mem_filter, Finset.mem_univ, true_and]
        exact (Finset.mem_filter.mp uMem).2
      _ ≤ 2 := unitsInverseSquareFiberCard_le_two value
  · exact badRoot
  · exact nonzero
  · exact degreeBound

/-- A curve-specific polynomial certificate with exactly the tuple and bad
dilation predicates used by `ECDLP.L49.DilationNondeg`. -/
def UpstreamDilationPolynomialCertificate
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (A B : K) (m : ℕ) (T : Finset K) (degree : ℕ) : Prop :=
  ∀ t : Fin (2 * m) → K, (∀ i, t i ∈ T) →
    ¬ ECDLP.L40.TrivL (List.ofFn t) →
    ∃ polynomial : Polynomial K,
      polynomial ≠ 0 ∧ polynomial.natDegree ≤ degree ∧
        ∀ u : Kˣ,
          (∃ P : Fin (2 * m) → (ECDLP.L49.Eu A B u).Point,
            (∀ i, P i ≠ 0 ∧ ECDLP.L43.xco _ (P i) = t i) ∧
              ∑ i, P i = 0) →
          polynomial.eval (((u : K)⁻¹) ^ 2) = 0

/-- A polynomial family satisfying construction, nonvanishing, degree, and
vanishing for the exact upstream bad predicate proves the upstream
`DilationNondeg` hypothesis. -/
theorem upstreamDilationPolynomialCertificate_implies_nondeg
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (A B : K) (m : ℕ) (T : Finset K) (degree : ℕ)
    (certificate : UpstreamDilationPolynomialCertificate A B m T degree) :
    ECDLP.L49.DilationNondeg A B m T degree := by
  classical
  letI pointFintype (W : WeierstrassCurve.Affine K) : Fintype W.Point :=
    ECDLP.L43.instFintypePoint W
  intro t tupleInFactorBase nontrivial
  obtain ⟨polynomial, nonzero, degreeBound, badRoot⟩ :=
    certificate t tupleInFactorBase nontrivial
  apply unitsInverseSquarePolynomialBadSetCardLe
    (polynomial := polynomial) (degree := degree)
  · intro u uMem
    exact badRoot u (Finset.mem_filter.mp uMem).2
  · exact nonzero
  · exact degreeBound

/-- Construction/nonvanishing/degree/vanishing form of the exact bridge.  It
makes the four remaining curve-specific proof obligations explicit for a
single supplied specialized polynomial family. -/
theorem upstreamDilationPolynomialFamily_implies_nondeg
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (A B : K) (m : ℕ) (T : Finset K) (degree : ℕ)
    (family : (Fin (2 * m) → K) → Polynomial K)
    (nonzero : ∀ t, (∀ i, t i ∈ T) →
      ¬ ECDLP.L40.TrivL (List.ofFn t) → family t ≠ 0)
    (degreeBound : ∀ t, (family t).natDegree ≤ degree)
    (vanishing : ∀ t u,
      (∃ P : Fin (2 * m) → (ECDLP.L49.Eu A B u).Point,
        (∀ i, P i ≠ 0 ∧ ECDLP.L43.xco _ (P i) = t i) ∧
          ∑ i, P i = 0) →
      (family t).eval (((u : K)⁻¹) ^ 2) = 0) :
    ECDLP.L49.DilationNondeg A B m T degree := by
  apply upstreamDilationPolynomialCertificate_implies_nondeg
  intro t tupleInFactorBase nontrivial
  exact ⟨family t, nonzero t tupleInFactorBase nontrivial,
    degreeBound t, vanishing t⟩

/-- The same certificate feeds directly into Kaarelson's exact average-energy
bound.  No restatement of `Eng` or `Mdil` is used here. -/
theorem upstreamDilationPolynomialCertificate_implies_avg_energy
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (hK : ringChar K ≠ 2) (A B : K)
    (n degree : ℕ) (T : Finset K)
    (nonsingular : ECDLP.L43.disc A B ≠ 0)
    (certificate :
      UpstreamDilationPolynomialCertificate A B (n + 1) T degree) :
    ∑ u : Kˣ,
        (ECDLP.L43.Eng T n ((u : K) ^ 4 * A) ((u : K) ^ 6 * B) : ℝ) ≤
      ECDLP.L49.Mdil K n T degree * (Fintype.card Kˣ : ℝ) := by
  exact ECDLP.L49.avg_energy_dil hK A B nonsingular T n degree
    (upstreamDilationPolynomialCertificate_implies_nondeg
      A B (n + 1) T degree certificate)

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

/-! ## The cubic CM automorphism on the `A = 0` short curve -/

/-- A field element whose cube is one is nonzero. -/
theorem cubeRootOfOne_ne_zero
    {K : Type*} [Field K] {omega : K} (omegaCube : omega ^ 3 = 1) :
    omega ≠ 0 := by
  intro omegaZero
  rw [omegaZero, zero_pow (by norm_num)] at omegaCube
  exact zero_ne_one omegaCube

/-- Multiplication of the `X`-coordinate by a cubic root of unity preserves
the affine equation, hence nonsingularity, of `Y² = X³ + B`. -/
theorem shortCM_nonsingular
    {K : Type*} [Field K] (B omega x y : K)
    [(shortCharacterCurve 0 0 B).IsElliptic]
    (omegaCube : omega ^ 3 = 1)
    (nonsingular : (shortCharacterCurve 0 0 B).toAffine.Nonsingular x y) :
    (shortCharacterCurve 0 0 B).toAffine.Nonsingular (omega * x) y := by
  apply WeierstrassCurve.Affine.equation_iff_nonsingular.mp
  rw [shortCharacterCurve_equation_iff]
  have equation :=
    WeierstrassCurve.Affine.equation_iff_nonsingular.mpr nonsingular
  rw [shortCharacterCurve_equation_iff] at equation
  simp only [zero_mul, zero_add, add_zero] at equation ⊢
  calc
    y ^ 2 = x ^ 3 + B := equation
    _ = (omega * x) ^ 3 + B := by rw [mul_pow, omegaCube, one_mul]

/-- On `Y² = X³ + B`, the secant-or-tangent slope transforms by `omega²`
under `(x,y) ↦ (omega*x,y)`. -/
theorem shortCM_slope
    {K : Type*} [Field K] [DecidableEq K]
    (B omega x₁ x₂ y₁ y₂ : K) (omegaCube : omega ^ 3 = 1) :
    (shortCharacterCurve 0 0 B).toAffine.slope
        (omega * x₁) (omega * x₂) y₁ y₂ =
      omega ^ 2 *
        (shortCharacterCurve 0 0 B).toAffine.slope x₁ x₂ y₁ y₂ := by
  have omegaNonzero : omega ≠ 0 := cubeRootOfOne_ne_zero omegaCube
  by_cases xEqual : x₁ = x₂
  · have scaledXEqual : omega * x₁ = omega * x₂ := congrArg (omega * ·) xEqual
    by_cases yOpposite : y₁ = -y₂
    · rw [WeierstrassCurve.Affine.slope_of_Y_eq scaledXEqual,
        WeierstrassCurve.Affine.slope_of_Y_eq xEqual] <;>
        simpa [shortCharacterCurve, WeierstrassCurve.Affine.negY] using yOpposite
    · have originalNotOpposite :
        y₁ ≠ (shortCharacterCurve 0 0 B).toAffine.negY x₂ y₂ := by
          simpa [shortCharacterCurve, WeierstrassCurve.Affine.negY] using yOpposite
      have scaledNotOpposite :
        y₁ ≠ (shortCharacterCurve 0 0 B).toAffine.negY (omega * x₂) y₂ := by
          simpa [shortCharacterCurve, WeierstrassCurve.Affine.negY] using yOpposite
      rw [WeierstrassCurve.Affine.slope_of_Y_ne scaledXEqual scaledNotOpposite,
        WeierstrassCurve.Affine.slope_of_Y_ne xEqual originalNotOpposite]
      simp only [shortCharacterCurve, WeierstrassCurve.Affine.negY,
        zero_mul, zero_add, add_zero, sub_zero]
      ring
  · have scaledXNotEqual : omega * x₁ ≠ omega * x₂ := by
      intro equality
      exact xEqual (mul_left_cancel₀ omegaNonzero equality)
    rw [WeierstrassCurve.Affine.slope_of_X_ne scaledXNotEqual,
      WeierstrassCurve.Affine.slope_of_X_ne xEqual]
    have differenceNonzero : x₁ - x₂ ≠ 0 := sub_ne_zero.mpr xEqual
    field_simp [omegaNonzero, differenceNonzero]
    rw [omegaCube, mul_one]

/-- The `X`-coordinate addition formula is equivariant under the cubic CM
scaling. -/
theorem shortCM_addX
    {K : Type*} [Field K] (B omega x₁ x₂ slope : K)
    (omegaCube : omega ^ 3 = 1) :
    (shortCharacterCurve 0 0 B).toAffine.addX
        (omega * x₁) (omega * x₂) (omega ^ 2 * slope) =
      omega * (shortCharacterCurve 0 0 B).toAffine.addX x₁ x₂ slope := by
  have omegaFourth : omega ^ 4 = omega := by
    calc
      omega ^ 4 = omega * omega ^ 3 := by ring
      _ = omega := by rw [omegaCube, mul_one]
  simp only [WeierstrassCurve.Affine.addX, shortCharacterCurve,
    zero_mul, zero_add, add_zero, sub_zero]
  calc
    (omega ^ 2 * slope) ^ 2 - omega * x₁ - omega * x₂ =
        omega ^ 4 * slope ^ 2 - omega * x₁ - omega * x₂ := by ring
    _ = omega * slope ^ 2 - omega * x₁ - omega * x₂ := by rw [omegaFourth]
    _ = omega * (slope ^ 2 - x₁ - x₂) := by ring

/-- The `Y`-coordinate addition formula is fixed by the cubic CM scaling. -/
theorem shortCM_addY
    {K : Type*} [Field K] (B omega x₁ x₂ y₁ slope : K)
    (omegaCube : omega ^ 3 = 1) :
    (shortCharacterCurve 0 0 B).toAffine.addY
        (omega * x₁) (omega * x₂) y₁ (omega ^ 2 * slope) =
      (shortCharacterCurve 0 0 B).toAffine.addY x₁ x₂ y₁ slope := by
  rw [WeierstrassCurve.Affine.addY, WeierstrassCurve.Affine.addY,
    WeierstrassCurve.Affine.negAddY, WeierstrassCurve.Affine.negAddY,
    shortCM_addX B omega x₁ x₂ slope omegaCube]
  simp only [shortCharacterCurve, WeierstrassCurve.Affine.negY,
    zero_mul, zero_add, add_zero, sub_zero]
  calc
    -(omega ^ 2 * slope *
          (omega * (shortCharacterCurve 0 0 B).toAffine.addX x₁ x₂ slope -
            omega * x₁) + y₁) =
        -(omega ^ 3 * slope *
          ((shortCharacterCurve 0 0 B).toAffine.addX x₁ x₂ slope - x₁) + y₁) := by
          ring
    _ = -(slope *
          ((shortCharacterCurve 0 0 B).toAffine.addX x₁ x₂ slope - x₁) + y₁) := by
          rw [omegaCube, one_mul]

/-- Coordinate action of the cubic CM map on affine points of
`Y² = X³ + B`. -/
noncomputable def shortCMPointMap
    {K : Type*} [Field K] (B omega : K)
    [(shortCharacterCurve 0 0 B).IsElliptic]
    (omegaCube : omega ^ 3 = 1) :
    (shortCharacterCurve 0 0 B).toAffine.Point →
      (shortCharacterCurve 0 0 B).toAffine.Point
  | .zero => .zero
  | .some x y nonsingular =>
      .some (omega * x) y
        (shortCM_nonsingular B omega x y omegaCube nonsingular)

@[simp]
theorem shortCMPointMap_zero
    {K : Type*} [Field K] (B omega : K)
    [(shortCharacterCurve 0 0 B).IsElliptic]
    (omegaCube : omega ^ 3 = 1) :
    shortCMPointMap B omega omegaCube
      (0 : (shortCharacterCurve 0 0 B).toAffine.Point) = 0 :=
  rfl

@[simp]
theorem shortCMPointMap_some
    {K : Type*} [Field K] (B omega x y : K)
    [(shortCharacterCurve 0 0 B).IsElliptic]
    (omegaCube : omega ^ 3 = 1)
    (nonsingular : (shortCharacterCurve 0 0 B).toAffine.Nonsingular x y) :
    shortCMPointMap B omega omegaCube
        (WeierstrassCurve.Affine.Point.some x y nonsingular) =
      WeierstrassCurve.Affine.Point.some (omega * x) y
        (shortCM_nonsingular B omega x y omegaCube nonsingular) :=
  rfl

/-- The cubic CM coordinate action preserves affine point addition. -/
theorem shortCMPointMap_add
    {K : Type*} [Field K] [DecidableEq K]
    (B omega : K) [(shortCharacterCurve 0 0 B).IsElliptic]
    (omegaCube : omega ^ 3 = 1)
    (left right : (shortCharacterCurve 0 0 B).toAffine.Point) :
    shortCMPointMap B omega omegaCube (left + right) =
      shortCMPointMap B omega omegaCube left +
        shortCMPointMap B omega omegaCube right := by
  let W := (shortCharacterCurve 0 0 B).toAffine
  rcases left with _ | ⟨x₁, y₁, leftNonsingular⟩
  · rfl
  rcases right with _ | ⟨x₂, y₂, rightNonsingular⟩
  · rfl
  by_cases opposite : x₁ = x₂ ∧ y₁ = W.negY x₂ y₂
  · have scaledOpposite :
        omega * x₁ = omega * x₂ ∧ y₁ = W.negY (omega * x₂) y₂ := by
      refine ⟨congrArg (omega * ·) opposite.1, ?_⟩
      simpa [W, shortCharacterCurve, WeierstrassCurve.Affine.negY] using opposite.2
    rw [WeierstrassCurve.Affine.Point.add_of_Y_eq opposite.1 opposite.2,
      shortCMPointMap_zero B omega omegaCube]
    simp only [shortCMPointMap]
    rw [WeierstrassCurve.Affine.Point.add_of_Y_eq
      scaledOpposite.1 scaledOpposite.2]
  · have omegaNonzero : omega ≠ 0 := cubeRootOfOne_ne_zero omegaCube
    have scaledNotOpposite :
        ¬(omega * x₁ = omega * x₂ ∧ y₁ = W.negY (omega * x₂) y₂) := by
      intro scaledOpposite
      apply opposite
      refine ⟨mul_left_cancel₀ omegaNonzero scaledOpposite.1, ?_⟩
      simpa [W, shortCharacterCurve, WeierstrassCurve.Affine.negY] using
        scaledOpposite.2
    rw [WeierstrassCurve.Affine.Point.add_some opposite]
    simp only [shortCMPointMap]
    rw [WeierstrassCurve.Affine.Point.add_some scaledNotOpposite]
    rw [WeierstrassCurve.Affine.Point.some.injEq]
    constructor
    · rw [shortCM_slope B omega x₁ x₂ y₁ y₂ omegaCube,
        shortCM_addX B omega x₁ x₂
          (W.slope x₁ x₂ y₁ y₂) omegaCube]
    · rw [shortCM_slope B omega x₁ x₂ y₁ y₂ omegaCube,
        shortCM_addY B omega x₁ x₂ y₁
          (W.slope x₁ x₂ y₁ y₂) omegaCube]

/-- The additive endomorphism of `Y² = X³ + B` induced by the cubic CM
coordinate action. -/
noncomputable def shortCMEndomorphism
    {K : Type*} [Field K] [DecidableEq K]
    (B omega : K) [(shortCharacterCurve 0 0 B).IsElliptic]
    (omegaCube : omega ^ 3 = 1) :
    (shortCharacterCurve 0 0 B).toAffine.Point →+
      (shortCharacterCurve 0 0 B).toAffine.Point where
  toFun := shortCMPointMap B omega omegaCube
  map_zero' := shortCMPointMap_zero B omega omegaCube
  map_add' := shortCMPointMap_add B omega omegaCube

/-- The quadratic cubic-root relation implies the cubic relation. -/
theorem cube_eq_one_of_sq_add_self_add_one_eq_zero
    {K : Type*} [Field K] {omega : K}
    (omegaRelation : omega ^ 2 + omega + 1 = 0) : omega ^ 3 = 1 := by
  calc
    omega ^ 3 = (omega - 1) * (omega ^ 2 + omega + 1) + 1 := by ring
    _ = 1 := by rw [omegaRelation, mul_zero, zero_add]

/-- The line through the `omega²` and `omega` images of one point is
horizontal, including the tangent case at `X = 0`. -/
theorem shortCM_orbit_slope_zero
    {K : Type*} [Field K] [DecidableEq K]
    (B omega x y : K) (omegaNonzero : omega ≠ 0)
    (omegaNonidentity : omega ≠ 1) :
    (shortCharacterCurve 0 0 B).toAffine.slope
      (omega ^ 2 * x) (omega * x) y y = 0 := by
  by_cases xEqual : omega ^ 2 * x = omega * x
  · have omegaTimesX : omega * x = x := by
      apply mul_left_cancel₀ omegaNonzero
      simpa [pow_two, mul_assoc] using xEqual
    have xZero : x = 0 := by
      have productZero : (omega - 1) * x = 0 := by
        calc
          (omega - 1) * x = omega * x - x := by ring
          _ = 0 := sub_eq_zero.mpr omegaTimesX
      exact (mul_eq_zero.mp productZero).resolve_left
        (sub_ne_zero.mpr omegaNonidentity)
    by_cases yOpposite :
        y = (shortCharacterCurve 0 0 B).toAffine.negY (omega * x) y
    · exact WeierstrassCurve.Affine.slope_of_Y_eq xEqual yOpposite
    · rw [WeierstrassCurve.Affine.slope_of_Y_ne xEqual yOpposite]
      simp [xZero, shortCharacterCurve, WeierstrassCurve.Affine.negY]
  · rw [WeierstrassCurve.Affine.slope_of_X_ne xEqual]
    simp

/-- The three points in one cubic-CM orbit sum to zero.  This is the
pointwise endomorphism identity `zeta² + zeta + 1 = 0`. -/
theorem shortCM_orbit_sum_eq_zero
    {K : Type*} [Field K] [DecidableEq K]
    (B omega : K) [(shortCharacterCurve 0 0 B).IsElliptic]
    (twoNonzero : (2 : K) ≠ 0)
    (omegaRelation : omega ^ 2 + omega + 1 = 0)
    (omegaNonidentity : omega ≠ 1)
    (point : (shortCharacterCurve 0 0 B).toAffine.Point) :
    shortCMEndomorphism B omega
          (cube_eq_one_of_sq_add_self_add_one_eq_zero omegaRelation)
        (shortCMEndomorphism B omega
          (cube_eq_one_of_sq_add_self_add_one_eq_zero omegaRelation) point) +
      shortCMEndomorphism B omega
          (cube_eq_one_of_sq_add_self_add_one_eq_zero omegaRelation) point +
      point = 0 := by
  let W := (shortCharacterCurve 0 0 B).toAffine
  let omegaCube := cube_eq_one_of_sq_add_self_add_one_eq_zero omegaRelation
  rcases point with _ | ⟨x, y, nonsingular⟩
  · rfl
  have omegaNonzero : omega ≠ 0 := cubeRootOfOne_ne_zero omegaCube
  have BNonzero : B ≠ 0 := by
    intro BZero
    have discriminantNonzero :=
      (shortCharacterCurve 0 0 B).isUnit_Δ.ne_zero
    rw [originalShortCurve_discriminant] at discriminantNonzero
    simp [BZero] at discriminantNonzero
  have firstNonsingular := shortCM_nonsingular B omega x y omegaCube nonsingular
  have secondNonsingular :=
    shortCM_nonsingular B omega (omega * x) y omegaCube firstNonsingular
  have notOpposite :
      ¬(omega ^ 2 * x = omega * x ∧ y = W.negY (omega * x) y) := by
    rintro ⟨xEqual, yOpposite⟩
    have omegaTimesX : omega * x = x := by
      apply mul_left_cancel₀ omegaNonzero
      simpa [pow_two, mul_assoc] using xEqual
    have xZero : x = 0 := by
      have productZero : (omega - 1) * x = 0 := by
        calc
          (omega - 1) * x = omega * x - x := by ring
          _ = 0 := sub_eq_zero.mpr omegaTimesX
      exact (mul_eq_zero.mp productZero).resolve_left
        (sub_ne_zero.mpr omegaNonidentity)
    have yNeg : y = -y := by
      simpa [W, shortCharacterCurve, WeierstrassCurve.Affine.negY] using yOpposite
    have twoY : (2 : K) * y = 0 := by linear_combination yNeg
    have yZero : y = 0 :=
      (mul_eq_zero.mp twoY).resolve_left twoNonzero
    have equation :=
      WeierstrassCurve.Affine.equation_iff_nonsingular.mpr nonsingular
    rw [shortCharacterCurve_equation_iff] at equation
    simp [xZero, yZero] at equation
    exact BNonzero equation.symm
  have slopeZero : W.slope (omega ^ 2 * x) (omega * x) y y = 0 :=
    shortCM_orbit_slope_zero B omega x y omegaNonzero omegaNonidentity
  have addXEq :
      W.addX (omega ^ 2 * x) (omega * x)
          (W.slope (omega ^ 2 * x) (omega * x) y y) = x := by
    rw [slopeZero]
    simp only [W, WeierstrassCurve.Affine.addX, shortCharacterCurve,
      pow_two, zero_mul, zero_add, add_zero, sub_zero]
    linear_combination -x * omegaRelation
  have addXZero : W.addX (omega ^ 2 * x) (omega * x) 0 = x := by
    simpa only [slopeZero] using addXEq
  have addYEq :
      W.addY (omega ^ 2 * x) (omega * x) y
          (W.slope (omega ^ 2 * x) (omega * x) y y) = -y := by
    rw [WeierstrassCurve.Affine.addY, WeierstrassCurve.Affine.negAddY,
      slopeZero, addXZero]
    simp [W, shortCharacterCurve, WeierstrassCurve.Affine.negY]
  have firstSum :
      WeierstrassCurve.Affine.Point.some (omega ^ 2 * x) y
          (by simpa [pow_two, mul_assoc] using secondNonsingular) +
        WeierstrassCurve.Affine.Point.some (omega * x) y firstNonsingular =
      -WeierstrassCurve.Affine.Point.some x y nonsingular := by
    rw [WeierstrassCurve.Affine.Point.add_some notOpposite,
      WeierstrassCurve.Affine.Point.neg_some,
      WeierstrassCurve.Affine.Point.some.injEq]
    exact ⟨addXEq, by
      simpa [W, shortCharacterCurve, WeierstrassCurve.Affine.negY] using addYEq⟩
  have secondPointEq :
      WeierstrassCurve.Affine.Point.some (omega * (omega * x)) y
          secondNonsingular =
        WeierstrassCurve.Affine.Point.some (omega ^ 2 * x) y
          (by simpa [pow_two, mul_assoc] using secondNonsingular) := by
    rw [WeierstrassCurve.Affine.Point.some.injEq]
    exact ⟨by ring, rfl⟩
  change
    WeierstrassCurve.Affine.Point.some (omega * (omega * x)) y _ +
        WeierstrassCurve.Affine.Point.some (omega * x) y _ +
      WeierstrassCurve.Affine.Point.some x y nonsingular = 0
  rw [secondPointEq, firstSum, neg_add_cancel]

/-- Abstract Eisenstein-norm calculation.  If an additive endomorphism
`zeta` satisfies `zeta² + zeta + 1 = 0` at `point`, then a relation
`a*point + b*zeta(point) = 0` forces the explicit norm
`a² - ab + b²` to annihilate `point`. -/
theorem cubicCMRelation_implies_norm_torsion
    {Point : Type*} [AddCommGroup Point]
    (zeta : Point →+ Point) (point : Point) (a b : ℤ)
    (orbitRelation : zeta (zeta point) + zeta point + point = 0)
    (linearRelation : a • point + b • zeta point = 0) :
    (a ^ 2 - a * b + b ^ 2) • point = 0 := by
  have zetaSquare : zeta (zeta point) = -(zeta point + point) := by
    rw [eq_neg_iff_add_eq_zero]
    simpa [add_assoc] using orbitRelation
  have mappedRelation := congrArg zeta linearRelation
  simp only [map_add, map_zsmul, map_zero] at mappedRelation
  rw [zetaSquare] at mappedRelation
  have conjugateRelation :
      (-b) • point + (a - b) • zeta point = 0 := by
    linear_combination (norm := module) mappedRelation
  linear_combination (norm := module)
    (a - b) • linearRelation - b • conjugateRelation

/-- Concrete norm-torsion endpoint for the cubic CM automorphism of
`Y² = X³ + B`. -/
theorem shortCMLinearRelation_implies_norm_torsion
    {K : Type*} [Field K] [DecidableEq K]
    (B omega : K) [(shortCharacterCurve 0 0 B).IsElliptic]
    (twoNonzero : (2 : K) ≠ 0)
    (omegaRelation : omega ^ 2 + omega + 1 = 0)
    (omegaNonidentity : omega ≠ 1)
    (point : (shortCharacterCurve 0 0 B).toAffine.Point) (a b : ℤ)
    (linearRelation :
      a • point + b • shortCMEndomorphism B omega
        (cube_eq_one_of_sq_add_self_add_one_eq_zero omegaRelation) point = 0) :
    (a ^ 2 - a * b + b ^ 2) • point = 0 := by
  apply cubicCMRelation_implies_norm_torsion
    (shortCMEndomorphism B omega
      (cube_eq_one_of_sq_add_self_add_one_eq_zero omegaRelation))
    point a b
  · exact shortCM_orbit_sum_eq_zero B omega twoNonzero omegaRelation
      omegaNonidentity point
  · exact linearRelation

/-- Two multiplicative integer sign characters separated at one group element
have zero inner product over the finite group. -/
theorem separatedSignCharacters_innerProduct_zero
    {Gamma : Type*} [Fintype Gamma] [DecidableEq Gamma] [Group Gamma]
    (left right : Gamma → ℤ)
    (leftMultiplicative : ∀ x y, left (x * y) = left x * left y)
    (rightMultiplicative : ∀ x y, right (x * y) = right x * right y)
    (separates : ∃ delta, left delta * right delta = -1) :
    (∑ gamma, left gamma * right gamma) = 0 := by
  obtain ⟨delta, separatesAtDelta⟩ := separates
  let productCharacter : Gamma → ℤ := fun gamma => left gamma * right gamma
  have translated :
      (∑ gamma, productCharacter (delta * gamma)) =
        ∑ gamma, productCharacter gamma := by
    exact Equiv.sum_comp (Equiv.mulLeft delta) productCharacter
  have translatedNeg :
      (∑ gamma, productCharacter (delta * gamma)) =
        -∑ gamma, productCharacter gamma := by
    calc
      (∑ gamma, productCharacter (delta * gamma)) =
          ∑ gamma, -productCharacter gamma := by
        apply Fintype.sum_congr
        intro gamma
        simp only [productCharacter, leftMultiplicative,
          rightMultiplicative]
        rw [← neg_one_mul (productCharacter gamma), ← separatesAtDelta]
        ring
      _ = -∑ gamma, productCharacter gamma := by simp
  have sumNeg :
      (∑ gamma, productCharacter gamma) =
        -∑ gamma, productCharacter gamma :=
    translated.symm.trans translatedNeg
  have sumZero : (∑ gamma, productCharacter gamma) = 0 := by omega
  simpa [productCharacter] using sumZero

/-- A sign-character projector with exactly one duplicate character isolates
the two corresponding summands together.  This is the algebra needed for the
size-two collision classes in the `A = 0` branch. -/
theorem twoCollisionSignCharacterProjector_isolates_pair
    {Index Gamma Point : Type*}
    [Fintype Index] [DecidableEq Index]
    [Fintype Gamma] [DecidableEq Gamma] [Group Gamma]
    [AddCommGroup Point]
    (character : Index → Gamma → ℤ) (coefficient : Index → ℤ)
    (point : Index → Point) (chosen partner : Index)
    (distinct : chosen ≠ partner)
    (multiplicative : ∀ index left right,
      character index (left * right) =
        character index left * character index right)
    (squareOne : ∀ index gamma,
      character index gamma * character index gamma = 1)
    (sameCharacter : ∀ gamma,
      character partner gamma = character chosen gamma)
    (separatesOthers : ∀ index,
      index ≠ chosen → index ≠ partner →
        ∃ delta, character chosen delta * character index delta = -1)
    (transformedRelation : ∀ gamma,
      ∑ index, (character index gamma * coefficient index) • point index = 0) :
    ((Fintype.card Gamma : ℤ) * coefficient chosen) • point chosen +
      ((Fintype.card Gamma : ℤ) * coefficient partner) • point partner = 0 := by
  have innerProduct : ∀ index,
      (∑ gamma, character chosen gamma * character index gamma) =
        if index = chosen ∨ index = partner then
          (Fintype.card Gamma : ℤ) else 0 := by
    intro index
    by_cases chosenEqual : index = chosen
    · subst index
      simp [squareOne]
    by_cases partnerEqual : index = partner
    · subst index
      simp only [sameCharacter]
      simp [squareOne]
    · rw [if_neg (not_or_intro chosenEqual partnerEqual)]
      exact separatedSignCharacters_innerProduct_zero
        (character chosen) (character index)
        (multiplicative chosen) (multiplicative index)
        (separatesOthers index chosenEqual partnerEqual)
  have projectedZero :
      ∑ gamma, character chosen gamma •
        (∑ index, (character index gamma * coefficient index) • point index) = 0 := by
    simp [transformedRelation]
  have rearranged :
      (∑ gamma, character chosen gamma •
        (∑ index, (character index gamma * coefficient index) • point index)) =
      ∑ index,
        (((∑ gamma, character chosen gamma * character index gamma) *
          coefficient index) • point index) := by
    simp_rw [smul_sum, ← mul_smul]
    rw [Finset.sum_comm]
    apply Finset.sum_congr rfl
    intro index _
    rw [← Finset.sum_smul, Finset.sum_mul]
    apply congrArg fun scalar : ℤ => scalar • point index
    apply Finset.sum_congr rfl
    intro gamma _
    ring
  rw [rearranged] at projectedZero
  simp_rw [innerProduct] at projectedZero
  have splitIndicator : ∀ index,
      (((if index = chosen ∨ index = partner then
          (Fintype.card Gamma : ℤ) else 0) * coefficient index) • point index) =
        (if index = chosen then
            ((Fintype.card Gamma : ℤ) * coefficient chosen) • point chosen
          else 0) +
        (if index = partner then
            ((Fintype.card Gamma : ℤ) * coefficient partner) • point partner
          else 0) := by
    intro index
    by_cases chosenEqual : index = chosen
    · subst index
      simp [distinct]
    by_cases partnerEqual : index = partner
    · subst index
      simp [chosenEqual]
    · simp [chosenEqual, partnerEqual]
  simp_rw [splitIndicator] at projectedZero
  simpa [Finset.sum_add_distrib] using projectedZero

/-- A size-two sign-character collision whose partner point is a signed CM
image yields an explicit norm-torsion relation for the chosen point. -/
theorem twoCollisionSignCharacterProjector_implies_cubicCMNormTorsion
    {Index Gamma Point : Type*}
    [Fintype Index] [DecidableEq Index]
    [Fintype Gamma] [DecidableEq Gamma] [Group Gamma]
    [AddCommGroup Point]
    (character : Index → Gamma → ℤ) (coefficient : Index → ℤ)
    (point : Index → Point) (chosen partner : Index)
    (distinct : chosen ≠ partner)
    (multiplicative : ∀ index left right,
      character index (left * right) =
        character index left * character index right)
    (squareOne : ∀ index gamma,
      character index gamma * character index gamma = 1)
    (sameCharacter : ∀ gamma,
      character partner gamma = character chosen gamma)
    (separatesOthers : ∀ index,
      index ≠ chosen → index ≠ partner →
        ∃ delta, character chosen delta * character index delta = -1)
    (transformedRelation : ∀ gamma,
      ∑ index, (character index gamma * coefficient index) • point index = 0)
    (zeta : Point →+ Point) (sign : ℤ)
    (partnerPoint : point partner = sign • zeta (point chosen))
    (orbitRelation :
      zeta (zeta (point chosen)) + zeta (point chosen) + point chosen = 0) :
    (((Fintype.card Gamma : ℤ) * coefficient chosen) ^ 2 -
        ((Fintype.card Gamma : ℤ) * coefficient chosen) *
          (((Fintype.card Gamma : ℤ) * coefficient partner) * sign) +
        (((Fintype.card Gamma : ℤ) * coefficient partner) * sign) ^ 2) •
      point chosen = 0 := by
  apply cubicCMRelation_implies_norm_torsion zeta (point chosen)
    ((Fintype.card Gamma : ℤ) * coefficient chosen)
    (((Fintype.card Gamma : ℤ) * coefficient partner) * sign)
    orbitRelation
  have isolated := twoCollisionSignCharacterProjector_isolates_pair
    character coefficient point chosen partner distinct multiplicative
      squareOne sameCharacter separatesOthers transformedRelation
  rw [partnerPoint] at isolated
  simpa only [smul_smul] using isolated

/-- Concrete short-curve specialization of the two-collision projector.  The
cubic CM endomorphism and its orbit relation are constructed internally; the
remaining geometric input is the signed identification of the partner point. -/
theorem twoCollisionSignCharacterProjector_implies_shortCMNormTorsion
    {K Index Gamma : Type*} [Field K] [DecidableEq K]
    [Fintype Index] [DecidableEq Index]
    [Fintype Gamma] [DecidableEq Gamma] [Group Gamma]
    (B omega : K) [(shortCharacterCurve 0 0 B).IsElliptic]
    (twoNonzero : (2 : K) ≠ 0)
    (omegaRelation : omega ^ 2 + omega + 1 = 0)
    (omegaNonidentity : omega ≠ 1)
    (character : Index → Gamma → ℤ) (coefficient : Index → ℤ)
    (point : Index → (shortCharacterCurve 0 0 B).toAffine.Point)
    (chosen partner : Index) (distinct : chosen ≠ partner)
    (multiplicative : ∀ index left right,
      character index (left * right) =
        character index left * character index right)
    (squareOne : ∀ index gamma,
      character index gamma * character index gamma = 1)
    (sameCharacter : ∀ gamma,
      character partner gamma = character chosen gamma)
    (separatesOthers : ∀ index,
      index ≠ chosen → index ≠ partner →
        ∃ delta, character chosen delta * character index delta = -1)
    (transformedRelation : ∀ gamma,
      ∑ index, (character index gamma * coefficient index) • point index = 0)
    (sign : ℤ)
    (partnerPoint : point partner = sign •
      shortCMEndomorphism B omega
        (cube_eq_one_of_sq_add_self_add_one_eq_zero omegaRelation)
        (point chosen)) :
    (((Fintype.card Gamma : ℤ) * coefficient chosen) ^ 2 -
        ((Fintype.card Gamma : ℤ) * coefficient chosen) *
          (((Fintype.card Gamma : ℤ) * coefficient partner) * sign) +
        (((Fintype.card Gamma : ℤ) * coefficient partner) * sign) ^ 2) •
      point chosen = 0 := by
  apply twoCollisionSignCharacterProjector_implies_cubicCMNormTorsion
    character coefficient point chosen partner distinct multiplicative
      squareOne sameCharacter separatesOthers transformedRelation
      (shortCMEndomorphism B omega
        (cube_eq_one_of_sq_add_self_add_one_eq_zero omegaRelation))
      sign partnerPoint
  exact shortCM_orbit_sum_eq_zero B omega twoNonzero omegaRelation
    omegaNonidentity (point chosen)

/-- Two short-curve points whose `X`-coordinates differ by the cubic CM
scaling and whose `Y`-coordinates have the same square differ by a sign from
the CM image. -/
theorem shortCMPoints_signedOrbit_of_sq_eq
    {K : Type*} [Field K] [DecidableEq K]
    (B omega : K) [(shortCharacterCurve 0 0 B).IsElliptic]
    (omegaCube : omega ^ 3 = 1)
    {x y x' y' : K}
    (nonsingular : (shortCharacterCurve 0 0 B).toAffine.Nonsingular x y)
    (nonsingular' : (shortCharacterCurve 0 0 B).toAffine.Nonsingular x' y')
    (xScaling : x' = omega * x) (squareEqual : y' ^ 2 = y ^ 2) :
    ∃ sign : ℤ, (sign = 1 ∨ sign = -1) ∧
      WeierstrassCurve.Affine.Point.some x' y' nonsingular' =
        sign • shortCMEndomorphism B omega omegaCube
          (WeierstrassCurve.Affine.Point.some x y nonsingular) := by
  rcases sq_eq_sq_iff_eq_or_eq_neg.mp squareEqual with yEqual | yNeg
  · refine ⟨1, Or.inl rfl, ?_⟩
    change WeierstrassCurve.Affine.Point.some x' y' nonsingular' =
      WeierstrassCurve.Affine.Point.some (omega * x) y
        (shortCM_nonsingular B omega x y omegaCube nonsingular)
    rw [WeierstrassCurve.Affine.Point.some.injEq]
    exact ⟨xScaling, yEqual⟩
  · refine ⟨-1, Or.inr rfl, ?_⟩
    change WeierstrassCurve.Affine.Point.some x' y' nonsingular' =
      -WeierstrassCurve.Affine.Point.some (omega * x) y
        (shortCM_nonsingular B omega x y omegaCube nonsingular)
    rw [WeierstrassCurve.Affine.Point.neg_some,
      WeierstrassCurve.Affine.Point.some.injEq]
    refine ⟨xScaling, ?_⟩
    simpa [shortCharacterCurve, WeierstrassCurve.Affine.negY] using yNeg

/-- A nonidentity cubic root of unity satisfies the quadratic CM relation. -/
theorem cubeRootOfOne_quadraticRelation
    {K : Type*} [Field K] {omega : K}
    (omegaCube : omega ^ 3 = 1) (omegaNonidentity : omega ≠ 1) :
    omega ^ 2 + omega + 1 = 0 := by
  have productZero : (omega - 1) * (omega ^ 2 + omega + 1) = 0 := by
    calc
      (omega - 1) * (omega ^ 2 + omega + 1) = omega ^ 3 - 1 := by ring
      _ = 0 := by rw [omegaCube, sub_self]
  exact (mul_eq_zero.mp productZero).resolve_left
    (sub_ne_zero.mpr omegaNonidentity)

/-- In the `A = 0` branch, a square product of two distinct scaled cubic
radicands constructs the nontrivial cubic CM ratio relating their scales. -/
theorem shortCubicPolynomial_scaled_pair_CM_ratio_of_isSquare
    {K : Type*} [Field K] (hK : ringChar K ≠ 2)
    (B left right : K)
    (nonsingular : (27 : K) * B ^ 2 ≠ 0)
    (BNonzero : B ≠ 0) (leftNonzero : left ≠ 0)
    (rightNonzero : right ≠ 0) (distinct : left ≠ right)
    (squareInFunctionField : IsSquare
      (algebraMap (Polynomial K) (FractionRing (Polynomial K))
        ((shortCubicPolynomial 0 B).comp (Polynomial.C left * Polynomial.X) *
          (shortCubicPolynomial 0 B).comp
            (Polynomial.C right * Polynomial.X)))) :
    ∃ omega : K,
      omega ^ 2 + omega + 1 = 0 ∧ omega ≠ 1 ∧ right = omega * left := by
  let omega := right / left
  have omegaCube : omega ^ 3 = 1 :=
    ((shortCubicPolynomial_scaled_pair_fraction_square_classification hK
      0 B left right (by simpa using nonsingular) leftNonzero rightNonzero
      distinct squareInFunctionField).2 rfl BNonzero)
  have omegaNonidentity : omega ≠ 1 := by
    intro omegaOne
    have rightEqLeft : right = left := by
      have multiplied := (div_eq_iff leftNonzero).mp omegaOne
      simpa using multiplied
    exact distinct rightEqLeft.symm
  refine ⟨omega, cubeRootOfOne_quadraticRelation omegaCube omegaNonidentity,
    omegaNonidentity, ?_⟩
  dsimp [omega]
  exact (div_mul_cancel₀ right leftNonzero).symm

/-- End-to-end `A = 0` size-two collision bridge.  A square-product scale
collision constructs the nontrivial cubic CM ratio; the two lifted points are
then signed CM images, and the duplicated sign-character projector produces
an explicit norm-torsion relation. -/
theorem shortCubicPairCollision_implies_CMNormTorsion
    {K Index Gamma : Type*} [Field K] [DecidableEq K]
    [Fintype Index] [DecidableEq Index]
    [Fintype Gamma] [DecidableEq Gamma] [Group Gamma]
    (B left right parameter y y' : K)
    [(shortCharacterCurve 0 0 B).IsElliptic]
    (hK : ringChar K ≠ 2) (twoNonzero : (2 : K) ≠ 0)
    (cubicNonsingular : (27 : K) * B ^ 2 ≠ 0)
    (BNonzero : B ≠ 0) (leftNonzero : left ≠ 0)
    (rightNonzero : right ≠ 0) (scalesDistinct : left ≠ right)
    (squareInFunctionField : IsSquare
      (algebraMap (Polynomial K) (FractionRing (Polynomial K))
        ((shortCubicPolynomial 0 B).comp (Polynomial.C left * Polynomial.X) *
          (shortCubicPolynomial 0 B).comp
            (Polynomial.C right * Polynomial.X))))
    (leftNonsingular :
      (shortCharacterCurve 0 0 B).toAffine.Nonsingular
        (left * parameter) y)
    (rightNonsingular :
      (shortCharacterCurve 0 0 B).toAffine.Nonsingular
        (right * parameter) y')
    (character : Index → Gamma → ℤ) (coefficient : Index → ℤ)
    (point : Index → (shortCharacterCurve 0 0 B).toAffine.Point)
    (chosen partner : Index) (indicesDistinct : chosen ≠ partner)
    (chosenPoint : point chosen =
      WeierstrassCurve.Affine.Point.some
        (left * parameter) y leftNonsingular)
    (partnerPoint : point partner =
      WeierstrassCurve.Affine.Point.some
        (right * parameter) y' rightNonsingular)
    (multiplicative : ∀ index gamma₁ gamma₂,
      character index (gamma₁ * gamma₂) =
        character index gamma₁ * character index gamma₂)
    (squareOne : ∀ index gamma,
      character index gamma * character index gamma = 1)
    (sameCharacter : ∀ gamma,
      character partner gamma = character chosen gamma)
    (separatesOthers : ∀ index,
      index ≠ chosen → index ≠ partner →
        ∃ delta, character chosen delta * character index delta = -1)
    (transformedRelation : ∀ gamma,
      ∑ index, (character index gamma * coefficient index) • point index = 0) :
    ∃ omega : K, ∃ sign : ℤ,
      omega ^ 2 + omega + 1 = 0 ∧ omega ≠ 1 ∧
      right = omega * left ∧ (sign = 1 ∨ sign = -1) ∧
      (((Fintype.card Gamma : ℤ) * coefficient chosen) ^ 2 -
          ((Fintype.card Gamma : ℤ) * coefficient chosen) *
            (((Fintype.card Gamma : ℤ) * coefficient partner) * sign) +
          (((Fintype.card Gamma : ℤ) * coefficient partner) * sign) ^ 2) •
        point chosen = 0 := by
  obtain ⟨omega, omegaRelation, omegaNonidentity, scaleRelation⟩ :=
    shortCubicPolynomial_scaled_pair_CM_ratio_of_isSquare hK B left right
      cubicNonsingular BNonzero leftNonzero rightNonzero scalesDistinct
      squareInFunctionField
  have omegaCube :=
    cube_eq_one_of_sq_add_self_add_one_eq_zero omegaRelation
  have xScaling : right * parameter = omega * (left * parameter) := by
    rw [scaleRelation]
    ring
  have leftEquation :=
    WeierstrassCurve.Affine.equation_iff_nonsingular.mpr leftNonsingular
  rw [shortCharacterCurve_equation_iff] at leftEquation
  simp only [zero_mul, zero_add, add_zero] at leftEquation
  have rightEquation :=
    WeierstrassCurve.Affine.equation_iff_nonsingular.mpr rightNonsingular
  rw [shortCharacterCurve_equation_iff] at rightEquation
  simp only [zero_mul, zero_add, add_zero] at rightEquation
  have squareEqual : y' ^ 2 = y ^ 2 := by
    calc
      y' ^ 2 = (right * parameter) ^ 3 + B := rightEquation
      _ = (omega * (left * parameter)) ^ 3 + B := by rw [xScaling]
      _ = (left * parameter) ^ 3 + B := by
        rw [mul_pow, omegaCube, one_mul]
      _ = y ^ 2 := leftEquation.symm
  obtain ⟨sign, signCases, signedPoint⟩ :=
    shortCMPoints_signedOrbit_of_sq_eq B omega omegaCube
      leftNonsingular rightNonsingular xScaling squareEqual
  have familyPartner : point partner = sign •
      shortCMEndomorphism B omega omegaCube (point chosen) := by
    rw [partnerPoint, chosenPoint]
    exact signedPoint
  have normTorsion :=
    twoCollisionSignCharacterProjector_implies_shortCMNormTorsion
      B omega twoNonzero omegaRelation omegaNonidentity character coefficient
      point chosen partner indicesDistinct multiplicative squareOne
      sameCharacter separatesOthers transformedRelation sign familyPartner
  exact ⟨omega, sign, omegaRelation, omegaNonidentity, scaleRelation,
    signCases, normTorsion⟩

/-- Negating a selected square root does not change its integer-valued Galois
sign character. -/
theorem galoisSqrtSign_neg
    {K L : Type*} [Field K] [Field L] [Algebra K L]
    (x : L) (sigma : L ≃ₐ[K] L) :
    galoisSqrtSign (-x) sigma = galoisSqrtSign x sigma := by
  classical
  by_cases fixed : sigma x = x
  · have negFixed : sigma (-x) = -x := by rw [map_neg, fixed]
    simp [galoisSqrtSign, fixed, negFixed]
  · have negNotFixed : sigma (-x) ≠ -x := by
      intro negFixed
      apply fixed
      have := congrArg Neg.neg negFixed
      simpa using this
    simp [galoisSqrtSign, fixed, negNotFixed]

/-- Chosen square roots of the same radicand have identical Galois sign
characters, independently of the choices of root signs. -/
theorem galoisSqrtSigns_eq_of_same_square
    {K L : Type*} [Field K] [Field L] [Algebra K L]
    {a : K} {x y : L}
    (xSquare : x ^ 2 = algebraMap K L a)
    (ySquare : y ^ 2 = algebraMap K L a) :
    ∀ sigma : L ≃ₐ[K] L,
      galoisSqrtSign y sigma = galoisSqrtSign x sigma := by
  intro sigma
  have squareEqual : y ^ 2 = x ^ 2 := ySquare.trans xSquare.symm
  rcases sq_eq_sq_iff_eq_or_eq_neg.mp squareEqual with equal | negated
  · subst y
    rfl
  · rw [negated, galoisSqrtSign_neg]

/-- If the product of two radicands is already a square in the base field,
then every Galois automorphism gives their selected roots the same sign. -/
theorem galoisSqrtSigns_eq_of_isSquare_mul
    {K L : Type*} [Field K] [Field L] [Algebra K L]
    {a b : K} {x y : L}
    (xNonzero : x ≠ 0) (yNonzero : y ≠ 0)
    (xSquare : x ^ 2 = algebraMap K L a)
    (ySquare : y ^ 2 = algebraMap K L b)
    (productSquare : IsSquare (a * b)) :
    ∀ sigma : L ≃ₐ[K] L,
      galoisSqrtSign y sigma = galoisSqrtSign x sigma := by
  obtain ⟨c, cSquare⟩ := productSquare
  intro sigma
  have xySquare : (x * y) ^ 2 = (algebraMap K L c) ^ 2 := by
    calc
      (x * y) ^ 2 = x ^ 2 * y ^ 2 := by ring
      _ = algebraMap K L a * algebraMap K L b := by rw [xSquare, ySquare]
      _ = algebraMap K L (a * b) := (map_mul (algebraMap K L) a b).symm
      _ = algebraMap K L (c * c) := by rw [cSquare]
      _ = (algebraMap K L c) ^ 2 := by simp [pow_two]
  have productFixed : sigma (x * y) = x * y := by
    rcases sq_eq_sq_iff_eq_or_eq_neg.mp xySquare with positive | negative
    · rw [positive, sigma.commutes]
    · rw [negative, map_neg, sigma.commutes]
  have fixedEquivalent : sigma x = x ↔ sigma y = y := by
    constructor
    · intro xFixed
      apply mul_left_cancel₀ xNonzero
      calc
        x * sigma y = sigma x * sigma y := by rw [xFixed]
        _ = sigma (x * y) := (map_mul sigma x y).symm
        _ = x * y := productFixed
    · intro yFixed
      apply mul_right_cancel₀ yNonzero
      calc
        sigma x * y = sigma x * sigma y := by rw [yFixed]
        _ = sigma (x * y) := (map_mul sigma x y).symm
        _ = x * y := productFixed
  by_cases xFixed : sigma x = x
  · have yFixed := fixedEquivalent.mp xFixed
    simp [galoisSqrtSign, xFixed, yFixed]
  · have yNotFixed : sigma y ≠ y := by
      exact fun yFixed ↦ xFixed (fixedEquivalent.mpr yFixed)
    simp [galoisSqrtSign, xFixed, yNotFixed]

/-- Concrete Galois-sign projector for one duplicated radicand pair.  Equal
radicands give the duplicate character; nonsquare products with every other
radicand construct all required separators. -/
theorem galoisTwoCollisionSignCharacterProjector_isolates_pair
    {K L Index Point : Type*}
    [Field K] [Field L] [Algebra K L]
    [FiniteDimensional K L] [IsGalois K L]
    [Fintype Index] [DecidableEq Index] [AddCommGroup Point]
    (radicand : Index → K) (root : Index → L)
    (coefficient : Index → ℤ) (point : Index → Point)
    (chosen partner : Index) (distinct : chosen ≠ partner)
    (rootNonzero : ∀ index, root index ≠ 0)
    (twoNonzero : (2 : L) ≠ 0)
    (rootSquare : ∀ index,
      root index ^ 2 = algebraMap K L (radicand index))
    (sameRadicand : radicand partner = radicand chosen)
    (otherNonsquare : ∀ index,
      index ≠ chosen → index ≠ partner →
        ¬ IsSquare (radicand chosen * radicand index))
    (transformedRelation : ∀ sigma : L ≃ₐ[K] L,
      ∑ index, (galoisSqrtSign (root index) sigma * coefficient index) •
        point index = 0) :
    ((Fintype.card (L ≃ₐ[K] L) : ℤ) * coefficient chosen) • point chosen +
      ((Fintype.card (L ≃ₐ[K] L) : ℤ) * coefficient partner) • point partner = 0 := by
  classical
  apply twoCollisionSignCharacterProjector_isolates_pair
    (fun index sigma ↦ galoisSqrtSign (root index) sigma)
    coefficient point chosen partner distinct
  · intro index left right
    exact galoisSqrtSign_multiplicative
      (rootNonzero index) twoNonzero (rootSquare index) left right
  · intro index sigma
    exact galoisSqrtSign_squareOne (root index) sigma
  · intro sigma
    apply galoisSqrtSigns_eq_of_same_square
      (x := root chosen) (y := root partner) (rootSquare chosen)
    simpa only [sameRadicand] using rootSquare partner
  · intro index notChosen notPartner
    exact galoisSqrtSigns_separate_of_not_isSquare_mul
      (rootNonzero chosen) (rootNonzero index)
      (rootSquare chosen) (rootSquare index)
      (otherNonsquare index notChosen notPartner)
  · exact transformedRelation

/-- Galois-sign pair projector in the form needed for cubic collisions: the
two radicands need not be equal; it is enough that their product is a square
in the base field. -/
theorem galoisTwoCollisionSignCharacterProjector_isolates_pair_of_isSquare_mul
    {K L Index Point : Type*}
    [Field K] [Field L] [Algebra K L]
    [FiniteDimensional K L] [IsGalois K L]
    [Fintype Index] [DecidableEq Index] [AddCommGroup Point]
    (radicand : Index → K) (root : Index → L)
    (coefficient : Index → ℤ) (point : Index → Point)
    (chosen partner : Index) (distinct : chosen ≠ partner)
    (rootNonzero : ∀ index, root index ≠ 0)
    (twoNonzero : (2 : L) ≠ 0)
    (rootSquare : ∀ index,
      root index ^ 2 = algebraMap K L (radicand index))
    (pairSquare : IsSquare (radicand chosen * radicand partner))
    (otherNonsquare : ∀ index,
      index ≠ chosen → index ≠ partner →
        ¬ IsSquare (radicand chosen * radicand index))
    (transformedRelation : ∀ sigma : L ≃ₐ[K] L,
      ∑ index, (galoisSqrtSign (root index) sigma * coefficient index) •
        point index = 0) :
    ((Fintype.card (L ≃ₐ[K] L) : ℤ) * coefficient chosen) • point chosen +
      ((Fintype.card (L ≃ₐ[K] L) : ℤ) * coefficient partner) • point partner = 0 := by
  classical
  apply twoCollisionSignCharacterProjector_isolates_pair
    (fun index sigma ↦ galoisSqrtSign (root index) sigma)
    coefficient point chosen partner distinct
  · intro index left right
    exact galoisSqrtSign_multiplicative
      (rootNonzero index) twoNonzero (rootSquare index) left right
  · intro index sigma
    exact galoisSqrtSign_squareOne (root index) sigma
  · intro sigma
    exact galoisSqrtSigns_eq_of_isSquare_mul
      (rootNonzero chosen) (rootNonzero partner)
      (rootSquare chosen) (rootSquare partner) pairSquare sigma
  · intro index notChosen notPartner
    exact galoisSqrtSigns_separate_of_not_isSquare_mul
      (rootNonzero chosen) (rootNonzero index)
      (rootSquare chosen) (rootSquare index)
      (otherNonsquare index notChosen notPartner)
  · exact transformedRelation

/-- End-to-end `A = 0` collision bridge with the character hypotheses
discharged by an actual finite Galois extension. Equal radicands force the
duplicated character, while nonsquare products with all other radicands
provide the separating automorphisms. -/
theorem shortCubicGaloisPairCollision_implies_CMNormTorsion
    {K L Index : Type*}
    [Field K] [DecidableEq K] [Field L] [Algebra K L]
    [FiniteDimensional K L] [IsGalois K L]
    [Fintype Index] [DecidableEq Index]
    (B left right parameter y y' : K)
    [(shortCharacterCurve 0 0 B).IsElliptic]
    (hK : ringChar K ≠ 2) (twoNonzeroK : (2 : K) ≠ 0)
    (twoNonzeroL : (2 : L) ≠ 0)
    (cubicNonsingular : (27 : K) * B ^ 2 ≠ 0)
    (BNonzero : B ≠ 0) (leftNonzero : left ≠ 0)
    (rightNonzero : right ≠ 0) (scalesDistinct : left ≠ right)
    (squareInFunctionField : IsSquare
      (algebraMap (Polynomial K) (FractionRing (Polynomial K))
        ((shortCubicPolynomial 0 B).comp (Polynomial.C left * Polynomial.X) *
          (shortCubicPolynomial 0 B).comp
            (Polynomial.C right * Polynomial.X))))
    (leftNonsingular :
      (shortCharacterCurve 0 0 B).toAffine.Nonsingular
        (left * parameter) y)
    (rightNonsingular :
      (shortCharacterCurve 0 0 B).toAffine.Nonsingular
        (right * parameter) y')
    (radicand : Index → K) (root : Index → L)
    (coefficient : Index → ℤ)
    (point : Index → (shortCharacterCurve 0 0 B).toAffine.Point)
    (chosen partner : Index) (indicesDistinct : chosen ≠ partner)
    (chosenPoint : point chosen =
      WeierstrassCurve.Affine.Point.some
        (left * parameter) y leftNonsingular)
    (partnerPoint : point partner =
      WeierstrassCurve.Affine.Point.some
        (right * parameter) y' rightNonsingular)
    (rootNonzero : ∀ index, root index ≠ 0)
    (rootSquare : ∀ index,
      root index ^ 2 = algebraMap K L (radicand index))
    (sameRadicand : radicand partner = radicand chosen)
    (otherNonsquare : ∀ index,
      index ≠ chosen → index ≠ partner →
        ¬ IsSquare (radicand chosen * radicand index))
    (transformedRelation : ∀ sigma : L ≃ₐ[K] L,
      ∑ index, (galoisSqrtSign (root index) sigma * coefficient index) •
        point index = 0) :
    ∃ omega : K, ∃ sign : ℤ,
      omega ^ 2 + omega + 1 = 0 ∧ omega ≠ 1 ∧
      right = omega * left ∧ (sign = 1 ∨ sign = -1) ∧
      (((Fintype.card (L ≃ₐ[K] L) : ℤ) * coefficient chosen) ^ 2 -
          ((Fintype.card (L ≃ₐ[K] L) : ℤ) * coefficient chosen) *
            (((Fintype.card (L ≃ₐ[K] L) : ℤ) *
              coefficient partner) * sign) +
          (((Fintype.card (L ≃ₐ[K] L) : ℤ) *
            coefficient partner) * sign) ^ 2) • point chosen = 0 := by
  classical
  apply shortCubicPairCollision_implies_CMNormTorsion
    B left right parameter y y' hK twoNonzeroK cubicNonsingular BNonzero
    leftNonzero rightNonzero scalesDistinct squareInFunctionField
    leftNonsingular rightNonsingular
    (fun index sigma ↦ galoisSqrtSign (root index) sigma)
    coefficient point chosen partner indicesDistinct chosenPoint partnerPoint
  · intro index sigma tau
    exact galoisSqrtSign_multiplicative
      (rootNonzero index) twoNonzeroL (rootSquare index) sigma tau
  · intro index sigma
    exact galoisSqrtSign_squareOne (root index) sigma
  · intro sigma
    apply galoisSqrtSigns_eq_of_same_square
      (x := root chosen) (y := root partner) (rootSquare chosen)
    simpa only [sameRadicand] using rootSquare partner
  · intro index notChosen notPartner
    exact galoisSqrtSigns_separate_of_not_isSquare_mul
      (rootNonzero chosen) (rootNonzero index)
      (rootSquare chosen) (rootSquare index)
      (otherNonsquare index notChosen notPartner)
  · exact transformedRelation

/-- Two nonidentity cubic roots of unity are either equal or one is the
square of the other. This is the algebraic two-root classification used to
turn a third `A = 0` collision into a full cubic orbit. -/
theorem nonidentityCubeRoots_eq_or_eq_sq
    {K : Type*} [Field K] {omega rho : K}
    (omegaRelation : omega ^ 2 + omega + 1 = 0)
    (rhoRelation : rho ^ 2 + rho + 1 = 0) :
    rho = omega ∨ rho = omega ^ 2 := by
  have productZero : (rho - omega) * (rho + omega + 1) = 0 := by
    calc
      (rho - omega) * (rho + omega + 1) =
          (rho ^ 2 + rho + 1) - (omega ^ 2 + omega + 1) := by ring
      _ = 0 := by rw [rhoRelation, omegaRelation, sub_self]
  rcases mul_eq_zero.mp productZero with equal | conjugate
  · exact Or.inl (sub_eq_zero.mp equal)
  · right
    linear_combination conjugate - omegaRelation

/-- In a prime-field interval with `3X < p`, one `A = 0` cubic collision has
no third member. Consequently, after selecting its unique possible partner,
the chosen radicand has nonsquare product with every remaining interval
radicand. -/
theorem zmodShortCubicFunctionRadicand_mul_not_isSquare_of_A_eq_zero_outside_partner
    {Index : Type*} [Fintype Index]
    (modulus X : ℕ) [Fact modulus.Prime]
    (hChar : ringChar (ZMod modulus) ≠ 2)
    (B : ZMod modulus)
    (cubicNonsingular : (27 : ZMod modulus) * B ^ 2 ≠ 0)
    (BNonzero : B ≠ 0)
    (scale : Index → ℕ)
    (scalePositive : ∀ index, 0 < scale index)
    (scaleBound : ∀ index, scale index ≤ X)
    (scaleInjective : Function.Injective scale)
    (intervalShort : 3 * X < modulus)
    (chosen partner : Index) (indicesDistinct : chosen ≠ partner)
    (chosenPartnerSquare : IsSquare
      (shortCubicFunctionRadicand 0 B
          (scale chosen : ZMod modulus) *
        shortCubicFunctionRadicand 0 B
          (scale partner : ZMod modulus))) :
    ∀ index, index ≠ chosen → index ≠ partner →
      ¬ IsSquare
        (shortCubicFunctionRadicand 0 B
            (scale chosen : ZMod modulus) *
          shortCubicFunctionRadicand 0 B
            (scale index : ZMod modulus)) := by
  have scaleBelow : ∀ index, scale index < modulus := by
    intro index
    have := scaleBound index
    omega
  have castNonzero : ∀ index, (scale index : ZMod modulus) ≠ 0 := by
    intro index castZero
    have values := congrArg ZMod.val castZero
    rw [ZMod.val_natCast_of_lt (scaleBelow index), ZMod.val_zero] at values
    exact (Nat.ne_of_gt (scalePositive index)) values
  have castDistinct : ∀ left right, left ≠ right →
      (scale left : ZMod modulus) ≠ (scale right : ZMod modulus) := by
    intro left right distinct castEqual
    apply distinct
    apply scaleInjective
    have values := congrArg ZMod.val castEqual
    simpa [ZMod.val_natCast_of_lt (scaleBelow left),
      ZMod.val_natCast_of_lt (scaleBelow right)] using values
  have chosenPartnerPolynomialSquare : IsSquare
      (algebraMap (Polynomial (ZMod modulus))
        (FractionRing (Polynomial (ZMod modulus)))
        ((shortCubicPolynomial 0 B).comp
            (Polynomial.C (scale chosen : ZMod modulus) * Polynomial.X) *
          (shortCubicPolynomial 0 B).comp
            (Polynomial.C (scale partner : ZMod modulus) * Polynomial.X))) := by
    simpa only [shortCubicFunctionRadicand, ← map_mul] using
      chosenPartnerSquare
  obtain ⟨omega, omegaRelation, omegaNonidentity, partnerOrbit⟩ :=
    shortCubicPolynomial_scaled_pair_CM_ratio_of_isSquare hChar B
      (scale chosen : ZMod modulus) (scale partner : ZMod modulus)
      cubicNonsingular BNonzero (castNonzero chosen) (castNonzero partner)
      (castDistinct chosen partner indicesDistinct)
      chosenPartnerPolynomialSquare
  intro index indexNotChosen indexNotPartner candidateSquare
  have candidatePolynomialSquare : IsSquare
      (algebraMap (Polynomial (ZMod modulus))
        (FractionRing (Polynomial (ZMod modulus)))
        ((shortCubicPolynomial 0 B).comp
            (Polynomial.C (scale chosen : ZMod modulus) * Polynomial.X) *
          (shortCubicPolynomial 0 B).comp
            (Polynomial.C (scale index : ZMod modulus) * Polynomial.X))) := by
    simpa only [shortCubicFunctionRadicand, ← map_mul] using candidateSquare
  obtain ⟨rho, rhoRelation, rhoNonidentity, indexOrbit⟩ :=
    shortCubicPolynomial_scaled_pair_CM_ratio_of_isSquare hChar B
      (scale chosen : ZMod modulus) (scale index : ZMod modulus)
      cubicNonsingular BNonzero (castNonzero chosen) (castNonzero index)
      (castDistinct chosen index indexNotChosen.symm)
      candidatePolynomialSquare
  have rhoNotOmega : rho ≠ omega := by
    intro rhoEqual
    apply castDistinct index partner indexNotPartner
    rw [indexOrbit, partnerOrbit, rhoEqual]
  have rhoSquare : rho = omega ^ 2 :=
    (nonidentityCubeRoots_eq_or_eq_sq omegaRelation rhoRelation).resolve_left
      rhoNotOmega
  exact zmodNoFullCubicOrbitInShortInterval modulus X
    (scale chosen) (scale partner) (scale index) omega
    (scalePositive chosen) (scaleBound chosen) (scaleBound partner)
    (scaleBound index) intervalShort omegaRelation partnerOrbit (by
      calc
        (scale index : ZMod modulus) = rho * (scale chosen : ZMod modulus) :=
          indexOrbit
        _ = omega ^ 2 * (scale chosen : ZMod modulus) := by rw [rhoSquare]
        _ = omega * (omega * (scale chosen : ZMod modulus)) := by ring
        _ = omega * (scale partner : ZMod modulus) := by rw [partnerOrbit])

/-- Prime-field `A = 0` interval endpoint. A selected square-product
collision supplies the CM partner, `3X < p` proves that every other product
is nonsquare, and the actual finite-Galois sign action yields the explicit
CM norm-torsion relation. -/
theorem zmodShortCubicIntervalGaloisPairCollision_implies_CMNormTorsion
    (modulus X : ℕ) [Fact modulus.Prime]
    {L Index : Type*} [Field L]
    [Algebra (FractionRing (Polynomial (ZMod modulus))) L]
    [FiniteDimensional (FractionRing (Polynomial (ZMod modulus))) L]
    [IsGalois (FractionRing (Polynomial (ZMod modulus))) L]
    [Fintype Index] [DecidableEq Index]
    (hChar : ringChar (ZMod modulus) ≠ 2)
    (twoNonzeroBase : (2 : ZMod modulus) ≠ 0)
    (twoNonzeroExtension : (2 : L) ≠ 0)
    (B parameter y y' : ZMod modulus)
    [(shortCharacterCurve 0 0 B).IsElliptic]
    (cubicNonsingular : (27 : ZMod modulus) * B ^ 2 ≠ 0)
    (BNonzero : B ≠ 0)
    (scale : Index → ℕ)
    (scalePositive : ∀ index, 0 < scale index)
    (scaleBound : ∀ index, scale index ≤ X)
    (scaleInjective : Function.Injective scale)
    (intervalShort : 3 * X < modulus)
    (chosen partner : Index) (indicesDistinct : chosen ≠ partner)
    (pairSquare : IsSquare
      (shortCubicFunctionRadicand 0 B
          (scale chosen : ZMod modulus) *
        shortCubicFunctionRadicand 0 B
          (scale partner : ZMod modulus)))
    (leftNonsingular :
      (shortCharacterCurve 0 0 B).toAffine.Nonsingular
        ((scale chosen : ZMod modulus) * parameter) y)
    (rightNonsingular :
      (shortCharacterCurve 0 0 B).toAffine.Nonsingular
        ((scale partner : ZMod modulus) * parameter) y')
    (root : Index → L)
    (rootNonzero : ∀ index, root index ≠ 0)
    (rootSquare : ∀ index, root index ^ 2 =
      algebraMap (FractionRing (Polynomial (ZMod modulus))) L
        (shortCubicFunctionRadicand 0 B
          (scale index : ZMod modulus)))
    (coefficient : Index → ℤ)
    (point : Index → (shortCharacterCurve 0 0 B).toAffine.Point)
    (chosenPoint : point chosen =
      WeierstrassCurve.Affine.Point.some
        ((scale chosen : ZMod modulus) * parameter) y leftNonsingular)
    (partnerPoint : point partner =
      WeierstrassCurve.Affine.Point.some
        ((scale partner : ZMod modulus) * parameter) y' rightNonsingular)
    (transformedRelation : ∀ sigma :
        L ≃ₐ[FractionRing (Polynomial (ZMod modulus))] L,
      ∑ index, (galoisSqrtSign (root index) sigma * coefficient index) •
        point index = 0) :
    ∃ omega : ZMod modulus, ∃ sign : ℤ,
      omega ^ 2 + omega + 1 = 0 ∧ omega ≠ 1 ∧
      (scale partner : ZMod modulus) =
        omega * (scale chosen : ZMod modulus) ∧
      (sign = 1 ∨ sign = -1) ∧
      (((Fintype.card
            (L ≃ₐ[FractionRing (Polynomial (ZMod modulus))] L) : ℤ) *
          coefficient chosen) ^ 2 -
        ((Fintype.card
            (L ≃ₐ[FractionRing (Polynomial (ZMod modulus))] L) : ℤ) *
          coefficient chosen) *
          (((Fintype.card
              (L ≃ₐ[FractionRing (Polynomial (ZMod modulus))] L) : ℤ) *
            coefficient partner) * sign) +
        (((Fintype.card
            (L ≃ₐ[FractionRing (Polynomial (ZMod modulus))] L) : ℤ) *
          coefficient partner) * sign) ^ 2) • point chosen = 0 := by
  classical
  have scaleBelow : ∀ index, scale index < modulus := by
    intro index
    have := scaleBound index
    omega
  have castNonzero : ∀ index, (scale index : ZMod modulus) ≠ 0 := by
    intro index castZero
    have values := congrArg ZMod.val castZero
    rw [ZMod.val_natCast_of_lt (scaleBelow index), ZMod.val_zero] at values
    exact (Nat.ne_of_gt (scalePositive index)) values
  have castDistinct : ∀ left right, left ≠ right →
      (scale left : ZMod modulus) ≠ (scale right : ZMod modulus) := by
    intro left right distinct castEqual
    apply distinct
    apply scaleInjective
    have values := congrArg ZMod.val castEqual
    simpa [ZMod.val_natCast_of_lt (scaleBelow left),
      ZMod.val_natCast_of_lt (scaleBelow right)] using values
  have polynomialPairSquare : IsSquare
      (algebraMap (Polynomial (ZMod modulus))
        (FractionRing (Polynomial (ZMod modulus)))
        ((shortCubicPolynomial 0 B).comp
            (Polynomial.C (scale chosen : ZMod modulus) * Polynomial.X) *
          (shortCubicPolynomial 0 B).comp
            (Polynomial.C (scale partner : ZMod modulus) * Polynomial.X))) := by
    simpa only [shortCubicFunctionRadicand, ← map_mul] using pairSquare
  apply shortCubicPairCollision_implies_CMNormTorsion
    B (scale chosen : ZMod modulus) (scale partner : ZMod modulus)
    parameter y y' hChar twoNonzeroBase cubicNonsingular BNonzero
    (castNonzero chosen) (castNonzero partner)
    (castDistinct chosen partner indicesDistinct) polynomialPairSquare
    leftNonsingular rightNonsingular
    (fun index sigma ↦ galoisSqrtSign (root index) sigma)
    coefficient point chosen partner indicesDistinct chosenPoint partnerPoint
  · intro index sigma tau
    exact galoisSqrtSign_multiplicative
      (rootNonzero index) twoNonzeroExtension (rootSquare index) sigma tau
  · intro index sigma
    exact galoisSqrtSign_squareOne (root index) sigma
  · intro sigma
    exact galoisSqrtSigns_eq_of_isSquare_mul
      (rootNonzero chosen) (rootNonzero partner)
      (rootSquare chosen) (rootSquare partner) pairSquare sigma
  · intro index indexNotChosen indexNotPartner
    exact galoisSqrtSigns_separate_of_not_isSquare_mul
      (rootNonzero chosen) (rootNonzero index)
      (rootSquare chosen) (rootSquare index)
      (zmodShortCubicFunctionRadicand_mul_not_isSquare_of_A_eq_zero_outside_partner
        modulus X hChar B cubicNonsingular BNonzero scale scalePositive
        scaleBound scaleInjective intervalShort chosen partner indicesDistinct
        pairSquare index indexNotChosen indexNotPartner)
  · exact transformedRelation

end TensorBoundaryBounds
