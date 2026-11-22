# Yield and rank frontier decomposition

Date: 2026-11-18

## Result

The canonical graph's two highest-impact index-calculus frontier nodes are
`HEUR-DECOMPOSITION-PROBABILITY` (14 downstream claims) and
`HEUR-RELATION-RANK` (11 downstream claims). They were too coarse to attack as
single units. The first round split them into 12 definitions/open obligations,
32 typed dependencies and six research targets in
[`ic-yield-rank-frontier-20261118.json`](ic-yield-rank-frontier-20261118.json).
This follow-up adds 11 formal theorem/counterexample/gap nodes and 14 typed
edges in
[`ic-yield-rank-formal-20261118.json`](ic-yield-rank-formal-20261118.json).

No new asymptotic distribution, rank or complexity result is claimed. Lean now
checks four boundary facts: solver-visible yield is bounded by mathematical
yield and equals it under completeness; equal conserved means can have
different target coverage; success-only retention can erase a real failure;
and row surplus can coexist with both nonzero left and right kernels. These
facts constrain the new gaps; they do not close the natural-family laws.

![Dependency view of fixed-population yield and relation rank. Green nodes are
checked or measured, blue nodes are definitions, yellow nodes are open, red
nodes are composition gates, and note-shaped nodes are executable research
targets.](yield-rank-frontier-20261118.svg)

Editable source:
[`yield-rank-frontier-20261118.dot`](yield-rank-frontier-20261118.dot).

## Requirement-to-evidence map

- **Complete population:** `DEF-IC-YIELD-COUNT-VECTOR` and
  `RT-IC-PUBLIC-POPULATION-CONTRACT`. Schema, validator and synthetic tests are
  complete; the natural population is admission-blocked.
- **Mean versus shape:** `THEOREM-YIELD-CONSERVATION` qualifies
  `GAP-IC-YIELD-SCALING-LAW`. The mean is checked; the target-wise law is open.
- **Solver visibility:** `THM-IC-SOLVER-VISIBLE-BOUND`,
  `CEX-IC-SOLVER-INCOMPLETENESS` and `RT-IC-SOLVER-VISIBILITY-AUDIT`. The
  generic boundary is checked; the production solver audit is open.
- **Complete relation trace:** `DEF-IC-RANK-SUPPORT-TRACE` and
  `GAP-IC-STOPPING-CENSORSHIP`. The contract is implemented; the natural
  population is admission-blocked.
- **Row count versus rank:** `GAP-IC-ROW-ENSEMBLE-LAW`,
  `GAP-IC-SUPPORT-CONNECTIVITY-LAW` and `GAP-IC-TERMINAL-RANK-LAW` remain open.
- **Correct kernel mode:** `THM-IC-UNUSED-COLUMN-RIGHT-KERNEL`,
  `CEX-IC-SURPLUS-RIGHT-RANK` and `RT-IC-KERNEL-MODE-CENSUS`. The first
  11-claim census is complete; one primary-text ambiguity remains.
- **End-to-end composition:** `GAP-IC-YIELD-RANK-COMPOSITION` and
  `RT-IC-YIELD-RANK-COMPOSITION` remain blocked on component certificates.

## What is already known

- `THEOREM-YIELD-CONSERVATION` and `THM-RP-YIELD-SUM-COUNT` fix the
  total/mean representation count for a finite domain. They do not determine
  variance, zero mass, tails, target uniformity or solver visibility.
- `MEAS-PF-YIELD-CENSUS` gives exhaustive small-prime, arity-two counts and the
  ordered/unordered pairing correction. It is not an asymptotic law for natural
  higher-arity families.
- `MEAS-RANK-DEFICIT-CENSUS` gives small-prime reusable/right-kernel samples in
  which ten surplus relations often miss full rank. It is not a replacement
  terminal-rank scaling law.
- `THM-LEFT-KERNEL-RELATIONS` gives single-target extraction from
  target-dependent relations under explicit scalar-field hypotheses. It does
  not give a reusable factor-base logarithm table or multi-target amortization.
- `THM-IC-SOLVER-VISIBLE-BOUND` proves target-wise visible yield is at most
  mathematical yield, with equality from completeness. It does not prove any
  production algebraic solver complete.
- `CEX-IC-MEAN-SHAPE-SEPARATION` gives equal total yield with different
  coverage, zero mass and fibres. It is not a natural-curve asymptotic model.
- `CEX-IC-STOPPING-CENSORSHIP` shows success-only selection can erase a real
  failure. It does not estimate bias in a particular admitted run.
- `CEX-IC-SURPLUS-RIGHT-RANK` shows row surplus can coexist with nonzero left
  and right kernels. It does not determine a natural ensemble's rank law.

The key separation is therefore:

```text
exact global mean
      |
      +--/--> target-wise usable-yield law

early independent rows
      |
      +--/--> support connectivity and terminal reusable rank

single-target left kernel
      |
      +--/--> reusable right-kernel/full-rank table
```

Each crossed arrow is deliberately absent from the graph unless a separate
bridge is supplied.

## Finite attack order

1. **Review the public population contract.** The fail-closed schema, validator
   and four synthetic positive/negative tests are implemented in
   [`research/ic_yield_rank_frontier_20261118/`](../../research/ic_yield_rank_frontier_20261118/README.md).
   Independent review is next. This does not authorize the scientific run.
2. **Audit solver visibility.** For one exact small prime-order curve and
   declared factor base, independently enumerate all group-law decompositions
   and compare them target by target with solver output. Preserve the first
   chart, multiplicity, rationality or verification mismatch.
3. **Finish kernel-mode classification.** The first 11-claim census is
   [recorded](../../research/ic_yield_rank_frontier_20261118/KERNEL_MODE_CENSUS_20261118.md).
   Resolve Theriault from the primary text, then formalize the graph-cycle and
   reusable/right-rank branches that remain load-bearing.
4. **Produce one admitted immutable population.** Only after custody and
   experiment admission, retain every registered target, failure, witness,
   normalized row and rank/support prefix. Do not reuse the held historical
   batch.
5. **Evaluate yield and rank from the same population.** A positive, null,
   negative, censored or custody-blocked result is retained symmetrically.
6. **Compose the certificate.** State exactly which scales and kernel mode the
   result covers. Do not extrapolate a finite panel into an asymptotic theorem.

## Acceptance details

### Public population contract

The immutable identity must bind the curve and prime-order subgroup, factor
base and symmetry quotient, target generator, arity, solver and checker
versions, scalar field, budgets, registered scale ladder and row-normalization
rule. Every target gets a terminal record, including zero decompositions,
solver failure, timeout and verification rejection.

### Solver-visible yield

The comparison uses two independent paths:

- direct finite group-law enumeration of mathematical witnesses; and
- the production algebraic solver followed by the production relation
  verifier.

Agreement is target-wise after the preregistered sign/order quotient. Aggregate
agreement alone is insufficient because errors can cancel.

### Rank and support

The replay record contains the exact normalized coefficient row over the
declared subgroup scalar field, rank after each prefix, duplicate class, live
and dead columns, support components and incidence-core size. Results are
reported separately for the single-target and reusable-table modes.

## Non-implications

- A conserved mean is not a Poisson law.
- An exhaustive toy census is not an asymptotic theorem.
- A solver success rate is not mathematical decomposition yield unless the
  witness correspondence is checked.
- Early row independence is not terminal rank.
- More rows than columns guarantees a nonzero left kernel, not a reusable
  full-column-rank table.
- A finite reusable-rank certificate does not prove a sub-rho attack exponent;
  solver, setup, filtering, linear algebra, descent, memory and verification
  must still compose on the same family.

## Reproduction

This was a corpus and source audit, not a scientific execution. Regenerate and
check it with:

```sh
node docs/ecdlp-hardness-corpus/validate.mjs
node docs/ecdlp-hardness-corpus/render.mjs
node docs/ecdlp-hardness-corpus/attack-graph.mjs --check
node docs/ecdlp-hardness-corpus/weakness.mjs --check
python3 research/ic_yield_rank_frontier_20261118/test_contract.py
dot -Tsvg docs/ecdlp-hardness-corpus/yield-rank-frontier-20261118.dot \
  -o docs/ecdlp-hardness-corpus/yield-rank-frontier-20261118.svg
```

Primary repository evidence used by this decomposition is linked through
`S-IC-SIX-OBLIGATIONS-2026`, `S-CENSUS-ROUND4` and `S-WEAKNESS-LEAN` in the
machine fragments. The formal instance-gate analysis is in
[`PROOF_MAP.md`](../../research/ic_yield_rank_frontier_20261118/PROOF_MAP.md).
No experiment or performance measurement was launched.
