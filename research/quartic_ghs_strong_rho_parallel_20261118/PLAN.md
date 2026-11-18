# Parallel GHS strong-rho optimization leads

Status: prospective coordination plan frozen on 2026-11-18 before any of the
six candidate branches is integrated. Base revision:
`ad2a014c0022cf000845014499a9649b6c2cef2d`.

## Request and boundary

Run all currently identified strong-rho optimization leads in parallel:
direct cubic inversion, batch-amortized setup, look-ahead retry compaction,
counter-free production arithmetic, hot-loop storage/hash cleanup, and fast
modular reduction with a prospective `b4/b8/b16` retune.

The common control is the public synthetic `q=223`, seed 1, target 0,
`ic-repeat`, `strong-b8` record retained in
`research/quartic_ghs_strong_rho_20261114/round-b/`. This local cell can
establish correctness and deterministic operation-count deltas only. It does
not discharge the frozen `q=4051..4194271` matched E2E panel and cannot move a
leaderboard.

## Parallel tracks and acceptance

| Track | Required invariant | Candidate-specific evidence |
|:--|:--|:--|
| direct inversion | equal field inverse, point results, planted log and walk | exactly four fewer `fq_mul` per `f3_inv`; same `fq_inv` |
| batch-amortized setup | identical sampled scalars and every constructed point | all precomputation charged cold; lower setup field counts |
| retry compaction | identical per-lane transition and collision/table semantics | fewer retry batch inversions with complete tail charge |
| counter-free mode | counted arm remains byte/count compatible; no-count arm returns identical mathematical output | build identity and explicit separation of count authority from wall-time binary |
| hot-loop cleanup | identical mathematical transitions and abstract band counts | retained raw-hash/allocation diagnostic changes; no inferred throughput |
| reduction + retune | modular products equal `% q` on boundaries and deterministic vectors; widths keep exact answers | unchanged operation counts for reduction; width protocol frozen before comparative output |

Every track retains compile failures, rejected variants, exact commands,
source hashes and raw controls. An implementation that cannot preserve its
invariant is reported as partial or rejected rather than silently changing the
walk.

## Integration and evidence gates

1. Each track works from the same base in an isolated branch and freezes its
   own requirements before candidate output.
2. The coordinator reviews results independently. Count-moving candidates are
   composed first; wall-only candidates remain separate until counted output
   is unchanged under composition.
3. The combined source must rerun the standalone self-test and paired q=223
   JSON comparison. Expected deltas are derived per phase and checked rather
   than selected post hoc.
4. Batch width is reselected only under its prospective protocol. Historical
   Round A and Round B results stay visible.
5. No profile, A/A, ABAB, isolated timing, paid job, or complete frontier run
   starts unless `scripts/checkpoint.sh push` and `gate` pass. At freeze time
   this runtime had no AWS identity, upload-token environment variable, or
   checkpoint token file.
6. A merged implementation or toy count reduction is not an E2E result.
   Re-establishing the restricted-family frontier still requires the complete
   same-target grid, isolation, calibration, rho-step admission, independent
   validation, PCC and publication gates.

## Coordination limitations

Conductor and the research-KB executable are unavailable in this runtime.
The first integration checkout attempt exhausted the 32 GiB overlay; only
reproducible Cargo `target/` directories were cleaned. Parallel scratch clones
therefore run on `/dev/shm` and must push commits promptly because that storage
is ephemeral. No source or research evidence was deleted.
