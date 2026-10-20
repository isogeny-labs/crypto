# Full-base common-public panel: live execution handoff

**Observed 2026-10-20 02:10:07 UTC: running.** Supervisor PID 80393 and
container `n83-full-m4-t0-u1` were active. The card started at
**02:06:17 UTC**. This timestamped observation does not guarantee later
liveness. [Exact observation](observation-20261020T0210Z/host-status.txt).

Four full-base constructed controls, four separate general-field replays
and both native off/on comparisons passed before public search. Their
[107-file archive](controls/README.md) and
[report](../reports/run-full-m4-controls-20261020/RUN_REPORT.md) are retained.
Model/row digests and search work agree off/on within each base.

The first natural arm, U target 0, had a sample at 204,567.343978 ms with
97,832 decisions, 145 conflicts, one restart, 98,678 XOR passes,
116,262 checked repivot visits, 289,583 row operations and 298,085,529
row scans. It was in XOR propagation. This is in-progress work; no final
outcome, natural row or rank is inferred from the sample.

The already-running card schedules eighteen natural attempts over all
eight frozen public points: U,D,D,U at index 0, then alternating D,U or
U,D at indices 1–7. Both bases contain 16,627 columns / 2,760,082 points.
All natural arms use the same binary and cache-on policy, free selectors
and phases, 10,000 conflicts, 1,790/1,800 s caps, CPU 31, one CPU quota,
4 GiB / zero swap and no worker network. Maximum natural allocation is
nine hours; completed conflict caps may end sooner. Shared host: L0.

- Root: `/home/ec2-user/n83-m23-20261019/full-m4-v1`.
- Source: `d9ef42301cc3d22f437e1f89304e673836afb6a3`.
- [Protocol](../FULL_M4_PROTOCOL.md), [host card](../FULL_M4_HOST_CARD.md),
  [exact shell card](../run-full-m4-card.sh).
- Final status will be `card-status.txt`, with `card-end.utc` on completion.
  Each run keeps progress, phase, final/cap, process/image and exit records.

On completion, archive every attempt, audit final/cap instrumentation and
source/input/resource identities, independently replay any returned rows,
and aggregate useful rank across the accepted rows. Retain every cap,
failure and interruption. Never retry in this directory; a Spot loss
needs a separately frozen continuation. Do not replace the pinned binary
with a later PR merge resolution.

The SSH script continues independently of this chat. No automatic chat
check-in or PR-activity subscription was available in this session. The
pending PR solver conflict and catalog validation gates remain separate
merge blockers. Full IC cold/reused cost, same-point rho and best-base
selection remain unresolved.
