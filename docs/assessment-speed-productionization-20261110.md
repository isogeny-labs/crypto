# Speed and productionization assessment (2026-11-10)

Scope: the three lab repositories as checked out on 2026-11-10 —
`crypto` (`f14b66786`), `cryptanalysis` (`e1fca154`), `ml-cryptanalysis`
(`0d225b6`). Read-only review plus local sanity measurements; no code was
changed for speed. Every number below is a **local sanity measurement**
(`[pcc:unverified]`), not a lab result: host is a 4-vCPU cloud VM (Intel Xeon
@ 2.30 GHz, Sapphire-Rapids-class flags incl. avx512, vpclmulqdq, gfni, adx;
15 GB RAM; no PMU, no GPU; rustc 1.97.0, gcc 13.3.0, cmake 3.28.3,
Python 3.13.18). Nothing here was isolated with `tools/isolated_bench.py`, so
wall times are indicative only. Per AGENTS.md §10, each speed item must be
re-measured from a recorded baseline (A/A, then ABAB) before any claim.

Method: five parallel reviews (two hot-path reviews at the Opus tier, three
survey reviews at the Sonnet tier) consolidated by the main session; the
highest-ranked code findings were spot-checked against the source by the main
session (marked "verified" below). Findings not spot-checked are the
reviewer's reading of the code and are marked "reported".

## 1. Executive summary

The compute kernels that matter most (`prime_fast`, the pclmul binary-field
code, `pq_f4_f2`, `f4_fp`, `PairTable`, the C `mont_mul`) are already tuned.
The remaining speed is in four places, in this order of leverage:

1. **Build configuration.** The `crypto` root crate has no `[profile.release]`,
   no `[workspace]`, no `.cargo/config.toml`, no `rust-toolchain.toml`: every
   lab binary ships at cargo defaults (codegen-units 16, no LTO, unwinding
   panics, unstripped, baseline x86-64). The C library has no LTO, no
   `-march` level in any deploy/CI/taskq build, and compiles every source
   twice. Expected 5–20 % wall on the kernels, 40 % less C compile CPU,
   and smaller binaries. Low effort, wall-clock only.
2. **Software 128-bit division on hot paths.** `u128 %` (`__umodti3`) and
   i128 extended-Euclid inversion in the stand-alone rho binaries
   (`volcano_rho`, `isoclass_rho`), in `ic_framework/linalg.rs`
   (`StructuredGauss`, Wiedemann), in `ecdl-tensor-ic`, and `ca_mulmod` /
   `ca_invmod` in the C GLV walk. The repo already has faster, value-identical
   primitives (`ic_boundary::{mulmod,invmod}`, `prime_fast::Fp64`,
   `shoup_mul`). 2–10 × per step on those paths, wall-clock only.
3. **Allocation and contention in inner loops.** Row clones per elimination
   step, per-block `Vec`s in the rootfind oracles, a global atomic
   `fetch_add` per row per pivot in `f4_fp`, a `Mutex<HashMap>` (SipHash) DP
   store in `rho_parallel`. 1.2–5 × on the affected phases, wall-clock only.
4. **Parallelism left on the table.** C Lanczos SpMV is single-threaded (69–75 %
   of IC precompute/linsolve); Wiedemann matvec in Rust is serial and
   reallocates; the CUDA kernel does Fermat inversion per 8 walks and 64-bit
   `%` per step. 2–3.5 × on LA with 4 cores; 1.5–2 × on the GPU step.

Productionization is dominated by three structural problems: ~1,090 Cargo
link targets and 333 lockfiles in `crypto`, 789 GitHub workflows across the
two repos (≈600 of them per-experiment one-shots, 85 of the `cryptanalysis`
ones unfiltered on every PR), and an 11 GB / 286k-file tree (`crypto`) that
every clone, CI job and hook walks. Also: the nightly/fuzz schedules in
`cryptanalysis` never fire; hand-mirrored enforcement tools
(`benchrec.py`, `progress_metrics.py`) have drifted between repos; and the
`benchrec` gate walks the whole 286k-file tree on every `git push` (a
measured fix is attached as a patch for the owner repo, see T1). The two cheapest build-only fixes (C5, C6 below) also land with
this assessment, in `cryptanalysis`.

## 2. Baselines recorded in this session

| What | Command | Result |
|---|---|---|
| `crypto` release build, default profile, 4 cores, cold target | `cargo build --release --bin ecbench --bin ic` | **9 m 38 s wall, 31 m 51 s user**; `ecbench` 67 MB, `ic` 59 MB (unstripped) |
| `cryptanalysis` configure + build, Release, 4 cores | `cmake -S . -B … -DCMAKE_BUILD_TYPE=Release && cmake --build … -j4` | 0.69 s + **24.5 s wall / 39.3 s user**; `ec_tau.c` alone 7.4 s (includes 294k generated lines); every TU compiled twice (shared + static) |
| `cryptanalysis` tests | `ctest -j4` | 19/19 pass, 7.8 s |
| `ca_perfbench` quick kernels (medians) | `ca_perfbench run --samples 3 --min-samples 3 --max-seconds 5` | mont_mul_x8 13.2 ms (0.79 ns/mul); invmod 29.8 ms (≈182 ns ≈ 420 cycles/inverse); rho_ec36 30.4 ms; glv_rho_j0_32 16.3 ms; ic_precompute_zp44 16.8 ms; linsolve_sparse 10.4 ms |
| `ca_bench generic --bits 36 --threads 4` | | EC rho-1 18 Mops/s (S=0.833), kangaroo 47 Mops/s, Zp rho-1 93 Mops/s |
| Callgrind (Ir share) | `valgrind --tool=callgrind ca_perfbench …` | glv_rho_j0_32: ec_hash 19.8 %, ec_batch_op 16.7 %, ca_ec_endo 14.6 %, glv_class_reduce 11.5 %, ca_invmod 11.0 %; rho_ec36: ca_invmod 8.0 %; ic_precompute_zp44: spmv64 68.9 %; linsolve_sparse: spmv128 75 % |
| `benchrec.py gate` | 3 runs | 0.77 / 0.82 / 0.83 s wall (a first cProfile run read 1.5 s, but that was profiler overhead); `artifact_manifests()` `rglob`s the 286k-file tree and parses 3,184 manifests. Pruned `os.walk` + one resolve per manifest → 0.58 / 0.55 / 0.64 s, identical output and exit code, same 3,184 manifests; a `git ls-files --others` variant measured slower (0.80–0.91 s). Shipped as a patch for the owner repo (T1) |
| `build_ic_leaderboard.py` build | `L.build(allow_unvalidated=True)` | 2.35 s; `sha256_file` called 3,545 × over 41 evidence dirs (0.48 s); `validation_gate.check_evidence_dir` 1.07 s |
| Bash-tool hook overhead | checkpoint + benchrec + vault PreToolUse on a non-matching command | ≈170 ms per Bash call (95 + 62 + 13 ms), 8 `python3` spawns |

Raw logs and callgrind outputs: session scratchpad (`build-release-default.log`,
`build.log`, `ctest.log`, `perfbench.jsonl`, `cabench.txt`, `cg_*.out`); not
committed.

## 3. Speed opportunities, ranked

Replay key (this lab seals replay certificates and counted units):
**WALL** = identical values, counted ops, RNG consumption and outputs;
**COUNTER** = a sealed counter changes (needs a counter version or a note);
**METHOD** = walk order or results change (needs a new method id).

### 3a. `crypto` (Rust)

| # | Finding | Where | Gain (est.) | Effort | Replay | Status |
|---|---|---|---|---|---|---|
| R1 | `Fp::mul` is `(a as u128 * b as u128) % p` (`__umodti3`); `inv_mod` is an i128 extended Euclid (one `__divti3` per step, ~40 steps); coefficient update `% n` per step | `src/bin/volcano_rho.rs:71,127,705`; `src/bin/isoclass_rho/field.rs:120-133`, `rho.rs:203-299` | 3–10 × per step | S | WALL (iteration counts identical; volcano rows report wall `ns`, so old/new timings must not be pooled) | verified |
| R2 | `StructuredGauss` uses u128 `%` for add/sub/mul, i128 Euclid `invmod`, clones the pivot row and the row being reduced on every elimination | `src/cryptanalysis/ic_framework/linalg.rs:70-80,100,231,276` | 2–5 × on the structured-gauss LA phase | S–M | WALL for values; `entries_scanned` charges the clones, so keep the increments (becomes modelled work) | verified |
| R3 | `f4_fp::count_ops` does a global `FIELD_OPS.fetch_add` + TLS + jv_bitops call once per row per pivot inside `par_iter_mut` | `src/cryptanalysis/f4_fp.rs:345-353` | 1.1–2 × on parallel RREF | S | WALL (same total) | verified |
| R4 | `IncrementalGauss` dense kernel: one hardware `div` per entry via `mulmod_u32`, double indexing with bounds checks | `src/cryptanalysis/ic_boundary.rs:1134-1199` | 2–4 × on the O(\|F\|³) LA phase | M | WALL if `row_ops`/`entries_scanned` increments are kept | reported |
| R5 | Rootfind oracles allocate ~10–15 `Vec`s per 32-point block (`row_points`, `BatchScratch::default()` per `add_many`, `vec![k,j,l]`) | `src/cryptanalysis/ic_framework/rootfind.rs:41,127-160` | 1.2–1.5 × on rootfind-mitm/pairs | S | WALL | reported |
| R6 | Reference rho prime step: one Euclid inversion (~19 `div`s) + 3 `mulmod_u32` divs per step via `CountedFp` | `ic_boundary.rs:929,2481,4741` | A: swap primitives under unchanged tallies, 1.5–2.5 ×; B: lock-step lanes with batched inversion, 3–6 × | M | A: WALL; B: METHOD (new id, e.g. `rho.negation_lanes_v1`). Never touch `rho.frozen_reference` | reported |
| R7 | `PrimeCurve` has no batched `PairedAdd` (one inversion per add in the prime oracles) | `rootfind.rs:24` | 3–5 × on prime rootfind oracles | M | COUNTER (invs→muls; ship as a new oracle name like `mitm-prime-batch`) | reported |
| R8 | No release profile; `#[target_feature]` pclmul kernels cannot inline across non-batched calls at baseline x86-64 | `Cargo.toml` (no `[profile.*]`) ; `semaev_decomp.rs:311-325` | 5–20 % | S (+ longer builds) | WALL; `binary_sha256` in host records changes; do **not** set `panic="abort"` (31 `catch_unwind` sites) | verified |
| R9 | `pollard_collab` walk on `BigUint` with a cloned modulus per element | `src/cryptanalysis/pollard_collab/walk.rs:102-172`, `ecc/field.rs:13-17` | 10–30 × on P-256 isogeny jobs | M–L | WALL if bit-identical (peers share DP records) | reported |
| R10 | Wiedemann `apply` allocates `y` per matvec, serial, scatter adds | `ic_framework/linalg.rs:~560-640` | 2–8 × on large cores | M | WALL if the formula-based `ops` are kept | reported |
| R11 | `ecdl-tensor-ic` inversion by Fermat (`pow_mod(a, m-2)`, 60–120 u128 `%`) on every add | `crates/ecdl-tensor-ic/src/field.rs`, `curve.rs:86-88` | 5–20 × per add | S | WALL unless `opcount.rs` prices the pow steps | reported |
| R12 | `rho_parallel` DP store is `Mutex<HashMap>` with SipHash, locked per DP | `prime_fast.rs:775,985` | <5 % normally; more with many threads / small dp_bits | S | WALL | reported |
| R13 | BSGS reference counters are a `BTreeMap<String,u64>` lookup (string compares) 2–4 × per step | `ecbench/generic.rs:48,108-127` | ≤1.2 × | S | WALL | verified — **downgraded**: the code already avoids allocation (`get_mut` first); only the compare cost remains |
| R14 | `PhaseCost::count` and `DescentAlgebraicOracle::decompose` allocate `String` keys per call | `ic_boundary.rs:588`, `ic_framework/plugins.rs:~3936` | small | S | WALL | reported |

Not recommended: running ecbench arms in parallel (breaks interleaved
isolation); changing RNG stream consumption; folding `f.mul(2,y)`/`f.mul(3,x2)`
in `double_raw` (COUNTER).

Bench coverage gap: there is no criterion/`#[bench]` anywhere; `benches/` holds
only PQC. The counted reference walks, the Gauss kernels alone, `rootfind`
`row_points`/`add_many`, Wiedemann matvec, `volcano_rho`/`isoclass_rho` step
functions, `pollard_collab::walk` and `ecdl-tensor-ic` arithmetic have no
microbench. Existing example benches: `f4_fp_bench`, `f4_f2_bench`,
`gf2_elim_bench`, `f2m_field_bench`, `fp_backend_bench`,
`prime_ecdlp_fast_bench`, `rho_worker_batch_benchmark`, `bsgs_fast_bench`,
`rootfind_probe_hash_bench`, `semaev_decomp_bench`, `koblitz_decompose_bench`,
`la_engine_bench`, `blocked_macaulay_bench`.

### 3b. `cryptanalysis` (C / CUDA)

| # | Finding | Where | Gain (est.) | Effort | Replay | Status |
|---|---|---|---|---|---|---|
| C1 | j=0 GLV class reducer makes 5 out-of-line `ca_ec_endo` + 6 vtable hashes per step, then re-hashes the chosen point; the factored reducer (`CA_RHO_J0_FACTORED_HASH`, documented bitwise-identical) is off by default | `src/curve.c:269-298,941`; `experiments/prime-j0-cost-aware-chain/RHO_J0_FACTORED_HASH.md` | 25–35 % of GLV wall (≈46 % of Ir) | S | WALL | reported |
| C2 | `ca_mix64(h) % c.r` and `(idx+1) % c.r` with `c.r` a runtime power of two → two 64-bit `div`s per step | `src/curve.c:895,947,955` | 5–15 % of GLV step | S | WALL | reported |
| C3 | `ca_invmod`: Euclid with 64-bit `div` and 128-bit coefficients, ≈420 cycles; 8–11 % of rho/GLV Ir, dominates `ec_add_chain` | `src/modarith.c:58` | ~2 × on inversions; 4–10 % on rho/GLV | S/M | WALL for values; `CA_FC_EUCLID` diagnostic counts division steps → redefine or guard | measured (callgrind) |
| C4 | Walk starts do one affine add (one inversion) per set scalar bit | `src/rho.c:421-445` | 10–25 % at ≤40 bits; negligible ≥55 bits | M | WALL for values; field-op diagnostics change | reported |
| C5 | **Broken option**: the four `CA_BUILD_*_BENCH` blocks were pasted four times (byte-identical); `-DCA_BUILD_TAU_CHAIN_BENCH=ON` failed at configure ("target with the same name already exists") | `CMakeLists.txt:232-433` | correctness | S | build only | **fixed** (companion `cryptanalysis` PR): each option alone and all four together configure and build |
| C6 | Every source compiled twice (SHARED and STATIC from `${CA_SOURCES}`); `ec_tau.c` (7.4 s) is the serial critical path | `CMakeLists.txt:135-154`; `src/ec_tau.c:15-40` | OBJECT library: measured 36.9 → 23.0 s user (−38 %), 12.9 → 11.1 s wall, fresh configure, 4 cores; ctest 19/19 both; `nm -D` symbol list and all 31 `ca_perfbench` fingerprints identical; `find_package`/pkg-config consumers from a moved prefix pass. Next: move generated tables to their own TU / `#embed` | S / M | build only | **fixed** (companion `cryptanalysis` PR) |
| C7 | No ISA level in any deploy/CI/taskq build (`-march` absent; only `nightly.yml` sets `CA_NATIVE`); gcc GNU-mode `-ffp-contract=fast` + FMA `-march` can change the `double` maths that picks `r`, `dp_bits`, `W` | `deploy/docker/Dockerfile:37`, `.github/workflows/deploy.yml:65`, `rho.c:1076,1108-1113`, `curve.c:760` | 5–15 % (mulx/adx) | S | WALL **only with** `-ffp-contract=off`; `-march=native` on mixed Spot families risks SIGILL | reported |
| C8 | No LTO (`CheckIPOSupported` absent); cross-file hot calls never inline | `CMakeLists.txt` | 3–10 % | S | WALL (with fp-contract off) | reported |
| C9 | Lanczos SpMV single-threaded (69–75 % of IC precompute / linsolve) | `src/linalg.c:390,416,541-575` | 2–3.5 × on LA with 4 cores | M | WALL (exact mod-q sums, order-independent) | measured (share) |
| C10 | CUDA: `(h >> 32) % g->r` 64-bit modulo per walk per step | `cuda/ca_device.cuh:221` | 10–25 % kernel | S | WALL | reported (no GPU here) |
| C11 | CUDA: Fermat inversion shared by only `CA_GPU_W=8` walks (~2/3 of all multiplications); ~200 registers/thread | `ca_device.cuh:113,418` | 1.5–2 × | L | WALL if walk ids / seed derivation kept | reported |
| C12 | CUDA: AoS walk state (256 B stride), per-step global gather of the multiplier table (≤32 KB), synchronous launches | `ca_device.cuh:313-381`, `cuda/gpu_cuda.c:43-60` | 10–30 % | M | WALL if per-launch DP order kept | reported |
| C13 | CUDA: 64-bit Montgomery product via `__umul64hi` with branchy carries | `ca_device.cuh:86-99` | 1.3–2 × on field mul | M | WALL | reported |
| C14 | Host `rho_walk` is ~280 B (256 walks ≈ 72 KB > L1D); `hist[4]` shifted per committed step | `src/rho.c:129-148,869-874` | 5–15 % | M | WALL | reported |
| C15 | GLV coefficient updates use `ca_mulmod` (`__umodti3`) twice on 5/6 of steps; `shoup_mul` exists in `linalg.c:93` | `src/curve.c:983-986` | 3–6 % | S | WALL | reported |

Lower priority: single global mutex per DP insert with rehash under lock
(`rho.c:926`, `util.c:110`); IC sieve per-(row,prime) `ca_invmod` + 6 divisions
and a while-loop `bitlen()` (1.6 % at 44 bits); GLV batch path through the
vtable; `crates/hashed-ecdl-tensor` has no release profile.

### 3c. `ml-cryptanalysis` (Python)

| # | Finding | Where | Gain | Effort | Status |
|---|---|---|---|---|---|
| M1 | Rho labelling walks in pure-Python bigints, store every point (no DPs) | `csd/rho.py:95-210` | 3–10 × on label generation via one more ctypes kernel (op-counts must stay bit-identical) | M | reported |
| M2 | Most `scripts/run_*` / `ic_*` drivers serial; `csd/ic_dreg.py` pure-Python GF(2^n) loops | `csd/ic_dreg.py:75-160,424-460` | ≈core-count × on sweeps | M | reported |
| M3 | Per-line `json.loads`, shards re-read per `load(table)` | `csd/dataset.py:154`, `csd/bulk/load.py` | seconds/run (cache `.npy` by source sha256) | S–M | reported |
| M4 | Per-feature Python loops in the GBM node split | `csd/solving_degree.py:171,215,269` | modest (small data) | M | reported |

### 3d. Python tooling on the agent-critical path (`crypto`)

| # | Finding | Where | Gain | Effort |
|---|---|---|---|---|
| T1 | `artifact_manifests()` `rglob`s the 286k-file tree on every `benchrec gate` (every push / PR create) | `tools/benchrec.py:590-612` | measured 0.80 s → 0.58 s with a pruned walk (`git ls-files --others` measured slower, kept opt-in); remaining ~0.33 s is the directory walk itself. **Not landed here**: `tools/benchrec.py` is a lock-pinned mirror owned by `crypto-autoresearcher` (`docs/shared-code.lock.json`; the `parity` PR check refused the edit), so the change ships as `docs/assessment-speed-productionization-20261110-benchrec.patch` to apply in the owner repo and re-lock | S |
| T2 | Hooks spawn `python3` 3–5 × per Bash call to read JSON fields; `vault-hook` already uses `jq` | `.claude/hooks/benchrec-hook.sh`, `checkpoint-hook.sh` | ~100–130 ms per Bash call | S |
| T3 | Leaderboard/validation gate re-hashes the same files (3,545 `sha256_file` calls, 41 dirs) | `scripts/build_ic_leaderboard.py`, `validation_gate.py` | 0.5–0.8 s of 2.35 s | S |
| T4 | `benchrec --verify-s3` spawns one `aws` process per object; `checkpoint.sh` calls `aws configure list-profiles` repeatedly | `tools/benchrec.py:334-438`, `scripts/checkpoint.sh:69-100` | seconds per record | S–M |
| T5 | `git cat-file` per sparse file in the browser/leaderboard builders | `scripts/build_lab_browser.py:236,259` | use `--batch` | S |

## 4. Productionization opportunities, ranked

### 4a. `crypto`

1. **Gate heavy deps behind features** (S–M). `aws-sdk-s3`/`sts`/`tokio` are used by one lib file (`src/ml_dataset/s3_budget.rs`); `redis` (+ OpenSSL via default `redis-native-tls`) by 2; `rusqlite` (bundled C SQLite) by 3 lib files and 1 bin. Every `cargo check` compiles all of them. Give bins `required-features`.
2. **Add profiles and pin the toolchain** (S). `[profile.release]` with `codegen-units=1`, `lto="thin"`, `strip=true`, `debug="line-tables-only"` (perf-usable); a separate `[profile.lab]` so frozen timing builds are unaffected; `[profile.dev.package."*"] opt-level=1`. `rust-toolchain.toml` (CI already pins 1.98 in `rust-lint.yml` while `rust-all-targets.yml` floats `@stable`; 199 `@stable` / 31 `@master` uses across workflows).
3. **Stop auto-discovering ~1,090 targets in the PR gate** (M). 30 `[[bin]]` + 9 `[[example]]` + 111 `src/bin/` + 908 `examples/` + 32 `tests/`, each linking the 940k-line lib; `cargo check --all-targets` is the PR critical path (25-min budget). Set `autoexamples=false`/`autobins=false`, move research bins to a `research-bins` member, keep a nightly full check.
4. **Real Cargo workspace** (L). 415 `Cargo.toml`, 290 with empty `[workspace]` stubs, 333 `Cargo.lock`; num-bigint at 0.4.6/0.4.8/0.5.1, serde_json at five versions, clap at three. `[workspace.dependencies]` for live crates; `exclude` frozen experiments (their lockfiles are evidence).
5. **Split `crypto_lib`** (L, high risk). `cryptanalysis` (599k lines) and `ml_dataset` (60k) out of the single crate; 47 `include_*!` and `#[path]` sites across `research/` are the hazard.
6. **Test tiers with nextest** (M). 6,922 `#[test]` + 227 integration tests, 165 `#[ignore]`; no workflow runs the full lib suite (only module-filtered subsets). `.config/nextest.toml` with fast/slow profiles.
7. **CI cache and workflow hygiene** (S–M). No sccache; `rust-all-targets` and `rust-lint` caches not shared; one workflow without `timeout-minutes`. 544 workflows: 237 dispatch-only (frozen history), 385 experiment-named; 16 push/PR workflows without `paths:`. Replace per-experiment workflows with one `workflow_call` runner + registry (0 current `workflow_call` users); respect `frozen-workflow-pins.json`.
8. **Repo diet** (M–L). 11.08 GB tracked / 286k files, pack 4.23 GiB; `research/` 9.83 GB in 262k files; 2.76 GB `.jsonl`, 2.06 GB `.gz`, 0.58 GB `.zst`, 0.39 GB `.pdf`, 0.27 GB `.log`. Route new evidence through the existing `bench.record/v1` S3-by-SHA path; make `large-files` a hard fail (>10 MiB, no label bypass); no history rewrite.
9. **Python tooling hygiene** (S–M). 295 `.py` under `scripts/` (99k lines) + 60 under `tools/`; no root `pyproject.toml`, ruff or mypy; `compose_koblitz_stage*` (74 files, 15k lines, median adjacent similarity 0.64) shares a 30-line skeleton worth factoring but is not a drop-in dedupe.
10. **Release engineering** (S–M). Root crate stays at 0.1.0; `release.yml` publishes a rolling `major.minor.run` tag with `ECC_MARCH=x86-64-v3`; no tags, no CHANGELOG, no release-please; no Docker layer cache or stripping in `isolab.yml`; no root `.dockerignore`.
11. **Session-start hook** (M). 900 s budget; `go install …@main` unpinned; 39 install/network lines in `setup-workstation.sh`; 1.5 GB `cryptanalysis` clone and a detached 3.6 GB SageMath start. Pin versions, bake into the image, make the clone lazy.
12. **Enforcement-tool parity** (M). A lock already exists: `docs/shared-code.lock.json` pins 22 shared files by SHA-256 (13 owned by `crypto-autoresearcher`, 9 by `crypto`), checked by `tools/check_lab_parity.py` in the `parity` PR job; an edit on a mirror fails until it lands in the owner and the lock is re-cut everywhere (this PR hit exactly that on `tools/benchrec.py`). The reviewers' "no parity mechanism" claim was wrong. What remains true: the `cryptanalysis` copies of `benchrec.py` (1,216 vs 1,315 lines), `test_benchrec.py` (199 vs 494), `progress_metrics.py` (305 vs 839) and `artifacts/*.py` differ from `crypto`'s, so either those entries do not list `cryptanalysis` in `repos` or they are outside the lock; add them, and route every tooling fix through the owner repo first.

### 4b. `cryptanalysis`

1. **Path-filter or consolidate the 85 PR workflows without `paths:`** and add a required-checks aggregator (M). 245 workflows, ≈215 per-experiment; `ci.yml` itself has no path filter.
2. **Schedules that never fire** (S). `nightly.yml` has only `workflow_dispatch` (valgrind linalg/indexcalc, long tests never run); `fuzz.yml`'s long job is gated on `github.event_name == 'schedule'` with no `schedule` trigger.
3. **ccache/sccache + apt cache in `ci.yml`** (S). 14–17 jobs rebuild from scratch, including the 294k generated lines; `apt-get` uncached every run; CUDA toolkit fetched with `--fetch` every run.
4. **Isolate `src/generated`** (M) into its own object/TU (see C6) so sanitizer/TSan jobs stop recompiling 250k lines of tables.
5. **Generated-code staleness gate** (S). Generator lives at `experiments/prime-j0-cost-aware-chain/make_tau_wide_orbits.py`; nothing in CI regenerates and diffs `src/generated`.
6. **Single-source the version** (S). `version.txt` 0.1.2 vs CMake `project(VERSION 0.1.0)` (→ SOVERSION), pyproject 0.1.0, Rust 0.1.0; release-please has no `extra-files` and `draft: true`.
7. **Fix the duplicated `CA_BUILD_*_BENCH` blocks** (S; C5) and add a CI job that enables all four.
8. **Root `pyproject.toml` with ruff/mypy and a tools test target** (S–M); 6,085 `.py` files, lint only under `bindings/python`.
9. **Tool parity with `crypto`** (M; see 4a-12).
10. **ctest `LABELS` fast/slow** (S); valgrind job hard-codes a list; Python unit tests and ecc2k130-usecase outside ctest; replay `fuzz/crashes` as regression tests.
11. **Harden images** (M). Floating base tags, no `@sha256` digests; CUDA `devel` images not multi-stage (`nvidia/cuda:12.8.1-devel`, `13.3.1-devel`); `ecc2k130/runner/deploy/Dockerfile` runs as root; no root `.dockerignore` (3.2 GB context).
12. **Observability beyond the coordinator** (M). `ca-coordinator` (Go, slog JSON, `/healthz`, `/readyz`, Prometheus `/metrics` behind auth, Helm ServiceMonitor) is the production-grade component; `gpu-health` and `cloud/*.py` have no metrics/health/structured logs; the `CA_*` compile-time algorithm switches are not recorded in `ca`/`ca_bench` output.
13. **Checkout weight** (L, high risk). 66k files, 60k under `experiments/`, pack 2.29 GiB; 17 `.olean` build outputs tracked; sealed protocols hash these paths, so move only new evidence to S3-by-SHA and use `actions/checkout` `sparse-checkout` in CI.

### 4c. `ml-cryptanalysis`

1. **Cached `.so` keyed only on source hash, built with `-march=native`** (`csd/elim.py:30-45`, `csd/macaulay.py:56-60`) → SIGILL risk on heterogeneous EKS Spot nodes; `macaulay.py` hard-codes `gcc`. Add CPU/compiler to the cache tag or build portable with an opt-in native flag (S).
2. **`.c` sources not shipped**: `packages=["csd"]` with no package-data → non-editable install raises `FileNotFoundError` when a compiler exists (S).
3. **C vs pure-Python paths not cross-checked in CI** (`CSD_ELIM_PURE`) (S).
4. **Packaging**: 17-line `pyproject.toml`, loose bounds, no lockfile, no entry points, no ruff/mypy (1,785 ambient-ruff findings), single Python 3.11 CI cell with unpinned CPU torch; 442 tests in 79 files, no xdist (S–M).
5. **Reproducibility**: `torch.manual_seed` only in three pipeline files, no `use_deterministic_algorithms`, no `SHA256SUMS` for `data/*.jsonl`; `scripts_p3_run.py` at the root hard-codes `/Volumes/SSD990-2/…` (28 files contain `/Volumes/` paths) (S).

## 5. Proposed sequence (each its own PR, each with its own evidence)

1. **Build-only, zero-risk** (this week): C6 OBJECT library + C5 duplicate-block fix (`cryptanalysis`); R8 `[profile.release]`/`[profile.lab]` + `rust-toolchain.toml` + feature-gated heavy deps (`crypto`); T1/T2 hook and gate speedups. Evidence: build wall/user before/after, `ctest` identical, `benchrec gate` output-set identical, hook exit codes identical.
2. **WALL-only kernel fixes, bundled by binary** so one A/A ecbench diff covers each: R1 (volcano/isoclass), R2+R10 (`ic_framework/linalg.rs`), R3+R5 (`f4_fp`, rootfind), C1+C2+C15 (GLV), C3 (`ca_invmod`, with the `CA_FC_EUCLID` decision made first). Gate: every counter, `field_ops`, `recovered`, replay digest and `ca_perfbench` fingerprint identical; only wall ns differs; measured per AGENTS.md §10 (recorded host, A/A, ABAB ≥5, `isolated_bench`), landed with `benchrec` records and a `benchmarks/<date>-<slug>/` row.
3. **Parallelism**: C9 Lanczos pool (1/2/4-thread fingerprint equality), R4 Gauss kernel, R6-A.
4. **COUNTER/METHOD items** (R6-B, R7, C4 field-op diagnostics, GPU coefficient-free walks) only after an owner decision on counter versioning, each under a new method/oracle id.
5. **Structural productionization**: 4a-3 (target discovery), 4b-1/2/3 (CI), 4a-12/4b-9 (tool parity), then the workspace conversion and the repo diet.

## 6. What this assessment did not do

- No isolated or lab measurement; no PMU, so no cycle/IPC figures (the
  `perf-ipc` skill refuses on this host); no GPU, so C10–C13 are untested.
- `ecc2k130/` (binary-field, `-march=native` Makefile build) and the Rust
  `suite/` F4/Macaulay in `cryptanalysis` were not reviewed.
- Reviewer gains are reading-based estimates unless marked "measured".
- No knowledge-base query was run: this is an engineering review of the
  implementation, not a research lead; any item that turns into a method
  change (R6-B, R7) must go through `research-kb` and the ecbench-bounds flow
  before it is claimed.

## Sources checked

Source spot-checks by the main session: `src/bin/volcano_rho.rs:68-135`,
`src/cryptanalysis/ecbench/generic.rs:44-52`,
`src/cryptanalysis/ic_framework/linalg.rs:70-80`,
`src/cryptanalysis/f4_fp.rs:345-354`, `Cargo.toml` (no `[profile]`, no
`[workspace]`), `AGENTS.md` §10 and "Implementation language: no Python".
Everything else: reviewer reports, retained in the session transcript.
