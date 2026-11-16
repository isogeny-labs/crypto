# LANE nested-dissection: fill-reducing elimination orders for the mixed-peel 2-core

All figures are exact counts from `raw_output.jsonl` / `summary_output.txt`, label **[pcc:unverified]**
(no pcc claim id, no replay certificate, no matched-rho band; not a benchmark-record row).

## Verdict
* Lever class: **native-only at best, and here it is a negative result.** No order beats Markowitz in total
  row_ops in any of the 12 cells except nested dissection at (|F|=512,m=3) 2/16 seeds and (|F|=1024,m=3) 8/16 seeds,
  where the mean is still worse (x1.074 and x1.034 of Markowitz). Counted GAE at w=0 is **unchanged**: linear algebra is
  unpriced at w=0, so every number here is native-only (priced only at w=0.1 / 1 GAE per op) and the group-operation
  count (oracle setup, trials, relations) is not touched. [pcc:unverified]
* Fill (new entries) is lower than Markowitz only for AMD at m=2 (9/16, 8/16, 11/16, 12/16 seeds for |F|=128..1024, but
  means differ by 1-3 entries); for m=3,4 AMD and ND have strictly more fill in 0/16 wins. [pcc:unverified]
* Why: the 2-core of a random m-sparse system is an expander-like graph with no small balanced separators; ordering cost (graph build + AMD/ND bookkeeping) exceeds the arithmetic saved, even where fill is equal. For m=2 the
  core is tiny (14.6 to 33.8 columns), nothing to gain. Markowitz's dynamic pivoting (with row choice by live weight)
  adapts to fill; a static order computed on the symmetric pattern A^T A does not. [pcc:unverified]
* Versus filter+lanczos (Part A medians, `analysis/partA_v3_summary.json`): the best order here is 1.10x to 3.05x
  Lanczos's row_ops for |F|=128..512, m=2,3 (Markowitz itself is the best order in all six cells), so the
  mixed-peel family does not beat filter+lanczos on row_ops at |F| >= 128 whichever order is used. No Lanczos row_ops exist
  in Part A for |F|=1024 or m=4, so those cells have no comparator. [pcc:unverified]
* Gap to matched rho: unchanged by this lane (no group operation saved); p24 incumbent ~56.6k GAE vs rho.negation ~5.0k.

## Method
`examples/ic_alt_nested_dissection.rs` (new file; src/ untouched).
* Stream: copy of `ic_mixed_solver_stream.rs` generator (same seed formula and RNG call order, produced sparse and lazily),
  |F| in {128,256,512,1024}, m in {2,3,4} (m draws with replacement, so rows can have fewer than m distinct factor-base
  entries), 16 seeds, plus the target column. Rows are read until the library `MatchGate` reaches structural full rank
  (= mixed-peel's stop), the matched `cols x cols` square system is solved. All 576 solves return the planted solution.
* Re-implementation of `solve_square` (two-sided peeling, then elimination of the core, back-substitution) with ops
  classified: `arith` (one per non-zero multiply-subtract, pivot factor, rhs update, back-substitution term, peel
  update) and `book` (peel weight reads/updates, Markowitz column scans, plus for AMD/ND the graph build and ordering ops).
  Pivot row = lightest live row of the pivot column, as in the baseline, for all orders.
* Orders on the core's column-interaction graph (two columns adjacent when they share an alive row):
  (a) `markowitz`: dynamic lightest-column choice, bit-for-bit the baseline loop;
  (b) `amd`: quotient-graph approximate minimum degree (Amestoy-Davis-Duff degree bound, element absorption, aggressive
  absorption of elements with w(e)=0; no supervariables, no mass elimination, no dense-row skipping);
  (c) `nd`: recursive bisection, components split first, BFS from a pseudo-peripheral vertex (two sweeps), separator = the
  level minimising 2|level|+|imbalance|, parts ordered first, separator last, leaves (<=12 vertices) or degenerate
  (<3 levels) by static degree; every BFS edge visit and degree read is counted.
* Cross-check: for every instance the re-implemented Markowitz `solve` ops equal `mixed-peel`'s row_ops minus
  (row conversion + gate visits + surplus-row checks) (field `lib_matches_reimpl`, true in 192/192), and the library
  answer is correct. `mixedpeel_equiv_row_ops` = solve ops + those three wrapper terms, so it is directly comparable with
  the library `mixed-peel` and with Part A. My medians equal Part A mixed-peel medians exactly at |F|=256,m=2 (13218.0)
  and within 0.1-1.1% elsewhere (5148.5 vs 5206.0 at 128/2; 112949.5 vs 112997.0 at 512/3); the small difference is
  unexplained (Part A v3 may predate the current library or differ in the stop check); Part A's JSON holds medians, not means.

Commands:
```
cargo build --release --example ic_alt_nested_dissection
cd /home/user/crypto
./target/release/examples/ic_alt_nested_dissection research/ic_sketch_mixed_solver_20261115/alt/nested-dissection/raw_output.jsonl 16   # 6.8 s CPU
python3 research/ic_sketch_mixed_solver_20261115/alt/nested-dissection/summarize.py > research/ic_sketch_mixed_solver_20261115/alt/nested-dissection/summary_output.txt
```

## Results (means over 16 seeds; full table incl. medians in `summary_output.txt`)
mixed-peel-equivalent row_ops (all of: conversion, gate, peel, core elimination, back-substitution, surplus check):

| |F| | m | core cols | Markowitz | AMD | ND | fill M / AMD / ND |
|---|---|---|---|---|---|---|
| 128 | 2 | 14.6 | 5432 | 5748 | 5699 | 9.1 / 7.4 / 8.9 |
| 128 | 3 | 84.6 | 11078 | 18267 | 16814 | 255 / 335 / 425 |
| 128 | 4 | 113.1 | 19430 | 40738 | 67783 | 817 / 1231 / 1588 |
| 256 | 2 | 20.8 | 13252 | 13731 | 13700 | 14.2 / 13.1 / 15.2 |
| 256 | 3 | 166.3 | 33639 | 52925 | 48289 | 730 / 1077 / 1334 |
| 256 | 4 | 224.5 | 72107 | 148453 | 257261 | 2665 / 4506 / 5850 |
| 512 | 2 | 28.2 | 32730 | 33417 | 33327 | 21.8 / 18.9 / 23.8 |
| 512 | 3 | 331.0 | 113482 | 169625 | 121855 | 2005 / 3203 / 4043 |
| 512 | 4 | 450.6 | 313576 | 644057 | 1214233 | 8578 / 16077 / 21651 |
| 1024 | 2 | 33.8 | 71218 | 72071 | 71934 | 26.8 / 22.3 / 28.0 |
| 1024 | 3 | 649.1 | 408945 | 605131 | 422871 | 5844 / 10634 / 12248 |
| 1024 | 4 | 901.3 | 1564219 | 3364696 | 6093749 | 29004 / 59258 / 82704 |

Part A filter+lanczos median row_ops: 128/2 4692.5, 128/3 9201.5, 256/2 10078.5, 256/3 17561.5, 512/2 21370.0, 512/3 37013.0.
Medians of the mixed-peel-equivalent here: Markowitz 5148.5, 10795.5, 13218.0, 33734.5, 33025.0, 112949.5 (best order
over Lanczos: 1.10, 1.17, 1.31, 1.92, 1.55, 3.05; never below 1).

Observation on where the ND/AMD cost goes: at m=3, |F|=1024 the ND arithmetic is 2.0x Markowitz's (168k vs 83k mean), but
bookkeeping is lower (146k vs 217k) because Markowitz's O(core^2) pivot scans (1 per live column per pivot) are replaced
by a one-off ordering cost of 140k); i.e. the only ND win is in *bookkeeping*, never in arithmetic or fill. A static-order
Markowitz-free scheme could therefore only save the scan term, which is a bookkeeping (native) cost; the lazy/bucket-queue
`mixed-peel-lazy` solver already targets that term. [pcc:unverified]

## Limits
* AMD is a simplified AMD (no supervariables or mass elimination), ND uses BFS level sets rather than a multilevel
  partitioner (METIS); better partitioners cannot create separators that random expander-like cores do not have, but this
  was not measured. Static orders on A^T A ignore numeric pivot-row choice. One host, shared CPU, no replay certificate.
* Pivot-row selection (`live_rows` scans in the elimination loop) is uncharged for all orders, as in the baseline.
* LA is unpriced at w=0: any gain here would be native-only (w=0.1 or 1 GAE/op). None was found.
