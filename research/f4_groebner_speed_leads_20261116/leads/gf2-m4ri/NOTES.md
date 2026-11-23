# Lead gf2-m4ri — NOTES  [pcc:unverified]

## Where the work goes (baseline, HEAD a53cc497, measured before any change)

koblitz_f4_kernel_bench (degree-3 Macaulay, public-synthetic K_0/F_2^31 dim-16 cell):
- x system: 1023 x 4369, rank 1022, rref word_ops 4,335,210 (gf2_elim kernel; the bench's
  "rref_kernel" label says active_suffix but rref_f2_counted routes 128x256+ to gf2_elim).
- sym system: 992 x 3872, rank 992, word_ops 3,628,128.
- Ablations (wall, indicative only, x system): default 8.2 ms; KIC_F4_KERNEL=legacy 10.4 ms
  (word_ops 8,930,014, suffix kernel because cols > 4*rows); KIC_F4_RREF_SUFFIX=1 10.4 ms;
  legacy+KIC_F4_M4RI_BLOCK=8 10.3 ms (M4RI never reached: cols > 4 rows); legacy+ALLOCATING 10.5 ms;
  KIC_GF2_SIMD=0 9.6 ms (same word_ops). The koblitz_groebner M4RI knobs do not touch this cell.
- callgrind (deterministic Ir, RAYON_NUM_THREADS=1; valgrind has no AVX-512 so gf2_elim runs AVX2):
  macaulay_profile 71,130,303 Ir = build_macaulay 37,952,323 (53%) + rref 32,941,397 (46%).
  Build split: macaulay_rows_monos_with_mask 15.2M (per-row sort of ~136 masks 10.6M, 1024 Vec allocs),
  macaulay_columns 11.4M (hash insert of 139,517 nonzeros 8.0M + key sort 1.8M),
  pack_rows_with_layout 10.7M (hashed F4ColumnIndex::get 8.1M; masks up to 2^32 so no dense index).
  => On this cell ROW GENERATION, not elimination, is the larger half.

## Mechanism (one change)

Sort-free parity row generation for cold Macaulay builds (build_macaulay_packed, used by
macaulay_profile, MatrixF4 nodes, and InheritedF4 roots on layout-cache misses):
every monomial of degree <= D in n <= 64 vars gets a direct slot (colex rank within degree,
<= 2^20 slots, D <= 6). Per product t*f: toggle slots in a parity bitset (even multiplicities
cancel), read odd slots back in term order clearing the bits, mark first-seen columns in a
seen bitset. Then sort only the distinct columns (same mono_key order), write slot->column, and
scatter each row. Removes per-row sort, per-row Vec allocation, the hash set and the hashed
column lookups. Thread-local scratch, no per-call O(slots) clearing. Same rows, row order,
columns, caps, layout-cache side effects; the bitop/v1 tally is charged with the old formulas
(accounting unchanged on purpose). Gate: KIC_M4RI_OLD=1 restores the old path exactly.
Fallback to old path: degree > 6, slots > 2^20, support outside n_vars, or a cached layout present.

## Prediction (written before measuring)

- Identical: rows/cols/rank, rref word_ops (4,335,210 x / 3,628,128 sym), bitop tally, stage-bench
  verdict digests and word_ops, crossbred verdicts and bit ops, f4_fp fingerprints (untouched module).
- callgrind x: build 38.0M Ir -> ~8-12M Ir; macaulay_profile 71M -> ~42-45M Ir (~1.6x).
- wall (indicative): x kernel median 8.2 ms -> ~5.5-6.5 ms.
- Production stage (groebner_stage_bench): gain only in the fraction of build time spent on cold
  builds (layout misses); expected small single-digit % or less if hits dominate.

## Build history (3 builds used, the cap)

- Build 1: first version (bin/ then, later overwritten). Build 2 (binaries kept in bin2/): compiled;
  new test `parity_rows_match_sorted_rows_and_tally` failed at case 3, tally (old 1387, new 1246).
  Cause (test bug, not production): case 3 uses a partial multiplier mask (`full & !1`), so the
  rows come from `cached_monomials_up_to_mask`, whose per-thread cache charges the schedule fill
  (64 + len) only on first use: gap 0 -> 65, gap 1 (12 monos over 11 vars incl. 1) -> 76;
  65 + 76 = 141 = 1387 - 1246. The old arm ran first and paid it. Fix (fmt/k.rs): one warm-up
  `run(false)` before the compared runs. The production code in fmt/k.rs is unchanged from build 2.
- Build 3 (after the container restart): `TEST_FILTER="koblitz_groebner inherited_f4"
  locked_build.sh $L "koblitz_f4_kernel_bench groebner_stage_bench crossbred_bench f4_fp_bench"
  src/cryptanalysis/koblitz_groebner.rs` -> rc 0, patch 504 lines (patch.diff == patch_build3.diff).
  The two-word filter made cargo run 0 tests, so I copied the test binary this build produced
  (target/release/deps/crypto_lib-24e7e65220b9e040, mtime 21:20:51 = my lock window, lists 6653
  tests incl. the new one) to bin/crypto_lib_test (sha256 4be3ca40...b292) and ran
  `bin/crypto_lib_test koblitz_groebner inherited_f4 --test-threads=2` (cwd /home/user/crypto):
  **112 passed, 0 failed, 6 ignored** (test_full.log), incl. parity_rows_match_sorted_rows_and_tally
  (60 random systems x {None,F5} x {reuse off,on}: nested and flat matrices, columns, rows_pruned,
  criterion_word_ops, layout hit/miss stats and bitop/v1 word_ops + hash_probes all equal).

## Results (build-3 binaries; res3/, cg3/)  [pcc:unverified]

Output identity (B = baseline/bin, N = bin, O = KIC_M4RI_OLD=1 bin):
- koblitz_f4_kernel_bench 5 {x,sym}: B = N = O on rows/cols/rank/syzygies/rref_kernel/word_ops
  (x: 1023x4369 rank 1022 word_ops 4,335,210; sym: 992x3872 rank 992 word_ops 3,628,128).
- groebner_stage_bench --out DIR (full frozen ladder): N vs O stage.json differ only in
  build_ns/reduce_ns/wall_ns and label; decomposed counts, word XORs (266054, 594722, 10813009,
  68049, 4936167, 28839793), F4 calls identical. (No baseline stage binary exists; O is the
  unchanged old code path in the same binary.)
- crossbred_bench 9 11 13: B = N = O after stripping the three wall columns (md5 18e63087...);
  verdicts `yes`, F4 bit ops 17,826,085 / 3,880,141,008, crossbred 108,117 / 24,350,101.
- f4_fp_bench 3 large: all 13 cases basis_fp, solve_fp, field_ops, solve_field_ops, max_rows,
  max_cols, steps, basis_len, solutions equal to baseline (module untouched).
- No counter changed: the bitop/v1 tally is charged with the old formulas on purpose.

Deterministic cost (callgrind Ir, RAYON_NUM_THREADS=1, 1 repeat, same binary N vs O):
| quantity | old (O) | new (N) | ratio |
|---|---:|---:|---:|
| x: build_macaulay_with_multiplier_mask | 37,946,157 | 21,232,596 | 1.79x |
| x: macaulay_profile (build + rref) | 71,106,632 | 54,390,687 | 1.31x |
| x: rref_f2_counted | 32,923,892 | 32,921,834 | 1.00 |
| sym: macaulay_profile | 59,984,560 | 46,893,654 | 1.28x |
| stage --rung 5 (K_1/2^23) program total | 2,171,029,245 | 2,167,381,102 | -0.17% |
| stage --rung 2 (K_0/2^13) program total | 547,600,028 | 544,836,987 | -0.50% |
Inside the new x build, parity_rows_with_mask is 16.5M Ir (the per-term colex rank loop and the
parity toggles), the rest is the column sort and packing.

Prediction check: identity predictions all held. Cost prediction missed on size: build predicted
38.0M -> 8-12M Ir, measured 21.2M; profile predicted ~1.6x, measured 1.31x (x) / 1.28x (sym).
Production-stage prediction ("small single-digit % or less") held: -0.2% to -0.5% of total Ir,
because production builds mostly hit the cached column layout and those take the unchanged path.

Wall (indicative only; the VM was running other leads' builds): kernel x median 18.4 ms (B),
4.41 ms (N), 9.57 ms (O); sym 8.77 (B), 3.62 (N), 7.99 (O). Not a claim; serial A/A stage decides.

## Commands (this session)
- valgrind --tool=callgrind --callgrind-out-file=cg3/{x,sym}_{new,old}.out bin/koblitz_f4_kernel_bench 1 {x,sym}
  (old arm with KIC_M4RI_OLD=1), same for groebner_stage_bench --rung {2,5} --out DIR;
  callgrind_annotate --inclusive=yes.
- Output-identity runs as listed above, outputs in res3/.

## Risks
- Gain is confined to cold (layout-miss) Macaulay builds: macaulay_profile, MatrixF4 nodes,
  InheritedF4 roots on misses. Production stage Ir moves by < 1%; this is a kernel-bench win,
  not a production-ladder win.
- Thread-local scratch grows to the largest slot space seen (<= 2^20 slots: 128 KiB per bitset,
  4 MiB slot_col) and is never shrunk.
- Fallback to the old builder for degree > 6, > 2^20 slots, support outside n_vars, or a cached
  layout present; a panic mid-build could leave the scratch's parity/seen bits dirty (no
  unwind guard), which would corrupt a later build on that thread.
- Layout-cache side effects mirrored by hand (miss counter, insertion, eviction rule); a future
  change to build_macaulay_packed's cache logic must be mirrored in build_macaulay_packed_parity.
- Merge: one dispatch point in build_macaulay_packed plus a `ParityPack` bound on its P; other
  leads touching build_macaulay_packed's signature will conflict there.
