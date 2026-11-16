# E1 protocol: boundary-tensor rank census

Status: **draft; do not collect affected data**
Protocol ID: `EXP-tensor-boundary-rank-census-20261031`

## Question

For exact Boolean tensor networks emitted by the hashed prime-field ECDLP construction, are the boundary tensors at preregistered semantic cuts more compressible than those of interior-rewired networks preserving the same local predicates and graph statistics?

The test is deliberately narrower than “can tensor contraction solve ECDLP?” It asks whether a concrete low-rank mechanism exists and survives held-out instances.

## Object being measured

For a vertex set on the left of a frozen cut, let the cut edges be `C`. Contract every internal left edge exactly while leaving `C` open. This produces the integer boundary tensor

```text
T_C : {0,1}^|C| -> nonnegative integers,
T_C(x) = number of satisfying left-side assignments extending x.
```

Sort cut edges by canonical IR descriptor. For every balanced split `C=A∪B` admitted by the cut cap, flatten the tensor into

```text
M_C[a,b] = T_C(a,b).
```

The primary rank is the minimum exact integer rank among all balanced splits when `|C| <= 16`; the canonical alternating split is also reported and is the only primary split if exhaustive split enumeration would exceed the frozen budget. This “best balanced rank” is an algorithmic opportunity measure, not a universal tensor-rank certificate.

Exact rank means rank over `Q`, computed by fraction-free elimination. Modular ranks under fixed 61-bit primes are lower bounds only. A cell without exact rank is censored for the primary verdict and remains in diagnostics.

## Prerequisite E0

No E1 row is admissible until the following pass on every fixture:

- Stable serialization and SHA-256 identity for a typed, versioned canonical IR.
- Exact equality of total count, every interface-prefix count, lexicographically least interface witness, and model multiplicity before and after round trip.
- Independent direct and functional contraction agree where both apply.
- Endpoint swap, interface permutation, predicate-parameter bit flip, predicate port reversal, and dropped pin are each detected as a hash change and either rejected or produce a deliberately unequal semantic certificate.
- Every production predicate carries a tagged kind and sorted parameters. Predicates at or below the frozen arity cap serialize their complete truth table; larger production predicates serialize a strict typed operation that is also the live evaluator before export. Tag, parameters, arity and the truth table or typed program share one semantic digest. Untagged predicates require the explicit bounded-truth-table policy reserved for tiny fixtures and controls.

E0 is a validity gate, not evidence for the compression hypothesis.

### Pre-freeze E0 clarification

The first draft required tags and parameters for production closures and mentioned truth tables only for tiny custom predicates. Source inspection exposed a semantic-identity gap: a tag identifies intended meaning but cannot prove that two closure bodies implement that meaning. E0 first attempted to strengthen every production vertex with a complete table under a 16-port cap. Its representative hash-network control failed as intended: the tiny production graph has a maximum predicate arity of 132 in the Fourier multiplication relation. The corrected hybrid stores exact tables through 16 ports and typed, strictly serialized value-relation programs above the cap. A programmed predicate uses that same typed relation as its live pre-export evaluator and its post-import evaluator, so closure identity is not trusted. This clarification was made before affected data and does not alter the E1 estimand, ladder, controls, or thresholds.

## Population and ladder

The intended target population is the repository's natural hashed-factor-base Boolean tensor encoding, not arbitrary hand-built networks.

Draft ladder:

| Factor-base bits `b` | Slot count `m` | Instances per cell | Status |
|---:|---:|---:|---|
| 6, 8, 10, 12 | 2, 3, 4 | 16 paired target/null instances | Subject to disjoint preflight |

There are 12 cells and 192 target/null pairs. Curves, targets, hash keys, and null rewiring seeds are derived from the frozen protocol hash. Natural instances are sampled before arm assignment. The same canonical target instance feeds every arm.

Preflight may reduce the ladder only before freeze, based on declared exact-rank memory and runtime limits and without using these cells. The final protocol must replace this table rather than silently skipping cells.

## Frozen cut law

The production encoder must tag semantic regions in the IR. For each slot chain, E1 takes the boundary immediately after slot `ceil(m/2)` and before the next point-addition block. Equality/fanout vertices required to make the cut legal remain on the producer side. If that cut has more than the frozen exact cap, the row is retained as censored; no smaller favorable cut is substituted.

Two diagnostic cuts may be reported:

- immediately after factor-base membership/hash acceptance;
- the deterministic balanced vertex separator produced by canonical min-fill with lexical tie-breaking.

Diagnostic cuts cannot decide the primary claim without a frozen amendment filed before data.

## Arms and controls

### Natural target

The exact canonical network emitted for the sampled curve, target, hash key, `b`, and `m`.

### Interior-rewired matched null

Copy the full target IR. Keep all vertices, predicate tags and parameters, interface edges, pins, vertex degrees, port counts, and local evaluation code. Keep every vertex on the same side of the frozen cut and keep interface-adjacent edges fixed. Apply seeded degree-preserving double-edge swaps separately to left-internal, right-internal, and cut-crossing edge classes. This preserves cut width and every vertex's number of incident cut edges. Reject self-loops, duplicate port use, disconnected constructions, and the original wiring.

At tiny enumerable cells, accept the first null whose `floor(log2(1+model_count))` equals the target stratum. At larger admitted cells, use the same exact total-count instrument and the same stratum rule. Stop after 256 attempts and mark `null_unmatched`; do not select the closest count. Both the first valid unconditioned draw and the conditioned draw are retained, so conditioning bias is visible.

This null isolates global composition while retaining local predicate complexity. It does not isolate every possible source of structure.

### Positive control

Use the same graph and interface dimensions with tagged copy/equality tensors arranged so the canonical flattening has analytically known rank 1, 2, or 4 depending on the seeded class. The rank implementation must return the declared rank exactly. A miss invalidates the corresponding rank size.

### A/A control

Serialize and import the same natural IR in an independent fresh process, recompute the boundary tensor, and compare its bytes, support, exact rank, counts, and operation counters. All exact fields must match bit for bit. Wall time is not primary.

## Variables

The causal comparison changes only `global_interior_wiring` between natural and matched-null arms. The following are fixed or paired: source semantic instance, local predicate implementation, vertex multiset, degree sequence, edge count, interface order, pins, cut law, exact arithmetic, rank implementation, cap, threads, and resource budget.

Randomized variables are instance seed, arm order, and null rewiring seed. Opaque arm labels are used until the analysis output is fixed.

## Metrics

Primary:

- `verified_status`: all total counts and witnesses replay in the canonical evaluator.
- `exact_integer_rank`: fraction-free rank of the selected balanced flattening.
- `normalized_log_rank = log2(rank) / min(|A|,|B|)`.
- `rank_storage_entries = rank * (2^|A| + 2^|B|)` as a derived factor-storage proxy.
- `boundary_support`: number of nonzero entries in `T_C`.

Secondary:

- exact integer additions, multiplications, comparisons, and maximum live integer entries needed to construct `T_C`;
- coefficient bit-length growth;
- fixed-prime modular rank lower bounds;
- complete wall time and peak RSS, descriptive only and valid only under the recorded isolation receipt.

No operation counter is converted into group additions. Counts, memory, setup, failures, and verification are reported separately.

## Analysis

The natural and null rows are paired by source instance. The primary effect per row is

```text
D = normalized_log_rank(natural) - normalized_log_rank(conditioned_null).
```

For each `(b,m)` cell, report the paired median `D` and a 95% bootstrap interval over instances. Fit a Theil–Sen slope of `log2(rank)` against `b` separately for each `m` and arm, with paired bootstrap intervals. Apply Holm correction over the three `m` families. A/A spread is exactly zero for exact fields; otherwise the affected rows are invalid rather than treated as noise.

The unconditioned null panel, canonical-split rank, support, operation counts, and diagnostic cuts are descriptive robustness analyses.

## Decision rule

E1 advances to E2 only if all correctness and controls pass and, on frozen held-out cells:

1. the corrected upper 95% interval for `D` is at most `-0.20` on each of the three largest `b` values for at least two of the three `m` values; and
2. for one of those `m` values, the natural rank-growth slope upper interval is at least `0.10` below the null slope lower interval; and
3. the result appears in both conditioned and unconditioned null panels with the same sign; and
4. at least 90% of declared target/null pairs have exact, uncensored ranks.

The registered route is **not advanced** if positive and A/A controls pass, at least 90% of pairs are exact, and the corrected intervals for `D` lie within `[-0.05,+0.05]` on the three largest `b` values for every `m`, with no slope separation. Everything else is `inconclusive`.

These are continuation thresholds, not theorem thresholds. A finite positive result does not prove subexponential contraction; a finite negative result does not disprove all tensor or Boolean structure.

## Holdout rule

Implementation debugging uses only hand-built fixtures and scratch cells with `b<=5`, disjoint seeds, and no production curves or targets. Once E1 is frozen, every declared `b>=6` cell is held out. The analysis binary is hashed before instance generation. Rows remain masked until `analysis.json` and figures are written.

E3 has a separate search split: the smallest two frozen E1 sizes form training only after E1 has been decided; the two largest sizes, unseen modulus classes, and an unseen target family are held out. Search cannot consume E3 holdouts.

## Invalidation and stopping

- Wrong count, failed witness replay, nonzero A/A exact difference, or failed positive rank invalidates the affected cell.
- A modular-only rank cannot decide E1.
- A null that fails the matching rule is retained but excluded from the conditioned comparison.
- Timeout, OOM, unmatched null, and nonzero exit are outcomes, never silently retried. Only recorded infrastructure failures may retry once.
- Fixed sample size and fixed per-row budget; no optional stopping.
- Any protocol change after affected output requires an amendment and makes the changed hypothesis exploratory.

## Freeze blockers

The exact boundary contraction and fraction-free rank core named below was
implemented on 2026-11-01 and is verified only on development controls; see
[`E1_IMPLEMENTATION.md`](E1_IMPLEMENTATION.md). This does not freeze the
protocol or satisfy the production generator, null, A/A, preflight or identity
fields.

The JSON record remains draft until all of these are present:

- canonical IR version and E0 passing commit;
- `tensor-census` binary hash and container digest;
- analysis source and hash;
- disjoint preflight runtime and memory measurements;
- test-specific power/resolution justification for the `0.20` paired effect;
- exact cut cap and final ladder;
- protocol SHA-256, commit, freeze timestamp, and derived seed manifest.
