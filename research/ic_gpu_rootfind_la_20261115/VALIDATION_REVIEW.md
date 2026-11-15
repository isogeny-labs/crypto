# Validation review: `research/ic_gpu_rootfind_la_20261115/REPORT.md` (round 5)

Validator, 2026-11-15. I did not write the code, the protocol, the specs, the job or the report.
Standing of this review: `[pcc:unverified]`. No pcc claim is registered for it.

**Disposition: `validated-scoped`.**

Every certificate holds, the identity claim holds, and every number in the REPORT table and in the
listed ranges recomputes from the raw records with my own code. The §5a band values match both
`ecbench band` and my own re-implementation of the `band.rs` rule. All 88 worker-free records (rho and
the CPU arms) replay identically on another host with a binary built from a clean commit. No
measured number is refuted.
The scope is narrower than the report's wording in places. The CPU solve ratio is attributed to the
solver change beyond what the data support (finding 3). The code at the measured commit differs from
the protocol commit in `src/`, and the report does not say so; the change does not touch the measured
path (finding 1). The `ecbench` binary that ran is not bound to any commit (finding 2). The rest are
wording, disclosure and record-keeping gaps (findings 4–13). The GPU records cannot be replayed
without a CUDA device, so their walls rest on the pod's own same-host audit. [pcc:unverified]

## What was validated

| item | value |
|---|---|
| validated objects | `219b1378f80c4db37c8b7dabca19bfa97c1ea9f9`, the branch head at dispatch, read with `git archive`. The REPORT validated is that commit's `REPORT.md` (blob `9dffac1f08`, sha256 `6325c02e…`). The corrections committed afterwards at `3767aaca3f` were not validated here. The study directory and the three records are byte-identical from `e673b23fc4` to `219b1378f8`. The remote branch later moved to `d71bc17b0f` (push 20:20:35Z); per the GitHub compare API, that change touches neither the study, the records, `gpu/rootfind` nor `ic_framework` |
| measured revision | `8c5ae6a512` (`REV` in the tarball). GitHub push activity: `5d618524e2` (protocol, specs, `job.sh`) at 17:07:15Z, `8c5ae6a512` at 18:56:53Z, `2f89e87c07` (record only) at 18:57:19Z. Command 3 of record r2 started 18:57:03Z, so `REV` was taken at `8c5ae6a512` |
| raw tarball | `raw/out_rla.tgz`: 145 432 B, sha256 `f1f774f6c18f2da3f3524e577d67b6de4bda716ddc56ce1c37ff581cafd08c34`. This equals `raw/out_rla.tgz.sha256` and the `raw` entry of records r2 and r3. It has 43 entries, all regular files or directories, with no links and no absolute or `..` paths. I extracted it into a fresh scratch directory |
| sessions | `smoke` (`ECBS1h7e02b5ff0c2a`) holds 36 records and `rf3la` (`ECBS1h5d935fcbaf36`) holds 132; every record has status `verified` [pcc:unverified]. `records.jsonl`, `plan.json` and `host.json` hash to the values in `session.json`. Sequence numbers are contiguous, there are no warm-ups, every record is L0, and every record and receipt carries one binary, `0742b939c210…`. `git_commit` and `git_dirty` are `null` in both sessions |
| specs | `sessions/{smoke,rf3la}/spec.json` are byte-identical to `specs/{smoke,rf3la}.json` at `5d618524e2`, `8c5ae6a512` and `219b1378f8` (sha256 `6acb1f10…`, `660123af…`). `job.sh` is unchanged since `5d618524e2` |
| pod audits | `audit-{smoke,rf3la}.json`: `ok`, 0 problems, 36/36 and 132/132 replays `identical`. The auditor was the same binary on the same hardware class (`ECBHW1hf98eb466ccc1`), so these are same-host replays |
| pod environment | `lscpu`: AMD EPYC 7543, 128 logical CPUs. `cgroup-cpu.txt`: no `cpu.max`; cgroup v1 quota 1360000 / period 100000 = 13.6 CPUs. GPU: A100 80GB PCIe, sm_80. `nvcc` 12.8. `daemon.log`: backend `cuda`, context ready 0.225 s; `daemon-start.txt`: 0.242 s. `build-serve.log` rc=0; `test-serve.log` ALL PASS (host-only `test_cpu`, built without CUDA) |
| daemon log | 160 `timing init` lines, which is 2 × (20 smoke + 60 rf3la daemon-worker records), one for the run and one for the replay of each. All 120 rf3la lines are `lookup=4` (pairs) on the cuda device: 72 at `n_fb=4096` and 48 at `n_fb=2048`, as the arms require |

## Requirements

| # | requirement | result | evidence |
|---|---|---|---|
| 1a | Protocol and specs pushed before the run | **pass** | GitHub push 17:07:15Z. The pod's first files are from 18:58 (`REV`, `cgroup-cpu.txt`, `test-serve.log`), and the first progress line is 19:16:30. `5d618524e2` is an ancestor of the #4310 merge `e0e3725bac` |
| 1b | Session specs equal the committed specs | **pass** | sha256 equal (table above) |
| 1c | Tarball hash | **pass** | sha256 above |
| 1d | Measured commit differs from the protocol commit only in `run_pod.sh` | **does not hold as stated; no effect on the measured path** | Finding 1. `gpu/` and `job.sh` are unchanged. `src/` changes in two files |
| 1e | Binary bound to the measured commit | **not verifiable** | Finding 2 |
| 2a | Certificates, all 168 records, re-checked with my own EC arithmetic [pcc:unverified] | **pass, 168/168** | `certs.py` (own affine arithmetic). It checks: p and r prime; G and Q on the curve; r·G = O; recovered·G = Q; recovered = planted mod r; the outcome flags. For the smoke curve, a and b are solved from G and Q. A tampered copy (recovered + 1; Q moved off the curve) fails as it should |
| 2b | Curve orders | **pass, 3/3** | r prime, r·G = O, t² ≤ 4p and r > 4√p, so #E = r. The trace and j-invariant match ICV1 fields 2 and 4 on all three curves (20, 38, 40 bits) |
| 2c | Planted scalars | **pass, 168/168** | Re-derived from `uniform_scalar_sha256_v1` (model hash, r, seed, index) with my own SHA-256 code, on all 8 workloads |
| 2d | Replays | **same host 168/168; cross-host, worker-free: smoke 16/16, rf3la 72/72 `identical`** | My replays used a different binary, `0afa6c10…` (built from a clean `219b1378f8`), on another hardware class (`ECBHW1hd21b654a34b7`). They covered every rho, rho A/A and CPU IC record: rf3la `rho-neg`, `rho-neg-aa` and the four CPU arms, 12 each. Both receipts are `ok` with 0 problems. GPU records were not replayed (no device) |
| 3a | Identity: every IC arm against `rf3-cpu-gauss` per (workload, round) | **pass, 84 checks, 0 mismatches** | `check.py`. The checked fields are trials, relations, matrix rank, rows and dependents, recovered, planted, status, S, total GAE, `cost.unpriced`, the factor-base hashes and the algorithm seed. Every phase's adds, doubles, scalar mults and GAE are included. So is every `native` and `work_native` counter outside linear algebra, except the `gpu_*` diagnostics. Keys are indexed, so a missing key fails |
| 3b | What differs inside linear algebra | **two counters** | Echelon against gauss differs only in `native.row_ops` and `work_native.gauss_entries_scanned`. The top-level `matrix_work` mirrors `row_ops`. `pivot_inversion_steps`, `row_build_*`, `rank`, `rows`, `columns` and the charged LA fields (all 0) are equal. The same solver gives identical LA maps on the CPU and GPU arms and on the A/A copy (0 mismatches) |
| 3c | Smoke identity | **pass, 20 checks** | Every daemon arm against its CPU arm (rf3: mt, gpud, gpuh, gpup; rf2: gpud), with the LA maps included. Smoke runs echelon only, so it does not compare the two solvers |
| 4 | Every number in REPORT.md | **pass for the table and ranges; findings 3–8 on wording** | "Confirmed numbers" below |
| 5 | Process and wording | **findings 9–13** | Below |

## Confirmed numbers

Recomputed from `records.jsonl` by `check.py`. Medians are over 6 runs per cell (3 targets × 2
rounds). Ratios are ratios of medians unless marked per pair.

- **Table, all 40 cells hold to the printed digit.**
  - S/rho: 340.38, 194.25, 737.48 and 383.21. These are the band's mean S at w = 0 over rho-neg's
    mean S (0.8577 and 0.7703).
  - CPU gauss solve: 5.6565, 8.0875, 20.168 and 20.021 s. CPU echelon: 5.3862, 6.4029, 19.715 and
    17.780 s.
  - GPU gauss: 0.2873, 1.5512, 0.38745 and 1.5412 s. GPU echelon: 0.1184, 0.2804, 0.2234 and
    0.3053 s.
  - GPU echelon/gauss: 0.4122, 0.1808, 0.5766 and 0.1981. CPU gauss over GPU echelon: 47.76, 28.84,
    90.28 and 65.58×.
  - GPU echelon set-up / relations / LA: 0.0382 / 0.0549 / 0.0226, 0.1215 / 0.0368 / 0.1174,
    0.0324 / 0.1622 / 0.0240 and 0.1212 / 0.0765 / 0.1037 s.
- **Ranges hold.**
  - Echelon/gauss `row_ops` is 0.1292–0.1420, identical on the CPU and the GPU (per pair
    0.121–0.149).
  - The A/A per pair is 0.9756–1.0714 (n = 12).
  - The CPU solve ratio is 0.7917–0.9776 (see finding 3 for what it measures).
  - Rho's median solve is 0.03974 s and 0.08662 s. The rho A/A per pair is 0.937–1.012 and
    0.987–1.005, with identical counts.
  - GPU echelon against rho in wall time: 2.980, 7.057, 2.579 and 3.524×, printed "2.6–7.1×". The
    example 0.223 against 0.087 s holds.
  - Prediction 2 holds (0.18 and 0.20 ≤ 0.5), and so does prediction 3 (0.129–0.142 inside
    0.10–0.20) [pcc:unverified].
  - "What bounds the solve" holds: at 4096, set-up 0.121–0.122 s, LA 0.104–0.117 s and relations
    0.037–0.077 s. At 39.3 bits and 2048, relations take 0.162 s of 0.223 s.
- **§5a band.**
  - `ecbench band` (local binary, default reading) reproduces `band.json` exactly (all 22 `mean_s`
    triples). My own implementation of the `band.rs` rule (`cost.unpriced` counts from the
    counters, then from the phase `native` maps; plus the counted `table_insert_attempts`) gives the
    same values under both readings.
  - GPU echelon against rho at w = 0 / 0.1 / 1: 340.4 / 342.0 / 349.5, 194.3 / 206.3 / 260.6,
    737.5 / 738.3 / 742.3 and 383.2 / 389.3 / 416.8. GPU gauss at 4096: 283.6 / 685.9 and
    430.2 / 641.8 at w = 0.1 / 1.
  - `ecbench band --ranks` reports echelon against gauss as "unresolved inside the band" (they tie
    at w = 0) and rho ahead of every IC arm at every weight, as the report says.
  - Under `--no-price-table-work` the ratios move by at most 23.1 at w = 1 (gauss, 37.4 bits, 4096) and
    no ranking changes.
- **Records.** The metric values of r2 and r3 hold: 0.305, 0.28 and 0.223 against baselines 1.541,
  1.551 and 0.387 and references 0.0866, 0.0397 and 0.0866. So does the r3 summary.
- **Figure.** `fig_la_gpu.svg`'s labels (CPU 5.7, 8.1, 20.2 and 20.0 s; the gauss and echelon solve
  values) match the table. Its phrase "cuts linear algebra about tenfold" is a GPU LA wall ratio of
  7.9–12.9×. `REPORT.pdf` carries the same text as `REPORT.md`.

## Findings

1. **The measured commit differs from the protocol commit in `src/`, and the report does not say
   so.** `git diff 5d618524e2 8c5ae6a512` changes:
   - `src/cryptanalysis/ic_framework/rootfind_gpu.rs` (+234 lines, `2ec4187be0`, pushed 17:34Z). This
     adds `worker=native`. The change is additive for `worker=daemon`: it adds `threads` to the
     parameter whitelist (refused without `native`) and strips it from the CPU oracle's parameters.
     It also resets `self.native = None` in `prepare` and adds a branch in the batch flush that a
     daemon arm never takes. `default_build` is unchanged for daemon arms, and these arms pass
     `probe_build=pairs` explicitly anyway.
   - `src/cryptanalysis/koblitz_index_calculus.rs`: transparent huge-page advice in the binary-field
     `PairSumTable` (`35eb722035`, from `origin/main` via #4316 `a6b170b388`, on which the branch
     was restarted at `764d560062`), and a test-only clippy rewrite (`036f1bc776`). The prime `rootfind-subtract` path does not use `PairSumTable`, and the advice
     never changes contents.

   `gpu/rootfind`, `job.sh` and the specs are unchanged, and `analyze.py` was added before the run
   (`5ae142b07c`, 18:55:42Z). By reading the code, none of these changes touches the measured arms.
   The report should still name them beside "`8c5ae6a512`, measured commit".
2. **The binary that ran is not bound to a commit.**
   - `run_pod.sh` copies `target/release/ecbench` as it stands; it does not build it from `REV`.
     Both sessions record `git_commit: null`.
   - The binary `0742b939…` is not kept anywhere on this host. I searched the repository and every
     `ecbench` copy in the scratchpad. `target/release/ecbench` was rebuilt at 20:10:43Z
     (`0afa6c10…`).
   - The binary differs from the one in the native-sweep sessions recorded at 17:40–17:45Z
     (`0d4e3842…`, clean `2ec4187be0` and `6cac7eb73d`), so it was rebuilt between 17:45Z and
     18:57Z. Record r2 was created at
     18:54:07Z with `commit_dirty: false`.
   - A release rebuild at `8c5ae6a512` to compare hashes was not possible: 3.0 GB of disk was free.
   - Mitigation: a binary built from a clean `219b1378f8` reproduces every worker-free record
     cross-host (requirement 2d). The GPU arms' counts equal the CPU arms' counts.
   - What stays unbound is the GPU arms' wall time and their device code path in that binary.
3. **"On the CPU the same solver change is 0.79–0.98× in solve" attributes more to the solver
   than the data support.**
   - If only the linear-algebra phase changed, the CPU ratio would be 0.968, 0.839, 0.990 and 0.937
     (gauss solve − gauss LA + echelon LA, over gauss solve).
   - The measured ratios are 0.952, 0.792, 0.978 and 0.888. The rest is relation-phase wall on
     identical counted work: medians 0.08, 0.40, 0.24 and 0.89 s apart, the gauss arm slower every
     time.
   - Per pair, the CPU ratio is 0.85–1.03 at 2048 and 0.61–1.12 at 4096. At 39.3 bits it straddles
     1 (0.91–1.03 and 0.87–1.12).
   - No CPU A/A arm was run. The protocol's reading rule reports a wall difference only beyond the
     A/A spread, and the only A/A (GPU echelon) spans 0.98–1.07.
   - On the GPU the measured and LA-only ratios agree to within 0.004, so the GPU reading is sound.
   - Supported reading: on the CPU the linear-algebra phase falls 9–11.5× in wall time (0.207 to
     0.023 s, 1.43 to 0.125 s). The expected whole-solve effect is 0.84–0.99×. The measured
     0.79–0.98× also includes relation-phase noise. [pcc:unverified]
4. **"falls from most of the solve to about a third at |F_x| = 4096".** The GPU echelon arm's LA
   share is 0.419 at 37.4 bits and 0.340 at 39.3 bits (per run 0.383–0.439 and 0.321–0.367).
   Supported reading: a third to two-fifths. The gauss share is 0.89 and 0.87.
5. **"every arm, CPU and GPU, can use `incremental-gauss-echelon` with the same answer and S"** goes
   beyond this round's evidence. Here gauss was compared with echelon on two arms only: `rootfind-subtract:m=3`
   and `rootfind-subtract-gpu:m=3,worker=daemon,backend=cuda,probe_build=pairs`. The comparison
   covered 2 curves × 3 targets × 2 sizes. The smoke session runs every arm with echelon only.
   Supported reading: every arm measured here, plus the arms of `research/ic_la_m3_20261115`.
6. **The wall interval behind "2.6–7.1× slower" is not named.**
   - The GPU solve runs from the factor base through verification (the phase walls sum to it). It
     excludes the daemon's start, 0.242 s, which is paid once per pod and includes the 0.225 s CUDA
     context.
   - The report gives the 0.242 s but does not say it lies outside the solve.
   - A cold single-target reading would be 5.4–13.2× rho's median.
   - The report already says this is not a comparison of equals. The interval still belongs in that
     sentence.
7. **Labels.**
   - The caption "(Medians of 6 runs …)" also covers the S/rho column. That column is a ratio of
     means (the band rule); its values hold.
   - "19:16–19:39Z" is the measurement window (first progress line 19:16:30, done 19:38:39), not the
     pod's life. The pod was running `job.sh` from about 18:58 (file times of `REV`, `cgroup-cpu.txt`
     and `test-serve.log`). Command 3 ran 18:57:03–19:39:26Z (2 542.8 s).
8. **The failed launches.** "each pod deleted at once, nothing ran" is consistent with the evidence
   but not shown by it. Record r2 holds only exit codes 22 and 1 and walls of 28.1 s and 32.7 s;
   there is no log. No tarball file predates command 3, which is consistent. Separately, the comment
   added to `run_pod.sh` in `8c5ae6a512` (committed 18:56:50Z) dates the 404s "18:55-19:05Z", a time
   after its own commit. The two failures started at 18:54:15Z and 18:55:18Z.
9. **The records' commit fields do not name the measured commit.**
   - r2's `commit` is `764d560062`, the commit at record creation. r3's is `1deabdf708`, the later
     IFMA commit at which the analysis ran.
   - `benchmarks/progress/PROGRESS.md` therefore lists the three round-5 metrics under
     `1deabdf70800`.
   - `8c5ae6a512` appears only in r2's summary text and in the tarball's `REV`.
   - r3's command points at a scratchpad directory. The `ecbench band` step that wrote the
     `band-rf3la.json` read by `analyze.py` is not a recorded command.
   - What the report discloses is accurate. `tools/benchrec.py` (`measurement_errors`) refuses
     `measured` for any command with a nonzero exit; r2 is closed censored, and r3's command is the
     analysis. r2's summary names r3 "1955Z" in error, and the report says so.
10. **The record fields are incomplete.**
    - `reports.pdf` in r2 and r3 names `REPORT.md`, not `REPORT.pdf`. The round-big validator raised
      the same point.
    - The metrics' `spread` is `null`, although the A/A spread was measured.
11. **The raw data are not in S3.** The 145 KB `.tgz` is committed to Git, with `raw.uploaded: false`
    and no `.artifacts.json` manifest (§13 routes archives over 64 KiB through the store). The cited
    "same waiver as earlier rounds" is not in the committed objects; the round-4 and round-big
    validators found none either. The sessions exist only inside the tarball, so CI's cross-host
    re-audit (§12) cannot see them.
12. **`work.complete: false` is not mentioned.** Every GPU record carries
    `work.complete: false`, with `gpu_batches`, `gpu_hits_confirmed_on_cpu`, `gpu_targets_swept`
    and `gpu_targets_unused` in `work.unpriced`. Every CPU and rho record has `complete: true`. The
    round-3 and round-4 validators raised the same omission. It does not affect S, which is equal to
    the CPU arm's by identity.
13. **Standing and gate.** The report is tagged `[pcc:unverified]` and `pcc.py lint-prose` passes on
    it. No leaderboard, `benchmarks/README.md` or `VALIDATION.json` row was added, so the validation
    gate was not breached. The ecbench compare files call the walls "admitted 0" (L0), and the
    report's "descriptive, not admitted comparisons" matches.

## Not checked

- **GPU replays.** The GPU records cannot be replayed without a CUDA device. Their counts are
  replayed only on the pod (same binary, same hardware class) and here only through identity to the
  CPU arm. Their walls (every GPU column, the A/A and the 0.18–0.58 ratios) rest on the pod's records
  alone.
- **Binary identity** (finding 2).
- **Code since the round-4 review.** I did not re-review the code between the round-4 review
  (`d905fb89d9`) and `5d618524e2`: `ic_boundary.rs` (`KSumTable` arity up to 9), a `plugins.rs`
  help string, and `ecbench` methods. I also did not re-review the `incremental-gauss-echelon`
  solver, which `research/ic_la_m3_20261115/VALIDATION_REVIEW.md` covers. `gpu/rootfind` is
  unchanged since the round-4 review. That `backend=cuda` fails closed is taken from that review.
  The relation-phase walls (100× the one-thread CPU arm, above a 14-thread emulation's ceiling) are
  consistent with the device.
- **Cost against the protocol's bound.** I did not check the cost (about 42 minutes of one A100 pod
  plus two short failed pods) against the bound.
- **Calibrated reading.** No `w_cal` reading.
- **Run route.** I did not check the route against the EKS default.

## Addendum: the corrections at `3767aaca3f` (read, not re-validated)

I read the `REPORT.md` diff from `219b1378f8` to `3767aaca3f`. Its new numbers are the ones above:
LA 9–11.5× on the CPU, LA-only 0.84–0.99×, per pair 0.61–1.12×, LA share 0.87–0.89 → 0.34–0.42,
cold 5.4–13.2×, and the command window 18:57–19:39Z. Its rewording follows findings 1–12. Two
notes:

- Its sentence that the validator "replayed every worker-free record on another hardware class"
  was committed at 20:33:54Z. At that time my rf3la replay was still running; it finished at
  20:35:19Z with 72/72 `identical`. So the sentence now holds, but it was written before its
  evidence existed. `0fdc9479af` committed an earlier copy of this review with the placeholder
  `REPLAY_RF3LA` in requirement 2d. This file replaces it.
- "the waiver was granted in conversation" cannot be checked from committed objects.

I did not check `REPORT.pdf` at `3767aaca3f`.

## Scripts and outputs (scratch, not committed)

`/tmp/claude-0/-home-user/648c3177-6621-55ff-b9d8-09ead07ab678/scratchpad/validator-r5/`:
`scripts/certs.py` (certificates, curve orders), `scripts/planted.py` (target law),
`scripts/check.py` (identity, LA differences, tables, per-pair ratios, rho, independent band),
`check.out`, `band/` (`ecbench band` default and `--no-price-table-work`), and `replay/` (cross-host
receipts).
