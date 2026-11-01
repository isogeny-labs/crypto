# Prepared index updates awaiting Conductor scope release

**Resolved, 2026-10-20 15:43 UTC:** the owner explicitly approved the two
benchmark rows. The scope was also available at the fresh check and was
reserved under T-4563. Both rows below are now applied to
`benchmarks/README.md`; the earlier problem-index edit remains in place.
This coordination gate is closed. The historical observations follow.

As of 2026-10-20 06:56 UTC, Conductor blocks these two index edits:

- `benchmarks/README.md`: task **T-4450**, “Review and qualify crypto PR 2281”.
  Joining that task for the two append-only rows also returned `block_conflict`.
- `problems/README.md`: task **T-4547**.

**Update, 07:09 UTC:** the problem-index scope became available. Task
T-4557 reserved it and applied the exact row below. Only the two
benchmark-index rows remain pending; no override was used for the
problem-index edit.

The finished reports and browsing copies are committed in this PR.
The required index changes remain pending until their scopes are released
or the owner provides an explicit coordination override. This is a
coordination gate, separate from the scientific evidence and GitHub CI.
The exact proposed rows follow, so the remaining edit is reviewable.

## Prepend after the benchmark table header

| 2026-10-20 | [`20261020-n83-full-m4-natural`](20261020-n83-full-m4-natural/) | full-size U/D m=4, eight common public points, eighteen attempts | 329-file audit passes; D nine conflict caps, U four conflict/five wall caps; worker interval sums D 2636.872934 s, U 14284.411736 s; different endpoints give no speed ratio; retained natural matrices rank zero, yield/full rank panel censored | Intel Xeon 6975P-C SSH Spot, CPU 31, 4 GiB zero swap, shared L0 | [`report`](../research/n83_prebuild_cost_m23_20261019/reports/run-full-m4-natural-20261020/RUN_REPORT.md) ([PDF](20261020-n83-full-m4-natural/RUN_REPORT.pdf)) |
| 2026-10-20 | [`20261020-n83-prebuild-reuse`](20261020-n83-prebuild-reuse/) | accounting addition from the retained same-D m=2/m=3 finite-query card, including fresh replay/container intervals and q=1,8,64 | derived q=64 prebuilt/on-demand budgets: m=2 983.472/2504.633 s, m=3 910.172/2468.314 s; 64-query panels unexecuted; full IC cost and matched rho open | retained Intel Xeon 6975P-C SSH Spot L0 observations; no new timing run | [`report`](../research/n83_prebuild_cost_m23_20261019/reports/reuse-20261020/RUN_REPORT.md) ([PDF](20261020-n83-prebuild-reuse/RUN_REPORT.pdf)) |

## Replace the existing N83 problem-index row

| [Primary N83 prebuilt-base quality for two to four summands](n83-prebuilt-base-quality-m2-m4.md) | open | D18 exported and all 2,760,082 points/labels replayed; finite m=2/m=3 same-set preparation and reuse accounting checked; sixteen smaller-base cache comparisons audited; full 16,627-class U/D eighteen-attempt panel audited (329 files), thirteen conflict and five wall caps | independently replayed natural useful-rank growth, qualified complete cold/reused IC costs, matched rho and base selection; retain every cap and interruption |
