# Sieve-row solve: Fast (Markowitz) vs SgeBw (SGE + block Wiedemann), 2026-11-15

Plan track T3; not under a frozen protocol.  Solver: `Solver::SgeBw` in
`crates/hec-native-ic/src/solve_bw.rs` (port of the `lpsgebw2` arm of crypto
`origin/main:examples/ic_improve2_06_genus3_kappa_bw.rs`).

## Runs

```sh
R=2:503,2:1009,2:2003,2:4001,2:8009,2:16411,3:401,3:1021,3:4001,4:127,4:1009
HEC_ROWS=$R HEC_SEEDS=3 HEC_SKIP_WALK=1 HEC_VERIFY=0 HEC_BSGS_MAX=2000000 \
  HEC_SIEVE_SOLVER=fast  sieve_probe > fast.txt 2>&1
HEC_ROWS=$R HEC_SEEDS=3 HEC_SKIP_WALK=1 HEC_VERIFY=0 HEC_BSGS_MAX=2000000 \
  HEC_SIEVE_SOLVER=sgebw HEC_BW_TRACE=1 sieve_probe > sgebw.txt 2>&1
python3 summarise.py fast.txt sgebw.txt > summary-table.md
```

Same instances, seeds (20260916..18) and relation sets for both solvers; every
solve used the first m+8 relations (1 round).  `sgebw.txt` carries one
`BWTRACE` line per solve call (SGE / Wiedemann split, residual size).
`sgebw-v1-rhsfold-uncounted.txt` is a first SgeBw run whose binary dropped
the `n` right-hand-side folding products from the count; v1 count + that
term equals the final count on all 33 runs (checked), nothing else differs.
`cx-sweep.txt`: the one-seed sweep used to pick `cx` (folding binary, before
the same `n`-product fix).  `host.txt`: `uptime` around the runs (load 15–125;
wall clock is descriptive only).

## Result (medians over 3 seeds; counts are the measurement)

| g | p | m | Fast mulmods | SgeBw mulmods | SgeBw/Fast | SGE share | residual N (N/m) | nnz/N | matvecs | Fast solve ms | SgeBw solve ms | rounds F/B (max) | correct F/B |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|:-:|
| 2 | 503 | 259 | 7692 | 59582 | 7.75 | 0.043 | 33 (0.127) | 19.2 | 69 | 0.21 | 0.36 | 1/1 | yes/yes |
| 2 | 1009 | 478 | 27398 | 174936 | 6.38 | 0.029 | 62 (0.130) | 21.4 | 112 | 0.50 | 1.13 | 1/1 | yes/yes |
| 2 | 2003 | 1000 | 175261 | 775270 | 4.42 | 0.014 | 132 (0.132) | 21.1 | 217 | 1.76 | 2.26 | 1/1 | yes/yes |
| 2 | 4001 | 2005 | 1346344 | 3059154 | 2.27 | 0.007 | 286 (0.143) | 18.9 | 448 | 8.57 | 4.70 | 1/1 | yes/yes |
| 2 | 8009 | 4036 | 8925667 | 11595271 | 1.30 | 0.004 | 547 (0.136) | 20.1 | 840 | 40.77 | 13.27 | 1/1 | yes/yes |
| 2 | 16411 | 8205 | 70290545 | 48135175 | 0.68 | 0.002 | 1139 (0.139) | 19.2 | 1728 | 267.77 | 75.35 | 1/1 | yes/yes |
| 3 | 401 | 196 | 19891 | 145381 | 7.31 | 0.019 | 56 (0.286) | 19.5 | 103 | 0.32 | 0.80 | 1/1 | yes/yes |
| 3 | 1021 | 506 | 193969 | 850139 | 4.38 | 0.008 | 137 (0.271) | 22.1 | 225 | 1.73 | 1.55 | 1/1 | yes/yes |
| 3 | 4001 | 1997 | 8538856 | 12794220 | 1.50 | 0.002 | 557 (0.279) | 21.8 | 855 | 43.33 | 13.04 | 1/1 | yes/yes |
| 4 | 127 | 66 | 4593 | 37435 | 8.15 | 0.034 | 26 (0.394) | 17.4 | 58 | 0.11 | 0.27 | 1/1 | yes/yes |
| 4 | 1009 | 488 | 488025 | 1699149 | 3.48 | 0.004 | 199 (0.408) | 21.3 | 318 | 3.88 | 2.35 | 1/1 | yes/yes |

Log-log slope of median solve mulmods against m:

- g=2, m 259..8205 (6 rows): Fast 2.67, SgeBw 1.95; upper half (m ≥ 2005): Fast 2.81, SgeBw 1.96
- g=3, m 196..1997 (3 rows): Fast 2.62, SgeBw 1.93; upper half (m ≥ 506): Fast 2.76, SgeBw 1.97

Rows with a run that took >1 solve round or returned a wrong k: none

Walk rows under `Solver::Dense` (sieve-probe-20261114): g2 m=8205 3.57M,
g3 m=1997 2.20M.  SgeBw on sieve rows is 13.5× (g2) and 5.8× (g3) above
those; Fast was 19.7× and 3.9×.

- **Scaling.** SgeBw grows like m^1.95 (g2, all six rows; 1.96 on m ≥ 2005)
  against Fast's m^2.67 (2.81 on m ≥ 2005).  g3: 1.93 vs 2.62 (3 rows).
- **Crossover.** SgeBw counts fewer mulmods than Fast only at large m:
  g2 ratio 2.27 at m=2005, 1.30 at m=4036, 0.68 at m=8205; log-interpolated
  crossover m ≈ 5.4k.  g3: 1.50 at m=1997, extrapolated crossover m ≈ 3.4k.
  g4 (m=488): 3.5×.  Below m ≈ 1000 SgeBw is 4–8× worse (fixed costs: ET=20
  extra sequence terms, generator, folding).
- **Where the cost is.**  SGE is 0.2–4% of SgeBw's count; Wiedemann is the
  rest: ≈ 1.52·n matrix-vector products of `nnz + n` each (83–85% of the
  count at m ≥ 2000) plus the mbasis generator and final sum (≈ 6.2·n²).  The residual left by SGE is a fixed
  fraction of m: n/m ≈ 0.13–0.14 (g2), 0.28 (g3), 0.40 (g4), with
  nnz/n ≈ 19–22 at cx = 32, so cost ≈ 37·n² ≈ 0.7·m² (g2), ≈ 3.2·m² (g3).
- **Correctness.**  `correct` true and `solve_rounds` = 1 on all 66 runs.
- **Wall clock** (descriptive): 75 ms vs 268 ms at g2 m=8205 (another run of
  the same binary: 50 ms).  The time ratio is larger than the count ratio
  (0.68) because the CSR mat-vec accumulates u128 products with one
  reduction per row, while Fast's merges do two REDCs per product; that is
  implementation, not algorithm.

## Design and deviations from the prior art

1. Markowitz SGE as in the prior art (`fill = (w−1)(r−2) − r`, singleton
   cascade first, stop when the minimum fill reaches `nnz/ncols + cx`, `k`
   never eliminated); a row that becomes k-only answers at once (Fast's rule).
2. **Fold, don't trim to square.**  The prior art trims to exactly square
   (heaviest rows first).  On sieve rows this lost rank: g3 p=61 had a
   full-rank 20×14 residual that became 13×13 of rank 11, and Wiedemann
   returned the same wrong k on every retry (the sieve test never finished;
   g4 p=31 failed 33 rounds).  Now up to 16 surplus rows are kept and folded
   in as `B = A_top + R·A_ex` (one random entry per row of R), applied
   without forming B; cost n products per mat-vec, counted.
3. Block Wiedemann with 2 coordinate projections (e_k first), iterative
   mbasis generator, ET = 20, as ported; scalar Berlekamp–Massey if n < 3.
4. **cx = 32** (prior default 2.0).  One-seed sweep over 2, 16, 32, 64, 128:
   cx = 32–64 minimises the count, 13–20% below cx = 2 at m ≈ 2000–8000.
5. Counting: every product mod ℓ is in `mulmods`, including products by ±1
   (the prior art did not count ±1 SGE products), the SGE row factor and
   right-hand-side product (Fast counts neither), mat-vec `nnz + n`,
   rhs folding, every generator product, the final sum.  Inversions are not
   counted (as in Fast).  So the comparison is, if anything, tilted against
   SgeBw.
6. u128 accumulators reduce every ⌊(2^128−1−ℓ)/(ℓ−1)²⌋ products with a
   reduction valid for any u128; tests cover ℓ = 2^61−1 and 2^62−57.

Tried and dropped (one seed, 2:4001, 2:16411, 3:4001): trimming to 16
surplus rows *before* SGE (+3–8% mulmods); a k-aware fill estimate
`(w−1)(r−3) − r` (no gain at its best cx).  Block size 3 was not run: by
the measured constants it would save ≈ 0.17·n mat-vecs (≈ 3.6·n²) but
roughly double the generator (+≈ 6·n²).

## Reading

SgeBw does what T3 asked for in exponent (≈2 instead of ≈2.8) but not in
constant: it beats Fast only above m ≈ 5k (g2) / 3.4k (g3), and stays an
order of magnitude above the walk rows' Dense solve at m=8205.  The residual
after SGE is ~m/7 (g2) with ~20 non-zeros per row, so any Wiedemann-type
finish costs ~37·(m/7)².  Getting near the walk's 3.57M would need a much
smaller residual (better filtering / merging of the sieve rows, e.g. using
that all rows of a line share α and β), not a different Krylov solver.

## Later finding (parent session, 2026-11-15)

On the pruned 2LP matrices of `LP-SIEVE-PROBE-20261115.md` (`lp-sieve-probe-20261115/solve-sgebw.txt`),
`Solver::SgeBw` returned a wrong `k` once: walk rows, (4, 509), seed 20261117, μ = 1, a system
`Solver::Fast` solved correctly. The `k·D₁ = D₂` check caught it. Cause not investigated; it may be
the rank-loss mode fixed above for sieve rows. SgeBw also cost 2–4× more mulmods than `Fast` on those
matrices.
