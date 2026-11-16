# First local epoch: source-stamp mismatch

This is a descriptive run report. It makes no performance verdict or
scientific state transition. Session `ECBS1h3cf526729f36` used the frozen
fresh-target spec `ECS1hb376719703c9` on the registered Koblitz E0
`F_(2^13)` curve, subgroup order 2003, eight public targets, four measured
rounds, one warmup and five arms. It wrote 200/200 records and 200/200 exact
verified answers [pcc:unverified]. The [replay receipt](../../audits/replay-all.json) reports
160/160 measured deterministic replays identical, with zero audit problems.
There were no failed or timed-out executions.

The macOS Apple M4 Pro host earned L0 on every execution. The binary SHA-256
was `c4b63be34e52621d5d43e9cfcb2107f596b1148c7d06715fb7560dcae32e5d80`.
The recorded git commit is `557f28ef...`, sourced at runtime from the
binary's other worktree path, while the protocol commit is `984cf46d...`.
The compiled Rust trees match, but the run's commit field misses the protocol
binding. [Amendment 1](../../AMENDMENT-1.md) records the exact discrepancy and
the custody-only rerun. The benchmark record is invalid for performance
admission; the observed numbers below remain visible.

Measured medians over 32 runs per arm, complete cold process wall:

| arm | median ms | work rows complete |
| --- | ---: | ---: |
| batch m=3 | 7.714 | 32/32 |
| batch m=3 A/A | 7.674 | 32/32 |
| scalar rootfinding m=3 | 7.589 | 32/32 |
| F6-IC | 38.935 | 32/32 |
| strong rho | 7.370 | 32/32 |

Derived from the 32 exact target-round pairs, the ratio of summed batch/F6
complete process wall is 0.1942; all 32 individual pair ratios are below 1.
Batch/A/A is 0.9988 and batch/scalar is 1.0065 by the same sum rule. These
are L0 observations with a source-stamp defect, not an admitted advantage.
The [SVG](paired-ratios.svg) graphs eight target-cluster ratios from the
raw records; [analysis.json](../analysis.json) retains the exact arithmetic.

The session files live in
`../../sessions/local-fresh-seed-202611160401/`; stderr and stdout are
`../../run.stderr.log` and `../../run.stdout.log`. The complete wall includes
process startup and all phases. The phase medians in `analysis.json` show
F6-IC relation work dominates its method call here; batch table setup and
relation search are much shorter. Hardware load was high and there was no
CPU reservation. The frozen EKS F4/F5/F6 panel and independent replay are
still required for a qualified fixed-panel comparison.

Canonical graphs checked: `docs/ic/LEADERBOARD.md`,
`docs/backend-leaderboard/LEADERBOARD.md` and the frozen combined panel's
`panel.svg`; none changes from an invalid L0 diagnostic.
