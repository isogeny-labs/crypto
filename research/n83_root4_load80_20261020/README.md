# N83 five-byte root index at 80% target load

The opt-in `KIC_INDEXED5_LOAD80=1` request changes the indexed five-byte
root-table allocation to `ceil(5*S/4)` for `S` retained S3 states. With the
existing constructor's `next_power_of_two(2*request)` slots, the exact
Koblitz N83 `a=1` symmetric-state bound keeps a `2^31`-slot, 10 GiB table
through K4549. K4550 crosses to `2^32` slots. The wide3 Bloom filter also
remains at 2 GiB through K4549. The default load75 path remains available,
and both flags together fail before index construction.

The [capacity projection](capacity_projection.json) uses the observed K4405
peak RSS of 22,356,250,624 bytes and its separately recorded table, Bloom,
and retained-state allocations. It projects 21.18 GiB peak RSS for K4500
under the same overhead. This is a sizing diagnostic; an actual K4500 or
K4549 process must pass the 24 GiB RSS and exact-output gates before entering
a timing comparison.

The historical RSS anchor is the K4405 arm in `points[0]` of
[`prior_highk_validated.json`](../n83_highk_online_distribution_20261020/prior_highk_validated.json),
at `raw_replay.resources.peak_rss_bytes`. It is one retained observation,
not a bound on other points or future allocations. The projection's
`source_sha256` identifies the historical example at commit
`6708007873f1868246b19b083bd93aea28560a99`, whose SHA-256 is
`4a94e4e570f0cc4e39ba5c0846e411a6c43820c28119f7b9802e549023596507`.
The load80 implementation added in `8d107823a0d36b806b7292eba0008fba515ccf0d`
has SHA-256 `716b4a688c23e4c02d852a6bfc5fca90d706ed3d4a922aec4894416e147ab9d4`.
These source bindings distinguish the anchor from the new prototype. The
five projection rows recompute exactly from the displayed formulas and
the fixed overhead; that arithmetic check does not independently replay
the historical run or measure load80 memory use.

The author reported bounded local K64 debug process controls for load75 and
load80 reaching the 180-second cap before an index receipt, labelled
`TIMEOUT_180S`. Their raw command, process outputs, and termination receipts
are not included in this three-file prototype, so those observations are
not independently verified here and provide no ordering or capacity measurement.

The next comparison should first run K4405 under load75 and load80 on the
same exact source and factor base, then compare K4500 and K4549 under load80
on frozen public points. Each new base needs its enumerated point digest,
verified subgroup usability, distinct-point count, manifest and candidate
ID. Record failed build/index/target attempts, cold setup and index costs,
one-target online intervals, same-point rho, and independent base, relation,
and scalar replay. Use the repository's Tier 1 cloud-instance gate for any
wall-time ordering. Until those receipts exist, the new arms have
`candidate_id: null` and `selected_best_total_runtime: null`.
