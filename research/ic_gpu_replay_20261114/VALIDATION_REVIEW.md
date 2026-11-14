---
title: "Validation review: fes-f2-gpu end-to-end, per-call overhead A/Bs, replay"
date: 2026-11-14
---

# Validation review: `fes-f2-gpu` reports and replay

Validator: the repository's `validator` agent (Claude Opus 5.5), dispatched by the producing session.
This review was written from the raw outputs with the validator's own code. It did not use the
producer's `analyze*.py` or `make_figure*.py` and did not edit any producer file.

**Frozen set.** Commit `95dce785b3859d93cc9ca39052ec3cb908bef7d8` (branch
`claude/index-calculus-gpu-optimization-otouqv`), as named by the dispatch. HEAD moved during the
review (`f83254d5f6` added a draft `REPORT.md` here, and `8bc8377356` added a record run entry). Every
artifact under test was therefore read from the git objects of `95dce785b3`, which are byte-identical
to the later commits for those paths. The draft `REPORT.md` is outside the frozen set; §9 cross-checks
its numbers against the same raw data, for information only.

**Scope.** (1) `research/ic_gpu_e2e_20261113/REPORT.md`, (2) `research/ic_gpu_overhead_20261114/REPORT.md`,
(3) the raw replay `research/ic_gpu_replay_20261114/raw/out_rp.tgz`.

## Summary: refuted and uncheckable claims first

| # | claim | where | verdict |
|--:|---|---|---|
| R1 | char-2 S varies "about ±0.5% between runs of the same arm" | report 2, Runs | **refuted**: the same-arm S max/min reaches 1.358 (A/B 2, n=29, `main`) and 1.165 (A/B 2, n=25, `merged`). Across all runs of one sweep it reaches 1.654. A/B 1 reaches 1.013. Counted ops are identical in every run; the spread comes entirely from the per-run measured word-XOR price (A/B 2, n=29: `ns_per_word_xor/ns_per_add` from 0.001580 to 0.002614). The report's own `RESULTS2.md` lists these S values. |
| R2 | `inherited-f4`'s counted S "stays 9× … lower" at n=23 | report 1, ratios | **refuted (rounding)**: 18444.03 / 1852.45 = **9.96×**, which rounds to 10×. The 53× (53.04) and 106× (106.00) gaps are confirmed. |
| R3 | CPU `fes-f2` at n=23 "13.65 → 13.44 ms per call (1.5%)" | report 2, A/B 1 | within rounding: computed 13.651 → 13.434 ms, which is 1.6% (1.5% and 1.7% for the two repeats). Not material. |
| U1 | the `ic` and `fes_cuda` binaries were built from the stated commits | all | **cannot check**: the bench JSON carries `software.git_commit = null` and `git_dirty = null`. The binding is the `REV` file plus the binary SHA-256 only, and a rebuild was out of scope (disk). |
| U2 | unit tests named in reports 1 and 2; the local probe (`from_poly` 0.461 → 0.041 ms, engine body 0.569 → 0.062 ms) | reports 1, 2 | **cannot check**: no raw output in the artifact set, and `cargo test` was not run (dispatch constraint). |
| U3 | pod and host identifiers, costs in dollars | all | **cannot check** from raw. The four runs did use four distinct GPUs and CPU models (serials and `lscpu` differ), consistent with four hosts. |
| U4 | kernel times "roughly 0.1, 0.2 and 1.3 ms" | report 1 | not in report 1's raw. Consistent with the replay's worker-measured 0.069, 0.141 and 1.273 ms. |
| U5 | "Runpod blocks the profiler counters (`ERR_NVGPUCTRPERM`)", "80 ms against 22 ms in earlier rounds" | draft report (§9) | not in this raw data |

Every other checked number is confirmed. Dispositions:

| artifact | disposition |
|---|---|
| report 1 (e2e) | **validated** for its counted claims (verified, calls, ops, hit rate, S/rho, `fes-f2-gpu` ≡ `fes-f2`) and table values. Correct R2. Its wall-time values are single observations that the replay does not reproduce (different code and host; §5). |
| report 2 (overhead A/Bs) | **validated** for both A/B tables, the spreads and the counted identity. Correct R1. |
| replay `out_rp.tgz` | **validated** as a reproduction of every counted field of report 1. It is not an independent replay in the validation gate's sense, because the producing session launched it. |

## 1. Artifact integrity

| file | recorded SHA-256 | recomputed | source of record |
|---|---|---|---|
| `research/ic_gpu_e2e_20261113/raw/out_e2e.tgz` | `9d1fbae0…29830ae` | match | `raw/SHA256SUMS` |
| `research/ic_gpu_e2e_20261113/raw/run1-unordered-koblitz-n23.json` | `33387fae…efcf5f` | match | `raw/SHA256SUMS` |
| `out_ab.tgz` (local copy, scratchpad `abres/`) | `dfcedbfa…dce0ba62`, 2 219 530 B | match, same size | `research/ic_gpu_overhead_20261114/raw/.artifacts.json` and `SHA256SUMS` |
| `out_ab2.tgz` (local copy, scratchpad `ab2res/`) | `8d090bd4…6f461a3`, 2 216 031 B | match, same size | same |
| `research/ic_gpu_replay_20261114/raw/out_rp.tgz` | `76fa075a…ad633` | match | `raw/SHA256SUMS` |

The two A/B tarballs are not in the repository: they are in the artifact store and could not be
fetched from this session. Their local copies match the manifest. Each tarball was extracted into its
own empty directory, and no member path was absolute or contained `..`. The replay sweep specs are
byte-identical to `research/ic_gpu_e2e_20261113/sweeps/`. The A/B sweeps are the same specs with the
mitm row and `inherited-f4` (and at n=25, `fes-f2`) removed; seeds are unchanged.

Provenance recorded in the raw data:

| run | `REV` | `ic` sha256 | GPU, driver, CUDA | CPU |
|---|---|---|---|---|
| e2e | `abecd26901` | `ad9084a9…` | A100 80GB PCIe, 595.91.07, 12.8 | Xeon Gold 6238R |
| A/B 1 | `abecd26901` / `6128d645d6` | `ad9084a9…` / `03d96415…` | same | EPYC 7502 |
| A/B 2 | `3e432229fc` / `40587624bb` | `895e8559…` / `cbd66d7e…` | same | EPYC 7763 |
| replay | `fa75167c45` | `85d155c7…` | same | EPYC 9554 |

`fa75167c45` and main `9b88564250` have no `.rs` difference (`git diff --stat`: only `gpu/fes/fes.cu`
and `gpu/rootfind/*`). The replay ran with `FES_SERVE_TIMING=1`, so the CUDA worker's per-call
event timers were active during it. The Rust-side timers (`FES_GPU_TIMING`) were off.

## 2. Discrete-log certificates (independent of the solver)

`src/bin/ic/bench.rs:504` plants `1 + seed_row mod (r − 1)`, where
`seed_row = sweep_seed + rep·0x9E37` is recorded in each row's `spec.seed`. I recomputed the planted
log from the sweep seed and repeat index and compared it with each row's `recovered` modulo `r`. I
also checked that `r` is prime (Miller–Rabin), that `r` divides the recorded group order, and that
the order is in the Hasse interval. For Koblitz n=23 I computed `#E(F_{2^23})` independently from
the Frobenius trace recurrence (a=1: 8 393 806, equal to the recorded order).

- **76 of 76 rows pass** across 30 result files (e2e 14, A/B 1 24, A/B 2 24, replay 14). The
  pre-fix run passes 8 of 8 rows. Every row also has `verified: true`, `solver_budget_seconds = 120`,
  `max_trials = 20 000 000` and `budget_exceeded = 0`.
- The rho references' `per_run.planted` sets equal the recomputed planted logs, and every rho run is
  verified. `rho_s` equals the recomputed mean of `per_run.s` to within 3e-16 relative.
- The worker serve smoke data from both A/B pods (`serve-smoke.bin`, 30 systems, n = 1…22) was
  brute-forced over all 2^n points with my own evaluator of `fes_eval`'s convention. Both the CUDA
  and host replies equal the complete zero set on every system: **459 050 solutions**, status 0,
  `total = listed`. The e2e text smoke (2 systems) was checked by hand: {0, 7} and {1, 2}.

## 3. Report 1 (end-to-end)

The recomputed values agree with the table in every cell. Hit rate is the mean over repeats, calls
and wall times are summed over repeats, and S/rho is the mean over repeats.

| instance | solver | verified | hit rate | calls | S/rho | solver wall s | whole solve s |
|---|---|:-:|--:|--:|--:|--:|--:|
| Koblitz n=23 | `inherited-f4` | 2/2 | 0.6238 | 2487 | 2014.13 | 38.46 | 38.68 |
| | `fes-f2` | 2/2 | 0.6225 | 1878 | 20053.83 | 26.60 | 26.78 |
| | `fes-f2-gpu` | 2/2 | 0.6225 | 1878 | 20053.83 | 1.88 | 2.18 |
| char-2 n=25 | `inherited-f4` | 1/1 | 0.6396 | 1193 | 4362.09 | 33.51 | 33.64 |
| | `fes-f2` | 1/1 | 0.6369 | 2338 | 231367.03 | 141.50 | 141.94 |
| | `fes-f2-gpu` | 1/1 | 0.6369 | 2338 | 231367.03 | 2.77 | 3.22 |
| char-2 n=29 | `inherited-f4` | 1/1 | 0.6305 | 8977 | 30856.36 | 893.02 | 895.51 |
| | `fes-f2-gpu` | 1/1 | 0.6305 | 8977 | 3270693.03 | 32.50 | 38.70 |

- **`fes-f2-gpu` ≡ `fes-f2` per repeat** (n=23, both repeats; n=25): calls, `ops`, hit rate and S
  are equal, and all 124–126 integer counted leaves of the row are equal once `gpu_*` keys are
  removed. Every `fes-f2-gpu` row has `gpu_worker = calls`, `gpu_fallback = 0` and `gpu_capped = 0`.
  Confirmed.
- Unknowns per system, read from `decomposition.system.n_vars`: 24, 26 and 30. Confirmed.
- Wall ratios: 26.78/2.18 = **12.27×** (12.3×), 141.94/3.22 = **44.05×** (44.1×), 17.73×, 10.44×,
  23.14×. Confirmed. S gaps: 9.96× (R2), 53.04×, 106.00×.
- Per call: 1.002, 1.183 and 3.620 ms. Confirmed. The time outside the worker round trip, per
  repeat, is 0.657, 0.439, 0.607 and 1.746 ms (report 2 states "0.44–0.66 ms … 1.75 ms"). Confirmed.
- **Pre-fix run** (`run1-unordered-koblitz-n23.json`): `fes-f2-gpu` made **2642** calls (1481 + 1161)
  against `fes-f2`'s **1878** (872 + 1006), with all rows verified. Confirmed. Observation: the
  unordered run's first repeat matches `inherited-f4`'s first repeat in calls (1481) and hit rate,
  not `fes-f2`'s.
- `fes-f2` at n=29 was not run, and its S equality there rests on the by-construction argument.
  Equality is measured at n=23 and n=25 (§8).
- `fes-f2` at n=29 estimated at 0.7 s × 8977 calls = 1.75 h. The arithmetic is confirmed; the
  0.7 s/call input comes from another study and was not checked.

## 4. Report 2 (per-call overhead A/Bs)

Method: solver ms/call = `wall_ns / calls` per row. Each value is the mean over the two runs of an
arm (ABBA: runs 1 and 4 against runs 2 and 3) for the same target. Round trip uses
`gpu_round_trip_ns / calls`, and the Rust side (rest) is the difference.

**Counted identity.** For every solver and repeat, all integer counted leaves are identical across all
four runs of both arms, in both A/Bs, and equal to the e2e and replay rows (§8). Every row is
verified and passes its certificate. Confirmed.

A/B 1 (`abecd26901` → `6128d645d6`):

| instance, target | solver ms/call | Rust side ms/call | whole solve s | claimed |
|---|--:|--:|--:|---|
| n=23, 1 | 1.0938 → 0.5802 (1.885×) | 0.487 → 0.103 | 1.074 → 0.639 (1.682×) | 1.094 → 0.580 (1.89×); 0.49 → 0.10; 1.07 → 0.64 (1.68×) |
| n=23, 2 | 0.6529 → 0.2360 (2.766×) | 0.406 → 0.105 | 0.775 → 0.394 (1.967×) | 2.77×; 0.41 → 0.11; 1.97× |
| n=25 | 1.107 → 0.484 (2.287×) | 0.604 → 0.125 | 3.044 → 1.612 (1.888×) | 2.29×; 0.60 → 0.13; 1.89× |
| n=29 | 2.619 → 1.543 (1.697×) | 1.039 → 0.176 | 27.35 → 17.45 (1.567×) | 1.70×; 1.04 → 0.18; 1.57× |

Largest same-arm max/min of solver ms/call: 1.199 (n=25, old), 1.147 (n=23, new, target 1) and
1.0036 (n=29). The report gives 1.20, 1.15 and 1.004. Confirmed.

A/B 2 (`3e432229fc` main → `40587624bb` merged):

| instance, target | solver ms/call | whole solve s | same-arm max/min | claimed |
|---|--:|--:|--:|---|
| n=23, 1 | 0.5272 → 0.5392 (0.978×) | 0.526 → 0.531 | 1.311 | no change; 0.53 → 0.53; 1.31 |
| n=23, 2 | 0.2045 → 0.1286 (1.590×) | 0.284 → 0.198 (1.433×) | 1.056 | 1.59×; 1.43× |
| n=25 | 0.423 → 0.367 (1.154×) | 1.246 → 1.114 (1.119×) | 1.088 | 1.15×; 1.12×; 1.09 |
| n=29 | 1.547 → 1.443 (1.072×) | 16.40 → 15.17 (1.081×) | 1.0001 | 1.07×; 1.08×; 1.000 |

- The binary-protocol saving per call is 0.075 ms (n=23, target 2), 0.056 ms (n=25) and 0.104 ms
  (n=29), against the stated "about 0.06–0.10 ms". The arm ranges do not overlap on these three. On
  n=23 target 1 they overlap ([0.517, 0.537] against [0.467, 0.612]). Confirmed.
- CPU `fes-f2`, target 1: 12.416 → 13.120 ms, same-arm spread 1.091. Confirmed.
- At 30 unknowns (merged): 1.443 ms per call, 1.421 ms round trip. Confirmed. The "kernel about
  1.30 ms" is consistent with the replay's 1.273 ms.
- R1 (S spread), same-arm S max/min:

  | A/B | n=25 | n=29 |
  |---|---|---|
  | A/B 1 | 1.0044 / 1.0130 | 1.0060 / 1.0029 |
  | A/B 2 | 1.0143 / **1.1649** | **1.3576** / 1.0137 |

  Koblitz S is identical everywhere (pinned price). Confirmed.

## 5. Replay against report 1

| field | Koblitz n=23 (8 rows) | char-2 n=25 (4 rows) | char-2 n=29 (2 rows) |
|---|---|---|---|
| verified, recovered | identical | identical | identical |
| calls, `ops`, hit rate | identical | identical | identical |
| all integer counted leaves (79–139 per row) | identical, 0 differences | identical | identical |
| S, S/rho | identical; ≤ 2.1e-16 relative (float rounding, repeat 0 rows) | × 0.94305–0.94308 (solver rows), × 0.99997 (mitm row) | × 1.06111–1.06112 |
| `rho_s` | identical | identical | identical |
| wall time | not reproduced (§5 note) | not reproduced | not reproduced |

- **The S differences come from the calibration, not the engines.** `priced_by` is `pinned` on
  Koblitz: all pinned prices scale by the same factor 0.634869 against `ns_per_add`, so ratios and S
  are unchanged. It is `measured` on char-2: the word-XOR price ratio moved by 0.943082 (n=25) and
  1.061122 (n=29). In both runs, `solver.gae = ops × ns_per_word_xor / ns_per_add` holds exactly.
  Re-pricing the replay's counts at the e2e solver price reproduces the e2e S to within 6.4e-5
  relative; the residual is the non-solver phases' measured prices. The solver phase is 99.82–100.00%
  of S in every solver row.
- Wall times differ, as expected and not as a refutation: different code (the replay includes the
  overhead changes merged since), a different CPU (EPYC 9554 against Xeon 6238R) and worker timers on.
  The replay's whole-solve ratios are 21.95× (`fes-f2` → `fes-f2-gpu`, n=23), 109.25× (n=25) and
  49.21× (`inherited-f4` → `fes-f2-gpu`, n=29), against report 1's 12.27×, 44.05× and 23.14×. Report 1's
  wall values remain single observations on one host with that code.

## 6. Batching bound (replay)

The worker's `serve-bin-timing` line is identical in each sweep log and in `progress.log`. Its
`calls` equals the JSON's summed `gpu_worker`: 1878, 2338 and 8977. Kernel and busy times are
averages over the worker's lifetime, which is the whole `ic bench` process, so both Koblitz repeats
are included.

| unknowns | kernel GPU µs/call | worker busy µs/call | round trip µs/call, all repeats | round trip − kernel | start-up in it? |
|--:|--:|--:|--:|--:|---|
| 24 | 68.96 | 100.38 | 406.1 | 337.2 | yes (repeat 0) |
| 24, repeat 2 only | (68.96) | (100.38) | 123.9 | **54.9** | no |
| 26 | 140.86 | 192.56 | 304.8 | **164.0** | yes (only repeat) |
| 30 | 1272.90 | 1304.96 | 1364.9 | **92.0** | yes (only repeat) |

- **Start-up attribution.** `gpu_round_trip_ns` wraps `solve_on_shared_server`, which spawns the
  process-wide worker on its first call (`fes_gpu.rs`). The worker does its CUDA allocations before
  its request loop, outside `busy` (`fes.cu` `serve_bin_main`). Therefore **repeat 0 of every sweep**
  carries the worker spawn and CUDA initialisation. For char-2 n=25 and n=29 that is the only repeat,
  so their bounds are over-estimates. On Koblitz, repeat 0's round trip is 731.8 µs/call, about
  530 ms more in total than repeat 2's per-call rate would give. That 530 ms bounds start-up plus
  first-call warm-up from above.
- As a share of the replay's own wall time, the bound × calls is 29.9% of whole solve (40.7% of
  solver wall) at 24 unknowns (repeat 2), 42.4% (52.0%) at 26 and 6.1% (6.7%) at 30. The kernel is
  93.3% of the round trip at 30 unknowns.

## 7. rootfind phase profile (replay)

The profile columns sum to 100% by construction (`tot = c0+…+c4`, sweep-ar = `c4 − c5`). The
printed sums are 99.9–100.1. **The three rounds agree to at most 0.1 percentage point** in every
(size, lookup, chunk). Confirmed.

Perturbation (instrumented ms / plain ms, three-round means; each "ms" is a single kernel run timed by
CUDA events, same instance seed 2):

| \|F\| | config | probes % | inv % | sweep-ar % | load+prefix+unwind % | profile ms | plain ms | ratio | regs plain → profile |
|---|---|--:|--:|--:|--:|--:|--:|--:|---|
| 65536 | sorted 16 | 62.3 | 17.5 | 12.9 | 7.4 | 427.66 | 463.87 | 0.922* | 96 → 110 |
| | sorted 32 | 67.5 | 10.7 | 14.0 | 7.8 | 450.56 | 627.27 | 0.718 | 188 → 164 |
| | sorted 64 | 66.9 | 5.6 | 14.0 | 13.6 | 663.29 | 645.19 | 1.028 | 255 → 255 |
| | hash 16 | 79.9 | 8.5 | 7.8 | 3.8 | 700.83 | 579.05 | 1.210 | 96 → 110 (global) |
| | hash 32 | 83.4 | 5.1 | 7.7 | 3.8 | 858.91 | 849.39 | 1.011 | 168 → 164 (global) |
| | hash 64 | 84.9 | 2.6 | 7.9 | 4.6 | 1231.95 | 1223.26 | 1.007 | 255 → 255 |
| 1024 | sorted 16 | 50.8 | 22.3 | 15.9 | 11.1 | 22.73 | 80.08 | 0.284* | 96 → 110 |
| | sorted 32 | 58.0 | 13.9 | 18.2 | 9.9 | 22.81 | 31.23 | 0.730 | 188 → 164 |
| | sorted 64 | 60.3 | 7.2 | 17.2 | 15.2 | 32.49 | 31.67 | 1.026 | 255 → 255 |
| | hash 16 | 41.3 | 25.5 | 20.7 | 12.5 | 20.15 | 20.03 | 1.006 | 102 → 110 (shared) |
| | hash 32 | 47.6 | 16.5 | 23.9 | 11.9 | 19.99 | 25.73 | 0.777 | 188 → 164 (shared) |
| | hash 64 | 49.5 | 10.1 | 24.7 | 15.8 | 24.69 | 24.38 | 1.013 | 255 → 255 |

- **The instrumented build is not the plain kernel with overhead added.** The ptxas register counts
  in `rf-build-*.log` differ between the builds. At 128 threads per block on sm_80, my occupancy
  calculation (a model, not a measurement) predicts a slower profile build where its occupancy drops
  (chunk 16 sorted/global: 5 → 4 blocks per SM), a faster one where it rises (chunk 32 sorted/shared:
  2 → 3), and no change elsewhere. The observed ratios follow that in all 10 non-starred rows
  (0.718–1.210). Only chunk 64 (255 registers in both builds) isolates the instrumentation
  overhead: 1.007–1.028.
- **\*Sorted 16 is confounded in both sizes.** It is the first timed configuration in every
  plain-build process. `throughput()` makes no CUDA call before its first timed launch, while
  `profile()` first calls `rf_profile_reset()` (`cudaMemcpyToSymbol`). The plain build's first
  launch therefore carries lazy module loading, consistent with the self-test's 59.96 ms first
  configuration. The model predicts a ratio above 1 here; 0.922 and 0.284 were observed. Neither is a
  valid perturbation measurement.
- The phase shares describe the instrumented kernel. Those at chunk 16 and 32 come from a kernel
  with different occupancy than production.

## 8. Counted cost, rho, and the reporting contract

- **Standing claim confirmed:** no `fes-f2-gpu` change alters counted S relative to `fes-f2`. The
  `fes-f2` and `fes-f2-gpu` rows of each (sweep, repeat) have identical integer counted leaves and
  identical key sets across five code revisions (`abecd26901`, `6128d645d6`, `3e432229fc`,
  `40587624bb`, `fa75167c45`) and four hosts: 19 comparisons each for the two n=23 repeats, 11 at
  n=25 and 9 at n=29.
- **No report claims an improvement** in counted cost or against matched rho. Report 1 and report 2
  both state engineering class, wall time as a practicality note, and no leaderboard row. Confirmed.
  This review does not admit any row to `docs/ic/` or `benchmarks/README.md`.
- **§5a sensitivity of the one ranking made** ("`inherited-f4` remains the cheaper engine in counted
  cost at every size"). With the solver's word XORs priced at w GAE each, the order holds at the
  calibrated price (0.0012 to 0.0022 GAE per op), at 0.1 and at 1 on all three instances, and at
  w = 0 on n=25 and n=29. On Koblitz n=23 it reverses only below w = 4.6e-8, i.e. with a free solver:
  S(w=0) is 3.20 for `inherited-f4` against 2.56 for `fes-f2(-gpu)`, which needs fewer trials. The
  Koblitz calibration is pinned, so the solver is priced and the ranking stands under §5a's reading.
  The reports do not show this band; that is a reporting-contract gap, not a refuted number.
- **Rho gap.** With the solver free (w = 0), the best IC row is still above matched rho on every
  instance (2.79× at n=23, 2.54× at n=25, 4.08× at n=29), so the direction (IC above rho) holds
  across the band. The magnitudes (2014×, 4362×,
  30856×) are almost entirely the solver's word-XOR price (≥ 99.8% of S). The bench JSON's rho
  reference emits no unpriced-native counts (`per_run` has `s`, `s_walk` and `steps` only), so rho's
  side of the §5a band cannot be listed from this data. The gaps are distances, not rankings that
  could flip.
- The m=83 and ECC2K-130 disclosure rules do not apply: the instances have degree 23, 25 and 29.

## 9. Producer draft `REPORT.md` at `f83254d5f6` (outside the frozen set; for information)

Checked against the same raw data, which is unchanged since `95dce785b3`. Every number agrees with
the recomputation above except the following:

- "≤ 42%" and "≤ 6%" (batching share of the solve): computed **42.4%** and **6.1%**. "≤ 30%" is
  29.9%, which is fine.
- "moves by 5–6%": computed −5.7% (n=25) and **+6.1%** (n=29). "By the same factor for every engine"
  holds (0.94305 against 0.94308); the mitm row moved by only 0.003%.
- "the GPU engine is already 12–44× faster (whole solve) than the CPU search": these are report 1's
  ratios on older code. This replay measures 21.95× (n=23) and 109.25× (n=25).
- The lazy-loading caveat names only sorted/16 at |F|=1024. Sorted/16 at |F|=65536 is equally the
  plain build's first configuration (ratio 0.922 against an expected ratio above 1), and the stated
  range "0.72–1.21×" includes it.
- The rootfind share table uses round-1 values. Each phase agrees with its three-round mean to
  within 0.1 pp, and the summed load+prefix+unwind column to within 0.2 pp.
- "memory-latency bound" is an interpretation. Sampled `clock64()` shares cannot separate latency
  stalls from throughput limits without counters, and the shares at chunk 16 and 32 come from a
  kernel with different occupancy.
- "S/rho reproduces exactly on the Koblitz curve": exact to 2.1e-16 relative. "Every counted field
  reproduces exactly in all 14 rows": confirmed. The draft's statement that the replay is "a
  reproduction on another host, not an independent replay in the validation gate's sense" is correct.

## 10. Reproduce this review

Validator scripts (scratchpad, not committed), Python 3.13.18 with numpy 2.5.3, each run with
`python3 -I`. The directory is
`/tmp/claude-0/-home-user/648c3177-6621-55ff-b9d8-09ead07ab678/scratchpad/validator/scripts/`:

| script | checks |
|---|---|
| `vload.py` | row loader, planted-log certificate, integer-leaf extraction |
| `check_e2e.py` | report 1, pre-fix run, replay against e2e |
| `check_ab.py` | report 2, both A/Bs |
| `check_cross.py` | counted identity across all 30 files, run parameters |
| `check_reprice.py` | calibration explanation of replay S |
| `check_band.py` | solver-price sensitivity, rho gap at w = 0 |
| `check_batching.py` | batching bound and start-up attribution |
| `check_rootfind.py` | profile shares, spreads, perturbation |
| `check_smoke.py` | brute force of the serve smoke systems |

SHA-256 of the scripts as run:

```
ac35c084f84afc3189f52d1fe0c30977932106dfd6103685c37fba937dedfdf3  vload.py
eab38705a10144e46b2f5b0c5f8f46684d8b6cc4785f3b10ed00f860f3d14f7d  check_e2e.py
ed0aa5747a63d87fc162902e7c6b7afcdc005b48dcfd649e9df32571cc01e673  check_ab.py
111e1e5d3ed12680d945ba0063e48234332cc26ffa3e5787b83853c69d425566  check_cross.py
77a7ae9aa72c5871b479158ed7ef2fb6887a763efab65e6d29d5e505584986e7  check_reprice.py
2bf016b248eb279b1a47a291d654a1725f08a17bb3e8526b9798d71f18cc6d67  check_band.py
b2d5fe9a480cf696cb0319e61cd4df2d4c43c9a81ad664cf7643f452edfaf340  check_batching.py
af8f0d210624791fb05511ec43b4ca4caf7531a6071df662c47158fb1b1af9d1  check_rootfind.py
14a3667796678ee2fea9afea9f00df67b34d1e1eb908753d1975e5a2dc560b62  check_smoke.py
1c295e00a0eafa1128dc8848005340e263c8e438bd9ed0442ac221376eb8ed4b  make_review_json.py
```

`make_review_json.py` writes `VALIDATION_REVIEW.json` from the result files of the scripts above.

The scratchpad is not durable, so the formulas each script applies are stated in the section that
uses it. The machine-readable companion is `VALIDATION_REVIEW.json`, beside this file. It is not a
`validation-gate/v1` validator report and does not admit any row.
