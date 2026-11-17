# K13 local diagnostic: source-bound repeat

This is a descriptive run report with no performance verdict or scientific
state transition. [Amendment 1](../../AMENDMENT-1.md) filed the source-stamp
correction before this repeat. The original attempt is retained separately;
these two epochs use the same targets and are not independent holdouts.

The unchanged frozen spec `ECS1hb376719703c9` used eight fresh public targets
of registered Koblitz E0 over `F_(2^13)`, subgroup order 2003, five arms,
four measured rounds and one warmup. Session `ECBS1h08f6d5475be8` wrote
200/200 records, with 200 verified answers, no failures or timeouts
[pcc:unverified]. The
[audit receipt](../../audits/custody-rerun/replay-all.json) reproduced all
160 measured deterministic executions with zero problems. All 200 work rows
exist, and each arm has 32/32 completely priced measured rows in `bitop/v1`.

The binary SHA-256 is
`c4b63be34e52621d5d43e9cfcb2107f596b1148c7d06715fb7560dcae32e5d80`;
the runtime capture now names this branch's commit
`02113398fc2c4544db35f879b6a55add1bff9767` and its worktree-local binary
path. `git_dirty=true` records the task-specific checkpoint and open run
artifacts; no compiled Rust source differed from the frozen parent. The exact
command is in benchmark record
`benchmarks/records/20261116T0413Z-rootfind-k13-f6-local-custody-rerun.json`.
Every execution earned L0 on an Apple M4 Pro host, with no CPU reservation.

Measured medians over 32 complete cold processes per arm:

| arm | process wall ms | method-call wall ms | median bitop/v1 |
| --- | ---: | ---: | ---: |
| batch m=3 | 8.277 | 0.419 | 4,939,443 |
| batch m=3 A/A | 8.187 | 0.420 | 4,939,443 |
| scalar rootfinding m=3 | 8.274 | 0.449 | 4,795,131 |
| F6-IC | 39.482 | 31.655 | 1,871,566,210 |
| strong rho | 7.794 | 0.106 | 2,000,346 |

Derived from 32 exact target-round pairs, the ratio of summed batch/F6
complete process walls is **0.2078**, with 32/32 individual ratios below 1
and pair range 0.123–0.477. Batch/A/A is 1.0245, batch/scalar 1.0163, and
batch/rho 1.0772 by the same sum rule. The A/A pair range is 0.809–1.374.
The [graph](paired-ratios.svg) displays the eight target-cluster ratios;
[analysis.json](analysis.json) carries the exact input digests and arithmetic.
The first epoch's corresponding batch/F6 ratio was 0.1942 but remains
custody-invalid and is not pooled with this repeat.
The local ratio has machine-readable measurement record
`pcc/claims/PCC-20261116-007049.json`; its PCC standing is `not_covered`
because the producer-authored [recompute](pcc-recompute-wrapper.json) and
[replay](pcc-replay-wrapper.json) wrappers are unsigned. Their input hashes
and arithmetic are inspectable, but no independent PCC verification follows.

The F6 relation phase has a 31 ms median, compared with 0.11 ms for batch
rootfinding, while process startup dominates the batch's cold wall. Those
phase figures are per-arm medians, not a paired stage ratio. The observed
batch-versus-scalar cold difference is smaller than the A/A spread; there is
no local evidence here that the added batching improves the complete solve
over scalar rootfinding [pcc:unverified]. Dense pair-table work is therefore not prioritized
by this diagnostic. The frozen same-host L2 panel still needs matrix F4, F5,
inherited F4, F6-IC, A/A and rho on its own targets, plus independent replay,
before the user's requested full comparison is established.

Raw files are in `../../sessions/local-custody-rerun-202611160401/`, command
logs are `../../custody-rerun.stdout.log` and
`../../custody-rerun.stderr.log`, work rows are `work-records.jsonl`, and the
replay receipt is linked above. Canonical graphs checked:
`docs/ic/LEADERBOARD.md`, `docs/backend-leaderboard/LEADERBOARD.md` and the
frozen combined panel's `panel.svg`; none changes from an L0 exploratory
observation.
