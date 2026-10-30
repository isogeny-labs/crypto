# Correlated packets with one common deformation

- **Status:** open
- **Kind:** missing kernel/compatibility construction and recursive cost proof
- **Affected work:** [packet-variant research](../research/prime_tensor_variants_20261030/README.md), [matched-level analysis](../research/prime_tensor_variants_20261030/reviews/matched-level.md)
- **Next action owner:** prime-field research; no automatic campaign scheduled
- **Last updated:** 2026-10-30

## Problem

We do not have a single low-degree deformation whose level-dependent two-axis
packet representations achieve the desired mode density while preserving its
actual jets and complete recursive costs.

This is the selected next theoretical bet in the current research stage. It
would strengthen the tensor backend rather than require a target-dependent
scalar factorization. Its relation to prime-field ECDLP would still use a
compact complete encoding and a charged reduction; neither novelty nor an
improved algorithm is established by posing this problem.

## Evidence and uncertainty

- **Written positive result:** matched-level origin recovery works. Restrict
  F(T1,T2) to G(T)=F(T,T), degree at most twice the individual degree. The
  source's tapered Hermite argument yields precision order MD_i/u; mixed-level
  samples are unnecessary. [Independent derivation](../research/prime_tensor_variants_20261030/reviews/matched-recovery-independent.md).
- **Written scoped obstacle:** imposing equality of the two axis bits at all
  u retained positions leaves L^2/2^u slots. Direct mode density has an extra
  factor 2^(u-i). Finite-support congruence-moment reduction restores the
  existing exp(O(M/sqrt(u))) enumeration envelope, but proves no improvement.
  The displayed reduced enumeration still has exponentially many admissible
  candidate tuples at that scale; this is not a nonzero-coefficient or runtime
  lower bound. [Moment analysis](../research/prime_tensor_variants_20261030/reviews/congruence-moments-independent.md).
- **Written counterfamily:** common origin, bounded degree and integral charts
  do not justify changing the deformation separately by level. The family
  H_i(T)=(1-binomial(T,2^i))^p_i has common origin one and all level-i charts
  zero modulo the requested precision, with the stated degree/bit bounds.
  [Full exact scope](../research/prime_tensor_variants_20261030/reviews/levelwise-binomial-counterexample.md).
- **Formally checked part:** nine Lean theorems prove signed binomial parity,
  rational-polynomial evaluation, origin one and node-value divisibility.
  Coefficientwise chart integrality, degree schedule and the full data-collision
  result remain unformalized. This is not a Lean proof of the full counterfamily.
- **Not established:** actual realization of that counterfamily by repacked
  Cauchy networks, impossibility of a different common kernel, better basis or
  cancellation, complete graph packing, or an improved ECDLP algorithm.
- **Further exact controls:** a physical-coordinate substitution is not a
  general Boolean-index gauge: the actual multilinear-extension kernels
  have a checked `4 versus6` composition counterexample. True group-local
  gauges preserve flattening determinantal ideals. An exterior lift has a
  genuine composition law, but retains its one-particle passive crossings;
  squarefree update enumeration alone retains the square-root count.
  [Follow-up with exact native controls](../research/prime_common_kernel_20261030/README.md).
  These close specific shortcuts, not the common-kernel problem.
- **Positive coded identity, scoped selector failure:** distinct binary
  syndromes yield one projector that exactly eliminates mismatched operator
  pairs. The underlying general projection laws are Lean checked. Its
  explicit Lagrange selector has crossing block
  `2^(2a+2)(cI+4D)` with `c` odd, leaving exponentially many nonzero Smith
  directions where the proposed weighted linear-mode template has only
  `O(a^2)` terms. The rank/valuation and density analysis is written and
  independently reviewed, with separate native implementations of the a3
  specialization. Binary-code instantiation and the Kronecker-product bridge
  remain unformalized. This rejects that selector, not other common kernels.
- **Exact-filter density bound:** over a field, exact unequal-pair
  annihilation plus injective matched maps makes the error-image assembly
  injective, so `(u+1)dimC<=dimV` for `u+1` labels. This is a general
  linear-algebra bound for that precise template, not a runtime lower bound
  or a universal quantum-code bound. [Proof and scope](../research/prime_common_kernel_20261030/reviews/exact-filter-packing.md).
  Singular matched maps remain outside the hypotheses; the source's local
  Cauchy maps are partial translations, so that distinction is substantive.

## What would unblock it

One sufficient target is a uniform common matrix deformation W(T) with W(0)=I
and contraction degree O(ML), whose active level i admits:

- [ ] A real packet construction with O(M/D_i^2) mode families for each pair
  (b1,b2), and coefficient valuation at least b1+b2+2, with no hidden density
  loss from simultaneous orientation constraints.
- [ ] Equality to the SAME deformation jets at the recovery precisions, or a
  sufficient proved congruence with its error charged. Arbitrary repacking,
  shared origin or port renaming alone is insufficient.
- [ ] Uniform child shrink, degree bounds, immutable entry precision, complete
  entry/description costs, and polynomial-space enumeration/recovery. A changed
  basis must include its construction and exact coefficient costs.
- [ ] Faithful formalization, native independent controls and full novelty
  review before any algorithm or implementation claim is promoted.

Under those hypotheses the plane-partition calculation would give
exp(O((M/D_i^2)^(1/3)*(MD_i/u)^(2/3)))=exp(O(M/u^(2/3))). If all global
invariants transferred, the existing compact hashed reduction would have a
conditional target exponent O(b/(log b)^(1/3)). This is a sufficiency target,
not a demonstrated runtime or algorithm.

## Next discriminating step

- **Action:** construct a fixed kernel with different valid local factorizations
  at different valuation levels; prove an actual common-family/jet identity
  before trying a relation search or fitting timing exponents.
- **Controls:** distinguish consistent conjugation of the old matrix from
  resetting it to a new product kernel; check the actual matrix/tensor identity,
  not only shared origin or sample agreement. For linear changes distinguish
  physical port coordinates from the full Boolean index space. For exterior
  lifts display the complete grouping signs and ambient intermediate modes;
  a one-particle composition law does not supply the child identity.
- **Limits:** mathematical construction and native development checks first.
  Performance work requires a prospective protocol and the existing EKS rules.
- **Outcomes:** a complete identity plus charged density supports a global
  theorem attempt; a counterexample closes only that proposed representation.
  A failed data-only bridge does not close all common-kernel constructions.
- **Recheck trigger:** a new invariant kernel, congruent-jet transform or
  uniformly cheaper moment representation with complete entry costs. A new
  coded/operator proposal must control the rank and valuations of its actual
  Boolean column forms; a short scalar operator sum alone is insufficient.
  An exact-filter successor must also charge the packing factor or exhibit
  explicitly which injectivity/cross-annihilation assumption it avoids while
  preserving the same common jets. Avoiding an assumption is not itself a
  proof of a faster factorization.

## Resolution and history

Unresolved. The 2026-10-30 analysis removes a suspected interpolation obstacle,
charges the actual simultaneous density and supplies a counterfamily to a
generic repacking shortcut. It does not construct the missing common kernel.
The further gauge/exterior controls preserve this unresolved status. Their
exact finite disagreements are algebraic and make no timing inference.
