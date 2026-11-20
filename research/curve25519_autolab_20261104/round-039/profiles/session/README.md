# Wall-time confirmation of the tw2l walk-table change (local `ecbench` sessions, ABBA)

Development evidence. Four `ecbench run` sessions on this host, run after the owner's checkpoint waiver of 2026-11-20 (command prefixed `CHECKPOINT_SKIP=owner-waiver-2026-11-20`). Protocol and thresholds were committed before any data: [`PROTOCOL.md`](PROTOCOL.md) (commit 9967e7647a, sha256 7aa6c767...), spec [`spec.json`](spec.json) (sha256 a96dc9e6...). Not an isolated-lab run, not a ratchet, not a leaderboard row. [pcc:unverified]

## Setup
- Binaries: `main` build sha256 1b86126f... (S1, S4) and branch build 38d7d1b0... (S2, S3), order S1 base, S2 new, S3 new, S4 base (`run_abba.sh`, `progress.log`). The branch build is the one in PR #4811; commits after it change only docs.
- Spec: round-039 holdout curve (p = 268435157, r = 33557809) and its 128 public targets, arms `inc`, `sel`, `inc-aa`, `sel-aa`; 4 rounds after 1 warmup, alternating order, 512 measured records per arm per session (2048 per session).
- Host: Intel Xeon 2.1 GHz, 4 vCPU, no PMU. Every session passed the quiet preflight and reserved a whole core: 2034 to 2044 of 2048 measured records per session are isolation L2, the rest L1 (4 to 14 per session). The host capsule (`S*/host.json`) is the container's, not the lab's.
- Metric: `time.solve_wall_ns` per record. Ratios are ratios of means, paired by (arm, workload, target, round), 95% bootstrap over the 128 targets (1000 draws, seed 20261120), `analyze.py`.

## Result against the preregistered thresholds
| quantity | result | threshold | |
|:--|--:|--:|:--|
| `sel` new / base (ABBA pooled) | 0.7718 [0.7608, 0.7842] | falsified above 0.90 | holds |
| `inc` new / base | 1.0212 [1.0065, 1.0365] | 0.95 to 1.05 | inside the band; interval excludes 1 |
| within-session A/A (`sel-aa/sel`, `inc-aa/inc`, 8 values) | 0.992 to 1.014 | 0.97 to 1.03 | pass |
| same binary, other session: `sel` S4/S1, S3/S2 | 0.9806, 0.9722 | 0.97 to 1.03 | pass, S3/S2 near the edge |
| same binary, other session: `inc` S4/S1, S3/S2 | 0.9825, 0.9768 | 0.97 to 1.03 | pass |
| counted cost, recovered scalar, status across the four sessions | 0 mismatches in 2048 record keys | none allowed | pass |
| `sel / inc` within session, base sessions | 1.1443 [1.1161, 1.1781] | n/a | the harness ratio |
| `sel / inc` within session, new sessions | 0.8649 [0.8366, 0.8967] | n/a | |

Mean solve wall (ms): `sel` 2.497, 1.935, 1.882, 2.449 and `inc` 2.180, 2.233, 2.181, 2.142 in S1 to S4.

`ecbench verify --replay 64 --replay-seed 20261120` with each session's own binary: all four report "audit OK: 2560 records, 2560 verified, 64 replays reproduced, 0 problems" (`S*/verify-receipt.*`; the producer's own audit, [pcc:unverified]).

## Reading
- The 26% cut from the unisolated harness shows again under `ecbench`'s own interleaved, core-reserved sessions: `sel` solve wall falls by 22.8% (ratio 0.772) with identical counted cost, scalar and status on every record. The earlier harness gave 0.739; the difference between 0.739 and 0.772 is larger than either interval, so the size of the cut is not pinned down to better than about 0.74 to 0.78 by this evidence.
- `inc`, whose code path does not use the walk table, is 2.1% slower in the new sessions (interval 0.7% to 3.7%), inside the preregistered band but not zero. Not explained: it could be binary layout, or the 2% session-to-session drift (later sessions are faster, both arms) not cancelling in four sessions. The within-session `sel / inc` ratio is not exposed to that and moves from 1.144 to 0.865 (a ratio of 0.756).
- Session-to-session drift of 2 to 3% is the noise floor here; the contrast is about ten times larger.
- Round 39's counted-GAE ratchet result is unchanged; this is an implementation change. The round's wall-time handicap (sel 1.2356x inc in the round-039 session) is now 0.865x inc in this host's sessions, a different host and spec, so the two do not transfer.

## Limits
- Local container host (4 vCPU shared VM, no PMU), not the Iso Lab; no calibration or L2 admission beyond `ecbench`'s own preflight and placement. A lab session with the native L2/replay/A/A admission is still the route for a quotable number.
- Four sessions of 25 s each; 128 targets, already exposed by the round-039 holdout (irrelevant to a wall-time test, and nothing here feeds a ratchet).
- Raw `records.jsonl` (about 10 MB per session) is not committed; sha256 is in `S*/records.jsonl.sha256` and each session re-runs in about 25 s with `run_abba.sh`.
- No benchrec record (S3 unreachable in this session). The hook-recorded bypass entry in `pcc/ledger.jsonl` did not appear in this working tree, so none is committed; the waiver is recorded here and in the session log.

Files: `analysis.txt`, `analysis.json`, `ratios.svg`, `analyze.py`.
