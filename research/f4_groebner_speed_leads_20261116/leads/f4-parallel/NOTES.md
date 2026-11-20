# Lead f4-parallel — notes  [pcc:unverified]

Scope: `src/cryptanalysis/f4_fp.rs` only. Base: HEAD a53cc49711 (branch claude/funny-brown-735uvs).
Baseline binaries: scratchpad/baseline/bin. Host: shared 4-core VM, other leads building and running; all
wall-clock numbers below are indicative only.

## 1. Pre-change observations (baseline binary, F4_DEBUG=1 phase timers, summed over steps)

Commands:
```
F4_DEBUG=1 [RAYON_NUM_THREADS=1|4] [F4_FP_SYMBOLIC_PAR=0|1] baseline/bin/f4_fp_bench 1 large 2> ... ; python3 phase.py
F4_FP_FL=0|1 RAYON_NUM_THREADS=1|4 F4_FP_SYMBOLIC_PAR=0 baseline/bin/f4_fp_bench 5 large
```
quad_n9 (ms, one run each, noisy): 4 thr: symbolic 31-35, matrix 13-16, reduce 98-110, update 8, interreduce 21;
1 thr: symbolic 25-38, matrix 11-33, reduce 177-246, update 7-8, interreduce 21.
* Symbolic preprocessing gets SLOWER with 4 threads: SYMBOLIC_PAR=1 vs 0 at 4 threads, 3 interleaved pairs:
  quad_n9 31.8/33.1/31.8 vs 22.8/22.7/21.1; quad_n8 13.2/16.3/11.0 vs 5.6/8.1/6.9; cubic_n5 6.1/5.2/5.0 vs
  2.5/4.0/2.9; overdet_n10 4.1/5.1/3.8 vs 1.7/2.4/2.4. Fingerprints and field_ops identical (diff of JSON).
  research/f4_fp_msolve_parity_20261011/REPORT.typ line 21: round-parallel symbolic was a win on quad_n11
  (one step 151 -> 49 ms), so it must stay for big steps.
* Final interreduction (interreduce_linear) ~21 ms at both 1 and 4 threads on quad_n9: no scaling.
* Reduce scales 1.8x on 4 threads.

## 2. Mechanism found by reading the code (before instrumenting)

1. `reduce_pairs_fl` builds B' in row blocks of FL_BLOCK=64 reducers (rightmost lead first). Each block:
   rows reduced in parallel against finished blocks, then the in-block dependencies applied on ONE
   thread. Also the parallel part only runs when `64 * n_free > PAR_WORK (32768)`, i.e. n_free > 512;
   below that the whole B' build is single-threaded. cubic_n5 has n_free ~ 250 in its big steps, so
   at 4 threads FL there is entirely serial while the cascade (FL off) is row-parallel: this explains
   "FL hurts small systems".
   Key fact: B'_k = B_k - sum_j A_kj B'_j and the factors A_kj are original reducer coefficients, so
   each free COLUMN of B' depends only on the same column of earlier rows. Column tiles are fully
   independent: no serial finish is needed at all.
2. `reduce_pairs_blocked` (pair rows D - C B') parallelises over 64-row blocks gated by
   `rows * n_free > PAR_WORK`; with ~60-400 pair rows that is 1-7 tasks for 4 threads.
3. `rref32_deferred`: chunk of 64 rows reduced in parallel against known pivots, then stored one at a
   time with re-reduction against chunk-mates on one thread. The first chunk (known = 0) is entirely
   serial (~C^2/2 row updates). Pair-row residues are very rank deficient (quad_n9 step 7: ~400 rows,
   rank ~48), so the first chunk is a large share of this function.
4. `symbolic_par` runs one parallel round per reducer-chain depth, with 3-4 rayon dispatches per
   round (lookup map, fold/reduce set, fix-up, reducer search) even for rounds with a handful of rows.

## 3. Changes (all default-on; `F4_FP_PAR_OLD=1` restores every old path exactly)

* `bprime_tiled`: column-tiled B' build; equal-work tile cuts (work per column = number of updates
  covering it), 3 tiles per thread (`F4_FP_PAR_FL_TILES`), parallel when the update work exceeds
  2^17 multiply-adds (`F4_FP_PAR_FL_WORK`). Same entries, same counted row ops (tiles partition each
  update's column range). Unit test `bprime_tiled_matches_row_build` (tilings 1,2,3,7 vs exact row build).
* `reduce_pairs_blocked`: parallel when entries * n_free / 2 > PAR_WORK, block rows =
  clamp(ceil(rows / (2 threads)), 8, 64). Same rows, same count.
* `rref32_deferred`: adaptive chunk size = clamp(#known pivots, 4, 64) (thread-count independent).
  Same unique RREF; the row-operation count may change slightly (that is the serial work being cut).
* Symbolic: `symbolic_par` only from `F4_FP_PAR_SYM_SEEDS` (default 1024) seed rows; inside it, a
  round with < `F4_FP_PAR_SYM_FRONTIER` (256) frontier rows runs on one thread (same computation),
  reducer search parallel only when new monomials x alive basis >= PAR_WORK.
* `F4_FP_PAR_PROF=1`: per-run tallies of row ops by section and by whether they ran spread or on one
  thread, printed to stderr ("f4par steps ..." before interreduction, "f4par final ..." after).

## 4. Prediction (written before any measurement of the new build)

* Fingerprints (basis_fp, solve_fp) identical to baseline for all 13 cases, at RAYON_NUM_THREADS=1,2,4.
* field_ops identical to baseline except where the deferred RREF runs with adaptive chunks: expected
  change below +-1 % of a case's field_ops (deferred is a small part of the reduction).
* Profile of the OLD path (F4_FP_PAR_OLD=1 F4_FP_PAR_PROF=1) on quad_n9: one-thread row ops (in-block
  B' finish + small-block B' + deferred store + serial pair/cascade) = 15-35 % of all row ops.
  Amdahl with s = 0.25: reduce speed-up at 4 thr <= 1/(0.25+0.75/4) = 2.3x; with the memory-bound
  kernel (scalar vs AVX-512 only 1.4x) the observed 1.8x fits.
* NEW path: one-thread row-op share on quad_n9 below 5 %; on cubic_n5 most B' work moves from one
  thread to tiles.
* Indicative wall clock at 4 threads: quad_n9 symbolic ~31 -> ~22 ms, reduce ~98 -> ~70-80 ms, total
  ~150-180 -> ~120-140 ms; cubic_n5 total ~18 -> ~14-15 ms (no longer slower than FL off / 1 thread).
  The uncounted serial phases (update ~8 ms, interreduce symbolic, matrix build) are untouched and cap
  the 4-thread speed-up on quad_n9 near 2x overall.

## 5. Build 1 results (patch.build1.diff, binary bin1/f4_fp_bench)

Build: `TEST_FILTER=f4_fp locked_build.sh LEADDIR f4_fp_bench src/cryptanalysis/f4_fp.rs` -> ok;
`cargo test --release --lib f4_fp`: 96 passed (incl. new `bprime_tiled_matches_row_build`). No warnings.

Correctness (deterministic): `f4_fp_bench 1 large` at RAYON_NUM_THREADS=1,2,4, default and F4_FP_PAR_OLD=1,
vs baseline binary: every line identical in basis_fp, solve_fp, field_ops, solve_field_ops, steps,
max_rows, max_cols, basis_len, solutions (13 cases x 6 arms; `python3 fps.py` + diff, empty).
Even the adaptive deferred chunks left field_ops identical (chunk + store row ops are the same set of
updates: quad_n9 old 45,462,085 + 9,446,366 = new 46,866,439 + 32,041 + 8,009,971 = 54,908,451).

Deterministic profile (F4_FP_PAR_PROF=1, 4 threads, first f4 call of the case, steps + interreduction):
| case | row ops | one-thread row ops OLD | one-thread NEW (build 1) |
|---|---|---|---|
| quad_n9 | 399,396,830 | 40,931,644 (10.2 %): B' in-block 26.0 M, B' small blocks 2.5 M, deferred store 9.4 M, pairs 2.3 M | 8,765,428 (2.2 %) |
| quad_n8 | 56,664,766 | 23,445,435 (41.4 %) | 2,593,925 (4.6 %) |
| cubic_n5 | 12,712,111 | 12,447,030 (97.9 %): B' 4.4 M all serial (n_free < 512), pairs 6.5 M serial | 1,101,293 (8.7 %) |
| overdet_n12 | 84,352,865 | 5,230,981 (6.2 %) | 4,266,139 (5.1 %) |

Amdahl on counted row ops alone (quad_n9, s = 0.102): reduce speed-up <= 1/(0.102 + 0.898/4) = 3.1x at 4
threads; observed reduce scaling ~1.8x, so the rest is the memory-bound kernel (scalar vs AVX-512 1.4x)
and the uncounted serial phases: symbolic (round-parallel was 1.4-5x SLOWER than sequential at these
sizes), interreduction symbolic preprocessing (~13 ms on quad_n9, single-threaded FxSet<Vec<u32>>),
basis update (~8 ms), matrix build (~11-13 ms, no scaling).
cubic_n5 at 4 threads was 98 % single-threaded in the counted reduction plus a slower parallel symbolic:
that is the whole reason FL+threads were slower than FL-off or 1 thread there.

Wall clock (indicative; host load 2-4 from other leads):
`timing.sh t1.jsonl 5` (5 interleaved rounds, base/old/new, 5 repeats each, medians of medians):
4 thr: quad_n9 base 186.6 old 181.0 new 160.5 (old/new 1.13); quad_n8 50.4/48.1/39.5 (1.22);
cubic_n5 23.5/19.5/17.0 (1.15); overdet_n10 32.8/30.1/26.4 (1.14); overdet_n12 45.4/45.8/43.5 (1.05);
quad_n7 16.9/14.1/12.0 (1.17).
1 thr (8 interleaved rounds, old vs new, 7 repeats): within noise (0.86-1.23, wins 2/8-7/8).
Section timers (F4_FP_PAR_PROF, 10 interleaved runs, medians, load ~4): symbolic quad_n9 61.8 -> 34.8 ms,
quad_n8 26.7 -> 5.0, cubic_n5 10.0 -> 2.4 (clear win). B' build: no gain (quad_n9 65.9 vs 66.5; cubic_n5
3.3 vs 4.0): splitting every update into ~8 column pieces of ~70 columns adds per-call overhead about equal
to the work (each update is only ~300 columns long on these sizes). Deferred: adaptive chunks raised the
parallel dispatches (cubic_n5 15 -> 51 chunks, quad_n9 47 -> 89, overdet_n12 23 -> 34) and overdet_n12 got
slower (35.4 -> 41.4 ms); the serial saving was only 1.4 M ops on quad_n9.

## 6. Build 2 changes (from what build 1 showed)

* Adaptive deferred chunks removed (back to fixed 64).
* B' default = row blocks as before (full-length updates), with (a) the spread gate on the block's update
  work (sum of reducer lengths * n_free / 2 > PAR_WORK) instead of block width, so narrow systems
  (cubic_n5) are no longer serial, and (b) the in-block finish split into column tiles (one per thread,
  equal-work cuts) when its work exceeds 2^16 (`F4_FP_PAR_FL_INBLOCK`): only the 22 % in-block part pays the
  tile overhead. Full column tiling stays available as `F4_FP_PAR_FL_MODE=tiles`.
* Unit test extended: `finish_block_tiled` at 1-4 threads equals the exact row build of a block.

## 7. Build 2 results -> build 3 changes

Build 2 (bin2, patch.build2.diff): fingerprints/counters identical (m2_*, n2_*, o2_* .fp files vs base.fp).
Section timers (prof2_new.txt vs prof2_tiles.txt) showed the full column-tiled B' build was competitive
once B' fits in memory comfortably, so build 3 changed only the B' dispatch:
* `fl_mode_tiles(entries)`: default = column tiles (`bprime_tiled`) while B' has <= 2^26 entries
  (`FL_TILES_MAX_ENTRIES`, the tiles hold a second copy of B'); larger B' uses the row blocks with the new
  gates and tiled in-block finish. `F4_FP_PAR_FL_MODE=tiles|rows` forces either.
* Profile helpers return early unless `F4_FP_PAR_PROF=1`.

## 8. Build 3 verification (bin3 = bin at 20:43, sha256 5314251202e8...; build3.out: 96 tests passed)

Commands (this session, after the container restart):
```
for t in 1 2 4: RAYON_NUM_THREADS=$t {baseline/bin, bin3, F4_FP_PAR_OLD=1 bin3}/f4_fp_bench 3 large > v3/*.json
python3 fps.py v3/X.json > v3/X.fp ; cmp each with v3/base_t4.fp
F4_FP_PAR_PROF=1 RAYON_NUM_THREADS={4,1} [F4_FP_PAR_OLD=1] bin3/f4_fp_bench 1 large ; python3 prof2.py ...
```
* All 9 arms (base/new/old x 1,2,4 threads): every line identical in basis_fp, solve_fp, field_ops,
  solve_field_ops, steps, max_rows, max_cols, basis_len, solutions for all 13 cases. Also f3_{new,old,rows,
  tiles}_t{1,2,4}.fp from the previous session (FL_MODE forced either way) identical to base.fp.
* Deterministic one-thread row-op share at 4 threads (first f4 call): quad_n9 10.2 % -> 2.5 %
  (10,169,782 of 399,396,830; 9,446,366 of them are the deferred store, untouched); quad_n8 41.4 -> 5.6 %;
  cubic_n5 97.9 -> 12.4 %; overdet_n12 6.2 -> 5.3 %. Counted Amdahl bound for quad_n9 reduction at 4 threads:
  3.1x -> 3.7x.
* Indicative wall clock (t4.jsonl, previous session, 8 interleaved rounds, base/old/new, `5 large`):
  4 thr base/new: quad_n7 13.2/9.2 (1.44x), quad_n8 43.5/33.1 (1.31), quad_n9 151.4/127.0 (1.19),
  cubic_n5 16.0/14.2 (1.13), overdet_n10 23.6/21.5 (1.09), overdet_n12 40.7/38.4 (1.06).
  1 thr: neutral except cubic_n5 14.3/15.4 (0.93) and quad_n7 9.8/10.7 (0.91): the column tiles cost
  per-tile overhead even when one thread runs them all. Re-checked this session (t6/t7.jsonl, load 4-5,
  10 rounds at 1 thread, new vs F4_FP_PAR_FL_MODE=rows): new/rows 1.03-1.08 on quad_n7/n9/cubic_n5.

## 9. Build 4 (final; f4_fp.build4.rs, patch.build4.diff == patch.diff, bin4/, build4.out)

Change: the default B' dispatch uses column tiles only with more than one rayon thread; on one thread the
row blocks run (forced `F4_FP_PAR_FL_MODE=tiles|rows` still win). Output-neutral by construction.
`TEST_FILTER=f4_fp locked_build.sh ... f4_fp_bench src/cryptanalysis/f4_fp.rs`: release ok, 96 passed.
bin4 sha256 ebbb50aa7dcbf197...
* Fingerprints and all counters: new / F4_FP_PAR_OLD=1 / F4_FP_PAR_FL_MODE=tiles at RAYON_NUM_THREADS=1,2,4
  (v4/*.fp) identical to the baseline for all 13 cases (`3 large`).
* Profile at 4 and 2 threads identical to build 3 (shares above; the gates are thread-count independent
  except the tile count, which does not change counted ops).
* Indicative wall clock (t8.jsonl, 7 interleaved rounds base/old/new, load 2.2-3.2):
  | case | 4 thr base | old | new | base/new | 1 thr base | new | base/new |
  |---|---|---|---|---|---|---|---|
  | quad_n7 | 13.7 | 13.8 | 9.9 | 1.39 | 10.8 | 10.8 | 1.00 |
  | quad_n8 | 45.9 | 43.9 | 39.8 | 1.15 | 53.0 | 49.0 | 1.08 |
  | quad_n9 | 166.2 | 166.0 | 147.2 | 1.13 | 221.9 | 218.5 | 1.02 |
  | cubic_n5 | 17.5 | 18.0 | 15.4 | 1.13 | 15.8 | 15.4 | 1.03 |
  | overdet_n10 | 27.2 | 26.4 | 23.5 | 1.16 | 24.4 | 26.9 | 0.91 (noise? 0.97-1.07 in other runs) |
  | overdet_n12 | 45.5 | 43.9 | 42.4 | 1.07 | 47.1 | 46.2 | 1.02 |
  4-thread / 1-thread scaling: quad_n9 base 1.34x -> new 1.48x (t8), 1.25 -> 1.52x (t4.jsonl);
  cubic_n5: 4 threads no longer slower than 1 thread (new 15.4 vs 15.4; base 17.5 vs 15.8).

## 10. Answer, prediction check, risks

Why 1.63x: (a) counted reduction was 10 % single-threaded on quad_n9 (B' in-block finish 26 M ops, small B'
blocks, deferred store 9.4 M) -> Amdahl <= 3.1x on reduce; (b) the AVX-512 kernel is memory-bound (scalar vs
SIMD 1.4x) so parallel parts scale ~1.8x; (c) uncounted serial phases: final interreduction (~21 ms, no
scaling), basis update (~8 ms), matrix build (~11-13 ms), and symbolic preprocessing that was SLOWER in
parallel (round-parallel with 3-4 rayon dispatches per round, 31.8 vs 22.8 ms on quad_n9).
Why FL/threads hurt cubic_n5: with n_free < 512 every 64-reducer B' block failed the width gate, so FL ran
98 % on one thread while FL-off ran the cascade in parallel; parallel symbolic added dispatch cost.
Prediction vs result: fingerprints identical (as predicted); field_ops identical (predicted +-1 %, got 0
because adaptive chunks were dropped); quad_n9 one-thread share 2.5 % (predicted < 5 %); quad_n9 total
4 thr 166 -> 147 ms indicative (predicted 120-140: under-delivered, the B' tile gain is small because each
update is only ~300 columns long and the memory-bound kernel caps scaling); cubic_n5 18 -> 15 ms (predicted
14-15, met).
Not done / next: deferred store (9.4 M ops, now 93 % of the counted serial work on quad_n9) is still
sequential; interreduction symbolic and basis update remain serial.
Risks: the cutoffs (PAR_SYM_SEEDS 1024, PAR_SYM_FRONTIER 256, FL_WORK 2^17, FL_INBLOCK 2^16,
FL_TILES_MAX_ENTRIES 2^26) were tuned on the 4-core VM and these 6 cases; the tiled B' holds a second copy
of B' (bounded at 256 MB); larger systems (quad_n10/11) were not timed here — the symbolic gate keeps
round-parallel symbolic for >= 1024 seeds, which the msolve parity report needs for quad_n11. All timing is
under shared load and indicative only; a serial A/A stage must confirm. [pcc:unverified]
