# Hashed-ECDLP boundary-rank family lower bound

- **Status:** `narrowed`; semantic slot-relation lower bound refuted at
  PCC-unverified scope, low-level BTNIR family bound open [pcc:unverified]
- **Kind:** proof gap; implementation gap
- **Affected work:** [rigorous boundary-rank bounds](../research/tensor_boolean_breakthrough_20261031/RIGOROUS_BOUNDS.md), [E1 protocol](../research/tensor_boolean_breakthrough_20261031/PROTOCOL.md), and any claim that the registered exact linear-message contraction is not subexponential
- **Next action owner:** tensor Boolean E1 study
- **Last updated:** 2026-11-18

## Problem

The encoding-independent semantic slot relation does not admit the proposed
exponential-rank lower bound uniformly over all seeds, or with nonnegligible
probability for a uniformly sampled seed, under the Bochum source threshold.
Its rank is at most the exact decomposition count, and that count has
expectation below two in the stated asymptotic regime. [pcc:unverified] The derivation and
claim ids are in
[`SEMANTIC_RANK_BOUND.md`](../research/tensor_boolean_breakthrough_20261031/SEMANTIC_RANK_BOUND.md).

The remaining problem is narrower: no uniform exponential-size nonzero minor,
or equivalent flattening-rank lower bound, is known for the frozen low-level
BTNIR Boolean-edge boundary matrices. Those matrices include a particular
arithmetic and equality-gadget representation and are not rank-equivalent to
the semantic slot relation.

The relevant family must use the frozen production encoding and a specified
cut sequence. For security parameter `n`, the required statement has the form
`rank_Q(M_n) >= 2^e(n)` with `e(n) = Omega(n)` (or with an explicitly established
linear relation between cut width and `n`). A finite census, including a full
rank at every tested size, cannot supply the universal quantifier.

## Evidence and uncertainty

- **Observed:** The exact census core computes integer boundary tensors and
  fraction-free ranks on bounded instances. It now emits independently
  recomputable nonzero-minor certificates. No production semantic cut
  generator, frozen rank ladder, or production certificate exists; the E1
  protocol remains draft.
- **Derived or formalized:** Existing Lean declarations show that a nonzero `k × k`
  minor forces every exact linear factorization through the boundary to have
  message dimension at least `k`. A second conditional theorem establishes an
  exponential work lower bound when a uniform exponential rank lemma, the
  linear-message factorization condition, and work at least message dimension
  are supplied. New locally kernel-checked declarations bound semantic
  relation rank by decomposition count and formalize the finite
  uniform-average obstruction. Their PCC claim standing is unverified.
  [pcc:unverified] See the
  [proof map](../research/tensor_boolean_breakthrough_20261031/RIGOROUS_BOUNDS.md#formal-proof-map).
- **Suspected explanation:** Unknown. The family may contain a structured
  permutation/triangular minor, or algebraic dependencies may keep its rank
  submaximal. Small instances discriminate candidates but cannot establish either
  asymptotic behavior.
- **Missing capability:** A frozen production BTNIR cut generator and a symbolic
  family argument that constructs a nonzero minor of exponential order (or an
  equivalent injection, fooling set, determinant formula, or rank argument).
- **What this evidence does not establish:** It does not refute nonlinear,
  approximate, randomized, differently encoded, or separator-avoiding
  algorithms. It does not disprove subexponential prime-field ECDLP in general.

## What would unblock it

- [x] Resolve the encoding-independent semantic relation matrix: rank is
  pointwise at most decomposition count and its source-threshold expectation
  is below two. This does not exclude an adversarial exceptional seed
  sequence. PCC standing remains unverified. [pcc:unverified]
- [ ] Freeze and validate the low-level BTNIR family `M_n`, its cut sequence, and
  the relation between `n`, boundary width, and the ECDLP security parameter.
- [ ] Prove for every admitted `n` an explicit nonzero minor of order
  `2^e(n)` with `e(n) >= c n` for a fixed positive `c`, or prove an equivalent
  rank lower bound, and formalize the family-specific argument without
  `sorry` or project axioms.
- [ ] For the targeted implementation, prove that every exact computation in
  scope induces the stated linear factorization and charges at least one work
  unit per message coordinate.

These conditions are sufficient only for an exponential lower bound on the
named exact linear-message architecture. They are not sufficient for a lower
bound on arbitrary ECDLP algorithms.

## Next discriminating step

- **Action or proof obligation:** Freeze the production BTNIR semantic-region
  cut, Boolean edge order, and generator, then search the smallest disjoint preflight instances for a
  stable triangular or permutation-minor rule whose indices can be defined for
  every `n`.
- **Inputs and controls:** Use the frozen canonical BTNIR semantics, exact
  integer counts, the registered canonical cuts, topology-preserving nulls,
  and independently recomputed minor determinants. Include rank-one,
  full-rank identity, singular and corrupted-certificate controls.
- **Execution limits and stop condition:** Stay within the E1 protocol's exact
  enumeration and cut-width caps. Stop the structural lead when its predicted
  minor is singular on any admitted preflight instance; do not replace the
  rule after inspecting the final ladder.
- **Outcomes and next actions:** A stable rule becomes a family-proof target;
  a counterexample rejects that rule only; cap exhaustion is inconclusive; an
  implementation mismatch returns to E0 equivalence work.
- **Recheck trigger:** Production cut generator completion or a new symbolic
  minor/rank construction.

A planned step is not a completed experiment. Apply the existing execution,
review, budget, and archival rules before acting on it.

## Resolution and history

2026-11-01: Architecture-level Lean bounds and finite nonzero-minor certificate
machinery were added. The uniform family lemma remained unresolved.

2026-11-18: The semantic slot-relation branch was closed negatively at
PCC-unverified scope: expected rank is below two under the source threshold,
so no all-seed or nonnegligible-probability exponential lower bound can hold
there. An exceptional selected seed sequence is not excluded. [pcc:unverified] The
low-level BTNIR edge-flattening branch remains open because no rank-equivalence
theorem connects it to the semantic matrix. Completing this record or merging
its PR does not establish a lower bound for arbitrary ECDLP algorithms.
