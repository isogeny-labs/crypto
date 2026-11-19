# EXP-R31-B8-CANCEL: exact signed-cancellation coverage on unchanged prime31

Registered 2026-11-19 UTC before adding the new Rust example or observing its CI run. [pcc:unverified]

## Question / nonduplication
The public-only squarefree coefficient ideal `M=H_B` from #4548 fixes eight distinct x coordinates. The B8 four-window 1,024-log pipeline #4728 instead uses eight *position-weighted* slots. Can the former (or an unweighted multiplicity-complete B8 variant) yield enough uniformly shifted public relations to feed the latter?

Previous constraints: #4716 establishes a subset-mask multiset ceiling for unsigned shared base points, #4717 studies algebraic multiplicity strata, #4547 seven-known-anchor completion is generic equivalent, #4728 implements weighted W8 with a generic grading oracle, #4692 owns nonenumerative RR9/F4 extraction, and #4801 rules out a separate all-sign low-degree interpolation family. **Do not implement a second F4 extractor or conflate algebraic counts with competitive solving work.**

## Predeclared exact mathematical tests
Let `P_1,...,P_8` be the first eight distinct rational x points of the original native Prime31 factor base and permit eight signed *unweighted* leaves (repetitions allowed), in any order. Every eight-leaf sum has the form `sum_i k_i P_i` with `k in Z^8`, `||k||_1 <= 8`, `||k||_1 = 8 (mod 2)`; conversely every such vector can be padded with inverse pairs to eight leaves.

The exactly countable coefficient-vector set size is
```
C_d(8) = 1 + sum_{r=2,4,6,8} sum_{j=1}^{min(d,r)}
          binom(d,j) 2^j binom(r-1,j-1)
```
for d=8; the five shell counts should be 1, 128, 2816, 27008, 157184 and total 187137. Multiple vectors may map to one group point: hence this is always an upper bound on actual distinct residual support.

For squarefree `M=H_B` with exactly those eight distinct x's, merely `2^8=256` sign sums are possible; this already accounts for the 8! permutation invariance of the EC sum. If `R` is uniform in the order `n=2147446831` full group, the fraction with an unweighted eight-leaf relation is at most `187137/n`; the squarefree fraction is at most `256/n`. These statements assume identical *unweighted* slot bases. They **do not** apply to W8's distinct position-dependent scalar-weighted slot maps.

## Rust test, grading isolation and measurements
- Reuse `prime31-algebraic-ic::{Arithmetic, Point, check_curve, PRIME31}`, original prime31 curve, first eight x's; no substituted curve, Python solver or new EC arithmetic.
- Enumerate each of the 187137 admissible signed net vectors exactly once with coefficient parity+L1 pruning. Precompute multiples `[-8,8]P_i` using the original Rust group operations. Store *group points* in a HashSet, counting real collisions; assert size <= analytic bound. This is explicitly a **generic grading oracle** excluded from any future candidate extractor.
- Separately enumerate the 256 squarefree sign vectors as an independent control and verify their sums belong to the broader support.
- Print immutable parameters, counts, exact support cardinalities, exact integer-vector cardinality, group operation counts, and zero unknown-index/log-recovery claims as JSON. Test the expected analytic shell sizes and actual first-eight curve points. Bound runner wall and preserve raw output.
- Reproduce the closed-form vector counts and squarefree group identities with independently authored BigInt arithmetic before promoting evidence. Exact numerical outcomes are not assumed in the protocol; report collisions and CI failures unchanged.

## Go/no-go
Do not call the existing squarefree `M=H_B` F4 candidate an appropriate oracle for the weighted rank-31 W8 pipeline. If unweighted coverage is negligible, mark the fixed-eight and same-base multiplicity forms as negative under *uniform shifted targets*, while keeping #4692's genuinely position-weighted, no-enumeration extractor active. Compare first with published PKM 2016 and APS 2018 index-calculus work as well as the existing Rust algebraic frontier, and only then with matched optimized BSGS/rho for whole solving.

No new complete non-generic ECDLP solver or speed advantage is predicted or claimed by this experiment.
