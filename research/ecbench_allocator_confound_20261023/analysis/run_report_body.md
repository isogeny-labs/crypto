# Run report: allocator confound study sessions, 2026-10-23 06:10 UTC

Descriptive run report for the 14 ecbench sessions of `research/ecbench_allocator_confound_20261023` (preregistered; interpretation is in `../../REPORT.md`, not here). Observations only; wall time is descriptive.

## What ran

- **Commands.** `run_all.sh` (ABBA of `base` and `fix` per spec) and the three ablation runs in `logs/`; every session is `ecbench run --wait --allow-busy --cpus auto --quiet --spec SPEC --out sessions/NAME`. Specs: `docs/ecbench/specs/smoke-prime.json` (sha256 684c9aeb...79cc2, spec id ECS1h94f54998e76f) and `specs/claim-koblitz-r10.json` (sha256 c23b5202...52392, spec id ECS1hd5d9c5a020f7).
- **Commits and binaries.** merge-base `3a1ea541d964d6ef9099300a9ab512d55eaa073b` (binary a59c4014ff52, grading v2); branch `ecbench/fix-runner` source `63fede816dcacc012580ce37fd40b4ad850ad867` (binary 26f25d9fa47a, grading v3); three scratch ablations of the branch source (`BINARIES.txt`).
- **Host.** Shared 4-vCPU Xeon 2.10 GHz VM, class ECBENV3had995aa2df0f; the cargo lock was held by other agents at every session start, load average 2.7 to 5.1. `--allow-busy`, whole core reserved by `--cpus auto`; levels earned were L0 (two ablations), L1 and L2, never L3 (`REPORT.md`, "Design").
- **Arms, workloads, seeds.** smoke-prime: 9 arms (`rho-neg` reference, `rho-plain`, `rho-frozen`, `bsgs`, `bsgs-il`, `bsgs-neg`, `kangaroo`, `ic-mitm`, `rho-neg-aa` A/A control), one registered 20-bit prime-order curve, 2 targets (target seed 20261002), 3 measured rounds + 1 warm-up, session seed 1. claim-koblitz-r10: `rho-strong` (reference), `rho`, `ic` (`koblitz-orbit:divisor=0;1`, `mitm-frobenius:m=2`) on Koblitz a=1 n=17 (`icv1-f2m17-tm101-00378d4e`), 2 public targets (target seed 11), 10 measured rounds + 1 warm-up, session seed 5.

## Runs requested and completed

{{table t10_sessions|session|session id|start UTC|duration|status|planned|records written|binary|grading|spec id}}

All 14 sessions completed with every planned execution recorded; every record `verified` (no failure, timeout or interruption to report). Raw records: `sessions/NAME/records.jsonl`; audit receipts: `audits/NAME.audit.json` (all `ecbench verify --replay-all --exit-code`: 0 problems).

## Measured values

Medians of `time.solve_wall_ns` (smoke, ms) of verified measured records; `base` and `fix` pooled over their two sessions (n = 12 per arm), each ablation one session (n = 6). Spread: quartiles and extremes in `analysis/summary.json`; per-record dots in the graph.

{{table t3_ablation|arm|base|fix (trim + probe)|fix-notrim (probe only)|fix-noprobe (trim only)|fix-notrim2 (neither)}}

claim-koblitz-r10 (us; `base` and `fix` n = 40 per arm, ablations n = 20):

{{table t3b_claim_ablation|arm|window|base|fix|fix-notrim|fix-noprobe|fix-notrim2}}

References and controls: `rho-neg` is the smoke reference, `rho-strong` the claim reference; the A/A pair `rho-neg` and `rho-neg-aa` medians differ by a factor 1.160 (base) and 1.003 (fix). Operation counts: identical across binaries for every record (`REPORT.md`, "Counts"); the section 5a band at 0, 0.1, 1 GAE puts `rho-strong` ahead of `ic` on claim-koblitz at every weight (S 4.004, 4.140, 5.361 against 16.852, 17.283, 21.160).

{{img fig1_solve_wall_by_arm|Recorded solve wall time per arm (dots, log scale), pooled median and quartile bar, n above each bar. Sessions `sessions/*`, run ids as in the table above; plotting source `analysis/plot_wall.py`, data `summary.json` beside this file.}}

## Missing or not measured

No independent replay (same host, same agent), no L3 isolation, no hardware counters (`perf_event_open` unavailable on this VM), no trace of `malloc_consolidate` itself.
