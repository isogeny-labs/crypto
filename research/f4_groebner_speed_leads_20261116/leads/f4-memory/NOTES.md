# Lead f4-memory: K-pivot fused (register-delayed) lazy updates in f4_fp.rs

All results here are [pcc:unverified] assertions of this lead.

## Mechanism (written before any measurement)

Every dense update in the deferred (p <= 2^16) path is `acc[from..] += f * pivot[from..]`
with `acc: [u64]` and `pivot: [u32]` (add_lazy). One update per pivot means every field
op moves 8 B acc load + 8 B acc store + 4 B pivot load = 20 B, and the 64-bit accumulator
row (n_free or n_cols words, up to ~2188 * 8 = 17.5 KB, block accumulators 64 x 256 x 8 =
128 KB in reduce_pairs_blocked > 48 KB L1d) is re-streamed once per pivot.

Change: gather up to K = 8 updates of the same accumulator and apply them in one pass
(`acc[x] += f1*p1[x] + ... + f8*p8[x]`, the partial sum kept in a zmm register), segmented
by each update's start column so no pivot is read left of its start. Field ops (MACs,
pivot words) are unchanged; accumulator words loaded+stored fall from one pass per update
to one pass per group of <= 8. Model: bytes/op 20 -> 4 + 16 * (acc_touch/ops); with
acc_touch/ops ~ 1/6 that is ~6.7 B/op, about 3x fewer bytes.

Applied at three places, additive (new functions, one dispatch line each):
 (A) reduce_lazy_by (rref32_deferred forward, chunk re-reduce, back-substitution): factors
     depend on acc, so the next pivot's factor is computed as
     reduce(acc[c] + sum_{g in group} f_g * p_g[c]) (exactly the value the sequential
     path sees mod p); the group is flushed when 8 nonzero factors are gathered.
 (B) reduce_pairs_fl B' construction (against_finished and the in-block finish): factors
     are the input coefficients, so any 8 updates of one row are fused.
 (C) reduce_pairs_blocked: hits sorted by (row, reducer) instead of (reducer, row); per
     column tile each row's hits are fused 8 at a time, so the row's tile accumulator
     (2 KB) stays in L1 and B' tiles come from L2.

Exactness: accumulator sums are exact integers; only the order of additions and the
normalisation schedule change. Normalisation now happens before a flush that would take
the count of unnormalised updates past 256 (the same bound as before), so no lane can
overflow; every factor equals the old one mod p, every stored row is fp.reduce of the
same residue. Output must be bit-identical; field_ops identical (counted with the same
formula, acc.len()-from per nonzero update).

Gate: F4_FP_MEM_OLD=1 restores the old path exactly (all three sites). Attribution
sub-toggles F4_FP_MEM_A=0, F4_FP_MEM_B=0, F4_FP_MEM_C=0 turn off one site each.
Instrumentation: F4_FP_MEM_STATS=1 prints, per f4() call on stderr, for each site the
field ops, accumulator words touched (new path) and flushes. Old-path acc_touch == ops by
construction (one pass per update).

## Prediction (before measuring)

- Fingerprints (basis_fp, solve_fp) and field_ops identical for all 3 large cases.
- acc_touch/ops: (A) ~0.13-0.2, (B) ~0.2-0.35 (bstart spread is wider), (C) ~0.15-0.3.
- Modelled bytes moved in the dense updates fall ~2.5-3.3x.
- Wall-clock (indicative only, noisy host): quad_n9 150 ms -> 105-125 ms (1.2-1.4x),
  quad_n8 similar ratio; cubic_n5 (p=31, small) about neutral. If the scalar-vs-AVX-512
  ratio of 1.40x means the loop is store/bandwidth-bound, fusion should recover most of
  the gap to compute-bound; if wall-clock does not move, the bound is elsewhere
  (symbolic preprocessing, allocation, rayon overhead).

## Commands run

    git show HEAD:src/cryptanalysis/f4_fp.rs > work/src/cryptanalysis/f4_fp.rs   # + newcode.rs spliced in, 4 dispatch lines, 1 unit test
    # pre-build check: standalone crate mini/ (the add_lazy..reduce_pairs_blocked region of f4_fp.rs
    # verbatim plus the new code, mem_site switched by an atomic): random rref32_deferred and FL
    # instances, sites {0,1,2,4,6,7}, p in {31, 65521}, normalisation exercised, rows/pivots/ops equal
    cargo test --release --offline   (mini; also with F4_FP_FL_BLOCKED=0)
    TEST_FILTER=f4_fp::tests locked_build.sh leads/f4-memory "f4_fp_bench" src/cryptanalysis/f4_fp.rs  # build 1
    baseline/bin/f4_fp_bench 3 large > baseline_bench.jsonl
    bin/f4_fp_bench 3 large > new_bench.jsonl ; F4_FP_MEM_OLD=1 bin/f4_fp_bench 3 large > old_bench.jsonl
    F4_FP_MEM_STATS=1 bin/f4_fp_bench 1 large 2> stats.err
    # interleaved arms, 6 rounds x {new, old, new2 (A/A), A0, B0, C0}, "5 large", 4 threads -> timing/t4.txt
    # 4 rounds x {new, old} RAYON_NUM_THREADS=1 -> timing/t1.txt
    cargo run --release --example mb|fs|kn   (mini microbenchmarks, kernels in isolation)
    locked_build.sh ... (build 2: + MemTimer wall-clock of reduce_pairs_fl / rref32_deferred under F4_FP_MEM_STATS)

## Results (build 1)

Correctness [pcc:unverified]: all 13 cases of `f4_fp_bench 3 large` give identical basis_fp,
field_ops, solve_fp, solve_field_ops, steps, max_rows, max_cols for baseline binary (HEAD
a53cc497), new default, and F4_FP_MEM_OLD=1; also identical across all 6 timing arms
(A/B/C sub-toggles). `cargo test --release --lib f4_fp::tests`: 27 passed, including the
new mem_fused_apply_matches_sequential and narrow_arithmetic_matches_wide_rref.

Deterministic traffic (F4_FP_MEM_STATS=1, quad_n9_p65521, one f4 call):

| site | field ops | acc words / ops (new) | flushes |
|---|---|---|---|
| A deferred rref | 69.3M | 0.142 | 19,077 |
| B B' build | 115.1M | 0.142 | 37,183 |
| C blocked pairs | 214.3M | 0.131 | 172,412 |
| total | 398.7M of 399.4M field ops (99.8%) | | |

Largest step: 735 free columns, 1548 reducers, 567 pair rows. quad_n8: 11.5M/16.7M/28.3M,
ratios 0.152/0.150/0.135. overdet_n12_m24: 62.3M/2.0M/19.7M, ratios 0.134/0.148/0.140.
Modelled bytes per field op in the dense updates: 20 B -> 4 + 16*0.136 = 6.2 B (3.2x fewer),
matching the prediction (0.13-0.35 predicted; observed at the low end).

Wall-clock, indicative only (host shared with other leads' builds; medians of 6 interleaved
rounds of 5 repeats, 4 threads): quad_n9 new 185.0 / A-A new2 174.3 / old 185.6 ms (ratio
1.00, inside the A/A spread of 6%); quad_n8 48.1 / 54.1 / 56.5; overdet_n12 30.8 / 35.1 /
46.9; cubic_n5 23.3 / 21.4 / 21.6. One thread: quad_n9 244.8 vs 255.8 (1.05), quad_n8 1.10,
cubic_n5 0.75, overdet_n10 0.85. No consistent wall-clock gain on quad_n9: the 3.2x cut in
modelled bytes did not translate.

Why (mini microbenchmark of the kernel alone, 735-wide rows, 200k updates, pool of pivot
rows of growing size, min of 5):

| pivot pool | one-at-a-time ns/op | fused-8 ns/op |
|---|---|---|
| 183 KB (L2) | 0.149 | 0.166 |
| 1.4 MB (L2) | 0.162 | 0.174 |
| 4.4 MB (L3, = quad_n9 B') | 0.222 | 0.189 |
| 17 MB (L3) | 0.431 | 0.366 |

The one-update AVX-512 kernel already runs at ~0.15 ns/op = ~2.5 cycles per 8-lane
iteration, near the port bound of vpmovzxdq (port 5) + vpmuludq + vpaddq; fusing removes
loads/stores that were not the bottleneck and adds per-segment overhead. It only wins once
the pivot set leaves L2. 399M ops x ~0.15-0.2 ns = 60-80 ms of single-thread kernel time
out of ~245 ms single-thread quad_n9: most of the wall-clock is outside the dense updates,
which is also why scalar-vs-AVX-512 is only 1.40x. The premise "the update loop is
memory-bound" is falsified at the kernel level [pcc:unverified].
mini rref32_deferred 567x735 dense (p=65521): 1 thread old 29.7 / new 25.6 ms (min),
4 threads old 23.2 / new 11.8 ms (min) - the deferred site is the one with a measurable gain.

## Results (build 2: same change + MemTimer phase clocks)

Output again identical to baseline for all 13 cases (basis_fp, field_ops, solve_fp,
solve_field_ops). Phase wall-clock under F4_FP_MEM_STATS=1, medians of 5 interleaved runs,
indicative only (t2.err):

| case, threads | arm | f4 ms | reduce_pairs_fl (B'+pairs) | rref32_deferred | rest |
|---|---|---|---|---|---|
| quad_n9, 1 | old | 271.8 | 165.1 | 31.1 | 75.6 |
| quad_n9, 1 | new | 254.3 | 145.6 | 32.5 | 76.2 |
| quad_n9, 4 | old | 199.7 | 88.2 | 21.6 | 89.9 |
| quad_n9, 4 | new | 183.7 | 77.0 | 19.5 | 87.2 |
| quad_n8, 4 | old | 52.8 | 18.8 | 6.2 | 27.7 |
| quad_n8, 4 | new | 62.0 | 22.6 | 6.8 | 32.7 |
| overdet_n12, 1 | old | 57.1 | 19.1 | 30.3 | 7.7 |
| overdet_n12, 1 | new | 47.7 | 16.5 | 22.9 | 8.3 |

Reading: in situ the FL phase runs at ~0.5 ns/field op (old, 1 thread), 2-3x slower than
the isolated kernel, so there IS a memory component at the site level (B' = 1548 x 735 x 4
= 4.4 MB > 2 MB L2); fusion trims the FL phase ~12% on quad_n9 (1 and 4 threads) and the
deferred phase on overdet_n12 ~25% (1 thread). quad_n8 at 4 threads moved the wrong way in
this sample (noise level on this host is +-10-15%). About 30-45% of quad_n9 f4 time
(76-90 ms) lies outside both dense phases (symbolic preprocessing, matrix build, pair
selection), untouched by any update-kernel change.

## Verdict

Candidate, small. Deterministic: accumulator words moved per field op cut from 1.0 to
0.13-0.15 at all three sites (7x), modelled update bytes 20 -> 6.2 B/op, output
bit-identical, field_ops unchanged. Wall-clock: FL phase ~ -12% on quad_n9 (indicative);
whole-f4 change on quad_n9 (-4% to -8%) is inside host noise and must be confirmed by the
serial A/A stage. The lead's premise is only half right: the isolated AVX-512 update kernel
is port-bound (vpmovzxdq + vpmuludq, ~2.5 cycles per 8 lanes), not bandwidth-bound; the
site-level slowdown comes from B'/pivot reads out of L3, which K-fusion does not remove.

Next step suggested (not done, no build left): C*B' in reduce_pairs_blocked is a dense-ish
product; a GEMM-style register-blocked microkernel (e.g. 4 rows x 16 columns of u64
accumulators in zmm registers, looping over reducers) would amortise each B' load and its
u32->u64 widening over 4 rows, attacking the port-5 bound; and storing B' as u16 when
p <= 65521 (entries < 2^16) halves the B' working set to 2.2 MB, about L2. Both must keep
field_ops as counted today (zero factors skipped in the count).

## Risks

- Normalisation schedule differs (before a flush that would pass 256 pending updates rather
  than after the 256th); bound unchanged, residues identical; covered by tests.
- Fused flush uses unsafe AVX-512 with asserted bounds; scalar fallback for simd 0/1
  (AVX2-only hosts lose the hand-vectorised path at the fused sites: compiler auto-vectorises).
- Duplicates reduce_pairs_fl's body (reduce_pairs_fl_mem) and reduce_pairs_blocked's
  (reduce_pairs_blocked_mem); the f4-parallel lead's edits to those originals would need to
  be mirrored at integration. Dispatch lines: top of reduce_lazy_by, reduce_pairs_fl,
  reduce_pairs_blocked; MemTimer line in reduce_pairs_fl and rref32_deferred;
  mem_stats_report in f4().
- Toggle: F4_FP_MEM_OLD=1 (all sites); F4_FP_MEM_A/B/C=0 per site; F4_FP_MEM_STATS=1 stats.

Files: patch.diff (build 2, 632 lines; build 1 patch in patch1.diff), bin/f4_fp_bench
(build 2), bin1/f4_fp_bench (build 1), baseline_bench.jsonl, new_bench.jsonl,
old_bench.jsonl, new2_bench.jsonl, stats.err, t2.err, timing/t4.txt, timing/t1.txt, mini/.
