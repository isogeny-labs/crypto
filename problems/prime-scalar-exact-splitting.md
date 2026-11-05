# Exact compact scalar-predicate splitting

- **Status:** open
- **Kind:** missing construction/proof; scoped obstructions to the unchanged packing template
- **Affected work:** [prime-field research continuation](../research/prime_tensor_variants_20261030/README.md), [concrete verifier](../research/prime_tensor_variants_20261030/reviews/compact-scalar.md), [heavy-predicate analysis](../research/prime_tensor_variants_20261030/reviews/heavy-predicates.md)
- **Next action owner:** prime-field algorithm research; no unattended job scheduled
- **Last updated:** 2026-10-30

## Problem

No uniform exact splitting of the public scalar-verification predicate is known
here that gives a sufficiently compact Boolean tensor network and preserves the
available contraction theorem's degree, description and evaluation bounds.

The scope is a nonsingular E/Fp, p>3, with known odd prime whole order r,
generator P and valid target Q; b=ceil(log2 p). The predicate accepts exactly
the canonical scalar k in [0,r) with [k]P=Q. It is polynomial-time evaluable
on a supplied k. That does not give a cheap exact weighted multilinear
extension, coefficient extractor, or public factorization of its singleton
support. The original goal is an essentially new classical exp(o(b)) ECDLP
algorithm, with all costs charged. Merely substituting a three-adic internal
variant into the existing hashed reduction does not satisfy novelty.

## Evidence and uncertainty

- **Observed development evidence:** the separate ternary packet and one-level
  units pass their frozen finite panels; 20 base label theorems plus eight
  integer-slot bridge theorems pass Lean and fresh replay. These are supporting results about an adaptation of
  the existing contraction template, not about the missing scalar encoding.
- **Written construction:** windowing plus honest input copies/output chunks
  costs O(mb) Boolean edges and degree O(b/m+b^(3/4)). With the existing
  s^(9/10) degree cap, its stated balance is O(b^(20/19)) edges. The theorem's
  resulting bound does not certify exp(o(b)).
- **Written scoped obstruction:** common uniform packet padding forces
  q<=3M/delta for an unsplit vertex of degree delta. A sharper degree-moment
  variance formula does not remove this incidence requirement. For the same
  windows, a,u=O(log m); the unchanged mode-count guarantee has exponent scale
  bm/sqrt(log m). These are limits on that proof/representation, not runtime
  lower bounds. [Independent check](../research/prime_tensor_variants_20261030/reviews/heavy-variance-independent.md).
- **Correction retained:** a proposed general code-occupancy barrier was false
  without extra structural premises. Repeated-slot counterexamples prevent
  using it to rule out all correlated packet constructions.
- **Missing capability:** a constructible exact splitting or weighted-extension
  routine with a proved closed recursive representation. General contraction
  formalization and complete native implementation audit are separate gaps.
- **What this does not establish:** impossibility, security, an original
  sub-exponential algorithm, a general encoding lower bound, or any measured
  performance result. The other written bounds are not yet Lean formalized.

## What would unblock it

The following is one sufficient route, not a necessary characterization of
all possible algorithms.

- [ ] Construct from public E,P,Q a uniform exact Boolean network with
  O(b*(loglog b)^C) edges for a fixed C, degree at most ceil(s^(9/10)),
  polynomial description size and the available theorem's charged evaluation
  and polynomial-space bounds. Include range guards, complete point cases,
  input copies and setup; supply no scalar advice or exponential lookup table.
- [ ] Prove that the count and prefix-conditioned counts recover the canonical
  scalar, and charge every query, setup and verification step. Alternatively,
  prove an explicit structure-aware contraction theorem that avoids this
  particular network requirement, with all recursive invariants exposed.
- [ ] Provide faithful Lean statements/proofs, an exact proof map and pinned
  dependencies; implement and independently replay native correctness controls
  before claiming implementation parity. Complete novelty and isolated scaling
  work remain downstream requirements, not consequences of a tiny panel.

For the displayed network route, the cited contraction exponent would be
O(b*(loglog b)^C/sqrt(log b))=o(b), including polynomially many prefix queries.
This is a conditional sufficiency calculation, not the missing construction.

## Next discriminating step

- **Action/proof obligation:** exhibit an exact compact weighted extension or
  split of one window's point-output predicate and prove closure after the
  source deformation and target-mode coefficient extraction.
- **Inputs/controls:** canonical scalar windows, full signed point outputs,
  identity/range exceptions; compare with the exact Boolean multilinear
  extension. Ordinary gate substitution already fails for Y AND Y: it yields
  z^2 instead of z. A singleton verifier's extension directly exposes log bits,
  so assuming its evaluator is not an independent algorithmic step.
- **Execution/stop condition:** mathematical construction and native development
  controls first; any scientific performance work needs a prospective protocol
  and the repository's EKS admission/reference/isolation rules.
- **Outcomes:** a construction with complete charged bounds supports the next
  reduction proof; a fixed counterexample closes only that construction;
  unavailable tooling or a bounded search leaves the mathematical question open.
- **Recheck trigger:** new compact predicate representation, a proved structural
  mode-aggregation invariant, or a stronger contraction theorem with usable
  explicit hypotheses.

## Resolution and history

Unresolved. On 2026-10-30, the ternary continuation supplied the scoped window,
variance, indivisibility and sparse-packet analyses and named the missing
constructible operation. Publishing or merging that continuation does not
resolve this problem or the original research goal.
