# Protocol: static occurrence-ranked variable relabelling for UNSAT-heavy inherited-F4 trees

Frozen 2026-11-19 before the registered run. All numbers produced are [pcc:unverified].
Single target, charged cold per attempt. No multi-target or batch method is used.

## Prior-art check (done first)

- Summation-variable symmetry breaking (lex order of summands), SAT: measured and off by
  default, `SatDecompositionOptions::symmetry_breaking` doc table in
  `src/cryptanalysis/koblitz_index_calculus.rs`; n83 all-summand order,
  `research/n83_chain_sat_order_20261031`.
- SAT branching order (L3), learned UNSAT triage, restarts:
  `research/pdp_ml_sat_ceiling_20261027`.
- XOR decided-row cache: `research/n83_xor_decided_cache_20261019`.
- F4 early abort: `research/lit2026_w4_early_abort_20261011`.
- Splitting rule lowest vs highest, and the layout/interleaved chain order, under inherited F4:
  `research/notes/ecc2k130/RESEARCH_INHERITED_F4.md`, `research/chain_split_order_20260924`
  (notes). MostFrequent/MOM were measured at Macaulay level on an older engine
  (`research/notes/index-calculus/RESEARCH_EXOTIC_COORDINATES.md` section 17/18).
- NOT found measured: a static, per-system occurrence ranking applied as a variable relabelling
  under the shipped `HighestFree` rule of `InheritedF4`.
  (research-kb was not queried: no MCP server in this subagent; grep of research/ only.)

## Candidate (ONE technique)

Rename the variables of each decomposition system (after the shipped trace row and the shipped
chain order) by a permutation `perm[old] = new` ranking variables by the number of monomials
that contain them (ties by old index). Solve with the unchanged `InheritedF4{max_degree:3}`
engine, unchanged split rule (`Auto` -> `HighestFree`, which splits on the highest-indexed
occurring free variable), unchanged node budget. Roots are mapped back by the inverse
permutation. Arms (per target, per repetition, order shuffled by a seeded SplitMix64):

- `base`: shipped system. `base2`: identical to `base` (A/A noise floor).
- `freq_hi` (PRIMARY): most frequent variable gets the highest index, i.e. is split on first.
- `freq_lo` (exploratory): most frequent variable gets the lowest index.

Implementation: `examples/ic_unsat_relabel.rs` (public crate APIs only).

## Disclosure of an exploratory smoke run

Before this protocol was written the harness was run once on 2 targets of K(1,17), dim 6,
m 3 (seed 1, targets k = 54394 and 31575) to check it builds and the gate holds. It showed
freq_hi and freq_lo slower than base. Predictions below are therefore NOT blind for K(1,17);
the registered run uses a different seed (20261119) and 60 fresh targets per cell. The smoke
output is not analysed or used.

## Instances (frozen)

Koblitz K(a,n), standard-subspace factor base dimension `dim = ceil(n/3)`, m = 3, targets
`[k]G` with k drawn by SplitMix64 (seed 20261119 xor cell constants) uniform in [1, r-1], 60
targets per cell, 3 repetitions per target:
(a,n) in {(0,13),(0,15),(1,15),(1,17),(1,19),(0,19),(0,23),(1,23)} (the usable n = 13..23 curves
of both families). Node budget 2000 reductions (base uses about 170 at n=17). Single thread
(`RAYON_NUM_THREADS=1`), one process at a time.

## Outcome measures

Per attempt: wall ms, counted word ops (`f4_word_ops_thread` plus `bitop/v1` tally word ops),
reductions, splits. Per cell: sums over targets and the median over repetitions, paired
ratio `arm / base` per target (geometric mean of per-target ratios), with 95% bootstrap
percentile intervals over targets (2000 resamples, seed 7). A/A: `base2 / base`.
A candidate is "improving" only if the word-op ratio upper CI is < 1 AND the wall ratio upper CI
is < 1 AND the A/A interval contains 1.

Split of cost: UNSAT attempts (no root) and SAT attempts (>= 1 root) are reported separately.

## Hard gate

Every attempt: if an arm does not exhaust its node budget it must return exactly the root
set of `base`; if it exhausts the budget (censored: its cost is a lower bound) it may only
return roots that `base` returns. The harness aborts on violation. `base` must not exhaust.
Censored attempts are counted and reported; their ratios are lower bounds on the slowdown.

## Predictions (falsifiable, frozen)

- P1 (gate): 0 verdict/root mismatches across all attempts of all arms. Falsified by any abort.
- P2 (A/A): geometric mean `base2/base` word-op ratio is exactly 1.000 (counts are
  deterministic) and wall ratio within [0.97, 1.03] in every cell.
- P3 (primary): `freq_hi` improves (as defined above) in at least 4 of the 8 cells.
  Honest prior given the smoke run: probably false; recorded anyway.
- P4: if `freq_hi` is not improving, `freq_lo` is worse than `base` by word ops in every cell
  (the shipped chain order is a local optimum among these three static orders).
- P5 (UNSAT share): refuted-system attempts (0 roots) are at least 70% of base word ops
  in every cell with n >= 17 (consistent with the 5.2-9.0x oracle ceiling).
- P6: the effect on total relation-phase cost equals the word-op ratio of the solver stage
  (relation phase here is the solver stage alone, one attempt per target, system build excluded
  from arms since it is shared); distance to rho is reported by scaling the existing matched
  reference rows in `research/f4_f5_ecbench_20261019/README.md` by the measured ratio.

Deviations from this protocol are recorded in README.md, never here.
