# Hashed-ECDLP boundary-rank family lower bound

- **Status:** `open`
- **Kind:** proof gap; implementation gap
- **Affected work:** [rigorous boundary-rank bounds](../research/tensor_boolean_breakthrough_20261031/RIGOROUS_BOUNDS.md), [E1 protocol](../research/tensor_boolean_breakthrough_20261031/PROTOCOL.md), and any claim that the registered exact linear-message contraction is not subexponential
- **Next action owner:** tensor Boolean E1 study
- **Last updated:** 2026-11-01

## Problem

We cannot establish an asymptotic lower bound for the production hashed
prime-field ECDLP tensor family because no uniform exponential-size nonzero
minor, or equivalent flattening-rank lower bound, is known for its semantic
boundary matrices.

The relevant family must use the frozen production encoding and a specified
cut sequence. For security parameter `n`, the required statement has the form
`rank_Q(M_n) >= 2^e(n)` with `e(n) = Omega(n)` (or with an explicitly proved
linear relation between cut width and `n`). A finite census, including a full
rank at every tested size, cannot supply the universal quantifier.

## Evidence and uncertainty

- **Observed:** The exact census core computes integer boundary tensors and
  fraction-free ranks on bounded instances. It now emits independently
  recomputable nonzero-minor certificates. No production semantic cut
  generator, frozen rank ladder, or production certificate exists; the E1
  protocol remains draft.
- **Derived or proved:** A Lean-checked theorem proves that a nonzero `k × k`
  minor forces every exact linear factorization through the boundary to have
  message dimension at least `k`. A second conditional theorem proves an
  exponential work lower bound when a uniform exponential rank lemma, the
  linear-message factorization condition, and work at least message dimension
  are supplied. See the [proof map](../research/tensor_boolean_breakthrough_20261031/RIGOROUS_BOUNDS.md#formal-proof-map).
- **Suspected explanation:** Unknown. The family may contain a structured
  permutation/triangular minor, or algebraic dependencies may keep its rank
  submaximal. Small instances discriminate candidates but cannot prove either
  asymptotic behavior.
- **Missing capability:** A production semantic cut generator and a symbolic
  family argument that constructs a nonzero minor of exponential order (or an
  equivalent injection, fooling set, determinant formula, or rank argument).
- **What this evidence does not establish:** It does not refute nonlinear,
  approximate, randomized, differently encoded, or separator-avoiding
  algorithms. It does not disprove subexponential prime-field ECDLP in general.

## What would unblock it

- [ ] Freeze and validate the production family `M_n`, its cut sequence, and
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

- **Action or proof obligation:** Implement the production semantic cut
  generator, then search the smallest disjoint preflight instances for a
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
machinery were added. The uniform family lemma remains unresolved. Completing
this record or merging its PR does not establish an ECDLP lower bound.
