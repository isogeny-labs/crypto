# Yield/rank frontier proof map

Date: 2026-11-18

This map applies the Lean instance gate before the two proof modules are
edited.  The statements are deliberately generic.  No theorem below asserts
an elliptic-curve yield law or a random-matrix rank law.

## Yield, solver visibility and censorship

1. **Structure above.** `Fintype` domains and codomains, `Finset` fibres and
   predicates on a finite domain.  The semantic object is a map `φ : τ → G`;
   a solver is represented only by the predicate selecting the domain elements
   it exposes.
2. **Theorem used as stated.** `IcNegative.sum_yield` and
   `IcNegative.total_yield_eq` from `IcNegative.YieldConservation`; the standard
   `Finset.card_le_card` theorem supplies subset monotonicity.
3. **Objects as instances.** The generic visible fibre is a filtered fibre of
   `φ`.  Two explicit maps `Bool → Bool` instantiate the same finite semantic
   structure: the constant map and the identity map.
4. **Assembly probe.** For any acceptance predicate, the visible fibre is a
   subset of the mathematical fibre, hence its cardinality is at most the
   mathematical yield.  Under pointwise completeness the two fibres are equal.
   Both facts use only `Finset.filter_subset`/extensionality.  The endpoint then
   applies `sum_yield` to the two Boolean maps and computes only their fibres.
5. **Missing components only.** A definition of solver-visible yield; the
   subset and completeness lemmas; explicit Boolean countermodels for
   mean-versus-shape, rejection of a real witness, and success-only retention.
6. **Inheritance table.** Fibre enumeration: `Finset.univ.filter`; total yield:
   `IcNegative.sum_yield`; equality under completeness: `Finset.ext`; finite
   countermodel arithmetic: kernel `decide` on `Bool` and `Nat`.

| statement | kind | owner | quantified structure | hypotheses |
|---|---|---|---|---|
| `visibleYieldAt_le_yieldAt` | law | `IcNegative.YieldVisibility` | finite map and decidable predicate | none |
| `visibleYieldAt_eq_of_complete` | law | same | finite map and predicate | pointwise completeness |
| `same_total_different_coverage` | endpoint/countermodel | same | two maps `Bool → Bool` | none |
| `reject_all_hides_real_witness` | endpoint/countermodel | same | identity map on `Bool` | none |
| `success_only_censors_failure` | endpoint/countermodel | same | complete Boolean population | none |

## Left/right-kernel scope

1. **Structure above.** Finite-dimensional function spaces over a field and
   the row-action linear map represented by `M : Fin n → Fin B → K`.
2. **Theorem used as stated.** `LeftKernel.exists_left_kernel` for `B < n`.
   No part of its finrank proof is copied.
3. **Object as an instance.** The zero matrix with `n = B + 1`, together with
   a coordinate vector `Pi.single j 1` in its right kernel.
4. **Assembly probe.** Apply `exists_left_kernel` to the zero matrix for the
   nonzero left-kernel vector.  Separately, direct simplification establishes the
   coordinate vector is a nonzero right-kernel vector.  Thus row surplus gives
   left-kernel existence but does not imply reusable right uniqueness.
5. **Missing components only.** A predicate for the right kernel, an unused-
   column lemma, and the zero-matrix endpoint combining that lemma with the
   existing left-kernel theorem.
6. **Inheritance table.** Left kernel: `LeftKernel.exists_left_kernel`; right
   action: finite sum; nonzero coordinate: `Pi.single`; zero-matrix action:
   `zero_mul` and `Finset.sum_const_zero`.

| statement | kind | owner | quantified structure | hypotheses |
|---|---|---|---|---|
| `unused_column_gives_nonzero_right_kernel` | law | `KernelModeCountermodels` | finite matrix over a field | an unused column |
| `surplus_can_have_both_nonzero_kernels` | endpoint/countermodel | same | zero `(B+1) × B` matrix | `0 < B` |

There are no new syntax types, no copied algebraic theorem, no raised limits,
and no `native_decide`.  The countermodels refute implications between scopes;
they do not refute a separately stated probabilistic law for a specified row
ensemble or solver.

## Hashed semantic relation rank

1. **Structure above.** Mathlib finite matrices over a field, finite row and
   column types, finite sets of matched tuples, and finite sums over a seed
   space.  The group operation is represented only by a supplied predicate
   `matches : Left → Right → Prop`; elliptic-curve arithmetic is not
   redefined in Lean.
2. **Theorem used as stated.** Mathlib
   `Matrix.rank_le_card_of_support_subset` bounds matrix rank by any finite set
   containing every nonzero row.  Standard `Fintype.card_le_of_injective` and
   finite-sum monotonicity supply the injection and averaging steps.
3. **Object as an instance.** The semantic relation matrix has entry one
   exactly when a left partial tuple and right partial tuple form a full
   decomposition.  Its finite set of matched left tuples is the row-support
   container, and its finite set of matching pairs is the decomposition set.
4. **Assembly probe.** Given a matched-left finset containing the row support
   and an injection from matched left tuples into complete pairs, apply the
   Mathlib rank theorem and card monotonicity to obtain
   `rank A ≤ #(complete pairs)`.  Sum that pointwise inequality over seeds;
   finite Markov counting gives
   `K * #{S | K ≤ rank A_S} ≤ ∑_S #decompositions(S)`.  A uniform lower bound
   then cannot exceed any supplied average decomposition bound.
5. **Missing components only.** The semantic zero-one matrix definition; the
   row-support containment proof; the witness-selection injection; the finite
   Markov inequality; and the uniform-average endpoint.  The exact hash
   expectation and real inequality below two remain the readable
   specialization of Bochum Lemmas 9 and 13, not a reimplementation of its
   probability model in Lean.
6. **Inheritance table.** Matrix rank:
   `Matrix.rank_le_card_of_support_subset`; nonzero-row container:
   `Function.support`; complete-pair injection: witness choice plus
   `Prod.fst`; cardinality: `Fintype.card_le_of_injective`; finite averaging:
   `Finset.sum_le_sum`; arithmetic cancellation: `Nat.le_of_mul_le_mul_right`.

| statement | kind | owner | quantified structure | hypotheses |
|---|---|---|---|---|
| `decompositionRelationMatrix` | definition | `TensorBoundaryBounds` | finite row/column types over `ℚ` | decidable match predicate |
| `decompositionRelationRankLeMatchedLeft` | endpoint | same | finite semantic relation matrix | none beyond decidability |
| `matchedLeftCardLeDecompositions` | instance obligation | same | matched rows and complete pairs | finite types |
| `decompositionRelationRankLeDecompositions` | endpoint | same | finite semantic relation matrix | finite types |
| `highRankSeedCountBound` | law | same | finite seed-indexed rank/count functions | pointwise rank/count bound |
| `uniformRankLeAverageSolutionBound` | endpoint | same | finite nonempty seed space | uniform rank floor and total-count bound |

The proof chain is:

```text
rank(A_S)
  ≤ # matched left tuples                         Mathlib row-support theorem
  ≤ # complete decompositions                    witness-selection injection

K · #{S : K ≤ rank(A_S)}
  ≤ ∑_{S : K ≤ rank(A_S)} rank(A_S)              threshold definition
  ≤ ∑_S #complete decompositions(S)               pointwise bound and inclusion
```

There is no composite unfolding, syntax type, raised limit or native
evaluation in this extension.  The report specializes the last line with the
source threshold; it does not promote the low-level BTNIR edge-flattening
matrix to the semantic matrix.

## No-subexponential-work goal

1. **Structure above.** Natural-valued work functions indexed by bit size,
   predicates selecting work functions admitted as correct for a problem, and eventual
   inequalities.  The study uses an explicit binary-work predicate: after
   raising work to any positive denominator `b`, it is eventually strictly
   below `2^(a*n)` for every positive numerator `a`.  This is an operational
   rational-rate definition; no claim of equivalence with every machine model
   is made.
2. **Theorem used as stated.** Standard natural-number power laws and order
   transitivity.  `Nat.one_lt_pow` supplies the constant-work instance.  The
   earlier `uniformRankLeAverageSolutionBound` is not reused as a work theorem:
   it has no solver or cost hypothesis.
3. **Object as an instance.** Constant-one work is the explicit
   subexponential instance.  On the one-element seed space, zero rank and zero
   solution count satisfy the semantic premises.  A solver predicate accepting
   exactly constant-one work supplies the logical countermodel.
4. **Assembly probe.** Define the two work predicates, check constant-one work,
   establish that a positive exponential lower bound contradicts the operational
   subexponential predicate, and instantiate the semantic premises at `PUnit`.
   The resulting model refutes the implication from semantic rank/count
   premises alone to a no-subexponential-solver conclusion.
5. **Missing components only.** The operational binary-work predicate; the
   positive exponential lower-bound predicate; the no-subexponential goal
   parameterized by a solver-correctness predicate; constant-one membership;
   conversion of an existing positive linear-exponent work floor into the
   asymptotic predicate; the generic bridge from exponential lower bounds; and
   the `PUnit` countermodel.  A real ECDLP algorithm semantics and a theorem
   mapping every correct solver to the low-level rank/message lower bound
   remain absent.
6. **Inheritance table.** Eventuality: explicit `∃ N, ∀ n >= N`; powers and
   multiplication: `Nat`; constant-work bound: `Nat.one_lt_pow`; contradiction:
   `lt_of_lt_of_le`; finite semantic premises: the existing natural-valued
   rank/count interface; seed space: standard `PUnit` instances.

| statement | kind | owner | quantified structure | hypotheses |
|---|---|---|---|---|
| `IsSubexponentialBinaryWork` | definition | `TensorBoundaryBounds` | `ℕ → ℕ` work function | explicit rational-rate criterion |
| `HasPositiveExponentialLowerBound` | definition | same | `ℕ → ℕ` work function | eventual positive rational-rate floor |
| `NoSubexponentialECDLPGoal` | definition | same | ECDLP-solver predicate on work functions | solver semantics deliberately supplied as a parameter |
| `constantOneWorkIsSubexponential` | instance obligation | same | constant-one work | none |
| `exponentialLowerBoundNotSubexponential` | law | same; upstream candidate for `crypto-lean` | arbitrary work function | positive exponential lower bound |
| `positiveLinearWorkLowerBoundHasPositiveExponentialLowerBound` | law | same; upstream candidate for `crypto-lean` | arbitrary work function | positive integer slope and pointwise work floor |
| `linearRankBridgeNotSubexponential` | endpoint | same | registered rank/message/work chain | family rank, separator, work-charge and positive-slope hypotheses |
| `noSubexponentialECDLPGoalOfExponentialLowerBound` | endpoint | same | arbitrary ECDLP-solver predicate | every accepted solver has an exponential floor |
| `semanticRankPremisesDoNotEstablishNoSubexponentialECDLPGoal` | endpoint/countermodel | same | one-element seed space and arbitrary ECDLP-solver predicate | semantic rank/count premises only |

The intended implication boundary is:

```text
every correct solver has a positive exponential work lower bound
  => no correct solver has subexponential binary work

semantic rank <= solution count and average solution count <= 2
  -/-> no correct solver has subexponential binary work
```

The first arrow needs an ECDLP-wide solver-to-work bridge.  The second arrow is
refuted by the explicit countermodel; it is not a proof that a real
subexponential ECDLP solver exists.

## Universal scheduling of fixed-arity candidates

1. **Structure above.** Natural-valued expected-work bounds indexed by input
   bit length, a countable family of candidate bounds, and the existing
   `IsSubexponentialBinaryWork` predicate.  The scheduler interface is a
   polynomial-simulation inequality: for every fixed candidate there is a
   constant overhead such that the universal expected-work bound is no larger
   than that overhead times a fixed power of the candidate bound.  A separate
   real-valued geometric-series statement represents the capped-repetition
   tail calculation.
2. **Theorem used as stated.** `Nat.pow_le_pow_left` transfers pointwise work
   domination through the rational-rate test.  Mathlib's geometric-series
   theorem supplies the finite or summable tail bound; no ECDLP algebra or
   probability theorem is re-derived in the study.
3. **Object as an instance.** The candidate family is the family of fixed-arity
   ECDLP work bounds.  The universal object is a verified dovetailing machine
   over all finite program descriptions.  The Lean endpoint does not assume a
   computable selector from arities to programs: the supplied simulation bound
   may be instantiated by any candidate program that occurs in the universal
   enumeration.
4. **Assembly probe.** Assume that, at every positive rational rate, one
   candidate and its polynomial-simulation overhead meet the corresponding
   eventual bound.  Rewrite the universal work through the supplied domination,
   raise both sides to the denominator power, and apply that candidate's bound.
   Separately, if each post-threshold stage is reached with probability at most
   `q^j` and costs at most a factor `B^j`, sum the geometric majorant with ratio
   `q*B < 1`.
5. **Missing components only.** Predicates for polynomial universal simulation
   and a vanishing-rate candidate family; their generic subexponential endpoint;
   and the geometric scheduler-tail lemma.  Still missing are the actual
   probabilistic-machine semantics, a checked universal-interpreter overhead,
   and an end-to-end theorem instantiating candidates with the paper's coverage,
   relation, planted-diagonal and recovery algorithms.
6. **Inheritance table.** Eventual rational-rate test:
   `IsSubexponentialBinaryWork`; power monotonicity: `Nat.pow_le_pow_left`;
   candidate choice: supplied family hypothesis; description enumeration and
   output verification: external machine-model obligations; scheduler tail:
   Mathlib geometric series over `ℝ`.

| statement | kind | owner | quantified structure | hypotheses |
|---|---|---|---|---|
| `HasPolynomialUniversalSimulationBound` | definition | `TensorBoundaryBounds` | universal and candidate natural work bounds | fixed simulation degree and per-candidate overhead |
| `HasVanishingRateCandidateFamily` | definition | same | candidate work family and simulation degree | eventual bound at every positive rational rate |
| `polynomialUniversalSchedulerIsSubexponential` | endpoint | same; upstream candidate for `crypto-lean` | universal and candidate work bounds | the two preceding interfaces |
| `geometricSchedulerTailBound` | law | same; upstream candidate for `crypto-lean` | nonnegative real stage ratio and scale | geometric ratio below one |

The proof chain for the asymptotic endpoint is:

```text
universal(n)
  <= overhead(m) * candidate(m,n)^degree             simulation interface

universal(n)^b
  <= (overhead(m) * candidate(m,n)^degree)^b         power monotonicity
  < 2^(a*n)                                           vanishing-rate family
```

The fixed-arity paper bound `p^(2/m+o(1))` would make the family premise true
for every fixed simulator degree by choosing `m` after the requested rational
rate.  That specialization is not claimed as Lean-verified until the
paper-to-machine cost bridge is supplied.
