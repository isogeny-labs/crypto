# Validation review: `research/ic_gpu_rootfind_m3_20261115/REPORT.md`

Validator (independent; did not write the code, the specs, the job or the report), 2026-11-15.
Standing of this review: `[pcc:unverified]` (no pcc claim is registered for it).

**Disposition: `validated-scoped`.**

The certificates, the identity claim and every table number hold. I recomputed them from the raw
tarball with my own code. I also replayed all 232 records on a CPU host of another hardware class
with the exact `ecbench` binary and an independently built host-emulation worker. What holds:

- **Identity.** In 128 of 128 GPU-vs-CPU pairs, `rootfind-subtract-gpu` (m = 2 and m = 3, spawn and
  daemon) and the IC A/A arms match `rootfind-subtract` exactly. The match covers every charged
  count, every non-`gpu_*` native counter, the `work` block, the recovered logarithm and the status,
  phase by phase. All 232 discrete-log certificates pass when re-verified with my own
  elliptic-curve code.
- **Wall time.** These are L0 observations on one A100 pod, against a single-threaded CPU arm.
  - **m = 2, daemon.** The daemon worker removes the per-process CUDA start. Its median solve is
    2.0–3.5× faster than a worker per run, and 20.8–127.8× faster than the CPU arm.
  - **m = 3, daemon.** The pair-table probe runs on the device. The median solve runs at
    0.41–0.86× the CPU arm's speed (25.2–31.2 bits), so the device arm is slower in every cell.
    The relation phase alone is faster on the device in 5 of 8 cells and slower in 3.
- **Distance to rho.** Rho is ahead in S and in wall time in every cell, and at every §5a band weight.

Nothing measured is refuted. The findings are:

- one rounding error in a table cell (finding 1): 3.2× should be 3.1×;
- eight statements the data do not support as written (findings 2–8 and 10), including a timestamp
  in `AMENDMENT_1.md` that contradicts its own claim;
- one undisclosed deviation (finding 9): a first-try daemon process was left running, unkilled, when
  the rf3 session ran;
- process gaps:
  - the raw data are not in S3, and the sessions are not committed loose;
  - the binary carries no source revision;
  - the §5a band is not stated.

PR #4217 had already merged (2026-11-15 04:23:03Z, merge `1c61f366dd`) before this review was
finished. The review therefore validates the merged head.

## What was validated

| item | value |
|---|---|
| validated objects | `origin/claude/index-calculus-gpu-optimization-otouqv` = `0fe6ff26f89abbb71e9c34fa1903e19fcf51073f` (the PR head, merged into `main` as `1c61f366dd`), read with `git archive` / `git show`; the working tree was not used |
| measured revision (asserted) | `219333b6ab` (`REV` in the tarball). `gpu/rootfind/`, `src/cryptanalysis/ic_framework/` and `src/cryptanalysis/ic_boundary.rs` are byte-identical at `219333b6ab` and `0fe6ff26f8` (`git diff --stat` empty) |
| raw tarball | `raw/out_rm3.tgz`, 225 932 B, sha256 `1742922aa6fdc0af2cebd56b8e89277e5de095690c48ccdb1728b1954314ca95`. This matches `raw/out_rm3.tgz.sha256`, REPORT.md and the benchmark record |
| tarball contents | 109 files. `sessions/{smoke,rf2d,rf3}` hold 28, 60 and 144 records. Also present: `audit-*.{json,log}`, `progress.log`, `daemon.log`, `daemon2.log`, `daemon-start.txt`, `run-*.log`, `table-*.md`, `compare-*.txt`, `build-serve.log`, `test-serve.log`, `sha256.txt`, `nvidia-smi-q.txt`, `nvcc.txt`, `serve-info.txt`, `rf3-amend1.json` and per-record `exec/*.stderr`. There is no `sessions/rf3-refused`: the refused run created no session directory (`audit rf3 rc=2 … session.json: No such file`) |
| session bindings | for all three sessions, `records.jsonl`, `plan.json` and `host.json` hash to `records_sha256`, `plan_sha256` and `host_sha256` in `session.json`. Sequence numbers are contiguous, there are no warm-ups, and every record is L0. The 60 `exec/*.stderr` files hash to their records' `outcome.stderr_sha256` |
| specs | `sessions/smoke/spec.json` and `sessions/rf2d/spec.json` are byte-identical to `specs/smoke.json` and `specs/rf2d.json` at `219333b6ab`. `sessions/rf3/spec.json` is byte-identical to `specs/rf3-amend1.json` at `5011af6f90` (sha256 `36904f0d…`). The amendment differs from `specs/rf3.json` only in the four curve sizes, the label and the description |
| binary | `binary_sha256 = 7ee3fe44…` appears in all 232 records, all three audit receipts and the pod's `sha256.txt`. It also matches the local `target/release/ecbench` (mtime 2026-11-15 02:47:58Z), which I copied and used for the replay |

Scratch work, not committed, is in
`/tmp/claude-0/-home-user/648c3177-6621-55ff-b9d8-09ead07ab678/scratchpad/validator-rm3/`:

- scripts: `scripts/certs.py`, `scripts/pairs_tables.py`, `scripts/band.py`, `scripts/probeset.py`,
  `scripts/replay.sh`;
- outputs: `tables.json`, the replay receipts in `replay/` and the positive control in `mutreplay/`.

## Requirement results

| # | requirement (dispatch item) | result | evidence |
|---|---|---|---|
| 1 | Tarball hash; sessions `smoke` (28), `rf3` (144, under AMENDMENT_1), `rf2d` (60), `rf3-refused` if present, audits, logs | **pass** | `raw/out_rm3.tgz`, `.sha256`; listing above. `rf3-refused` is absent and was never created |
| 2a | All records verified | **pass**: 28/28, 60/60, 144/144 | `sessions/*/records.jsonl` `outcome.status`; `session.json` `status_counts` |
| 2b | Certificates re-verified with independent code | **pass, 232/232** | `scripts/certs.py`: see "Certificates" |
| 2c | Replay audits reproduced | **pass, pod and cross-host** | pod: `audit-{smoke,rf2d,rf3}.json` (28/60/144 replays, 0 problems; same env class as the sessions). Cross-host: see "Cross-host replay" |
| 2d | 128 GPU-vs-CPU pairs equal in S, total GAE, logarithm, status, relations, targets, relation-phase adds | **pass, and stronger than claimed** | 12 smoke + 36 rf2d + 80 rf3 = 128 pairs, matched by (curve, m, \|F_x\|, workload, round), 0 mismatches. My comparison also covers: every phase's adds, doubles, scalar mults and GAE; every native counter except `gpu_*`; every `work_native` counter; `counters`, `cost.unpriced`, `field_ops` and `boundaries`; the `work` block (bitops per phase, reusable and per-target); `fb_sha256`; and `algorithm_seed` |
| 3a | Every number in both REPORT tables | **pass, with one exception**: 3.2× (m = 2, 30.0 bits, daemon vs spawn) is 3.146, so 3.1× | `scripts/pairs_tables.py`, `tables.json`; also matches `results.json` in all 193 compared values |
| 3b | Ranges quoted in prose | **pass, except one range**: "0.010–0.016 s" for the device relation phase (finding 2). The "costs as much as the sweep" statement is also not supported (finding 3) | see "Confirmed numbers" and findings 2–7 |
| 3c | Daemon start 0.376 / 0.383 / 0.291 s | **pass** | `daemon.log` (context ready 0.376 s, listening 0.377 s); `daemon-start.txt` (0.383, 0.003 for the aborted try, 0.291); `daemon2.log` (0.281 / 0.282 s) |
| 4a | Specs committed and pushed before the run | **pass** | `219333b6ab` committed 02:39:29Z. GitHub activity shows it pushed at 02:39:45Z as the branch creation. The pod's first command ran at 02:48:43Z (`nvidia-smi-q.txt`), and the first session started at 03:03:25Z |
| 4b | AMENDMENT_1 committed before any rf3 record | **pass** (but see finding 8) | `5011af6f90` committed 03:09:46Z and pushed 03:09:49Z. The rf3 session started at 03:10:48.368Z (`session.json` `started_unix_ms`); the first record is 03:10:48.371Z |
| 4c | Benchmark record on a clean commit | **pass** (with process gaps) | `benchmarks/records/20261115T0239Z-ic-gpu-rootfind-m3.json`: `commit 219333b6ab…`, `commit_dirty false`, `created_utc 02:39:51Z`. Gaps: `raw[0].uploaded false` and `reports.pdf` names `REPORT.md` |
| 4d | The job's CUDA-backend check | **pass** | `job.sh` l.30 checks both the daemon log and `serve-info.txt`; `job_rf3.sh` l.14 checks the daemon log. Every spawn-arm record's `exec/*.stderr` (4 + 24 + 32) names `cuda: NVIDIA A100 80GB PCIe`, and so do all 104 INIT lines in `daemon.log` / `daemon2.log`. The check fired once (03:09:57 ABORT, stale socket) |
| 5a | `rootfind-subtract-gpu:m=3` charged exactly as `rootfind-subtract:m=3` | **pass by reading, conditional on kernel completeness; holds empirically on every pair** | see "Code review" |
| 5b | Can the returned relation differ? | **only if the device misses a probe-set hit** (a false negative). False positives cannot change the relation or the charge | see "Code review"; the positive control shows the replay audit catches dropped hits |
| 5c | Daemon state across connections and runs | **no result-affecting leak found**; timing side effects exist | see "Code review" |
| 6 | Wording | **findings 1–10** | see "Unsupported or imprecise statements" |

## Certificates

`scripts/certs.py` uses its own affine short-Weierstrass arithmetic and imports no repository code.
It ran the following checks on 8 curves:

- `a` and `b` derived from two of the curve's three points (the generator and two targets), and the
  third point checked to lie on the curve;
- the curve nonsingular, and `j` equal to field 4 of the ICV1 string;
- `p` and `r` prime (deterministic Miller–Rabin), `r = p + 1 − t = #E`, cofactor 1;
- `r·G = O`.

It ran these checks on each of the 232 records:

- `recovered·G = Q`, `planted·G = Q` and `recovered = planted`;
- the planted scalar recomputed from the target law: `1 + derive_u64("uniform_scalar_sha256_v1",
  [model, r, target_seed, index]) mod (r − 1)`, with SHA-256 of canonical JSON and the model hash
  taken from the ICV1 suffix. It matches in every record.

Result: **0 failures out of 232**.

## Cross-host replay

The exact `ecbench` binary (`7ee3fe44…`, copied from `target/release/ecbench`) re-ran
`ecbench verify --replay-all --exit-code` on copies of the three sessions. The host was this
4-vCPU Intel Xeon VM. Its auditor env and hardware classes, `ECBENV3hb516f42f4ab3` and
`ECBHW1he9e33daae9e7`, differ from the pod's `ECBENV3h5b7f4ffbe683` and `ECBHW1h0ae8d56da942`.

The GPU-arm records were replayed through a host-emulation worker that I built from the
`gpu/rootfind` sources at `0fe6ff26f8` (identical to `219333b6ab`) with `make serve test`. The tests
reported "ALL PASS". The worker's sha256 is `cd7b6226…`, against the pod's CUDA worker `d35e52d6…`.
Spawn arms found the worker beside `ecbench`. Daemon arms connected to a
`rootfind_serve --daemon` started beside it (log in `replay/daemon-local.log`).

| session | records | replays reproduced | detail | audit checks | receipt sha256 (prefix) | time |
|---|--:|--:|---|---|---|---|
| smoke | 28 | 28 | all "identical" | 18/18 true (incl. `spec_id`, `plan_hash`, `records_hash`, `host_hash`) | `a3174d28…` | 1 s |
| rf3 | 144 | 144 | all "identical" | 18/18 true | `7a9d1ad2…` | 8 min 54 s |
| rf2d | 60 | 60 | all "identical" | 18/18 true | `8dba1097…` | 8 min 06 s |

**232 of 232 reproduced, 0 problems.** The local daemon served 52 INITs: 8 smoke, 32 rf3 and 12
rf2d daemon records. The receipts are scratch outputs (`replay/audit-{smoke,rf3,rf2d}-local.json`)
and are not committed.

This also shows that the counted records do not depend on the worker build or the backend, which
is the intent of the design. It shows as well that a `-gpu` record can be produced with the sweep on
the CPU emulation backend, and nothing in the record shows which backend ran. The run's CUDA
evidence is the stderr files and the daemon logs, not the records.

**Positive control (mine).** I built a worker with one change in probe mode: it drops every hit with
`j mod 7 = 3` (`mutreplay/`, worker sha256 `5ca3551f…`). Replaying `smoke` through it fails, and the
audit exits 1:

- all 8 m = 3 GPU records (seq 4, 5, 11, 12, 15, 16, 22, 23) report "differs in total_gae, phase
  counts, phase gae";
- the 4 m = 2 daemon records still reproduce, because the mutation touches probe mode only.

So the replay audit and the CPU-pair equality both detect a kernel that misses probe hits.

## Confirmed numbers (recomputed from the records)

Medians are over the 4 runs (2 targets × 2 rounds) per arm and curve. A ratio is median over
median. Solve time is `time.solve_wall_ns`. Set-up is the `oracle_setup` phase's `wall_ns`, and
"relations" is the `relations` phase's `wall_ns`. S/rho is the IC arm's mean S over the `rho-neg`
arm's mean S on the same curve. The IC S is identical on every arm.

**m = 2, |F_x| = 4096 (rf2d)**

| log2 r | mean S | rho mean S | S/rho | CPU solve | spawn solve | A/A copy | spawn set-up | daemon solve | daemon set-up | daemon vs CPU | daemon vs spawn |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 27.485 | 6456.9 | 1.1044 | 5846.6 | 2.1699 | 0.3671 | 0.3222 | 0.2081 | 0.1046 | 0.0044 | 20.750 | 3.510 |
| 29.985 | 13965.1 | 1.2258 | 11392.7 | 9.6714 | 0.3822 | 0.3933 | 0.2088 | 0.1215 | 0.0022 | 79.613 | **3.146** (report: 3.2) |
| 31.262 | 24151.4 | 1.0880 | 22197.8 | 27.3644 | 0.4241 | 0.5058 | 0.1557 | 0.2142 | 0.0043 | 127.775 | 1.980 |

**m = 3 (rf3)**

| log2 r | \|F_x\| | mean S | rho mean S | S/rho | CPU solve | CPU set-up | daemon solve | daemon set-up | daemon vs CPU | relations CPU → daemon | relations ratio |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 25.197 | 1024 | 183.32 | 1.0260 | 178.7 | 0.1072 | 0.0845 | 0.2466 | 0.2147 | 0.435 | 0.0133 → 0.0213 | 0.62 |
| 27.825 | 1024 | 80.68 | 0.4532 | 178.0 | 0.1165 | 0.0837 | 0.2320 | 0.2080 | 0.502 | 0.0189 → 0.0097 | 1.96 |
| 29.170 | 1024 | 64.93 | 1.2533 | 51.8 | 0.1351 | 0.0839 | 0.2589 | 0.2287 | 0.522 | 0.0304 → 0.0099 | 3.07 |
| 31.193 | 1024 | 79.76 | 1.8637 | 42.8 | 0.2142 | 0.0836 | 0.2490 | 0.2153 | 0.860 | 0.1068 → 0.0123 | 8.70 |
| 25.197 | 2048 | 689.88 | 1.0260 | 672.4 | 0.5074 | 0.4503 | 1.2252 | 0.9480 | 0.414 | 0.0311 → 0.2626 | 0.12 |
| 27.825 | 2048 | 280.21 | 0.4532 | 618.3 | 0.5013 | 0.4317 | 1.0757 | 0.9972 | 0.466 | 0.0370 → 0.0490 | 0.75 |
| 29.170 | 2048 | 178.98 | 1.2533 | 142.8 | 0.5168 | 0.4302 | 1.0613 | 0.9935 | 0.487 | 0.0447 → 0.0291 | 1.54 |
| 31.193 | 2048 | 99.75 | 1.8637 | 53.5 | 0.6025 | 0.4322 | 1.0955 | 0.9942 | 0.550 | 0.0765 → 0.0158 | 4.83 |

Every REPORT cell rounds from these, except the 3.2× noted in the m = 2 table. The prose ranges were
checked as follows.

- **Set-up growth.** Daemon minus CPU set-up is 0.124–0.145 s at 1024 and 0.498–0.566 s at 2048. The
  report's 0.12–0.15 s and 0.50–0.57 s are confirmed.
- **Probe sets.** The worker logs give 1 020 568, 1 044 099, 1 046 726 and 1 048 117 at 1024, and
  3 768 907, 4 121 822, 4 165 129 and 4 187 100 at 2048. The report's 1.02–1.05M and 3.77–4.19M are
  confirmed. I also recomputed all eight independently (`scripts/probeset.py`). The script counts the
  distinct abscissae of the negation-folded pair sums over the smallest-|F_x| abscissa base, and all
  eight values match the worker logs exactly. That confirms what `PairTable::map_abscissae` sends to
  the device.
- **Relation-phase speedups.** At 1024 and log2 r ≥ 27.8 they are 1.96–8.70×. At 2048 and
  log2 r ≥ 29.2 they are 1.54–4.83×. The report's 2.0–8.7× and 1.5–4.8× are confirmed.
- **Relation-phase times.** "0.010–0.016 s" is not confirmed; see finding 2.
- **Spawn A/A spread.** The spawn arm's A/A copy differs by −12.2%, +2.9% and +19.3%, which confirms
  "3–19%".
- **CPU A/A.** The `rf3-cpuaa-F1024` solve medians differ by −0.6%, +0.9%, −1.3% and +1.7%
  (relations 0.4–1.1%, set-up 0.4–1.9%). That is within 2%; see finding 5.
- **Solve ratios.** Daemon vs CPU is 0.414–0.860 and spawn vs CPU 0.208–0.441. The report's
  0.41–0.86× and 0.21–0.44× are confirmed.
- **Distance to rho, wall time.**
  - The rho medians come from the `rho-neg` arm's `solve_wall_ns`: 0.600, 0.742, 3.275 and 9.046 ms
    (rf3), and 1.452, 4.886 and 4.863 ms (rf2d).
  - The m = 3 daemon arm is 27.5–2040.5× rho's median solve, and the m = 2 daemon arm is
    24.9–72.0×. The report's 28–2 040× and 25–72× are confirmed.
- **Distance to rho, S.** m = 3 is 42.8–672.4× and m = 2 is 5846.6–22197.8×, at w = 0. These
  confirm the report's numbers, but see finding 4.
- **Benchmark record metrics.** All four metrics and the summary line match: 0.214 / 27.364 /
  0.00486; 0.004 / 0.156; 0.249 / 0.214 / 0.00905; and 79.8 / 1.86.

**§5a band (my computation; not stated in the report).** I used `priced_S(w) = (G + w·U)/√r`, where
U is the counts behind `cost.unpriced` plus the counted `table_insert_attempts` (the default reading
since 2026-11-08). The IC/rho ratio at w = 0, 0.1 and 1 is:

- m = 3, |F_x| = 1024: 178.7/184.2/209.7, 178.0/213.0/374.9, 51.8/67.7/139.5 and 42.8/48.6/74.6;
- m = 3, |F_x| = 2048: 672.4/677.1/698.9, 618.3/644.0/762.9, 142.8/157.7/225.4 and 53.5/80.8/204.0;
- m = 2: 5847–5919, 11393–11449 and 22198–22279.

Rho is ahead at every weight. The order holds across the band.

## Code review (claim 5)

I read these at `0fe6ff26f8` (identical to `219333b6ab`):

- `src/cryptanalysis/ic_framework/rootfind_gpu.rs`;
- `RootfindSubtractOracle` in `plugins.rs` (`decompose`, `lookup`, `charge_sweep_setup`,
  `charge_candidate`, `sweep`, `pair_abscissae`);
- `PairTable::{build_negation_folded, map_abscissae, probe, get}` and `PrimeCurve::key` in
  `ic_boundary.rs`;
- `gpu/rootfind/{rootfind.cuh, rootfind_api.cpp, rootfind_cuda.cu, rootfind_serve.cpp,
  rootfind_host.hpp}`.

**The charge is identical by construction.**

- Before the first candidate, `charge_sweep_setup` charges what CPU `decompose` charges: +2 muls, one
  inversion, 3k muls, and the `sweep_field_*` tallies.
- A candidate the device does not report costs, on the CPU m = 3 path, `charge_candidate`
  (1 add, 4 muls, 1 sqr) plus `PairTable::probe`'s `lookups += 1`, and `get` then returns `None`
  with no group operation. That is exactly `charge_misses(1)`.
- A reported candidate goes through `charge_candidate` and then the CPU oracle's own `lookup`, which
  is the same function with the same charges.
- After the last reported position, the remaining `2k − next` positions are charged as misses.
- A target whose abscissa is in the base (the degenerate rows) is handed whole to the CPU oracle.
  That is equivalent to the CPU's per-row `degenerate[i]` handling, because the reps cover
  `abscissa_list`.
- The candidate order matches. ε = +1 (`λ = (y_R + y_i)/(x_R − x_i)`, summand `rep.plus`) is the even
  position in both the kernel and the CPU loop.

**The probe-set semantics are right.**

- `PrimeCurve::key` is `((x + 1) << 1) | sign`, so `map` keys `>> 1` are `x + 1`, and `map_abscissae`
  returns exactly the abscissae under which `get` succeeds.
- The kernel compares full 62-bit Montgomery values (`F::eq`) in probe mode, so key truncation cannot
  cause a false negative.
- `RootfindSubtractOracle` always builds the plain-map table. `map_abscissae` would return `None` for
  the blocked, packed or line layouts, and `prepare` would then fail closed.
- My independent recount of all eight probe-set sizes confirms this on the measured workloads.

**When can the returned relation differ?** Only when the device fails to report a position where
x(R ∓ F_i) is a pair-table abscissa, i.e. on kernel incompleteness. The oracle would then charge
that position as a miss and return a later relation, or none.

- **False positives are harmless.** A reported position that is not a hit costs
  `charge_candidate` + `lookups += 1`, which is the same as a miss, and returns nothing.
- **Duplicates and order are handled.** Hits are sorted per target, and `pos < next` skips repeats.
- **A batch that overflows the relation buffer is split.** The buffer cap is 4·4096 + 4096 = 20 480.
- **One target that overflows panics**, so it fails closed and is never silent.
- **Completeness rests on the Eytzinger lower bound** (`lookup=1` for every m = 3 session here) and
  on the packed hash for small sets (smoke). Both are covered by `test_cpu`'s lookup checks (47 618).
  Empirically, it held on all 72 m = 3 GPU-vs-CPU pairs (64 in rf3, 8 in smoke). The positive
  control above shows that a miss would surface both as a pair mismatch and as a replay failure.

**Daemon state.** I found no result-affecting state that leaks across connections or runs.

- `serve()` keeps `rf64_session *s`, `cap` and the buffers as locals of one connection.
- Every INIT destroys and recreates the session; an empty probe set gives `probe_x = nullptr`, so
  probe mode is off.
- End of input destroys the session.
- There are no static tables in `rootfind_api.cpp`, `rootfind_host.hpp` or `rootfind_cuda.cu`. The
  only static state is `rf_backend_info`'s string and the profile counters, which `RF_PROFILE=0`
  compiles out.
- On the Rust side, `prepare` always re-INITs and clears `swept`, `pending` and `counters`. The
  connection is thread-local, one per process under ecbench, and a mode change drops it.

What does persist across connections is process-global:

- **The CUDA context.** A sticky CUDA error in one connection would fail every later connection.
  That is fail-closed, not a wrong result.
- **Lazy kernel loading on the first launch.** The first record after each daemon start pays about
  0.055 s more in its relation phase: smoke seq 2 had 0.0615 s against 0.0043–0.0058 s for later
  daemon records, and rf3 seq 3 had 0.0766 s against 0.0207–0.0217 s. This is a timing leak into
  the first record, not a count leak.
- **The serial accept loop.** A client that holds its connection blocks every other client. Two
  concurrent ecbench processes would serialise there, which affects timing only.

Minor: the per-INIT log line prints `rf_backend_info()`, which reports device presence, not the
session's resolved backend. Under `RF_BACKEND_AUTO` the two conditions are the same.

## Unsupported or imprecise statements (claim 6)

1. **"daemon vs spawn 3.2×" (table (3), 30.0 bits).** The median ratio is 0.3822 / 0.1215 = 3.146,
   so 3.1×. The 3.2 comes from dividing the rounded cells, 0.382 / 0.121.

2. **"The device takes the relation phase down to 0.010–0.016 s where it is largest (2.0–8.7× at
   log2 r ≥ 27.8, |F_x| = 1024; 1.5–4.8× at ≥ 29.2, 2048)".**
   - In the cells quoted, the daemon's relation-phase medians are 0.0097, 0.0099, 0.0123, **0.0291**
     and 0.0158 s, so the range is 0.010–0.029 s, not 0.010–0.016 s.
   - The prose also omits that the device relation phase is slower than the CPU's in 3 of 8 cells:
     0.62× at 25.2 bits / 1024, 0.12× at 25.2 / 2048 and 0.75× at 27.8 / 2048. At 25.2 bits / 2048
     every device run splits its batches 126 times (`gpu_batch_splits`) because the dense pair table
     overflows the relation buffer. The cause is the splits, not the set-up.

3. **"the pair table (≈ |F_x|² additions, CPU, charged, the same on both arms) costs as much as the
   sweep".** On the CPU arm the pair table costs 0.4–51× the relation phase in GAE: 1 049 600 or
   4 196 352 set-up GAE, against 81 836–2 909 510 relation GAE. In wall time it costs 0.8–14.5×. It
   costs about as much only at 31.2 bits, |F_x| = 1024. Everywhere else the table dominates; at
   2048 it is 5.6–51× in GAE.

4. **"S is 43–672× rho against m = 2's 5 800–22 000×" and "m = 3 is the right method to
   accelerate".**
   - The ratios are at w = 0 only. The §5a band is not stated; it holds, with m = 3 at 42.8–762.9×
     and m = 2 at 5 847–22 279× across w = 0, 0.1 and 1.
   - The reference is a 4-run rho mean whose per-curve values vary by a factor of 4.1 (0.453–1.864
     against an expected ≈ 0.89–1.3). The jump from 178× at 27.8 bits to 52× at 29.2 bits, for
     example, comes mostly from rho's mean S moving 0.45 → 1.25, not from the IC.
   - The m = 2 and m = 3 rows are on different curves (seeds 20261114 and 20261115) and different
     |F_x|.
   - "the right method to accelerate" is an interpretation and should be labelled as one. The data
     support the narrower statement: at log2 r ≈ 31.2–31.3, m = 3 has 240–300× less counted S than
     m = 2 with |F_x| = 4096.

5. **"CPU arm A/A within 1–2% (`rf3-cpuaa-F1024`)".** The per-workload solve medians differ by
   0.6–1.7%, so "within 2%" is supported. But this covers |F_x| = 1024 only. There is no A/A arm for
   the m = 3 GPU arms or for any daemon arm. The daemon arm's own per-run spread is larger (rf3
   25.2-bit / 1024: solve 0.217–0.316 s, relations 0.021–0.077 s) because of the first-connection
   effect in the code review. The m = 3 conclusions do not depend on this: the CPU and daemon
   per-run ranges are disjoint in every cell.

6. **"The per-solve set-up falls from 0.16–0.25 s to 2–4 ms".**
   - These are per-workload medians of the spawn arm and its A/A copy (0.156–0.251 s) and of the
     daemon (0.0022–0.0044 s). Individual runs range 0.151–0.305 s and 0.0021–0.0055 s.
   - The daemon also cuts the relation phase by 0.054–0.058 s per solve (0.076 → 0.022, 0.114 →
     0.059 and 0.201 → 0.144 s), which is consistent with the per-process first-kernel-launch cost.
     That cause is a hypothesis. So the 2.0–3.5× is not set-up alone.

7. **The daemon's one-time cost: "a one-time host cost outside every record", "paid once per host,
   not per solve", "The daemon's one-time 0.38 s equals about two spawned solves' set-up".**
   - In this run a daemon was started three times: 0.383 s; 0.003 s for the aborted try; 0.291 s.
     The cost is per daemon start, not per host.
   - Part of the one-time CUDA warm-up lands inside the first record after each start, about
     +0.055 s in its relation phase (see the code review).
   - "CUDA context ready 0.376 s after launch" is the daemon's own clock from its start. The 0.383 s
     is the job shell's clock from before launch to the socket file. Both are as logged.

8. **`AMENDMENT_1.md`: "Amendment 1 (2026-11-15 03:12Z, before any rf3 record existed)" and
   "Choosing the smaller sizes was made without seeing any m = 3 result".**
   - **The timestamp.** Git shows the amendment committed at 03:09:46Z and pushed at 03:09:49Z. The
     rf3 session ran 03:10:48–03:12:03Z. The time in the heading would therefore place the amendment
     after the rf3 records, contradicting its own claim. The claim itself (before any record) is
     true by the git and push evidence; the heading time is wrong.
   - **"Without seeing any m = 3 result".** The smoke session contained m = 3 arms (`rf3-cpu`,
     `rf3-gpu`, `rf3-gpud` on a 20-bit curve). It completed at 03:03:28Z, six minutes before the
     amendment. Whether the author looked at it cannot be established, so this statement is
     unverifiable as written.
   - The choice of sizes was largely forced: `prime_search` stops at 32 bits, and 30 and 32 were
     kept. The direction of any size effect is acknowledged in REPORT ("at these sizes").

9. **Run section: "The first try of `job_rf3.sh` aborted on a stale socket file … no record was
   taken".**
   - True. But the first-try script exits on ABORT without killing the daemon it had started
     (`job_rf3.sh` at `5011af6f90`: `exit 1` with no `kill $DPID`). That daemon would have unlinked
     the stale socket and bound its own. The second try then ran `rm -f` on the socket path, which
     left that daemon unreachable. Unless the end of the launching exec session killed it, it stayed
     alive and idle, holding a CUDA context on the A100, through the rf3 session. This is
     undisclosed. Being idle and unreachable, it cannot change counts and is unlikely to change wall
     time, but it is an uncontrolled process during the measurement.
   - The fixed `job_rf3.sh` (`4961ee3cd1`) was committed at 03:10:44Z but pushed only at 03:11:41Z,
     after the rf3 session started. The script that ran was therefore not pushed before the run. The
     report does not claim it was, and the spec freeze is unaffected, but it should be stated.

10. **"Binaries: `ecbench` built from `219333b6ab` on a clean tree (`REV`), `rootfind_serve_cuda`
    built on the pod from the same tree".**
    - The binary hash is bound everywhere (see the table above). Its source revision and clean-tree
      state are not: `session.json` has `git_commit: null` and `git_dirty: null`, `host.json` says
      `commit ? (dirty: None, source: ?)`, and the binary embeds no revision.
    - The rootfind sources copied to the pod are not hashed in the tarball.
    - Both statements are therefore unverifiable from the artifacts. What is verifiable: the local
      `target/release/ecbench` with the same hash has mtime 02:47:58Z, 8.5 min after `219333b6ab`
      was committed, and the rootfind sources are identical at `219333b6ab` and `0fe6ff26f8`.

**Smaller points.**

- The figure says "A100 + one CPU core". The CPU arms are single-threaded
  (`RAYON_NUM_THREADS=1`) but unpinned (`--cpus none`) on a 252-thread host.
- "Counted S is unchanged by design" is true. The device's extra work is outside S and outside
  the §5a band, because the `gpu_*` counters are not in `cost.unpriced`:
  - full sweeps;
  - 6–999 unused swept lane targets per m = 3 run (`gpu_targets_unused`);
  - up to 126 batch splits per run.

  That is a stated accounting choice of the oracle. The report should name these counts, as the
  previous review asked.

## Process and reporting-contract gaps (not errors in the numbers)

- **Raw data are not in S3.** The benchmark record says `uploaded: false`, and I could not check the
  bucket (HTTP 403 on `head-object`). The 225 932-byte `.tgz` is committed to Git. That is an archive
  over 64 KiB, which the artifact-store policy (AGENTS.md §13) routes to S3 with a manifest.
- **The sessions are inside the tarball, not committed loose** under the research directory, so CI's
  cross-host session re-audit (AGENTS.md §12) does not see them. The pod's own replay audits ran on
  the same host class as the sessions. The cross-host replay above is the independent one.
- **The record's `reports.pdf` names `REPORT.md`,** not `REPORT.pdf`.
- **Wall time is L0.** Every record is L0, with blocker "L1: no CPU reservation", and wall time is
  read without an L2 admission. The report discloses L0. Host contention was low: PSI avg10 ≤ 0.09
  and run delay ≤ 2.1 ms per solve.
- **The report omits ecbench's own wall verdicts.** Every committed `compare-*.txt` reports
  "measurement quality: 0 / 100 … admitted 0". Its pooled wall ratios agree in direction with the
  report's medians:
  - daemon / spawn (m = 2): 0.354 [0.290, 0.431];
  - m = 3 daemon / CPU: 1.73 [1.47, 2.05] at 1024 and 2.11 [1.97, 2.25] at 2048;
  - m = 3 spawn / CPU: 3.69 [3.08, 4.43] and 2.66 [2.50, 2.84];
  - CPU A/A: 1.020 [0.983, 1.059].
- **The §5a band and the calibrated weight `w_cal` are not stated.** The band holds as computed
  above.
- **The leaderboard and gate are respected.** No leaderboard or `benchmarks/README.md` row was added
  (PR file list checked). This is consistent with the validation gate, since no `VALIDATION.json`
  exists.

## Not checked or not checkable

- The pod id, cost, deletion and the "02:48Z–03:14Z" window. The first artifact is
  `nvidia-smi -q` at 02:48:43; the last log line is 03:13:17Z.
- The "Checks before the run" (whole-pipeline equality tests on four curves of 14–20 bits; the
  positive control "4 545 against 4 579"; the local smoke 28/28). No log of them is committed. The
  test code exists and matches the description (curves of 14, 17, 18 and 20 bits). My record-level
  positive control and cross-host replay stand in for them partly.
- Whether the pod's `rootfind_serve_cuda` was built from the committed sources. Its hash is
  recorded, but the sources and build inputs on the pod are not. The CUDA kernel's completeness on
  the device is evidenced only by the pair equalities and the pod replays, not by an independent
  device run.
- Whether the orphaned first-try daemon was in fact alive during rf3. This is inferred from the
  script; no process list was captured.
- Whether the A100 measurements reproduce. I have no GPU; the cross-host replay checks counts, not
  wall time.
- S3 contents (access denied).
