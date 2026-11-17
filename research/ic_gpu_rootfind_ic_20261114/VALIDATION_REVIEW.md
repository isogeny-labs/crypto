# Validation review: `research/ic_gpu_rootfind_ic_20261114/REPORT.md`

Validator (independent; did not write the code, the spec or the report), 2026-11-15.

**Disposition: `validated-scoped`.** The independent recomputation reproduces every table number
and every headline number in REPORT.md exactly. The scoped claim that holds:

- **Identity:** on three prime-order curves (log2 r = 27.5, 30.0, 31.3), `rootfind-subtract-gpu`
  returns the same relations, recovered logarithm and counted cost as `rootfind-subtract` in all 24
  CPU/GPU pairs. Every charged counter and every non-`gpu_*` native counter is equal, phase by phase.
- **Wall time:** the GPU arm's median relation phase is 21.4–121.2× faster and its median solve
  2.5–43.5× faster than a **single-threaded** CPU arm. These are L0 wall observations on one A100
  pod, and ecbench rated their measurement quality 0/100 with 0 runs admitted.
- **Prefetch:** Eytzinger prefetch is 1.21–1.92× slower on the sorted path. The hash path is within
  0.5%.

No number is refuted. The findings are:

- six statements that the data do not support as written;
- one inverted ratio label in RESULTS.md;
- a missing isolation disclosure;
- missing provenance bindings: the session has no git commit, and the spec freeze is not
  established on the server side;
- unmet reporting rules: the §5a band is not stated, §15 is not addressed, the sessions are not
  committed loose, and the data are not in S3.

None of these changes the direction or the size of the measured ratios.

## What was validated

| item | value |
|---|---|
| repository state | `origin/main` = `6ef2197cf99f1798c0bd5fa121508673fb9fc591`, read with `git archive` / `git show`; working tree not used |
| research directory tree | `fc9add2f21b2` at `origin/main`, identical at the PR #4178 merge `b61a431d27` |
| measured revision (asserted) | `aeeb30b883` (the `REV` file in the tarball, RESULTS.md). `plugins.rs` and `gpu/rootfind/` are byte-identical at `aeeb30b883` and `origin/main`. `rootfind_gpu.rs` differs only by `9b8a06417c` (rustfmt, pushed 23:48Z, after the run). |
| raw tarball | `raw/out_ric.tgz`, sha256 `8ab256652ca6f2a1bb4fe1f90b4e9e37dad70a086b56e022d42de84c8dc543f4`, matches `raw/out_ric.tgz.sha256` and REPORT.md |
| tarball contents | 141 files: `sessions/main` (72 records), `sessions/smoke` (16), `audit-{main,smoke}.json`, `progress.log`, `tp-*-{1..5}.log`, `tpsmall-*-{1..5}.log`, and `selftest-`, `build-` and `test-*.log` for D, D2, P3, P4, P5 and P4L2. All are present, as claimed. |
| session hash bindings | `records.jsonl`, `plan.json` and `host.json` hash to the `records_sha256`, `plan_sha256` and `host_sha256` in `session.json`, for both sessions |
| binary | `binary_sha256 = e320d171…` in every record. It equals the pod's `sha256-ic.txt` and the local `target/release/ecbench` (mtime 2026-11-14 22:55Z). |

Scratch work (scripts and outputs, not committed) is in
`/tmp/claude-0/-home-user/648c3177-6621-55ff-b9d8-09ead07ab678/scratchpad/validator-ric/`:
`scripts/check_ic.py`, `scripts/check_pf.py`, `ic_rows.json` and `replay/`.

## Requirement results

| # | requirement | result | evidence |
|---|---|---|---|
| 1 | Tarball hash and contents as described | **pass** | `raw/out_ric.tgz`; `tar -tzf` listing |
| 2a | 72/72 verified | **pass** | `out_ric/sessions/main/records.jsonl`: 12 records per arm × 6 arms, all `verified`, rounds 0 and 1, no warm-up |
| 2b | Independent certificates (my own affine EC code, not the solver) | **pass, 72/72 main + 16/16 smoke** | a and b derived from G and Q and checked against a third point. p and r prime, r = p + 1 − t. j matches field 4 of the ICV1 string. r·G = O. recovered·G = Q, recovered = planted, planted·G = Q for every workload. |
| 2c | Replay audit reproduced 72 | **pass (pod)**; **pass (cross-host)** — see §Cross-host replay | `out_ric/audit-main.json`, `audit-main.log`: "72 records, 72 verified, 72 replays reproduced, 0 problems". The auditor's env class equals the session's (same pod). |
| 2d | 24 pairs equal in S, total GAE, recovered, relations, targets, relation-phase adds | **pass; stronger than claimed** | Pairs are matched by (workload_id, round). The checks also cover every phase's adds, doubles, scalar_mults and gae; every `native` counter except `gpu_*`; every `work_native` counter; `counters`; `cost.unpriced`; `work.bitops`; `fb_sha256`; and `algorithm_seed`. All 24 pairs match on all of these. |
| 2e | Wall table (medians, ratios) | **pass (numbers)** / **disclosure gap** (L0, quality 0/100) | recomputed from `time.solve_wall_ns` and the `relations` phase `wall_ns`; see below |
| 2f | mean S and S/rho | **pass at w = 0**; band holds but is not stated | rho-neg mean S per curve 0.9775, 1.3689 and 1.1315 |
| 2g | Breakdown statements | **partly supported** (per-workload medians, not per solve or per run) | see "Unsupported or imprecise statements" |
| 2h | specs/main.json = what ran | **pass** | byte-identical to `sessions/main/spec.json` (sha256 `0b18d10e…`). The cross-host auditor's `spec_id` check also passes. |
| 2i | Spec committed before the run | **not established** | see "Provenance" |
| 3 | GPU oracle charged as `rootfind-subtract` and returns its first hit | **pass by reading, conditional on kernel completeness**; empirically holds on all 24 pairs | see "Code review" |
| 4a | Prefetch medians per arm and config, ratios vs D | **pass** (every RESULTS.md cell reproduced to 0.01 ms) | `tp-*-*.log`, `tpsmall-*-*.log` (5 rounds × 6 arms × 2 workloads, all `cuda`) |
| 4b | A/A D2 vs D ≤ 0.16% | **pass**: 0.123% (\|F\| = 65536) and 0.161% (\|F\| = 1024) | same |
| 4c | Prefetch 1.21–1.92× slower on the sorted path | **pass**: 1.312–1.919 (65536) and 1.205–1.812 (1024) | same |
| 4d | Hash path within 0.5% | **pass**: max \|r − 1\| 0.15% (65536) and 0.45% (1024). By code, `RF_EYT_PF` appears only in `SortedTable` (Eytzinger). | `gpu/rootfind/rootfind.cuh` at `aeeb30b883` |
| 4e | 3/3 selftests and host tests before timing | **pass** | `selftest-*.log`: 3× "SELFTEST PASSED" and no "FAIL" per arm. `test-*.log`: "ALL PASS", rc = 0. `build-*.log`: rc = 0 with the intended `RF_EYT_PREFETCH{,_L2}` flags. `progress.log` puts the selftests (23:39:19–26) before round 1 (23:39:57). |
| 5 | Wording | **findings below** | |

## Confirmed numbers (recomputed from the records and logs)

Wall time is the median of the 4 runs (2 targets × 2 rounds) per arm. The ratio is median/median.

| log2 r | \|F_x\| | mean S | rho mean S | S/rho | CPU solve | GPU solve | ratio | CPU rel. | GPU rel. | ratio |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 27.485 | 4096 | 6259.1 | 0.9775 | 6403 | 1.881 | 0.489 | 3.85 | 1.809 | 0.0702 | 25.77 |
| 27.485 | 16384 | 3639.0 | 0.9775 | 3723 | 2.718 | 1.073 | 2.53 | 1.958 | 0.0915 | 21.40 |
| 29.985 | 4096 | 18006.6 | 1.3689 | 13154 | 12.392 | 0.507 | 24.46 | 12.292 | 0.146 | 84.44 |
| 29.985 | 16384 | 14371.0 | 1.3689 | 10498 | 12.094 | 1.386 | 8.72 | 11.101 | 0.143 | 77.84 |
| 31.262 | 4096 | 26009.6 | 1.1315 | 22986 | 28.011 | 0.643 | 43.54 | 27.911 | 0.246 | 113.23 |
| 31.262 | 16384 | 23264.9 | 1.1315 | 20560 | 28.569 | 1.511 | 18.90 | 27.475 | 0.227 | 121.25 |

All of these match REPORT.md, RESULTS.md, `results.json` and `fig_ic.svg`. The "GPU batches /
targets swept" column of RESULTS.md matches too (48/49152, 29/29696, 287/293887, 79/80896,
636/651264 and 163/166912).

Medians of the paired ratios, for reference, are 3.98, 2.45, 23.9, 8.81, 43.0 and 18.4 (solve). The
report's "median of 4 paired runs" means the ratio of the two medians.

Also confirmed:

- **Curves:** log2 r = 27.5, 30.0 and 31.3; two targets each; two rounds; arm order alternating.
- **Batching:** one lane round is one kernel launch (`gpu_batches == lane_rounds` in 24/24 GPU
  records).
- **Fallback:** the CPU fallback path for degenerate targets (`gpu_cpu_fallback_targets`) ran in 13
  of the 24 GPU records, and those pairs are still identical.
- **Fail-closed re-addition:** `frobfold_mismatches = 0` everywhere.
- **Threads:** the ecbench child is single-threaded on both arms (`threads_before/after = 1`). On the
  GPU arm the child's user/wall ratio is 0.09–0.46, so it mostly waits on the worker.
- **Backend:** each of the 24 GPU executions' stderr (`sessions/main/exec/*.stderr`) prints
  `cuda: NVIDIA A100 80GB PCIe (sm_80, 108 SMs)` with `lookup=1` (sorted, Eytzinger) and
  `chunk=32`. With the AUTO backend that the Rust side sends, `resolve_backend` selects CUDA when a
  device is present, and a CUDA session error fails with no fallback. So the GPU arm ran on the
  device.
- **Environment:** host AMD EPYC 9454 (`lscpu.txt`), nvcc 12.8 (`nvcc.txt`; driver CUDA 13.2), A100
  80GB PCIe.
- **§5a band (recomputed from the records' unpriced counters):** the totals match the pod's
  `compare-rho-F*.txt` exactly. For example, `lookups` is 7 968 226 678 on the F4096 arms and
  `canonicalisations` is 457 031 on rho-neg. Per-curve IC/rho at w = 0, 0.1 and 1:
  - F4096: 6403/6420/6495, 13154/13165/13213 and 22986/23001/23068;
  - F16384: 3723/3733/3779, 10498/10507/10546 and 20560/20574/20634.

  IC is above rho at every weight on every curve, so the "3 700–23 000×" distance holds across the
  band.
- **Prefetch:** RESULTS.md's per-configuration medians and ratios at both workloads are confirmed,
  including the best configurations (D sorted/C16 360.27 ms at \|F\| = 65536; hash/C16 17.71 ms for
  every arm at \|F\| = 1024). The largest within-arm max/min is 1.263 (P3 sorted/C16, 65536) and
  1.071 (1024). RESULTS.md states the same.
- **Section (1):** `research/ic_gpu_rootfind_lookup_20261114/VALIDATION_REVIEW.md` exists with
  disposition "validated, scoped". The four corrections (1.5–3.3%, 15–20%, 0.67–0.71×,
  `c308625f57`) are present in that REPORT.md, and "15–21%" is gone.

## Cross-host replay

The exact binary (`e320d171…`, the local `target/release/ecbench`) re-ran
`ecbench verify --replay-all --exit-code` on copies of both sessions on a 4-vCPU Intel Xeon VM. The
receipts name auditor env and hardware classes different from the pod's (`ECBENV3h7a426069b1f8` /
`ECBHW1hdee168f164cd`). The GPU-arm records were replayed through a different worker build,
`target/release/rootfind_serve`, host emulation only, sha256 `240f76cf…`, against the pod's
`f7c496d6…` CUDA worker.

- smoke: **16 records, 16 verified, 16 replays reproduced, 0 problems**. All 18 audit checks are
  true, including `spec_id`, `plan_hash`, `records_hash` and `host_hash`. Auditor classes:
  `ECBENV3h3a76a971d59d` / `ECBHW1h33f0d9a6b3db`. Receipt sha256 `d5dfc1e6…`.
- main: **72 records, 72 verified, 72 replays reproduced (all "identical"), 0 problems**. All audit
  checks are true, and the run took 11 min 53 s. Auditor classes: `ECBENV3hb516f42f4ab3` /
  `ECBHW1he9e33daae9e7`. Receipt sha256 `92a79022…`.

The receipts are scratch outputs (`replay/audit-{main,smoke}-local.json`), not committed.

This also shows that the counted record does not depend on the worker build or backend, which is the
intent. It also shows the point raised under finding 4 below: a `-gpu` record can be produced with
the sweep on the CPU emulation backend, and nothing in the record shows it.

## Code review (claim 3)

Read at `aeeb30b883`: `src/cryptanalysis/ic_framework/rootfind_gpu.rs`, `RootfindSubtractOracle` in
`plugins.rs` (lines 3136–3460), `collect_and_solve_relations_touching` in `ic_boundary.rs`, and
`gpu/rootfind/{rootfind.cuh, rootfind_api.cpp, rootfind_serve.cpp, rootfind_cuda.cu}`.

**Charging is identical by construction.**

- `charge_sweep_setup` charges what CPU `decompose` charges before the first candidate: +2 muls, one
  inversion, 3k muls, and `sweep_field_{muls,adds}`.
- `charge_misses(n)` charges exactly the CPU's cost per missed candidate: one add, 4 muls, one sqr
  and one lookup.
- A kernel-reported candidate is charged `charge_candidate` (the same add, 4 muls and sqr) plus
  `cpu.lookup`. That is the CPU's own lookup, with its ledgered re-addition and mismatch counter.
- A refutation is charged up to 2k candidates. Degenerate targets (`x_R ∈ base`) go to
  `cpu.decompose` whole, and the infinity case returns before any charge on both paths.

**The returned relation is always the CPU's.**

- The kernel's j and τ are used only to filter (`j >= i`). The candidate T is recomputed on the CPU
  and looked up with the CPU index, and the relation returned is `cpu.lookup`'s.
- So a false positive from the kernel is charged exactly as the CPU charges that miss, and it cannot
  produce a false relation.
- The kernel's ε convention (`ε = +1` is `R − F_i`, λ = (y_R + y_i)/(x_R − x_i)) and its indices
  (original abscissa indices through `SortedTable.idx` / `HashTable.vals`) match the CPU sweep's
  order, `pos = 2i + [ε < 0]`.
- Overflow fails closed: the worker returns `RF_ERR_OVERFLOW`, the Rust side requires `status == 0`
  and `found == written`, and otherwise the run panics. A dead worker also panics.

**The j ≥ i argument is correct.** If the CPU's first hit had j < i, then at abscissa j the candidate
R − τF_j = εF_i would be a base point, because the base is negation-closed. The CPU would have hit
there first, at position 2j + [τ < 0] < 2i.

**The one path where the ledgers could differ is a false negative from the kernel.** If the kernel
misses the CPU's first hit (a wrong lookup, field arithmetic or index), the GPU oracle charges more
misses and returns a later hit or none. Nothing checks for this at run time. Negation closure makes
the `j ≥ i` filter safe; it does not guarantee completeness. In this run no such event changed any
counter in any of the 24 pairs. Those pairs cover 1 261 748 decomposed targets (sum of `targets_tried` on
the GPU arms), and a missed first hit would have changed the charged additions.

**A second, undisclosed difference between the arms.** `rootfind-subtract-gpu` returns
`line_guard() = true`, so the GPU arm's relation loop uses `GuardLineSet`, while the CPU arm uses
`fast_map` (`collect_and_solve_relations_touching`). The code documents the two as decision- and
count-identical, and the 24 pairs agree on every counter. It is nonetheless a second
implementation difference inside the timed relation phase: a wall-time confound, probably small (one
probe per target), and unquantified. REPORT.md describes the GPU oracle as "`rootfind-subtract` with
the sweep done on the worker" without mentioning it.

**Uncharged device work, quantified.** The kernel sweeps all 2|F_x| candidates of every swept target.
Device candidates (`gpu_targets_swept × 2|F_x|`) against charged relation-phase additions are 1.174,
4.883, 1.025, 1.415, 1.011 and 1.158 for the six rows, and 1.168 overall. The design is disclosed
(the `fes-f2-gpu` precedent), but the only figure the report gives is "about 300–500" unused targets.
That understates the uncharged work at \|F_x\| = 16384, where most of it is the rest of the sweep of
targets that decompose early.

I did not run `cargo test --release --lib rootfind_gpu`. Another session was running
`cargo test --release` in the shared target directory, with about 10 GB RSS on a 15 GB host, and its
working tree differs from `origin/main`. A concurrent build would contend for the lock and memory and
would test a moving tree. The cross-host replay of the actual records is the stronger check of those
records. The 48-run test and its mutation control therefore remain unverified by me (below).

## Unsupported or imprecise statements in REPORT.md

1. **"`gpu_targets_unused`, about 300–500 a run"**: not supported per run. The 24 values range from
   3 to 984, with a median of 454, and only 5 of the 24 lie in [300, 500]. "About 300–500" matches
   only the per-workload medians (308–502). See also "Uncharged device work" above.
2. **"oracle set-up, 0.25–0.35 s a solve"**: these are per-workload medians (0.250–0.346 s).
   Individual solves range from 0.215 to 0.391 s.
3. **"at \|F_x\| = 16384, linear algebra (0.69–0.94 s, the same on both arms)"**: 0.690–0.940 s are
   the GPU arm's medians. The CPU arm's medians are 0.711, 0.934 and 1.029 s, and individual runs on
   both arms range from 0.509 to 1.056 s. The work is the same; the quoted range is one arm's.
   ("The relation phase is 0.07–0.25 s" is likewise per-workload medians, 0.070–0.246 s; individual
   runs range from 0.067 to 0.275 s.)
4. **"No worker means `prepare` fails: a `-gpu` arm never silently runs on the CPU"**: true of the
   CPU *oracle*, false of the CPU in general. If only the host build `rootfind_serve` is beside the
   executable, or the CUDA build finds no device, the sweep runs on the emulation backend. That
   backend is multi-threaded over every hardware thread (192 on this pod). The backend is
   deliberately not in the record; it appears only on the execution's stderr. My replay did exactly
   this. In *this* run all 24 GPU executions' stderr shows the A100 present, so the measured GPU
   figures are device figures. The general statement should be narrowed.
5. **"A base that is not negation-closed is refused (that is what guarantees the kernel's j ≥ i hits
   contain the CPU's first hit)"**: negation closure is necessary for the filter to be safe. The
   guarantee also needs the kernel to be complete (no false negatives), which is supported only
   empirically, by the 24 pairs and the claimed tests.
6. **"S … growing with r, as a two-summand subtraction search must (its relation work grows like r,
   rho's like √r)"**: the growth is observed. "Grows like r" is a heuristic stated as fact. The
   counted total fitted over the three sizes grows as r^1.05 at \|F_x\| = 4096 and r^1.22 at 16384,
   which is faster than r. Three sizes are also below §5's four-size minimum for an empirical
   exponent.
7. **Wall table without its isolation level.** Every record is L0, with blocker "L1: no CPU
   reservation (--cpus none …)", and the run used `--allow-busy`. The pod's own
   `ecbench compare` read-outs (`compare-F4096.txt`, `compare-F16384.txt`) say "measurement
   quality: 0 / 100; paired attempts 12, verified 12, admitted 0" and "A/A bonus unavailable: the
   spec permits wall runs below L2". REPORT.md presents the wall table as a result without this
   qualification. The only A/A is rho-neg-aa against rho-neg (wall 1.0123 [1.000, 1.025]), on 0.6–11 ms
   rho solves, not on the IC arms (AGENTS.md §12: a wall figure is read against the session's A/A
   interval). The 2.5–121× ratios are far outside any plausible L0 noise. They are L0 observations
   and should be labelled as such.
8. **"S / rho" and "3 700–23 000×" are given at w = 0 only.** The §5a band is not stated; it holds
   (above). The wall-clock distance to the matched rho arm is not stated either. Per pair, the GPU
   arm's solve wall is 57–817× (median 141×) rho-neg's at F4096 and 141–2024× (median 362×) at
   F16384, on single-threaded rho.
9. **"23:05Z–23:42Z"**: the job's first command (`nvidia-smi -q`) is stamped 23:00:22 on the pod, and
   `progress.log` runs from 23:13:50 to 23:42:00. The 23:05Z start does not appear in the artifacts
   (minor).
10. **Prefetch causal sentence** ("the extra instruction and the divergent per-thread prefetch
    addresses cost more than the latency they hide; the paired lookups already overlap two
    descents"): an interpretation with no profiler evidence in the artifacts. Label it as a
    hypothesis.

## Error in RESULTS.md

**"Best configuration per arm" ratios are inverted.** `analyze.py` line 131 prints `mb / v` and
labels it "× the default's best". At \|F\| = 65536 the prefetch arms' best (hash/C16, 407.05–407.93
ms) is **1.130–1.132×** the default's best (360.27 ms), not 0.883–0.885×. The \|F\| = 1024 lines are
1.000 either way. REPORT.md does not repeat the error.

## Provenance and reporting-contract gaps

- **Measured revision not bound by the session.** `session.json` and `host.json` have
  `git_commit: null` and `git_dirty: null`. `aeeb30b883` rests on the operator-written `REV` file.
  The binary hash equals a local build made at 22:55Z, three minutes *before* `aeeb30b883` was
  committed (22:58:22Z). The bench record
  (`benchmarks/records/20261114T2258Z-ic-gpu-rootfind-ic.json`) names
  `commit a95274095f`, `commit_dirty: true`. Strings unique to the new oracle are in the binary,
  which is consistent but not a proof. The pod-built `rootfind_serve_cuda` (`f7c496d6…`) cannot be
  rebuilt here.
- **Spec freeze not established independently.** The spec bytes are what ran. Its only timestamp
  before the run is the self-asserted commit date (22:58:22Z). GitHub's branch activity shows the
  branch first created at **23:26:02Z** (at `621066c0d3`, which contains `aeeb30b883`). That is after
  the main run (23:13:56–23:19:54Z) and its audit (23:25:42Z). The PR was opened at 23:47:23Z.
- **Sessions are not committed loose.** They exist only inside the tarball, so CI cannot re-audit
  them (AGENTS.md §12).
- **Artifact policy.** The 113 398-byte `.tgz` is committed to Git with no `.artifacts.json`, and the
  bench record says `uploaded: false`. The S3 waiver is recorded in RESUME.md, not in REPORT.md. The
  archive is a plain `tar czf`, not the deterministic §13a recipe.
- **§15 (fastest external implementation) is not addressed.** The wall result is a within-method
  component ratio against our own single-threaded CPU oracle. A multi-threaded CPU arm is missing;
  it is disclosed in "Not done". The scope above is limited accordingly.
- **The §5a calibrated weight `w_cal` is not reported.** The order holds trivially.
- **The A/A builds differ in hash.** D (`ae948cb1…`) and D2 (`35aee6ae…`) were built in different
  directories with `-lineinfo`. D2 is a rebuild A/A, not a same-binary A/A. This is fine but should be
  said.
- Not applicable: the m = 83 / ECC2K-130 rules (prime-field curves of 28–32 bits). No leaderboard or
  `benchmarks/README.md` row cites this study, consistent with "adds no scoreboard or leaderboard
  row".

## Not checked or not checkable

- **Checks before the run:** the 48-run `cargo test --lib rootfind_gpu` equality, its mutation
  positive control ("recovered: Some(11) against None"), and the "locally with the host worker"
  smoke. No log of any of them is in the artifacts. The test also passes silently when
  `ICX_ROOTFIND_WORKER` is unset. I verified the test's structure: 4 curves (14, 17, 18 and 20 bits)
  × 2 planted × 3 target sources × 2 seeds = 48, with `assert_same` over every ledger. My
  cross-host replay of the pod smoke session stands in for the local smoke.
- **Pod details:** the pod id `1ku6389hniumbb`, "about $1.50" and "deleted" are not in the artifacts.
- **Rebuild:** whether `e320d171…` is bit-for-bit the `aeeb30b883` build. That needs a
  fresh-target-directory build, which was outside this review's limits.
- **Worker CPU time:** the worker is a separate process and not in the records' rusage, so "one
  thread plus the device" rests on the CUDA backend selection, not on a measurement.
