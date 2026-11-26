# Warmup-only preflight outcome and prospective repairs

All **36/36 calls** completed on **12 distinct public targets**, with identical
per-target certificates and structural counters across parent/candidate/A/A.
All 36 external Native replays, 36 OpenSSL positives, 36 wrong-scalar rejections
and 36 substituted-target rejections pass. The 239-entry post-call manifest
matches. Unsigned outcome record: `PCC-20261123-a79757`. [pcc:unverified]

Phase 05 ran 13:37:24–13:39:07 UTC on 23 November 2026, exit 0, no physical stop.
The sampled whole-cgroup peak was 13,817,049,088 bytes, including retained other
research. This is not per-process peak RSS. The 103-second coarse wall span
supports a conservative 1300s main planning estimate using 104 × 10 × 1.25.
All preflight observations are warmups; none is a main sample. [pcc:unverified]

Review [11](reviews/PREFLIGHT-DESIGN-REVIEW-11.json) retains four real limitations
of the successfully executed preflight: nested timeout groups, failure-path
finalization/clock validation, unpinned local thin launchers and per-target A/A
order imbalance. The successful run did not test those failure paths. Original
043 preparation, source files, manifests and raw records remain unchanged.
[pcc:unverified]

Separate V2 source files and wrappers fix these prospectively. Ten controls pass;
controlled failed-worker and group-signal fixtures behave as required. A later
source-pinned clean-target build reproduces all four earlier V2 executable bytes,
then passes copied-unit/source/binary checks. Review
[12](reviews/MAIN-READINESS-REVIEW-12.json) preserves the original V2 pre-unit
source-custody gap and the remaining prelaunch gates; it does not retroactively
accept the later reproduction. [pcc:unverified]

V2 analysis of an exclusive copy of valid preflight data reproduces the original
analysis/CSV/SVG bytes. A separate synthetic measured-summary fixture duplicates
the 36 warmups into 360 metadata cells to exercise all 36 nine-value groups and
min/median/max calculations. It adds no solver calls or timing observations and
is excluded from the main data. [pcc:unverified]

See [MAIN-PROTOCOL.md](MAIN-PROTOCOL.md) for the fixed main census, complete
control rules, host/source custody and enforced phase budgets. The main panel
has not yet started at this preparation snapshot. The earlier 24-call cohort,
historical round24 results and round25 cloud protocol/attempt are unchanged.
