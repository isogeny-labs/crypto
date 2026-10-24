# Run report: allocator confound study sessions, 2026-10-23 06:10 UTC

Descriptive run report for the 14 ecbench sessions of `research/ecbench_allocator_confound_20261023` (preregistered; interpretation is in `../../REPORT.md`, not here). Observations only; wall time is descriptive.

## What ran

- **Commands.** `run_all.sh` (ABBA of `base` and `fix` per spec) and the three ablation runs in `logs/`; every session is `ecbench run --wait --allow-busy --cpus auto --quiet --spec SPEC --out sessions/NAME`. Specs: `docs/ecbench/specs/smoke-prime.json` (sha256 684c9aeb...79cc2, spec id ECS1h94f54998e76f) and `specs/claim-koblitz-r10.json` (sha256 c23b5202...52392, spec id ECS1hd5d9c5a020f7).
- **Commits and binaries.** merge-base `3a1ea541d964d6ef9099300a9ab512d55eaa073b` (binary a59c4014ff52, grading v2); branch `ecbench/fix-runner` source `63fede816dcacc012580ce37fd40b4ad850ad867` (binary 26f25d9fa47a, grading v3); three scratch ablations of the branch source (`BINARIES.txt`).
- **Host.** Shared 4-vCPU Xeon 2.10 GHz VM, class ECBENV3had995aa2df0f; the cargo lock was held by other agents at every session start, load average 2.7 to 5.1. `--allow-busy`, whole core reserved by `--cpus auto`; levels earned were L0 (two ablations), L1 and L2, never L3 (`REPORT.md`, "Design").
- **Arms, workloads, seeds.** smoke-prime: 9 arms (`rho-neg` reference, `rho-plain`, `rho-frozen`, `bsgs`, `bsgs-il`, `bsgs-neg`, `kangaroo`, `ic-mitm`, `rho-neg-aa` A/A control), one registered 20-bit prime-order curve, 2 targets (target seed 20261002), 3 measured rounds + 1 warm-up, session seed 1. claim-koblitz-r10: `rho-strong` (reference), `rho`, `ic` (`koblitz-orbit:divisor=0;1`, `mitm-frobenius:m=2`) on Koblitz a=1 n=17 (`icv1-f2m17-tm101-00378d4e`), 2 public targets (target seed 11), 10 measured rounds + 1 warm-up, session seed 5.

## Runs requested and completed

| session | session id | start UTC | duration | status | planned | records written | binary | grading | spec id |
|:--|:--|:--|:--|:--|:--|:--|:--|:--|:--|
| base-smoke | ECBS1hd3725d8a32c3 | 06:10:03 | 3 s | complete | 72 | 72 | a59c4014ff52 | 2 | ECS1h94f54998e76f |
| fix-smoke | ECBS1hff17ab9636f0 | 06:10:16 | 5 s | complete | 72 | 72 | 26f25d9fa47a | 3 | ECS1h94f54998e76f |
| fix-smoke-rev | ECBS1h5d29a35e9eba | 06:10:23 | 5 s | complete | 72 | 72 | 26f25d9fa47a | 3 | ECS1h94f54998e76f |
| base-smoke-rev | ECBS1h420293eaa9c5 | 06:10:30 | 3 s | complete | 72 | 72 | a59c4014ff52 | 2 | ECS1h94f54998e76f |
| base-claim | ECBS1h7c0082ac7265 | 06:10:36 | 4 s | complete | 66 | 66 | a59c4014ff52 | 2 | ECS1hd5d9c5a020f7 |
| fix-claim | ECBS1h157a7553e392 | 06:10:42 | 5 s | complete | 66 | 66 | 26f25d9fa47a | 3 | ECS1hd5d9c5a020f7 |
| fix-claim-rev | ECBS1ha418537656ed | 06:10:49 | 5 s | complete | 66 | 66 | 26f25d9fa47a | 3 | ECS1hd5d9c5a020f7 |
| base-claim-rev | ECBS1h4d78060e6148 | 06:10:56 | 3 s | complete | 66 | 66 | a59c4014ff52 | 2 | ECS1hd5d9c5a020f7 |
| fix-notrim-smoke | ECBS1h628195e31e14 | 06:39:19 | 5 s | complete | 72 | 72 | 6fbdb7c8f352 | 3 | ECS1h94f54998e76f |
| fix-notrim-claim | ECBS1hc3e67b6813c4 | 06:39:26 | 4 s | complete | 66 | 66 | 6fbdb7c8f352 | 3 | ECS1hd5d9c5a020f7 |
| fix-notrim2-smoke | ECBS1h0aa59508bb08 | 07:15:51 | 5 s | complete | 72 | 72 | 4e9927811f4d | 3 | ECS1h94f54998e76f |
| fix-notrim2-claim | ECBS1h9d777ef2e237 | 07:15:58 | 5 s | complete | 66 | 66 | 4e9927811f4d | 3 | ECS1hd5d9c5a020f7 |
| fix-noprobe-smoke | ECBS1h322f7c2815eb | 07:42:48 | 4 s | complete | 72 | 72 | 591b9a402a4d | 3 | ECS1h94f54998e76f |
| fix-noprobe-claim | ECBS1h11617fea463f | 07:42:55 | 4 s | complete | 66 | 66 | 591b9a402a4d | 3 | ECS1hd5d9c5a020f7 |

All 14 sessions completed with every planned execution recorded; every record `verified` (no failure, timeout or interruption to report). Raw records: `sessions/NAME/records.jsonl`; audit receipts: `audits/NAME.audit.json` (all `ecbench verify --replay-all --exit-code`: 0 problems).

## Measured values

Medians of `time.solve_wall_ns` (smoke, ms) of verified measured records; `base` and `fix` pooled over their two sessions (n = 12 per arm), each ablation one session (n = 6). Spread: quartiles and extremes in `analysis/summary.json`; per-record dots in the graph.

| arm | base | fix (trim + probe) | fix-notrim (probe only) | fix-noprobe (trim only) | fix-notrim2 (neither) |
|:--|:--|:--|:--|:--|:--|
| rho-neg | 5.833 | 0.25 | 0.236 | 0.277 | 6.57 |
| rho-plain | 6.144 | 0.277 | 0.25 | 0.275 | 6.738 |
| rho-frozen | 6.091 | 0.33 | 0.281 | 0.271 | 6.594 |
| rho-neg-aa | 6.767 | 0.25 | 0.221 | 0.278 | 6.419 |
| bsgs | 5.392 | 0.186 | 0.188 | 0.212 | 6.461 |
| bsgs-il | 5.262 | 0.162 | 0.14 | 0.182 | 5.769 |
| bsgs-neg | 5.303 | 0.137 | 0.123 | 0.14 | 6.177 |
| kangaroo | 6.24 | 0.196 | 0.172 | 0.215 | 8.342 |
| ic-mitm | 1.891 | 1.83 | 1.866 | 1.88 | 1.906 |

claim-koblitz-r10 (us; `base` and `fix` n = 40 per arm, ablations n = 20):

| arm | window | base | fix | fix-notrim | fix-noprobe | fix-notrim2 |
|:--|:--|:--|:--|:--|:--|:--|
| ic | solve | 1412.7 | 1354.2 | 1342.1 | 1456.2 | 1518.4 |
| ic | online | 127.1 | 119.1 | 113.5 | 125.2 | 120.3 |
| rho | solve | 5886.9 | 249.1 | 198 | 296.4 | 5638.7 |
| rho | online | 5885.3 | 245.6 | 194.1 | 292 | 5635 |
| rho-strong | solve | 5658.4 | 148.7 | 147.3 | 170 | 5631.3 |
| rho-strong | online | 34.7 | 38 | 37.7 | 42.6 | 39.2 |

References and controls: `rho-neg` is the smoke reference, `rho-strong` the claim reference; the A/A pair `rho-neg` and `rho-neg-aa` medians differ by a factor 1.160 (base) and 1.003 (fix). Operation counts: identical across binaries for every record (`REPORT.md`, "Counts"); the section 5a band at 0, 0.1, 1 GAE puts `rho-strong` ahead of `ic` on claim-koblitz at every weight (S 4.004, 4.140, 5.361 against 16.852, 17.283, 21.160).

![Recorded solve wall time per arm (dots, log scale), pooled median and quartile bar, n above each bar. Sessions `sessions/*`, run ids as in the table above; plotting source `analysis/plot_wall.py`, data `summary.json` beside this file.](fig1_solve_wall_by_arm.png)
[SVG](fig1_solve_wall_by_arm.svg)

## Missing or not measured

No independent replay (same host, same agent), no L3 isolation, no hardware counters (`perf_event_open` unavailable on this VM), no trace of `malloc_consolidate` itself.
