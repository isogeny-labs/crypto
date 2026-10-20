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

The bounded local K64 debug process controls for load75 and load80 each
reached the 180-second cap before an index receipt. Their status is
`TIMEOUT_180S`; they provide no ordering or capacity measurement.

The next comparison should first run K4405 under load75 and load80 on the
same exact source and factor base, then compare K4500 and K4549 under load80
on frozen public points. Each new base needs its enumerated point digest,
verified subgroup usability, distinct-point count, manifest and candidate
ID. Record failed build/index/target attempts, cold setup and index costs,
one-target online intervals, same-point rho, and independent base, relation,
and scalar replay. Use the repository's Tier 1 cloud-instance gate for any
wall-time ordering. Until those receipts exist, the new arms have
`candidate_id: null` and `selected_best_total_runtime: null`.
