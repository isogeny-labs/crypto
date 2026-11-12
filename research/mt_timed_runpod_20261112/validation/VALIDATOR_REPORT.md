# Validator report: mt_timed_runpod_20261112 (timed IC vs rho on a RunPod pod, std vs fast hasher)

Independent validator subagent (claude-opus-5-5), 2026-11-12. Validator code in `scripts/`, none of it the
producer's. No RunPod call was made: the pods are gone; everything below is from the committed evidence and local
reruns on the session VM.

- **Validated commit:** `43fc3b0e61907dbc12b95ddd471678535311b011` (protocol `fde73fb405`, amendment A1 `7934b7e343`).
  HEAD moved to `51ca1eb6d6` during validation; that commit touches only
  `benchmarks/records/20261112T1629Z-mt-timed-runpod.json`. The study, the harness, `Cargo.toml`, `Cargo.lock`,
  `src/`, `crates/` and the instances are identical to `43fc3b0e61` and were clean before and after the builds.
  `51ca1eb6d6` itself was not validated.
- **RESULTS.md headline and table** (`validator-report.json`): **pass, validated.** All 89 headline numbers
  recompute within rounding. 388 of 388 `summary.json` fields match exactly. Nine non-blocking RESULTS/protocol
  amendments are required (R-F3 to R-F11). R-F12 is a note with no change asked.
- **Replay certificate** (`replay-certificate.json`): **verified.**
- **Claim statement** (`claim-report.json`): **fail, insufficient_evidence.** Every number in it is correct, but it
  is not endorsed as worded (R-F1, R-F2). A proposed wording is given and endorsed.

## Requirements

| # | requirement | result | evidence |
|---|---|---|---|
| 1a | build identity | pass. Both builds rebuilt from `43fc3b0e61`, each in its own fresh target dir under the validator's scratch, `--locked --offline`. std `7ac0cf63fe0960670c6d22c5d8c0bb612ec105a6e12b67e4cd4929034d58189b` and fast (`--features mt-fasthash`) `afc768c574e131bb899c9481f51592b8bc0dd72cfc46a3f445101f6da619c483` are bit-identical to `launch/payload.json`, `host.json` `bin_sha256` and every row's `bin_sha256`. The target-dir path does not enter the binary. | `replay-certificate.json` binary |
| 1b | source diff | pass. `examples/mt_fieldops.rs` at `a992ed6c80` (the validated timed study) and at `43fc3b0e61` normalise to the same text once the allowed rewrites are applied. Those rewrites are: drop `use std::collections::HashMap`, add the `HState`/`MtFastH` alias block, change `HashMap::new()` to `default()` and `with_capacity(x)` to `with_capacity_and_hasher(x, Default::default())`, and give the opcost std table explicit `std::collections::HashMap` types. The default build uses `RandomState` (SipHash); the feature build uses `BuildHasherDefault<MtFastH>`. The `Cargo.toml` change is only `mt-fasthash = []` and its comment. | `source_diff.json` |
| 1c | identical counted operations | pass. Every key of both builds was replayed with the rebuilt binaries: std 612/612 keys (1836 rows) and fast 612/612 (1836 rows) match exactly. That covers every printed field except `ms_arm` and `ms`: ok, targets, I, M, S, H, L, N, the phases and the arm-specific counters. In the rows, the counters and phases are equal across builds and repeats for all 612 configuration keys. The std counters equal the earlier timed study's rows for all 612 keys (1836 rows). | `replay_check.json`, `recompute.json` checks |
| 2a | payload integrity | pass. The launcher's payload file hashes to the pin `70770d45...` (also in `create.json`). Its 15 members (run.py, the two binaries, the 12 instances) equal the committed `run.py`, the validator's rebuilt binaries and the committed instances. There are no extra members. The file rebuilds bit for bit with gzip mtime 16:32:28Z, found independently by window search. Attempt 1's pin `d44404ff...` rebuilds bit for bit from `fde73fb405`'s `run.py` (gzip mtime 16:29:11Z, 33 s before the protocol commit). `podjob.py` matches `create.json`'s `podjob_sha256`. | `payload_check.json`, `payload_check_a1.json` |
| 2b | receipts | pass. `collect.json` hashes equal the committed `results/holdout/*` files, and attempt 1's `collect.json` hashes equal `attempts/a1/results/*`. Timeline: protocol 16:29:44Z, a1 pod 16:30:47, a1 upload 16:31:20, a1 collect/delete 16:31:34, A1 commit 16:32:29, a2 pod 16:32:34, upload and job start 16:33:08, collect 16:58:47, delete 16:58:48. Attempt 1: 3672/3672 taskset failures ("Invalid argument"), no row with `ok` or `ms_arm`. Attempt 2: 3672 executions, 0 errors. | `launch/`, `attempts/a1/` |
| 2c | A1 legitimate | pass in substance. The amendment, its new `run.py` (pin = middle CPU of `sched_getaffinity`) and the new pin were committed before attempt 2's pod existed, and attempt 1 produced no measurement. There is no workload, analysis or rule change. The text's time "16:40Z" is wrong (R-F8). | git log, receipts |
| 2d | secrets | pass. Attempt 2's pod token (checked by value) is not in any commit in `fde73fb405^..51ca1eb6d6` (git grep and pickaxe). The study has no 48-hex string; every long hex string there is a sha256. The receipts keep only env key names. They do commit the RunPod `consumerUserId`/`machineId` (identifiers, not credentials; R-F11). | R-F11 |
| 3 | correctness | pass. `ok == targets` in all 3672 rows (26,320,896 targets). The validator's dump builds ran a nonce sample: nonce `5a7b5fe5a26fbf2bd874f3c10df7e3ad`, drawn 17:14:31Z before sampling. The sample is 42 runs, 21 per build: every arm at every size and each build. All their counters match the RunPod rows. 301,056 recovered logs (132,077 distinct scalar multiplications) were checked with the validator's own EC code: Q on the curve, log equal to the planted log, [log]G = Q, with 0 failures. The planted targets are identical across arms and builds. The 12 instances have p and n prime, cofactor 1 and G of order n. | `ec_check.json` |
| 4 | recompute | pass. Per build and cell the validator recomputed: ratios, protocol bootstrap and exact 4^4 bootstrap (identical, both equal the range), decisions, size-level counts, fast/std, the fixed-key control, A/A, round drift, opcost and winners. The protocol shuffle order was re-derived: all 3672 rows sit in the pre-registered order. The producer's `analyze.py`/`mk_runs.py` rerun reproduces `summary.json`, `analysis.txt` and `runs.json` byte for byte. | `recompute.json`, `results_check.json` |
| 5 | wording and limits | RESULTS: diagnostic seconds, shared host, A/A drift, the n = 4 sign rule and labelled interpretation are all present. Findings R-F3 to R-F7 apply. Claim statement: R-F1 and R-F2 block it as worded. | below |

## Recomputed table (solve-phase wall time; median of 4 configurations per cell; interval = range)

| bits | T | std IC/rho | fast IC/rho | IC fast/std | rho fast/std | IC/BSGS std / fast | A/A rep0/rep1 std / fast | fixed-key std / fast |
|---|---|---|---|---|---|---|---|---|
| 24 | 1024 | 0.884 [0.861, 0.984] faster | 0.893 [0.809, 0.974] faster | 0.892 | 0.878 | 1.518 / 1.534 | 0.968 / 1.039 | 0.819 faster / 0.932 faster |
| 24 | 4096 | 0.733 [0.718, 0.825] faster | 0.776 [0.765, 0.799] faster | 0.890 | 0.845 | 1.473 / 1.459 | 1.075 / 1.075 | 0.696 faster / 0.794 faster |
| 24 | 16384 | 0.700 [0.697, 0.712] faster | 0.712 [0.704, 0.726] faster | 0.900 | 0.886 | 1.464 / 1.414 | 1.096 / 1.023 | 0.688 faster / 0.725 faster |
| 28 | 1024 | 0.791 [0.766, 0.824] faster | 0.739 [0.719, 0.754] faster | 0.918 | 0.981 | 1.442 / 1.457 | 1.024 / 1.045 | 0.814 faster / 0.722 faster |
| 28 | 4096 | 0.833 [0.819, 0.835] faster | 0.779 [0.751, 0.794] faster | 0.926 | 0.989 | 1.485 / 1.497 | 1.138 / 1.097 | 0.849 faster / 0.783 faster |
| 28 | 16384 | 0.953 [0.936, 0.995] faster | 0.944 [0.929, 0.975] faster | 0.946 | 0.961 | 1.536 / 1.615 | 1.029 / 1.108 | 0.948 faster / 0.895 faster |
| 32 | 1024 | 1.053 [1.029, 1.083] slower | 1.011 [0.996, 1.031] unranked | 0.963 | 1.003 | 1.520 / 1.602 | 1.142 / 1.101 | 1.050 slower / 0.987 unranked |
| 32 | 4096 | 1.170 [1.136, 1.195] slower | 1.108 [1.076, 1.145] slower | 0.938 | 0.986 | 1.600 / 1.619 | 1.121 / 1.014 | 1.149 slower / 1.107 slower |
| 32 | 16384 | 1.179 [1.161, 1.192] slower | 1.148 [1.110, 1.171] slower | 0.949 | 0.975 | 1.578 / 1.599 | 1.086 / 1.050 | 1.184 slower / 1.125 slower |

Size level, per configuration:

- std: 12/12 below 1 at 2^24 (0.697-0.984) and at 2^28 (0.766-0.995); 12/12 above 1 at 2^32 (1.029-1.195).
- fast: 12/12 below 1 at 2^24 (0.704-0.974) and at 2^28 (0.719-0.975); 11/12 above 1 at 2^32 (0.996-1.171).
- Per configuration, fast/std is 0.82-0.98 for the IC and 0.81-1.07 for rho, and IC/BSGS is 1.31-1.67.
- Including the shared verification pass (`ms`), the decisions are the same in all 18 build-cells.

Controls:

- Round drift (median key time relative to its key median, both builds): 1.032, 1.000, 1.000.
- Median |log dev| is 0.008 (p95 0.11). It is 0.020 when the zero self-term is excluded.
- The A/A interval contains 1 in 4/9 cells (std) and 3/9 (fast).
- The 1-min load average at the executions was 37.6-93.5 (medians by round 86.6, 77.3, 61.0).
- Steal was 0 at every execution.

Opcost:

- GAE 98.5-140.0 ns, about 2.05x faster than the VM.
- I/M 23.2-27.2; canonicalisation plus hash 0.0059-0.0074 GAE.
- std lookup: 0.058-0.110 GAE at 2^10-2^16 entries and 0.744-1.453 GAE at 2^22-2^24 (VM 0.27-0.85).

Host replication: the earlier VM cell medians recompute to 0.778-0.829 at 2^24, 0.932 faster / 1.053 / 1.007
unranked at 2^28, and 1.114-1.289 at 2^32, as RESULTS says.

## Findings

R-F1 and R-F2 block the claim statement as worded. None blocks RESULTS.md. Full text, evidence and amendments are
in `validator-report.json` and `scripts/findings.json`.

- **R-F1.** The statement's ranges are cell medians, but the statement does not say so and puts them beside
  per-configuration counts. Per configuration, IC/rho is 0.70-0.98 at 2^24 (std), not 0.70-0.88. The statement also
  drops the 4-of-4 sign-rule disclosure that the validated predecessor carried.
- **R-F2.** The statement omits that the fast build's 2^32, T = 1024 cell is unranked (1.011 [0.996, 1.031]).
- **R-F3.** RESULTS gives the "load average 40-89"; the recorded values are 37.6-93.5. 40 and 89 are the medians of
  the last and first sixths of the run.
- **R-F4.** "The pinned CPU's SMT sibling was idle apart from the pod's own job server" is not supported by any
  artifact. No sibling topology or per-CPU use was recorded, and a cpuset on a shared host does not show exclusive
  use.
- **R-F5.** Two pre-registered predictions are reported incompletely.
  - Prediction 1's magnitude clause ("within about 10% of the VM") fails in 4 of 9 cells, by up to 21% at 28 b
    T = 4096. The outcome says only "holds in direction".
  - Prediction 2's per-arm figures are not evaluated: ic5rw 0.85 (predicted about 0.6), ic5ks 0.84, ic5 0.93,
    ic5w 0.93, rhodpu 0.98, rho2xks 0.84.
- **R-F6.** Two small errors in RESULTS:
  - The 2^32 per-configuration maximum is 1.195, which rounds to 1.19, not the stated 1.20.
  - In the fast build at 28 b T = 1024, rho2xks is the fastest rho in 1 of 4 configurations, against "rhodpu
    elsewhere".
- **R-F7.** Two problems in the A/A section:
  - The A/A figure of 0.008 includes the zero self-term. Over the non-median repeats it is 0.020.
  - "Effects at 2^32 (5-20%) larger than the drift" does not hold for the fast build's 2^32, T = 1024 cell (1%),
    and 5-20% mixes cell medians with a per-configuration maximum.
- **R-F8.** A1 is stamped "16:40Z". The commit is at 16:32:29Z, and attempt 2 was created at 16:32:34Z and started
  at 16:33:08Z. The substance is legitimate; the time is wrong.
- **R-F9.** Both `runs.jsonl` files (2.66 MB and 1.33 MB) are committed to Git, against AGENTS.md section 13 (store
  plus `.artifacts.json`, as the earlier study did).
- **R-F10.** The payload is not deterministic as the docstring says: the gzip header holds `time.time()`. It was
  reproduced here by mtime search. The rows carry no commit field; the binding is through `bin_sha256`.
- **R-F11 (note).** No token was found. The receipts carry the RunPod `consumerUserId` and `machineId`.
- **R-F12 (note, limit).** Wall times cannot be replayed. They rest on files the producer collected, but they are
  internally consistent:
  - 1428.2 s of runs plus 25.7 s of opcost fit within the 1539 s between upload and collection.
  - The counters replay exactly.
  - The prices are plausible for an EPYC 9575F.

## Claim statement

Judged statement: sha256 `3fd446d9b35ba809e2a8eae29c1e802b2c1a12763c7c15c433ade25228af6b6b`. It is not endorsed.

Endorsed alternative (`claim-report.json` `proposed_wording`, sha256
`a8be619270b5eee0c323cd8d25535e3f043e3e08b6fd66351df30cd192a23ef9`):

> On one pinned CPU of a RunPod Secure Cloud cpu5c pod (AMD EPYC 9575F; a shared cloud host, so the seconds are diagnostic), in this harness's implementation, 36 configurations (the 12 prime-order Weierstrass curves of the v5 study, group orders about 2^24, 2^28 and 2^32, seed 43, T = 1024/4096/16384) x 17 run keys x 3 interleaved repeats for each of two builds (std HashMap with SipHash; a fast multiplicative hasher, with identical counted operations), every target verified: per (size, T) cell, the median over its 4 configurations of the solve-phase wall time of the fastest of four multi-target index-calculus variants divided by that of the fastest of ten multi-target rho runs is 0.70-0.88 (std) and 0.71-0.89 (fast) at 2^24 and 0.79-0.95 and 0.74-0.94 at 2^28 (faster in all 3 cells and in 12 of 12 configurations at each size and build; single configurations 0.70-0.99), and 1.05-1.18 at 2^32 in the std build (slower in all 3 cells; 12 of 12 configurations above 1) and 1.01-1.15 in the fast build (slower in 2 cells, unranked at T = 1024; 11 of 12 configurations above 1). Each cell decision is a 4-of-4 sign rule (the bootstrap interval of a median of 4 equals their range). By cell median, the fast hasher cuts the solve time of the fastest IC by 4-11% and of the fastest rho by up to 15% (0.3% slower in the 2^32, T = 1024 cell), and the IC takes 1.41-1.62x the time of a paired BSGS; the A/A control drifted (the first of three interleaved rounds about 3% slower), disclosed.

## Not done

- **No timing reproduction.** The RunPod host cannot be re-run from here, so wall times are judged by internal
  consistency (R-F12).
- **No recompute of the counted section 5a band.** It belongs to the v5 counted study.

## Reproduce

`W=<scratch> bash scripts/run_all.sh` runs the command sequence. Each build takes about 14 minutes on the session VM
and needs about 1.6 GB of disk.
