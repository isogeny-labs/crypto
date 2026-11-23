# Perf round 3: koblitz_fast scalar bulk canonicalisation (WALL only)

Scope: `src/cryptanalysis/koblitz_fast.rs` only. Base commit `f0a2fd7694`
(branch `claude/performance-optimization-parallel-vgenii`). Worktree
`/home/user/wt/crypto-koblitz-fast`, branch `perf/crypto-koblitz-fast`.
The prior round-2 note `docs/perf/20261112-tet-crypto-ic-relations-r2.md`
was not present in the checkout; the profile in
`docs/perf/20261112-rust-deep-audit/profile-tables.md` was used instead.

Note: `batch_inv_clmul` (Gf2, `semaev_decomp.rs`) and `witnesses_fast_scan`
(`koblitz_index_calculus.rs`) are outside the scope list and were not touched.

## Commit 7fbccb51d2: scalar bulk canon

`FrobeniusCanon::canon_in_place` portable path (the path Callgrind runs, since
valgrind has no AVX-512, and the path of non-AVX-512 hosts):

1. `coords()` takes the low byte and shifts by a constant 8 (no `8*i` variable
   shift or enumerate index).
2. `least_rotations` builds Z_1..Z_16 first; when no lane has a 16-bit zero run
   (taken ~always for random words) it lifts only Z_8..Z_1 instead of all six
   levels. Falls back to the full lift otherwise. Same `run`, same output.
3. The scalar bulk loop is also compiled with `bmi1,bmi2,lzcnt` behind a
   runtime check (`KIC_CANON_BMI2=0` keeps the baseline build as a same-binary
   control): `shlx/shrx/lzcnt` instead of `cl` moves and `bsr/xor`.

Counters (`count_canonicalisations`), RNG, records unchanged.

### Callgrind Ir (scalar path), base vs candidate, 1 thread, `--instr`

| kernel | base Ir | cand Ir | change | fingerprint |
|:--|--:|--:|--:|:--|
| koblitz_collect_k0n41_w164_u4096 | 339,988,217 | 290,856,687 | -14.5% | 5db139c2b89ef0f5 = |
| koblitz_collect_k0n53_w477_u1024 | 246,268,640 | 210,899,246 | -14.4% | cbaf6ffbd5aff92b = |
| koblitz_descent_k0n53_m2_x4 | 420,896,988 | 368,692,492 | -12.4% | a0fb1ad57bf29cd2 = |
| koblitz_pair_table_k0n41_folded | 103,733,389 | 91,170,180 | -12.1% | 2778abd161ae396d = |

canon_in_place went from 203.2M to ~154M Ir (k0n41 collect). Binaries were [pcc:unverified]
measured as `perfbench.c2`; the final commit differs from it only in one
comment line (verified by diff of the source change).

### Native wall (AVX-512 path), local, unisolated, [pcc:unverified]

`perfindex compare --rounds 6 --taskset 2`, load average ~1.2, shared VM:
PI 1.008x (CI 0.919-1.127), every kernel "neutral" within A/A noise (+-4.5%
median). The AVX-512 canon path is untouched (only `coords`, used by its
remainder and by `canon`, changed), so neutral is the expected result.
`perfindex null` on the base binary: PI 1.006x, median A/A noise +-18%
(very noisy host).

### Native wall, scalar path forced with `KIC_SCAN_SIMD=0`, [pcc:unverified]

Medians 1.10x-1.31x, best-of 1.07x-1.16x, but A/A noise +-3% to +-25% and
CIs include 1.0 on three of four kernels: indicative, not established.
Standalone microbenchmark of the scalar bulk key (n=53, 4M words, 3 runs):
~28-31 ns vs ~21-31 ns per word; noisy, not claimed.

## Identity checks

- Callgrind fingerprints above equal for all four kernels (`perfindex`/perfbench
  fingerprint = output hash).
- `perfindex compare` (native AVX-512 path, and with `KIC_SCAN_SIMD=0`) ran
  without an output-mismatch refusal.
- Unit tests: `cargo test --release --lib cryptanalysis::koblitz_fast`: 40
  passed, 0 failed, 1 ignored (includes the new
  `the_scalar_bulk_keys_agree_with_and_without_bmi2`, plus the existing
  exhaustive `least_rotation_matches_exhaustive_scan` for every n in 2..=63
  with long-run lanes, which exercises both the shallow and the full lift).
  With `KIC_SCAN_SIMD=0` two tests fail by design
  (`avx512_kernel_matches_scalar_arithmetic`,
  `the_folding_kernel_matches_the_scalar_addition_on_slices` assert that the
  SIMD kernel was exercised); all others pass, also with `KIC_CANON_BMI2=0`.

## Reverted / not pursued

- Replacing the last `std::array::from_fn` stage of `least_rotations` by a
  loop: 0.1% Ir change (the outlined `try_from_fn` in the profile is the
  `coords` stage); kept as a plain loop, no measured effect.
- `add_many_lazy_inline` double `push` per element (~71 Ir/element): estimated
  <=2% of kernel Ir; not attempted.
- AVX-512 canon (`least_rotation16_funnel_avx512`): one `vpshldq` plus one
  `vpminuq` per rotation per register, already at the port limit; no change.
  AVX-512 add_many kernels were not modified (no PMU on this host to guide).

## Deferred method changes (would alter counters/methods; not done)

- A different canonical-key function (e.g. fewer rotations via orbit
  structure) would change keys or counted canonicalisations.
- Fusing canonicalisation with `add_many_lazy` batches changes the counted
  chunking (16 vs 8 whole-chunk accounting in `count_canonicalisations`).
