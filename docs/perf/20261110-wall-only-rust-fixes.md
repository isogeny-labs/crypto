# Wall-only Rust fixes from the 2026-11-10 assessment (R1, R2, R3, R5, R8, R10, R11, R14)

Implements the WALL-class items of
[`docs/assessment-speed-productionization-20261110.md`](../assessment-speed-productionization-20261110.md) §3a
under AGENTS.md §10: every change must leave values, counted units, RNG
consumption and outputs identical; only wall time may differ. R6-B and R7
(METHOD/COUNTER) are out of scope and untouched; R4, R6-A, R9, R12 and R13 were
not part of this task.

All numbers below are from one local 4-vCPU VM, unisolated (`--cpus none`, no
`tools/isolated_bench.py`), so every wall figure is **local, unisolated,
[pcc:unverified]**. Instruction counts (Callgrind Ir) are deterministic but
were not filed as pcc claims either: **[pcc:unverified]**. Nothing here is a
speed claim. Identity is the result; times are indicative only.

## Baseline and host

- Baseline: commit `4e909cd242`, built with `cargo build --release --bin
  ecbench --bin ic --bin volcano-rho --bin isoclass-rho` from a clean tree
  (the first build was redone after a source edit landed during it, so the
  kept binaries are from the pristine tree). Examples `f4_fp_bench` (with
  the `field_ops` print of commit `9d9f556145` added to the example only) and
  `la_engine_bench` were built against the same baseline library.
- Final: commit `6ab79a24ae`. Hashes of both sets: [`binaries.sha256`](20261110-wall-only-rust-fixes/binaries.sha256).
- Host ([`host-baseline.txt`](20261110-wall-only-rust-fixes/host-baseline.txt)):
  Intel Xeon @ 2.30 GHz, 4 vCPU (1 thread/core), popcnt, bmi2, adx, avx2,
  avx512f, avx512ifma, pclmulqdq, vpclmulqdq; 15 GB RAM; Linux 6.18.44 x86_64;
  rustc 1.97.0 (2d8144b78 2026-07-07). No PMU, so no `perf`; Callgrind
  (valgrind) gives the instruction counts.
- Timing: [`abab.sh`](20261110-wall-only-rust-fixes/abab.sh), baseline (A) and
  candidate (B) alternating, 5 rounds, `RAYON_NUM_THREADS=1`, whole-process
  wall time. A/A noise floor (baseline vs baseline, volcano-rho run below):
  A 512/530 ms, B 501/516 ms (min/median), so differences under about 5% are
  noise here.

## End-to-end identity gate (ecbench)

Specs: [`gate-prime.json`](20261110-wall-only-rust-fixes/gate-prime.json)
(rho.negation, rho.plain, bsgs.textbook, bsgs.negation, kangaroo.vow,
ic.pipeline mitm with the default incremental-gauss and with
structured-gauss, rootfind-mitm:m=3 + structured-gauss, rootfind-pairs:m=3;
prime_search 20 and 24 bits, 2 targets each, 36 records),
[`gate-koblitz.json`](20261110-wall-only-rust-fixes/gate-koblitz.json)
(rho.signed_frobenius_strong, rootfind-mitm, rootfind-pairs +
structured-gauss, descent-algebraic + f4-f2; Koblitz a=0 n=13, a=1 n=17;
16 records) and
[`gate-tensor.json`](20261110-wall-only-rust-fixes/gate-tensor.json)
(ic.tensor_contraction_v1 m=2, prime_search 12 and 14 bits, 4 records; it
was split out because at 20 and 24 bits the arm hit the 300 s watchdog,
whose outcome depends on wall time).

```bash
ecbench run --spec gate-X.json --out DIR --cpus none --allow-busy --quiet   # baseline and final binary
norm-records.sh DIR/records.jsonl | sha256sum                                 # both sides
ecbench verify --dir FINAL_DIR --replay-all --out FINAL_DIR.audit.json        # final binary
```

[`norm-records.sh`](20261110-wall-only-rust-fixes/norm-records.sh) drops only
`binary_sha256`, `isolation`, `record_id`, `session_id`, `time`,
`online.phases_ns`, and keys ending `_ns`/`_ms` or starting `ns_per_`
(the descent solver block carries `build_ns`, `eliminate_ns`, `f4_wall_ns`).
Everything else - `outcome.recovered`, `cost.s`, `total_gae`, phase
`adds`/`doubles`/`gae`/`native`, `counters`, `field_ops`, `work`
(`gauss_entries_scanned`, `pivot_inversion_steps`, `bitop_*`), solver
extras - is compared.

| session | records | normalised SHA-256 (baseline = final) | `verify --replay-all` (final) |
|---|---|---|---|
| gate-prime | 36 | `c082ff06f75ec62a…` | OK, 36/36 replays reproduced |
| gate-koblitz | 16 | `70bd285eef524ee8…` | OK, 16/16 |
| gate-tensor | 4 | `3360e5c9d7a042ee…` | OK, 4/4 |

The same comparison passed for the intermediate binaries of R5, R11 and R14.

## Per item

| # | files | identity evidence | Callgrind Ir before → after | wall before → after (min/median, ABAB ×5) |
|---|---|---|---|---|
| R1 | `src/bin/volcano_rho.rs`, `src/bin/isoclass_rho/{field,rho,main,search}.rs`, `src/cryptanalysis/prime_fast.rs` (`MulModN`) | volcano-rho `--bits 30 --depth 2 --seed 7 --trials 20 --per-level 3 --controls 1`, negation on and off, and `--bits 44 --depth 1 --seed 3 --trials 4`: `trials.csv` (cols 1-6), `instances.jsonl`, `results.json` without timing keys identical. isoclass-rho `selftest`, `search --bits 30`, `walk` (p = 546252379, ells 3,5), `measure --mode class` and `--folds`: identical. Unit test `mulmod_n_matches_the_128_bit_remainder` | volcano 6,559,563,410 → 1,696,036,083 (was `Curve::add` 43%, `__divti3` 27%; now `rho_native` 31%). isoclass measure 1,927,609,105 → 469,673,760 (was `inv_mod` 39%, `__divti3` 27%; now `ec_add` 29%) | volcano 501/506 → 320/324 ms; isoclass 175/187 → 105/109 ms |
| R2 | `src/cryptanalysis/ic_framework/linalg.rs` (StructuredGauss) | `la_engine_bench ours --arm sgauss\|igauss` on four systems (2048×3; 4096×2 seed 5; 1024×4 wide 62-bit modulus; 512×3 positive mod 1000003): JSON identical minus `generate_s`/`solve_s` (sgauss `work` 14181163, 81676, 10896086, 256876). `entries_scanned` and `pivot_inversion_steps` through the ecbench gate | sgauss 1024×4 wide: 1,617,886,213 → 1,336,736,357 (was `sparse_sub_scaled` 51%, `__umodti3` 19%, malloc 3.5%; now `sparse_sub_scaled_into` 75%) | 2048×3: 211/214 → 161/176 ms; 1024×4 wide: 133/141 → 98/103 ms; ecbench gate-prime session 5560/5651 → 5111/5172 ms (all arms, R1-R14 together) |
| R10 | same file (Wiedemann `apply`) | buffer reuse only; same loop order. Reached only via `ic_wide` when r ≥ 2^64; `ic_wide` tests pass (2) | not measured | not measured |
| R3 | `src/cryptanalysis/f4_fp.rs`, `examples/f4_fp_bench.rs` | `f4_fp_bench 1 large` (13 cases: fingerprints, shapes, `field_ops`, `solve_field_ops`) identical under `RAYON_NUM_THREADS=4`, `=1`, and `=1` + `F4_FP_THREAD_OPS=1`. `f4_fp::` tests 42/42 | 3,573,932,454 → 3,573,948,429 (unchanged single-threaded) | 364/420 → 387/419 ms (no difference) |
| R5 | `src/cryptanalysis/ic_framework/rootfind.rs` | ecbench gate-prime and gate-koblitz identical with an R5-only binary; `ic_framework::rootfind` tests 2/2 | `ecbench audit-ir` [`perf-r5.json`](20261110-wall-only-rust-fixes/perf-r5.json) (Koblitz n=19): rootfind-mitm 29,348,763 → 22,638,916; rootfind-pairs 464,468,154 → 292,442,731 (bitops, GAE identical) | whole session 490/498 → 450/472 ms |
| R11 | `crates/ecdl-tensor-ic/src/field.rs` | `opcount.rs` read first: it prices no inversion step, so R11 applies. gate-tensor identical; unit test `inv_mod_equals_the_fermat_power` (13 moduli incl. composites) | audit-ir [`perf-r11.json`](20261110-wall-only-rust-fixes/perf-r11.json) (12-bit, 1 target): 55,556,070,039 → 54,301,507,979 (−2.3%) | 4053/4289 → 3911/4025 ms (R11-only binary; within spread) |
| R14 | `src/cryptanalysis/ic_boundary.rs` (`bump_counter`, `PhaseCost::count/work`), `ic_framework/plugins.rs`, `pq_descent_symbolic.rs` | all three gates identical with an R14 binary | audit-ir [`perf-r14.json`](20261110-wall-only-rust-fixes/perf-r14.json), previous commit → R14: descent f4-f2 273,480,362 → 273,323,514; rootfind-mitm 1,702,975 → 1,709,109 (+0.4%) | final vs baseline 450/470 → 473/493 ms; R11 → R14 binary 448/477 → 449/481 ms (no difference) |
| R8 | `Cargo.toml` (`[profile.lab]`) | `cargo metadata` parses it without warning; `[profile.release]` untouched; no `panic = "abort"` | not built | not built |

Library tests run on the final tree (`cargo test --release --lib --no-run`,
then the harness filtered): `ic_framework::linalg` 7, `ic_framework::rootfind`
2, `ic_framework::tests` 23, `prime_fast` 11 (+1 ignored), `f4_fp::` 42,
`ic_wide` 2, `pq_descent_symbolic` 4, `ic_boundary::tests` 60 (+4 ignored),
`koblitz_groebner::` 91 (+5 ignored, includes `f4_rref_tests`): all pass.
`ecdl-tensor-ic --lib`: 3 pass.

## Tried and rejected

- **Binary extended-GCD inverse** (R1, first attempt): fewer instructions
  (volcano 3.83e9 Ir) but slower than the baseline in single wall samples
  (0.63 s vs 0.54 s): an unpredictable branch per bit. Replaced by
  `ic_boundary::invmod` (word-division Euclid, 32-bit asm below 2^32).
- **Branchy compare-and-subtract** (R2, first attempt): `submod` and the final
  subtraction of `MulModN::redc` as `if` branches made StructuredGauss
  slower than baseline (2048×3: 200/208 → 259/269 ms) despite fewer Ir. Both
  became `std::hint::select_unpredictable`, as `prime_fast::Fp64` already does.
- A micro-benchmark on this host put a dependent `u128 %` chain at about
  5.8 ns a step against 8.1 ns for the two-reduction Montgomery canonical
  product (independent products: 2.7 vs 2.3 ns). `MulModN` wins on
  throughput, not latency; the large R1 gain comes mostly from the inverses
  (`__divti3` Euclid gone).
- **R10 transpose-CSR gather and rayon**: not done. Modular sums are
  order-independent, but the struct is reachable only for r ≥ 2^64 and no
  workload of this gate exercises it, so a reordering could not be pinned.
- **R8 lab build**: not built; a single-codegen-unit LTO build needs a second
  target directory and the host had about 3-5 GB free.

## Notes for whoever lands this

- volcano-rho rows report wall `ns`: rows from before and after R1 must not
  be pooled on time.
- Host records' `binary_sha256` changes with these commits (and with the lab
  profile, if used); counted units do not.
