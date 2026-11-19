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
   For a fixed numerator `c`, choose `m = c * degree * b + 1`; positivity of
   `a` makes `c * degree * b < a * m`, so the fixed-numerator interface supplies
   the vanishing-rate premise without assuming numerator `2`.
   Separately, if each post-threshold stage is reached with probability at most
   `q^j` and costs at most a factor `B^j`, sum the geometric majorant with ratio
   `q*B < 1`.
5. **Missing components only.** Predicates for polynomial universal simulation,
   a vanishing-rate candidate family and a fixed-numerator candidate family;
   their generic subexponential and fixed-numerator conversion endpoints; and
   the geometric scheduler-tail lemma.  Still missing are the actual
   probabilistic-machine semantics, a checked universal-interpreter overhead,
   and an end-to-end theorem instantiating candidates with the paper's coverage,
   relation, planted-diagonal and recovery algorithms. [pcc:unverified]
6. **Inheritance table.** Eventual rational-rate test:
   `IsSubexponentialBinaryWork`; power monotonicity: `Nat.pow_le_pow_left`;
   candidate choice: supplied family hypothesis; fixed-arity choice: natural
   multiplication monotonicity and `Nat.lt_succ_self`; description enumeration and
   output verification: external machine-model obligations; scheduler tail:
   Mathlib geometric series over `ℝ`.

| statement | kind | owner | quantified structure | hypotheses |
|---|---|---|---|---|
| `HasPolynomialUniversalSimulationBound` | definition | `TensorBoundaryBounds` | universal and candidate natural work bounds | fixed simulation degree and per-candidate overhead |
| `HasVanishingRateCandidateFamily` | definition | same | candidate work family and simulation degree | eventual bound at every positive rational rate |
| `HasFixedNumeratorCandidateFamily` | definition | same | fixed-arity candidate family, simulation degree and fixed numerator | eventual candidate bound whenever `numerator * degree * b < a * m` |
| `fixedNumeratorCandidateFamilyHasVanishingRate` | law | same; upstream candidate for `crypto-lean` | same | fixed-numerator family interface |
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

Either fixed-arity paper bound, dense `p^(3/m+o(1))` or planted sparse
`p^(2/m+o(1))`, would make the family premise true for every fixed simulator
degree by choosing `m` after the requested rational rate. That specialization
is not claimed as Lean-verified until the paper-to-machine cost bridge is
supplied. [pcc:unverified]

The same implication uses no special property of numerator `2`. For a fixed
numerator `c`, simulation degree `degree`, and requested rational rate `a/b`,
choose `m = c * degree * b + 1`. Since `a > 0`, this gives
`c * degree * b < a * m`. The fixed-numerator family interface then supplies
the exact eventual natural-power inequality required by
`HasVanishingRateCandidateFamily`. This assembly is the reason the dense
`p^(3/m+o(1))` Theorem 5 route suffices without sparse Wiedemann--Dixon.
[pcc:unverified]

## Near-uniform pair-event transfer

1. **Structure above.** A finite output type, a natural weight giving the
   number of uniform random seeds that produce each output, a Boolean pair
   event, a common fibre floor and a distortion constant.
2. **Theorem used as stated.** Finite sums over the output-pair type and natural
   multiplication monotonicity.
3. **Goal below.** A division-free comparison between the weighted pair-event
   count and the uniform pair-event count.
4. **Assembly probe.** Lower-bound every good pair's weight product by
   `lower^2`, sum over the good-pair filter, then combine
   `total <= distortion * card * lower` with squaring and multiplication by the
   good-pair count. Rearrange products to obtain
   `total^2 * goodCount <= distortion^2 * card^2 * weightedGood`.
5. **Missing components only.** The weighted-event lower bound and its
   distortion corollary. The curve sampler must separately expose a finite
   uniform seed space and give the identity a positive fibre.
6. **Inheritance table.** `Finset.sum_le_sum`, `Nat.mul_le_mul`,
   `Nat.pow_le_pow_left`, filtered-cardinality summation and semiring
   normalization.

| statement | kind | owner | quantified structure | hypotheses |
|---|---|---|---|---|
| `weightedPairEventLower` | law | `TensorBoundaryBounds`; upstream candidate for `crypto-lean` | finite output type, natural weights and pair event | common pointwise fibre floor |
| `nearUniformPairEventTransfer` | endpoint | same | same plus total seed count and distortion | total no larger than distortion times uniform baseline |
| `identityInclusiveWeight` | definition | same | finite output type with distinguished identity, base weights, and branch mass | none |
| `identityInclusiveWeight_floor` | law | same | same | base fibre floor away from the identity |
| `identityInclusiveWeight_total` | law | same | same | none |
| `identityInclusiveTotalUpper` | law | same | same plus distortion | positive distortion and base total bounded by `distortion * (card - 1) * lower` |
| `identityInclusiveNearUniformPairEventTransfer` | endpoint | same | same plus pair event | positive distortion, nonidentity fibre floor, and the base total bound |
| `rejectionSamplerWeight` | definition | same | finite seed and output types, one-trial `Option`-valued sampler | none |
| `rejectionSamplerAcceptedSeeds` | definition | same | finite seed type and one-trial sampler | none |
| `rejectionSamplerWeight_sum` | law | same | same | none |
| `uniformAcceptedFibres_card` | law | same | same plus common fibre size | every accepted output fibre has that size |
| `uniformAcceptedPairEventWeight` | law | same | same plus pair event | every accepted output fibre has the common size |
| `rejectionTrialTailHasSum` | law | same | natural total and accepted one-trial seed counts | positive accepted count and accepted count no larger than total |
| `uniqueAcceptedSeedWeight_eq_one` | law | same | one-trial sampler and an encoder from outputs to seeds | decoding the encoded output succeeds; every successful decoding seed equals the encoded seed |
| `rejectionTrialExpectation_le` | law | same | total, accepted, and cap | positive accepted count and `total <= cap * accepted` |
| `affineSamplerRoot` | definition | same | Weierstrass curve over a field and x-coordinate | none; noncomputably chooses a nonsingular root when one exists |
| `affineSamplerRoot_spec` | law | same | same plus a supplied nonsingular affine point | existence of the supplied point |
| `AffineSamplerCoordinates` | definition | same | Weierstrass curve over a field | none |
| `instFintypeAffineSamplerCoordinates` | instance | same | finite field and affine curve | none |
| `affineSamplerCoordinatesEquivPoint` | equivalence | same | same | none |
| `instFintypeAffineSamplerPoint` | instance | same | finite field and affine curve | none |
| `affineSampler_point_card` | law | same | finite field and affine curve | none |
| `affineSignedTrial` | definition | same | Weierstrass curve and signed-x seed | none |
| `affineSignedEncode` | definition | same | nonsingular affine coordinates | none |
| `affineSignedTrial_encode` | law | same | same | none |
| `affineSignedTrial_unique` | law | same | signed-x seed and accepted affine output | the trial accepts that output |
| `affineSignedTrial_weight_eq_one` | law | same | finite field and affine curve | none |
| `affineSignedTrial_accepted_card` | law | same | finite field and affine curve | none |
| `affineSignedTrial_pairEvent_uniform` | endpoint | same | finite field, affine curve, and pair event | none |
| `affineSignedTrialTailHasSum` | law | same | finite field and affine curve | at least one affine point |
| `affineSignedTrialExpectation_le_four` | law | same | finite field and affine curve | positive affine-point count and `2 * card K <= 4 * affinePointCount` |
| `affineFullSignedTrial` | definition | same | finite field, affine curve, identity-or-signed-x seed | none |
| `affineFullSignedEncode` | definition | same | full identity-or-affine coordinate output | none |
| `affineFullSignedTrial_encode` | law | same | full curve output | none |
| `affineFullSignedTrial_unique` | law | same | full sampler seed and accepted output | the trial accepts that output |
| `affineFullSignedTrial_weight_eq_one` | law | same | finite field and affine curve | none |
| `affineFullSignedTrial_pairEvent_uniform` | endpoint | same | finite field, affine curve, and full-curve pair event | none |
| `affineFullSignedTrial_accepted_card` | law | same | finite field and affine curve | none |
| `affineFullSignedTrialTailHasSum` | law | same | finite field and affine curve | none |
| `affineFullSignedTrialExpectation_le_four` | law | same | finite field and affine curve | `2 * card K + 1 <= 4 * card W.Point` |
| `hasseSquareImpliesHalfFieldTrace` | arithmetic bridge | same | natural field size and nonnegative trace deficit | field size at least 16 and `trace^2 <= 4 * fieldSize` |
| `hasseTraceBoundGivesSamplerFour` | arithmetic bridge | same | natural field size, point count, and trace deficit | field size at least 16, `fieldSize + 1 <= pointCount + trace`, and `trace^2 <= 4 * fieldSize` |
| `affineFullSignedTrialExpectation_le_four_of_hasseSquare` | endpoint | same | finite field, affine curve, and natural trace deficit | field size at least 16, `card K + 1 <= card W.Point + trace`, and `trace^2 <= 4 * card K` |
| `integerHasseGivesSamplerFour` | arithmetic bridge | same | natural field size and point count | field size at least 16 and the integral squared Hasse inequality `((q+1-N):Int)^2 <= 4q` |
| `affineFullSignedTrialExpectation_le_four_of_atlasHasse` | endpoint | same | finite field and elliptic affine curve | field size at least 16; imports the source-pinned Atlas Hasse theorem |
| `threeModFourSqrt` | definition | same | finite field and field element | none |
| `threeModFourSqrt_sq` | law | same | finite field, field element, and square witness | field cardinality is 3 modulo 4 |
| `RingCounted.squareCached` | definition | same | counted scalar | none |
| `RingCounted.mulCachedRight` | definition | same | counted scalar and available scalar | none |
| `countedBinaryPow` | executable definition | same | monoid element and natural exponent | none |
| `countedBinaryPow_zero` | law | same | same | none |
| `countedBinaryPow_bit` | law | same | monoid element and a valid nonzero binary digit | none |
| `countedBinaryPow_value` | law | same | same | none |
| `countedBinaryPow_ops_le` | cost bound | same | same | none; at most twice the binary length |
| `countedThreeModFourSqrt` | executable definition | same | finite field and field element | none |
| `countedThreeModFourSqrt_value` | law | same | same | none |
| `countedThreeModFourSqrt_ops_le` | cost bound | same | same | none; at most twice the binary length of `(|K|+1)/4` |
| `countedThreeModFourSqrt_ops_le_fieldSize` | cost bound | same | finite field and field element | field cardinality is 3 modulo 4; at most twice the binary length of `|K|` |
| `countedThreeModFourSqrtBitWork` | definition | same | finite field, field element, and supplied scalar multiplication bit cost | none |
| `countedThreeModFourSqrtBitWork_le` | cost bound | same | same plus modulus-bit proxy and polynomial scalar-cost parameters | field cardinality is 3 modulo 4 and the supplied scalar multiplication cost obeys the displayed polynomial bound |
| `countedThreeModFourSqrt_sq` | law | same | finite field, field element, and square witness | field cardinality is 3 modulo 4 |
| `shortCurveRhs` | definition | same | affine Weierstrass curve and x-coordinate | none |
| `shortThreeModFourRoot` | executable definition | same | finite-field affine curve and x-coordinate | none |
| `shortThreeModFourRoot_spec` | law | same | elliptic affine curve, x-coordinate, and supplied nonsingular point | field cardinality is 3 modulo 4 and curve coefficients `a1 = a3 = 0` |

The identity-inclusive adapter adds exactly `lower` seeds mapping to the
distinguished identity.  Its total-size theorem derives the original
near-uniform interface from a base sampler supported on the other
`card-1` outputs, and the endpoint applies `nearUniformPairEventTransfer`
without requiring the base sampler itself to hit the identity.  Instantiating
the base weights and their bounds with the paper's concrete affine encoding
remains a curve-specific obligation.

The rejection-sampling layer models one trial as `Seed -> Option Output`.
It proves that the accepted-seed cardinality is the sum of the output fibre
cardinalities, that equal nonzero fibres make the conditioned accepted output
exactly uniform, and that every pair event receives exactly the square of the
common fibre multiplier.  Its geometric tail theorem gives the exact expected
trial series `total / accepted`.  For the paper's affine sampler, the remaining
specialization must exhibit the finite `2p` signed-x seed type, the bijection
between accepted seeds and affine points (including the half-accepted zero
root), and the curve-cardinality lower bound used to make the expectation
constant. [pcc:unverified]

The concrete signed-x layer works for any Mathlib affine Weierstrass curve.
It chooses one nonsingular y-root for each soluble x, uses `Bool` to select
that root or `negY`, and rejects one of the two bits when the roots coincide.
The decoder laws prove exactly one accepted `K x Bool` seed per nonsingular
affine coordinate pair.  The coordinate sigma type is explicitly equivalent
to nonzero affine points (via `Option` for the identity), so the successful
seed count is the number of affine points and every conditioned pair event is
exactly uniform.  Executable square-root cost and the Hasse inequality needed
for the constant retry bound were initially separate obligations.  The
three-mod-four short-curve root and source-pinned Atlas Hasse theorem below
close them on that curve class. [pcc:unverified]

Adding one distinguished identity seed gives a single finite rejection sampler
`Option (K x Bool) -> Option (Option AffineSamplerCoordinates)`.  The same
encoder/uniqueness method proves every full curve output has exactly one
accepted seed.  Through `affineSamplerCoordinatesEquivPoint`, its accepted
count is exactly `card W.Point`, its seed count is `2 * card K + 1`, and every
conditioned full-curve pair event is uniform with no distortion loss.  The
exact retry expectation is `(2 * card K + 1) / card W.Point`; a Hasse-derived
cardinality inequality was the sole remaining premise for the checked
four-trial corollary.  The arithmetic bridge derives that inequality from the
usual integral squared Hasse bound once the field has at least 16 elements.
The original Mathlib/CryptoLean tree supplied no Hasse theorem, but the project
now pins Meta's Atlas development at
`a4557099d45c620a038e90a9c4b4d9ab08294b81`.  Atlas's Manin proof supplies
the squared bound, and `affineFullSignedTrialExpectation_le_four_of_atlasHasse`
composes it into an assumption-free elliptic-curve endpoint. Claim
`PCC-20261119-c7c015` binds that endpoint and passes its local verifier. Claims
`PCC-20261119-c1ad13`, `PCC-20261119-e5e1f9`, and
`PCC-20261119-54ee4b` bind the curve-level Hasse arithmetic endpoint, counted
square-root bit-work theorem, and short-curve root interface respectively;
all three pass their local Lean verifiers. [pcc:unverified]

## Dense cubic row-cost exponent

1. **Structure above.** Natural field-size proxy `p`, factor-base scale `X`,
   fixed arity `m`, and dense row dimension `rows`.
2. **Theorem used as stated.** Monotonicity of natural powers and the semiring
   identities `(a*b)^n = a^n*b^n` and `a^(r*s) = (a^r)^s`.
3. **Goal below.** An exact integer form of the dense `p^(3/m)` exponent.
4. **Assembly probe.** Raise `rows <= 3X` to `3m`, expand the product, rewrite
   `X^(3m)` as `(X^m)^3`, and raise `X^m <= p` to the third power.
5. **Missing components only.** The natural-power inequality. Executable ring
   operation and bit-cost bounds remain separate machine obligations.
6. **Inheritance table.** `Nat.pow_le_pow_left`, `mul_pow`, `pow_mul`, and
   multiplication monotonicity.

| statement | kind | owner | quantified structure | hypotheses |
|---|---|---|---|---|
| `denseCubicRowBoundAtFixedArity` | law | `TensorBoundaryBounds`; upstream candidate for `crypto-lean` | natural `p`, `X`, `m`, and row count | `rows <= 3X`, `X^m <= p` |

## Bird planted-dense fallback

1. **Structure above.** A square matrix over a commutative ring, Mathlib's
   verified Bird determinant, the adjugate formula, and fixed-arity row bounds.
   [pcc:unverified]
2. **Theorem used as stated.** `BirdDet.det_eq_birdDet`,
   `Matrix.adjugate_apply`, `Matrix.nonsing_inv_apply`, and
   `ZMod.inv_coe_unit`; natural power monotonicity for the cost exponent.
3. **Goal below.** Replace every cofactor determinant in the adjugate by the
   verified Bird recurrence, and record the conservative numerator-6 exponent
   obtained from `n^2` determinant calls at advertised `O(n^4)` cost.
4. **Assembly probe.** Define `birdAdjugate A i j` by applying
   `BirdDet.birdDet` to the row-updated matrix used by
   `Matrix.adjugate_apply`; extensionality and Bird correctness give equality
   with `A.adjugate`. Scale it by the concrete `ZMod` inverse of the Bird
   determinant and, under `IsUnit A.det`, identify that scalar with Mathlib's
   nonsingular inverse scalar. Separately raise `rows <= 3X` to `6m`, rewrite
   `X^(6m) = (X^m)^6`, and apply `X^m <= p`.
5. **Missing components only.** The Bird determinant, adjugate, executable
   `ZMod` inverse equality, and numerator-6 natural-power bound are checked.
   Still missing is an eager array implementation and kernel-checked operation
   counter: Mathlib's source comment states `O(n^4)`, but its function-valued
   recurrence does not itself specify an evaluation cost model.
6. **Inheritance table.** `BirdDet.det_eq_birdDet`,
   `Matrix.adjugate_apply`, `Matrix.nonsing_inv_apply`, `ZMod.inv_coe_unit`,
   matrix extensionality, `Nat.pow_le_pow_left`, `mul_pow`, and `pow_mul`.

| statement | kind | owner | quantified structure | hypotheses |
|---|---|---|---|---|
| `birdAdjugate` | definition | `TensorBoundaryBounds`; upstream candidate for `crypto-lean` | finite square matrix over a commutative ring | none |
| `birdAdjugate_eq_adjugate` | law | same | same | none |
| `birdZModInverse` | definition | same | square matrix over `ZMod q` | none |
| `birdZModInverse_eq_inv` | law | same | square matrix over `ZMod q` | `IsUnit A.det` |
| `birdSixthPowerRowBoundAtFixedArity` | law | same | natural `p`, `X`, `m`, and row count | `rows <= 3X`, `X^m <= p` |

## Eager Bird refinement

1. **Structure above.** Length-`n^2` vectors backed by Lean arrays, conversion
   between those vectors and `Matrix (Fin n) (Fin n) R`, and Mathlib's Bird
   matrix recurrence. [pcc:unverified]
2. **Theorem used as stated.** `Matrix.ofArray_ofFn`,
   `BirdDet.birdDetSpec_eq_det`, and function-iterate induction.
3. **Goal below.** Materialize every Bird recurrence state with
   `Vector.ofFn`, prove matrix/vector round trips, prove one eager step refines
   `BirdDet.stepEntry`, and conclude that the eager determinant has the same
   value as `Matrix.det`. Instrument addition, multiplication, and negation,
   prove the exact `3(n-i-1)+3` count for one output entry, sum it over the
   materialized state, and obtain an explicit quartic full-determinant budget.
4. **Assembly probe.** Flatten a matrix in row-major order, rebuild it with
   `Matrix.ofArray`, use the existing round-trip theorem, then induct over the
   `k` eager steps in dimension `k+1`.
5. **Missing components only.** The eager value refinement, scalar-ring
   operation counter, and `n^2`-cofactor aggregation. Operand bit-cost bounds,
   array indexing/allocation costs, and the cost of concrete `ZMod` inversion
   remain separate obligations.
6. **Inheritance table.** `Matrix.ofArray_ofFn`,
   `BirdDet.birdDetSpec_eq_det`, `Function.iterate_succ_apply`, matrix
   extensionality, and vector size invariants.

| statement | kind | owner | quantified structure | hypotheses |
|---|---|---|---|---|
| `BirdFlatMatrix` | definition | `TensorBoundaryBounds`; upstream candidate for `crypto-lean` | commutative ring and dimension | none |
| `birdMatrixToFlat` | definition | same | finite square matrix | none |
| `birdFlatToMatrix` | definition | same | length-`n^2` vector | none |
| `birdFlatToMatrix_matrixToFlat` | law | same | finite square matrix | none |
| `eagerBirdStep` | definition | same | two length-`n^2` vectors | none |
| `eagerBirdStep_refines` | law | same | two length-`n^2` vectors | none |
| `eagerBirdDet` | definition | same | length-`n^2` vector | none |
| `eagerBirdDet_eq_det` | law | same | finite square matrix via flattened input | none |
| `RingCounted` | definition | same | scalar value paired with natural operation count | none |
| `countedBirdSumFrom_value` | law | same | counted suffix sum over a commutative ring | none |
| `countedBirdSumFrom_ops` | law | same | counted suffix sum and constant per-term cost | valid suffix and per-term cost hypotheses |
| `countedBirdStepEntry_value` | law | same | instrumented Bird scalar entry | none |
| `countedBirdStepEntry_ops` | law | same | instrumented Bird scalar entry | valid row index |
| `eagerBirdStepRingOps_le` | law | same | dimension | none |
| `eagerBirdDetRingOps_le_quartic` | law | same | dimension | none |
| `eagerBirdAdjugateRingOps` | definition | same | dimension | none |
| `eagerBirdAdjugateRingOps_le_sixth` | law | same | dimension | none |
| `eagerBirdZModInverseRingOps` | definition | same | dimension | none |
| `eagerBirdZModInverseRingOps_le` | law | same | dimension | none |
| `eagerBirdZModInverseBitWork` | definition | same | dimension and supplied scalar-operation/inversion bit costs | none |
| `eagerBirdZModInverseBitWork_le` | law | same | dimension, modulus bit length, fixed scalar and inversion polynomial parameters | supplied scalar-operation and inversion costs satisfy the stated polynomial bounds |
| `denseDot` | definition | same | finite vectors over a commutative ring | none |
| `denseDot_eq_sum` | law | same | same | none |
| `countedDenseDot` | definition | same | same | none |
| `countedDenseDot_value` | law | same | same | none |
| `countedDenseDot_ops` | law | same | inner dimension | none |
| `eagerDenseMatrixMul` | definition | same | finite rectangular matrices | none |
| `eagerDenseMatrixMul_eq_mul` | law | same | same | none |
| `eagerDenseMatrixMulRingOps` | definition | same | row, inner, and column dimensions | none |
| `eagerDenseMatrixMulRingOps_eq` | law | same | finite rectangular matrices | none |
| `eagerBirdPlantedKernelRingOps` | definition | same | planted and remainder dimensions | none |
| `eagerBirdPlantedKernelRingOps_le` | law | same | planted and remainder dimensions | none |
| `eagerBirdPlantedKernelBitWork` | definition | same | dimensions and supplied scalar-operation/inversion bit costs | none |
| `eagerBirdPlantedKernelBitWork_le` | law | same | dimensions, modulus bit length, fixed scalar and inversion polynomial parameters | supplied scalar-operation and inversion costs satisfy the stated polynomial bounds |

The inverse budget counts one determinant for the scalar denominator, all
`n^2` row-update determinants used by `birdAdjugate`, `n^2` final scalar
multiplications, and one scalar inverse.  The bit-work theorem is deliberately
conditional: Mathlib implements `ZMod.inv` through `Nat.gcdA`, but the pinned
library supplies no Turing-machine or other formal bit-runtime theorem for
that extended-GCD implementation.  Thus the new law isolates that missing
arithmetic premise instead of attributing an unproved runtime to evaluator
execution.  Array access, allocation, and integer-indexing costs remain a
separate machine-model obligation.

The rectangular postprocessing uses a structurally recursive dense dot
product.  Its counted erasure theorem binds the counter to the value, its
operation theorem gives exactly `2f` charged operations per entry (one
multiplication and one addition per term, including the zero accumulator), and
the matrix theorem identifies the materialized entry formula with
`Matrix.mul`.  Summing over all `s*f` output entries and charging their
negations closes the scalar-operation part of `-R * B^-1`; memory and indexing
remain explicitly outside the model.

## Executable planted kernel

1. **Structure above.** A unit-determinant square block `B` over `ZMod q`, a
   rectangular remainder block `R`, the checked executable Bird inverse, and
   Mathlib's block matrix constructors. [pcc:unverified]
2. **Theorem used as stated.** `birdZModInverse_eq_inv`,
   `Matrix.fromCols_mul_fromRows`, and `Matrix.nonsing_inv_mul`.
3. **Goal below.** Replace the noncomputable `B⁻¹` in the upstream planted
   kernel `(-R B⁻¹ | I)` by `birdZModInverse B`, prove equality with the
   canonical planted kernel under `IsUnit B.det`, and prove both annihilation
   and right-inverse laws directly for the executable matrix.
4. **Assembly probe.** Rewrite the executable inverse to `B⁻¹`, expand the
   block products, and apply the nonsingular inverse multiplication law.
5. **Missing components only.** Algebraic binding of the executable inverse to
   the upstream planted-kernel formula. Costs for `R * B⁻¹`, array management,
   scalar bit operations, and the concrete upstream-module substitution remain
   separate obligations.
6. **Inheritance table.** `birdZModInverse_eq_inv`,
   `Matrix.fromCols_mul_fromRows`, `Matrix.neg_mul`, `Matrix.mul_assoc`, and
   `Matrix.nonsing_inv_mul`.

| statement | kind | owner | quantified structure | hypotheses |
|---|---|---|---|---|
| `executableBirdPlantedKernel` | definition | `TensorBoundaryBounds`; upstream candidate for `crypto-lean` | square and rectangular matrices over `ZMod q` | none |
| `executableBirdPlantedKernel_eq` | law | same | same | `IsUnit B.det` |
| `executableBirdPlantedKernel_mul` | law | same | same | `IsUnit B.det` |
| `executableBirdPlantedKernel_rightInv` | law | same | same | `IsUnit B.det` |
| `executableBirdPlantedKernel_spec` | law | same | same | `IsUnit B.det` |
| `executableBirdKerMat` | definition | same | upstream-shaped integer planted and remainder blocks | none |
| `executableBirdKerMat_eq` | law | same | same | reduced planted determinant is a unit |
| `executableBirdKerMat_spec` | law | same | same | reduced planted determinant is a unit |

## First-moment coverage repair

1. **Structure above.** A finite seed space, a natural-valued factor-base size
   on each seed, a positive scale `X`, finite sums and filtered cardinalities.
   A seed is called large when `X <= 2 * value(seed)`.
2. **Theorem used as stated.** `Finset.sum_filter_add_sum_filter_not` partitions
   the total by the large-value predicate; `Finset.sum_le_sum` and natural
   multiplication monotonicity bound the two parts. The existing
   `highRankSeedCountBound` supplies finite Markov counting for the energy
   function. No character sum or elliptic-curve fact is re-proved in these
   generic lemmas.
3. **Object as an instance.** Seeds are nonzero dilation parameters and
   `value(u)` is the signed factor-base cardinality. The checked Atlas-Hasse
   character-sum bridge supplies `3 X #Seed <= 4 sum value`; the checked
   quadratic-fibre bound is `value(u) <= 2X`.
4. **Assembly probe.** Split the doubled total sum into large and small seeds.
   Each large summand is at most `4X`; each small summand is at most `X` by the
   definition of the filter. Compare this upper bound with four times the
   supplied first-moment lower bound and cancel positive `X` to get
   `#Seed <= 6 #large`. Apply `highRankSeedCountBound` at threshold `12μ`
   to the energy function, use `sum energy <= μ #Seed`, and cancel positive
   `μ` to obtain `12 #bad <= #Seed`. The existing event-intersection lemma
   then gives the `1/12` joint event in one endpoint application.
   [pcc:unverified]
5. **Missing components only.** The corrected Semaev nondegeneracy argument
   and its resulting total-energy estimate.  The character sum, concrete
   factor-base first moment, finite Markov step, and event intersection are
   now checked locally.
6. **Inheritance table.** Population split:
   `Finset.sum_filter_add_sum_filter_not`; large-part cardinality:
   `Finset.sum_const`; pointwise bounds: supplied upper bound and filter
   membership; cancellation: `Nat.le_of_mul_le_mul_left` with `X > 0`.

| statement | kind | owner | quantified structure | hypotheses |
|---|---|---|---|---|
| `firstMomentForcesLargeValueSeeds` | law | `TensorBoundaryBounds`; upstream candidate for `crypto-lean` | finite seed-indexed natural values | positive scale, pointwise `2X` upper bound, scaled `3/4` mean lower bound |
| `twelveMarkovBadSetBound` | endpoint | same | finite seed-indexed natural energies | positive mean scale and total energy at most scale times population |
| `oneSixthEventSurvivesOneTwelfthBadSet` | law | same; upstream candidate for `crypto-lean` | finite seed space and two event finsets | event size at least `1/6`, bad-set size at most `1/12` |
| `firstMomentAndEnergyGiveOneTwelfthGoodSeeds` | endpoint | same | finite factor-base and energy functions | hypotheses of the first-moment and total-energy interfaces |
| `dilationLargeFactorBaseSeeds_of_card_ge_324` | endpoint | same | nonsingular short curve and positive finite set of nonzero abscissas | odd characteristic and field cardinality at least `324` |
| `dilationAndEnergyGiveOneTwelfthGoodSeeds_of_card_ge_324` | endpoint | same | the concrete dilated factor base and a supplied natural-valued energy | previous row plus a positive mean scale and total-energy upper bound |

The exact checked conclusions are `card Seed <= 6 * card {seed | X <= 2 *
value seed}` and, after removing a bad set of cardinality at most `1/12`,
`card Seed <= 12 * card(good)`. Thus the corrected first-moment and energy
events intersect with finite uniform probability at least `1/12` once their
source estimates are supplied. [pcc:unverified]

## Hasse bridge for the first-moment character sums

1. **Structure above.** A finite field `K` of odd characteristic, coefficients
   `a₂ a₄ a₆ : K`, the short Weierstrass model
   `Y² = X³ + a₂X² + a₄X + a₆`, and Mathlib's integer-valued quadratic
   character.  The model is assumed elliptic only where Atlas's Hasse theorem
   is invoked. [pcc:unverified]
2. **Theorem used as stated.** `quadraticChar_card_sqrts` identifies the size
   of every `Y`-fibre with `1 + χ(rhs)`.  Atlas's source-pinned
   `hasse_bound` bounds the square of `|K| + 1 - #E(K)` by `4|K|`.
3. **Object as an instance.** The first sum uses coefficients `(0,A,B)`.
   For `B ≠ 0`, inversion in the second paper sum sends
   `χ(x(x³+Ax+B))` to `χ(1+Az²+Bz³)` on `Kˣ`; scaling
   `(X,Y)=(Bz,By)` gives the short model
   `Y²=X³+AX²+B²`.  Its discriminant is `B²` times the original short
   curve discriminant, so the original nonsingularity and `B ≠ 0` make it
   elliptic.
4. **Assembly probe.** Count affine equation solutions by `X`-fibres, adjoin
   the point at infinity, rearrange the resulting integer equality, and
   substitute it into Atlas Hasse.  Separately prove the inversion/scaling
   identity for the nonzero-domain character sum.  These steps produce two
   square bounds without appealing to an unformalized generic Weil theorem.
5. **Missing components only.** None for the character-sum/first-moment slice.
   Both the `B = 0` conic branch and the `B ≠ 0` elliptic branch, the exact
   square-map average, integral radius conversion, and the clean
   `|K| >= 324` specialization are checked.  Total energy is a separate
   Semaev-nondegeneracy obligation.
6. **Inheritance table.** `quadraticChar_card_sqrts`,
   `WeierstrassCurve.Affine.pointEquiv`, `Finite.card_option`,
   `WeierstrassCurve.Affine.equation_iff`, and
   `MathlibExt.AlgebraicGeometry.EllipticCurve.HasseBoundWanted.hasse_bound`.

| statement | kind | owner | quantified structure | hypotheses |
|---|---|---|---|---|
| `shortCubicCharacterSum` | definition | `TensorBoundaryBounds`; upstream candidate for `crypto-lean` | finite field and three coefficients | none |
| `shortCharacterCurve` | definition | same | field and three coefficients | none |
| `shortCharacterCurve_point_card_eq_card_add_one_add_characterSum` | law | same | finite odd-characteristic field | curve elliptic |
| `shortCubicCharacterSum_sq_le` | endpoint | same | finite odd-characteristic field | curve elliptic |
| `quarticCharacterSum_eq_transformedCubic_sub_one` | law | same | finite odd-characteristic field, short coefficients `A,B` | `B ≠ 0` |
| `transformedShortCurveIsElliptic` | law | same | field, short coefficients `A,B` | original discriminant nonzero and `B ≠ 0` |
| `quarticCharacterSum_add_one_sq_le_of_nonsingular` | endpoint | same | finite odd-characteristic field, short coefficients `A,B` | original discriminant nonzero |

The implemented quartic endpoint is stronger than the provisional table: it
also covers `B = 0` by an explicit equivalence between `Y²=X²+A` and the
nonzero field elements, and exposes the unified declaration
`quarticCharacterSum_add_one_sq_le_of_nonsingular`.  The next exact averaging
layer uses these declarations:

| statement | kind | owner | quantified structure | hypotheses |
|---|---|---|---|---|
| `sum_square_eq_characterWeighted` | law | `TensorBoundaryBounds` | finite odd-characteristic field and integer-valued function | none beyond odd characteristic |
| `sum_nonzero_inverseSquare_eq_characterWeighted` | law | same | nonzero field elements and integer-valued function | none beyond odd characteristic |
| `dilationCharacterTotal_eq_cubic_add_quartic` | law | same | nonsingular short curve and nonzero abscissa `t` | odd characteristic |
| `dilationFactorBaseFirstMoment` | endpoint | same | finite finset of nonzero abscissas | explicit integral square-radius and density inequalities |
| `dilationLargeFactorBaseSeeds_of_card_ge_324` | endpoint | same | finite finset of nonzero abscissas | nonsingular short curve, odd characteristic, positive base and `|K| >= 324` |

## Dilation polynomial root-count bridge

1. **Structure above.** A finite field, nonzero dilation parameters, a tuple
   index, a tuple-dependent bad predicate, and a univariate polynomial in the
   inverse-square parameter `s = u⁻²`.
2. **Theorem used as stated.** A nonzero polynomial has at most its natural
   degree many distinct roots. The map `u ↦ u⁻²` has fibres of size at
   most two: equality of inverse squares is contained in the roots of
   `value * X² - 1`, itself a nonzero polynomial of degree at most two.
3. **Object as an instance.** For a fixed nontrivial Semaev tuple `t`, the bad
   predicate is the existence of points above `t` on the dilated curve whose
   sum is zero. The intended polynomial is the specialized summation
   polynomial `g_t(s)` from Lemma N'. [pcc:unverified]
4. **Assembly.** The pinned `fast-ecdlp` package is now imported directly.
   `upstreamDilationPolynomialFamily_implies_nondeg` proves the exact upstream
   `ECDLP.L49.DilationNondeg` proposition from construction, nonvanishing,
   degree, and bad-dilation vanishing of `g_t`.
   `upstreamDilationPolynomialCertificate_implies_avg_energy` then feeds that
   proposition to upstream `avg_energy_dil`, with its original `Eng` and
   `Mdil` definitions. Claims `PCC-20261119-1bad7c` and
   `PCC-20261119-a2b707` pass their local verifiers. [pcc:unverified]
5. **Missing components only.** Construct `g_t` and prove its degree and
   vanishing laws. For nonvanishing, the checked pieces now include the cubic
   rational-function-square-to-scaling classification, the short-interval
   orbit exclusion, the abstract character projector, character orthogonality
   from pairwise sign separation, concrete Galois sign construction and
   separation from nonsquare radicand products, the canonical quadratic
   splitting field and roots, the affine-point Galois action, derivation of all
   transformed relations from one original relation, and the corrected final
   transcendence contradiction. In the `A != 0` interval branch, normalization
   of independently scaled cubics plus the short-sum exclusion of ratio `-1`
   now constructs nonzero radicands and proves pairwise nonsquareness. Still
   missing are the lifted original relation and its point nonsingularity,
   torsion-to-division-polynomial vanishing, and the `A = 0` CM
   endomorphism/finite-kernel specialization.
   [pcc:unverified]

| statement | kind | owner | quantified structure | hypotheses |
|---|---|---|---|---|
| `polynomialPullbackBadSetCardLe` | law | `TensorBoundaryBounds` | finite seed space mapped to a field | bounded fibres, bad points map to roots, nonzero polynomial, degree bound |
| `inverseSquareFiberCard_le_two` | law | same | nonzero elements of a finite field | none beyond field structure |
| `InverseSquarePolynomialCertificate` | interface | same | tuple-indexed bad dilation predicates | one nonzero bounded-degree vanishing polynomial per tuple |
| `inverseSquarePolynomialCertificate_card_le` | endpoint | same | previous row | certificate; conclusion is the exact `2D` bad-set bound |
| `unitsInverseSquarePolynomialBadSetCardLe` | law | same | unit group of a finite field | nonzero bounded-degree polynomial vanishing on the bad units |
| `UpstreamDilationPolynomialCertificate` | interface | same | exact pinned `ECDLP.L49` tuple and bad predicates | construction, nonvanishing, degree and vanishing certificate |
| `upstreamDilationPolynomialFamily_implies_nondeg` | endpoint | same | exact pinned upstream `DilationNondeg` | supplied specialized polynomial family and its four laws |
| `upstreamDilationPolynomialCertificate_implies_avg_energy` | endpoint | same | exact pinned upstream `Eng` and `Mdil` | odd characteristic, nonsingular curve and certificate |
| `shortCubicPolynomial_scale_classification` | law | same | odd-characteristic field and short-cubic coefficients | a supplied nonzero nonidentity scaling symmetry |
| `polynomial_constantMulSquare_of_isSquare_fractionRing` | law | same | polynomial UFD and its fraction field | the polynomial image is a square in the fraction field |
| `shortCubicPolynomial_fraction_square_product_classification` | endpoint | same | nonsingular short cubic over an odd-characteristic field | its product with a nontrivially scaled copy is square in the rational function field |
| `zmodNoFullCubicOrbitInShortInterval` | law | same | three positive residue representatives and a cubic root of unity | representatives at most `X`, `3X < p` |
| `characterProjector_isolates_term` | law | same | finite character and point index sets, additive commutative point group | character orthogonality and every transformed relation |
| `separatingSignCharacters_orthogonality` | law | same | finite group and finite family of integer-valued characters | multiplicativity, sign-square-one and pairwise separation |
| `separatingSignCharacterProjector_isolates_term` | endpoint | same | previous row plus an additive commutative point group | pairwise character separation and every transformed relation |
| `galoisSignCharacterProjector_isolates_term` | endpoint | same | finite Galois extension containing indexed square roots | nonzero roots, characteristic not two, pairwise nonsquare radicand products, every transformed relation |
| `quadraticRadicandSplittingField_isGalois` | law | same | canonical splitting field of indexed quadratic factors | odd characteristic, nonzero injectively indexed radicands |
| `transformedRelations_of_addEquiv_eigenvectors` | law | same | finite additive point group with indexed eigenvectors | one original relation and the action/eigenvector identity |
| `affinePointGaloisAddEquiv_eigenvector` | law | same | short-form affine elliptic curve after base change | selected square root and point nonsingularity |
| `quadraticSplittingFieldAffinePointProjector_isolates_term` | endpoint | same | canonical quadratic splitting field and short-form affine points | nonzero pairwise-nonsquare radicands, point nonsingularity and one original relation |
| `shortCubicPolynomial_scaled_pair_fraction_square_classification` | law | same | two nonzero distinct short-cubic scales over a field | odd characteristic and nonsingular cubic |
| `zmodPositivePairRatio_ne_neg_one` | law | same | positive natural representatives in a prime residue field | their sum is below the modulus |
| `zmodShortCubicFunctionRadicands_pairwise_not_isSquare_of_A_ne_zero` | endpoint | same | injectively indexed positive prime-field interval scales | `A != 0`, nonsingular cubic and every distinct scale pair has sum below the modulus |
| `algebraic_ne_const_mul_transcendental` | law | same | field extension, algebraic coordinate and transcendental parameter | nonzero constant multiplier |
| `divisionPolynomial_ne_zero_at_const_mul_transcendental` | law | same | Weierstrass curve over the constant field and extension parameter | division index nonzero in the constant field, nonzero constant multiplier |

Claim `PCC-20261119-998f26` binds the endpoint and passes its local verifier;
signed standing remains unverified. [pcc:unverified]

Claims `PCC-20261119-603d9c`, `PCC-20261119-0f3ed3`,
`PCC-20261119-31e2b5`, and `PCC-20261119-752c5b` bind the four checked
Lemma N' components and pass their local verifiers; signed standing remains
unverified. [pcc:unverified]

Claim `PCC-20261119-1ae928` binds the separation-to-orthogonality projector
endpoint. Claim `PCC-20261119-545ba9` binds the concrete Galois-sign endpoint:
given square roots in a finite Galois extension, nonsquare pair-products force
pairwise sign separation. Claim `PCC-20261119-ae2460` binds the stronger
quadratic-splitting-field affine-point endpoint: the extension, roots, Galois
action and transformed relations are constructed internally from one original
relation. The remaining obligation is the Semaev-specific source of its
radicands, lifted points and original relation, not a supplied extension,
character table or transformed-relation family. [pcc:unverified]

Claim `PCC-20261119-4e1bc6` binds the `A != 0` indexed interval-radicand
endpoint. It packages `l(s*t)` in the rational function field, proves nonzero
for positive scales, normalizes every distinct pair to Lemma 5.4, and excludes
the only exceptional ratio using the short interval. It does not cover the
`A = 0` cubic-character collisions or construct the original Semaev relation.
[pcc:unverified]

Claims `PCC-20261119-cd776f` and `PCC-20261119-fc17e6` bind respectively the
polynomial constant-times-square and the stronger rational-function-square
classification endpoints. Both pass their local Lean verifiers. The latter
discharges the square-product-to-scaling obligation; the separate Galois-sign
endpoint discharges character construction and separation after the finite
Galois square-root data are supplied. [pcc:unverified]

Claims `PCC-20261119-1bad7c` and `PCC-20261119-a2b707` bind the exact upstream
nondegeneracy and average-energy endpoints. Claim `PCC-20261119-a69a2d`
binds the division-polynomial transcendence contradiction. All three local
verifiers pass; signed standing remains unverified. The remaining
torsion-coordinate step is now the precise group-law lemma that a nonzero
torsion point makes the indexed division polynomial vanish at its
x-coordinate. [pcc:unverified]

## Planted-diagonal bad shifts

1. **Structure above.** A commutative field, a square finite matrix, a finite
   set of candidate scalar shifts, and a finite family of bad-shift sets on a
   common shift space.  The matrix dimension is its finite index cardinality;
   no elliptic-curve or relation-matrix representation is introduced here.
2. **Theorem used as stated.** `Polynomial.card_le_degree_of_subset_roots`
   bounds a finite set of distinct roots by polynomial degree.
   `Matrix.eval_charpoly` identifies evaluation of the characteristic
   polynomial with `det(tI - M)`, while `Matrix.charpoly_natDegree_eq_dim`
   supplies its degree. Standard `Finset.card_biUnion_le_card_mul` bounds a
   union of finitely many bad sets.
3. **Object as an instance.** For one residual block matrix `M`, the bad
   shifts are the candidate values `t` for which `det(tI - M) = 0`.  For the
   paper's simultaneous planting step, one such bad set is supplied for each
   selected prime and all sets live on the same finite set of integer shifts.
4. **Assembly probe.** Filter the candidate values by zero evaluation, show
   their underlying multiset is contained in the roots of `charpoly M`, and
   apply the root-cardinality theorem. Rewrite evaluation using
   `Matrix.eval_charpoly` and degree using
   `Matrix.charpoly_natDegree_eq_dim`. Pull this bound back along an injective
   encoding of abstract shifts. For shifts `Fin nu0`, prove the natural cast
   into `ZMod ell` is injective from `nu0 <= ell` by applying `ZMod.val` and
   reducing values below the modulus. For a family of at most `w` bad sets,
   apply the finite-union cardinality bound; subtract the union from the
   candidate set and use `8 * w * f <= #candidates` to derive the exact
   division-free `7/8` survival inequality. [pcc:unverified]
5. **Components formalized here.** The generic filtered-root lemma, the
   characteristic-polynomial bad-shift endpoint, its pullback through an
   injective shift encoding, the bounded-natural-to-residue injection, the
   finite-union bound and the `7/8` survival arithmetic. Still missing is the
   end-to-end bridge from curve coverage and the decomposition machine through
   executable dense elimination and the expected-time restart loop.
6. **Inheritance table.** Distinct roots:
   `Polynomial.card_le_degree_of_subset_roots`; determinant identity:
   `Matrix.eval_charpoly`; degree: `Matrix.charpoly_natDegree_eq_dim`; union
   size: `Finset.card_biUnion_le_card_mul`; complement split:
   `Finset.card_sdiff_add_card_inter` and subset monotonicity; bounded residue
   cast: `ZMod.val_natCast_of_lt` and `Fin.ext`.

| statement | kind | owner | quantified structure | hypotheses |
|---|---|---|---|---|
| `polynomialBadValuesCardLeDegree` | law | `TensorBoundaryBounds`; upstream candidate for `crypto-lean` | field polynomial and finite candidate values | polynomial nonzero |
| `charpolySingularShiftCardLeDimension` | endpoint | same | finite square matrix and finite candidate shifts | none beyond field and finite decidable index |
| `charpolySingularEncodedShiftCardLeDimension` | law | same | injectively encoded abstract shifts and square matrix | injective shift encoding |
| `zmodNatCastInjectiveOnFin` | instance obligation | same | bounded naturals mapped to a residue ring | range no larger than modulus |
| `zmodCharpolySingularShiftCardLeDimension` | endpoint | same | bounded natural shifts and matrix over a prime residue field | prime modulus and range no larger than modulus |
| `finiteBadShiftUnionCardBound` | law | same | finite family of bad finsets on a common shift type | pointwise cardinality bound |
| `sevenEighthsShiftsSurvive` | endpoint | same | candidate finset and bad-set family | bad sets lie in candidates, family/card bounds, `8wf <= #candidates` |

These theorems certify the generic planted-diagonal counting step only. They
do not themselves certify that the paper's residual blocks have the asserted
form or that successful shifts recover an ECDLP relation. The separately
pinned upstream v2 modules `L114_Trans`, `L114_Plant`, and `L114_Main` do check
the stronger algorithm-facing transcript, planted-kernel, round-failure and
successful-round implications, parameterized by their explicit interfaces.
The same pinned tree's `L116_Alg`, `L116_Chernoff`, and `L116_Main` additionally
construct the actual row collector, prove its row relations and adaptive cap
bound, derive tag surjectivity from generation, and discharge the prime-power
generation hypothesis. Their targeted build and axiom audit pass; coverage,
oracle binding and executable Howell cost remain. `L118_Tot` also checks the
reciprocal-probability bound for two uniform generators; transfer to the
paper's affine near-uniform sampler and the remaining expected-time assembly
remain external.
[pcc:unverified]
