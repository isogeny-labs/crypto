# Tet offensive, area crypto-f4: F4 over F_p and towers, WALL-only

Branch `perf/crypto-f4`, cut from `claude/performance-optimization-parallel-vgenii`
at `aaf15e6bdc`. Scope: `src/cryptanalysis/f4_fp.rs`,
`src/cryptanalysis/f4_fp_tower.rs` (nothing else edited). Rules: AGENTS.md
section 10; every value, counted unit (`FIELD_OPS`, `muladds`, `native_ops`,
`norm_ops`), RNG draw, record and output is unchanged. Not pushed.

All numbers are **[pcc:unverified]**. Instruction counts are deterministic
callgrind `Ir` (1 thread, `perfbench run --instr`, valgrind reports AVX2 only,
so the AVX2 paths are what is counted; real wall runs on this Sapphire Rapids
VM take the AVX-512 paths). Wall figures are **local, unisolated,
[pcc:unverified]**: shared 4-vCPU VM, `--cpus none`, load average 0.2 to 0.3
at the A/B (1.6 over the previous 5 minutes, another agent building earlier).

## Method

1. Read `F4F5_ROWBUILD_AND_ISO_FIELD_20260910.md` (row-build sort and Field::sqr
   attempts were neutral or negative: not repeated) and the 20261112 audit.
2. Baseline callgrind of every `fp_gb` Quick kernel, then per-function profile
   of the largest in scope: `tower_f4_kummer_m3_n9` (36.7 percent in
   `lazy_axpy_avx2`, 3.3 + 1.5 + 0.9 percent in the `(u32,u32,u32)` entry
   sort), `f4_pkm_kummer_m2_t5` (35 percent `rref_ckpt`, 30 percent the
   non-deferred pair-row reducer, p = 786433 > 2^16), `f4_autoreduce_pkm`
   (85 percent `reduce_skipping`, i.e. `first_divisor`).
3. Four candidate changes, one build each batch (a release build of the crate
   takes 11 to 19 minutes here), per-kernel Ir against the base binary.

Binaries (`cargo build --release --example perfbench -j2`, sha256):
`base` (aaf15e6bdc) `eadcd7bc...c112`; `cand1` (first three changes plus the
reverted counter batching) `bfb0439c...6039`; `cand2` (final tree minus
comment-only edits) `0ae79ede...e7`.

## Commits and per-kernel Ir (base -> final, fingerprints identical)

Full table: [`20261112-tet-crypto-f4/ir-base-vs-final.txt`](20261112-tet-crypto-f4/ir-base-vs-final.txt).

| kernel | base Ir | final Ir | change |
|:--|--:|--:|--:|
| tower_f4_kummer_m3_n9 | 902,566,331 | 785,539,177 | -12.97 % |
| tower_f4_kummer_m2_n12 | 497,077,420 | 449,187,719 | -9.63 % |
| tower_f4_kummer_m2_n12_planted | 581,308,254 | 533,096,272 | -8.29 % |
| tower_f4_kummer_m4_n8 | 361,993,859 | 326,809,565 | -9.72 % |
| f4_pkm_kummer_m2_t5 | 252,020,207 | 222,166,114 | -11.85 % |
| f4_pkm_kummer_m3_t2 | 29,658,367 | 28,948,975 | -2.39 % |
| f4_autoreduce_pkm_m2_t4_x2 | 158,949,279 | 148,488,126 | -6.58 % |
| other fp_gb kernels (p <= 2^16 F4, sig_tower, gaudry, buchberger) | | | within +-0.9 % (layout noise; sig/gaudry/buchberger code is untouched) |

1. `3ab8291287` f4_fp_tower: dense `Kernel` gets an *exact* mode. The dense
   elimination's accumulators take at most `2 * n_cols` products between two
   reductions and are only read through `Fp::reduce`, so when
   `(2 n_cols + 4)(p-1)^2 + p < 2^64` (p = 786433 here) the conditional
   subtraction of a multiple of p after every product is dropped (as `Acc`
   already does for the sparse echelon). Plus: the S-row `(column,row,coef)`
   entries sort by the unique key `(column,row)`. Tower kernels -8.3 to
   -13.0 percent Ir (`muladds` is part of the kernel fingerprint: equal).
2. `31a5f761e7` f4_fp: the non-deferred pair-row reducer (p > 2^16) adds
   products unreduced when `(2^64-p)/(p-1)^2 >= 64` (1.6e7 for p = 786433),
   reducing at the reducer-column read and at read-out. With
   `fd55624eed` below: `f4_pkm_kummer_m2_t5` 252.0 M -> 230.0 M (this
   commit alone, cand1 binary) -> 222.2 M (final).
3. `3afdf60d9b` f4_fp: `first_divisor` tests divisibility on the two 64-bit
   halves (no 128-bit borrow chain; every lead field is at most 127, or the
   128 of `SET_NEVER`). autoreduce -6.6 percent Ir. Randomized test against
   `packed_divides`.
4. `fd55624eed` f4_fp: `rref_ckpt` eliminates by a pivot's non-zero list when
   at most a quarter of its tail (32 or more entries) is non-zero; same
   entries in the same order, same row-operation count. pkm_m2_t5 a further
   -3.4 percent Ir. Randomized test against the scan.

## Reverted

- `8cc802670e` / revert `5eeb0d0570`: batching `count_ops` once per reduced
  row in `reduce_lazy_by` and the Faugere-Lachartre loops. Ir got worse:
  `f4_quad_n7_p65521` 88.51 M -> 90.11 M (+1.81 percent), katsura6 +0.99
  percent, cyclic5 +0.69 percent. Not kept.

## Identity checks

- perfbench fingerprints (basis, steps, rows, cols, `muladds`, trace, ...):
  equal for all 19 `fp_gb` kernels, base vs each candidate binary.
- Counter identity for `f4_fp` (not in the perfbench fingerprint): a temporary
  driver ([`f4_ident_driver.rs.txt`](20261112-tet-crypto-f4/f4_ident_driver.rs.txt),
  not committed as an example) ran 10 dense systems over p = 31, 65521,
  65537, 786433, 1,000,003, 268,435,399 and 4,294,967,291 and printed basis
  hash, steps, rows, cols, basis size, `F4Report.field_ops`, and the deltas of
  `field_ops_total`, `native_ops_total`, `norm_ops_total`. Base and candidate
  outputs are byte-identical (`diff` empty;
  [`ident-base.txt`](20261112-tet-crypto-f4/ident-base.txt),
  [`ident-cand.txt`](20261112-tet-crypto-f4/ident-cand.txt)). The p = 268M
  and 1,000,003 cases take the unreduced pair-row path, 4,294,967,291 the
  reduced fallback.

## Tests

`cargo test --release --lib` filters on the final tree: `f4_fp_tower` 29
passed, `f4_fp::` 44 passed (including the new
`first_divisor_agrees_with_packed_divides` and
`sparse_pivot_elimination_matches_the_dense_scan`), `sig_fp_tower` 8,
`tower` 98, `gaudry` 58, `inherited_f4` 16 (2 ignored), `prime_f4_wide` 93.
The `the_kernels_agree_with_exact_arithmetic` test now also checks the exact
kernel against 128-bit arithmetic. Not completed: the `pkm` and `f4_` name
filters (over the 110 s per-command limit of this session; not run to the end).

## Wall (secondary, local, unisolated, [pcc:unverified], RAYON_NUM_THREADS=1)

`perfbench run --samples 5`, interleaved base/final, 3 rounds, median ns:

| kernel | base | final |
|:--|:--|:--|
| tower_f4_kummer_m3_n9 | 115.4, 109.3, 122.3 ms | 109.8, 100.5, 106.0 ms |
| f4_pkm_kummer_m2_t5 | 32.0, 29.6, 33.4 ms | 26.0, 25.0, 28.5 ms |
| f4_autoreduce_pkm_m2_t4_x2 | 14.1, 14.9, 15.0 ms | 13.9, 12.5, 13.0 ms |

Final is faster in every round and position. No A/A control was run, so the
spread of the base column (up to 12 percent) is the noise bound; wall claims
are indicative only.

## Deferred method changes (would change a counter or method; not done)

- Pivot rows are normalised and every row operation counted whether or not
  the entry is zero in `rref32`; skipping zero entries there would change
  `field_ops`.
- A cheaper `eliminate` for p > 2^16 with lazy 64-bit rows would need the
  `found` column test to reduce first; it keeps outputs but changes the
  checkpointed row representation (`rref_ckpt` snapshots store reduced rows).

## Not attempted / ideas for the next round

- `mul_terms_fp` allocates four small vectors per call (callgrind: mul_elem
  10.5 percent, scan_products 8.7 percent of tower m3_n9); a per-thread
  scratch would help but touches the cached-product layout.
- `State::insert` (4.7 percent) + pair `retain` (2.2 percent) in the tower.
- `symbolic` `Interned::id` and malloc/free are 20 to 30 percent of the
  small-p kernels (quad_n7: 9.8 + 8 + 3.6 percent).

Disk: the build target and test binaries were deleted; the host had 1.7 GB
free at the lowest point because other agents were also building.
