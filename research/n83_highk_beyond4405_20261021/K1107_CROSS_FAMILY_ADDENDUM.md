# Current-source K1107 cross-family holdout addendum

The current source reproduces the K1107 root11 base and completed a full
process control on an earlier public fixture. The local process reached rank
1,107, recovered and independently replayed that fixture scalar, and recorded
334.80775 ms of exclusive target work. The current-source candidate is
`IC1N83Ckb1a1fb183762PDP4rootRCguidedLAgaussTDdirectISO0hf1851768e85a`;
its manifest SHA-256 is
`67cbd5d4f12ab5bfe703714ab66f0c779b5d708aaa79a6399b13d4217b4f9176`.
The control establishes the source bridge and executable path for the frozen
Tier 1 comparison. Its public fixture is excluded from the sealed holdout.

This addendum freezes the cross-family holdout extension before any Tier 1
holdout outcome. The original [PROTOCOL.md](PROTOCOL.md) screen, four arms,
source-bound dispatch, 1,536 public screen points, and 16 sealed one-target
holdout points are unchanged. The selected fixed31 high-K candidate and K1107
use the same freshly built source and binary on each holdout point. Each gets
its own same-point rho solve, separate process, fresh setup, and three rho A/A
calibration processes before and after the candidate/rho pair. The high-K
candidate runs first at even point indices and K1107 first at odd indices.
Within each candidate pair, IC/rho order alternates by point and arm. This is
256 ordered events, starting with high-K at the preregistered primary point 0.
Rho seeds are `2026120300 + 2 * point_index + arm_index`, with arm index 0
for high-K and 1 for K1107. The original 2,400-second per-child cap, 26 GiB
child RSS cap, four pinned CPUs, Tier 1 telemetry/noise gate, and ten-hour Pod
deadline apply to every event. Preserve every capped, missing, failed, noisy,
or unverified row as `BOUNDED_UNKNOWN`.

Point 0 is the primary cross-family result. Each candidate must recover its
scalar and pass independent group replay, exact source/base/index/rank checks,
phase accounting, same-point rho replay, resource checks, zero observed CPU
steal/throttling, and the 20% before/after calibration gate. Its row reports
the exclusive single-target IC and rho online wall times and their ratio,
with cold process and preparation separately. The primary cross-family row
reports `K1107_IC_online_ms / selected_highK_IC_online_ms` only if both
candidate rows pass. The producer's online phase includes its scalar recovery
check; the independent post-run verifier is recorded separately. Each of
points 1–15 remains an individual one-target replication with its own
candidate/workload/run IDs and correctness receipts.

For the secondary 16-point cross-family selector, require all 16 matched
candidate pairs to pass. Drop exact timing ties for an exact two-sided sign
test at strict `p < 0.05`. A candidate is selected only if it has more pairwise
wins, a lower geometric mean IC online time, and the sign threshold passes.
Retain all 16 paired timings, signs, geometric means, exact p-value, and a
descriptive 95% normal approximation interval on paired log ratios. Missing
or inconsistent pairs yield `BOUNDED_UNKNOWN`. The point-0 result remains the
headline single-target comparison; this 16-point selector describes
replication and does not replace that metric.

The K5500 split-index finalist still requires a complete current-source
index, rank, target, and independent replay gate before any campaign-wide
N83 factor-base selection. The fixed31 screen selects only within its frozen
monolithic frontier; this addendum adds a direct K1107 comparison to the
same sealed one-target workload.

The Pod-side selector is provisional until the external controller transfers
the SHA-256 checked raw archive, terminates the Pod, and records confirmed
provider absence. `verify_provider_custody.py` binds these local receipts to
the same allocation and checks the five-minute retirement margin. Promote a
Tier 1 row only when both scientific replay and provider custody pass.
