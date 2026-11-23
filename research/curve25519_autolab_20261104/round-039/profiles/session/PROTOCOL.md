# Wall-time confirmation of the tw2l walk-table change: protocol (written before data)

Development evidence only; a local `ecbench` session (isolation level L0), not an isolated-lab run. It tests one implementation change; it is not a ratchet, a leaderboard row or a speed claim against rho. Waiver: the owner granted the checkpoint waiver in this session on 2026-11-20 (user message "Checkpoint waiver"); runs carry `CHECKPOINT_SKIP=owner-waiver-2026-11-20`, which the hook records as a pcc ledger bypass entry.

## Question
Does the `TwTable` change (branch `perf-tw-table-20261119`, `ecbench` sha256 38d7d1b0...) cut the `sel` solve wall time at k = 28 against the `main` build (sha256 1b86126f...), with counted cost unchanged, under `ecbench`'s own interleaved session?

## Design
- Spec `spec.json` (sha256 recorded in the session log): round-039 holdout curve and 128 public targets, arms `inc`, `sel`, `inc-aa` (copy of `inc`), `sel-aa` (copy of `sel`); 4 rounds after 1 warmup, alternating order, 640 executions per arm per session. Same curve and target seed as the round-039 holdout (already exposed); counted cost is deterministic, so exposure does not matter for a wall-time test and no result here feeds a ratchet.
- Four sessions in the order base, new, new, base (ABBA) on this host, so linear drift cancels in the base/new contrast. `--cpus auto`, shared lock, `--wait`; a busy preflight is recorded, not hidden (`--allow-busy` only if the preflight fails, and then every row stays below L2).
- Metric: `solve_wall_ns` per record (exclusive solve timer); counted `total_gae` per record as the invariance check.

## Prediction (from the earlier harness A/B, not from this data)
- `sel` mean solve wall, new / base: 0.74 (earlier harness: 0.739).
- `inc` mean solve wall, new / base: 0.98 to 1.00.
- Counted cost and recovered scalar identical per record across all four sessions.

## Falsification
- `sel` new / base above 0.90 (bootstrap interval over workloads, paired by workload and round): the 26% cut does not hold in a session.
- `inc` new / base outside 0.95 to 1.05, or either A/A ratio (`sel-aa` / `sel`, `inc-aa` / `inc`, and S1 / S4 for the same binary) outside 0.97 to 1.03: the noise floor is too high to read the contrast; report it as inconclusive.
- Any record with different counted cost or scalar between the binaries: the change is not behaviour-preserving; stop.

## Reporting
Per-arm means, paired ratios with 95% bootstrap intervals, A/A ratios, `ecbench verify` receipts (and a replay of a sample), host capsule, preflight outcome. Labelled development evidence; no benchrec record unless S3 is reachable, no leaderboard or `benchmarks/` row.
