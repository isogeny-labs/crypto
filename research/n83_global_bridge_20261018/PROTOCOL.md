# N83 selected-finalist same-point bridge

This bridge resolves the single-target online factor-base choice between the
winner of the six-arm/extended finalist path and the winner of the independent
K3300/K3500/K3596 root4 reserve. It uses 16 new public points of
`EC1N83Ckb1a1haacb69ee9f37`: polynomial-basis `GF(2^83)`, modulus terms
`[0,2,4,7,83]`, curve `y² + xy = x³ + x² + 1`, subgroup order
`8569786107849059`, cofactor `1128547018`. The point file SHA-256 is
`a767c6acb37174593b8dd50cddfac7ed5be02498a00d199754a234606a8cbd64`.
The scalar rule is
`1 + little_endian(SHA256(domain || u32_le(i))) mod (r-1)` for `i=0..15`,
with domain `n83-global-bridge16-batched-roots-20261018-v1`. The scalar
sidecar is reserved for independent replay. The
[exclusion audit](fresh_exclusion_audit.json) confirms zero overlap with
31,520 earlier public N83 points, including the 16-point root4 reserve,
and zero overlap with the separately archived 32-point finalist fixture.
These inputs were sealed while both upstream runs were pending.

## Selection rule fixed before upstream outcomes

First, independently replay all upstream archives and provider job records.
The finalist path must produce an admitted `online.selected_candidate_id`
through the extended holdout. The root4 reserve must separately produce an
admitted `online.selected_candidate_id`. Bind the exact two validated result
files and selection files by SHA-256 in the dispatch. Preserve their original
candidate manifests, executable source hashes, and run IDs.

If either online selection is null, record `UPSTREAM_UNRESOLVED` and do not
dispatch. If both IDs are identical, record `CONVERGED_IDENTICAL` with that
candidate ID and both upstream evidence links; the bridge has no distinct
arms to compare. Otherwise, freeze the ordered pair as finalist winner first,
root4 winner second, with no descriptive substitution. A selected candidate
must have one exact manifest and executable policy; any duplicate ID with
different manifests fails the dispatch.

## Tier 1 paired experiment

Use two distinct overlapping Tier 1 cloud allocations, eight points per
allocation. Rotate arm order by point index and alternate IC/rho order by
point and fixed arm. Each candidate-point pair starts a fresh IC process
with empty caches and a separate same-point rho process. Run three fixed-point
rho calibrations before and after every pair. Preserve every timeout, OOM,
noisy pair, and partial receipt. Require at least four visible CPUs, enough
memory for a 15.25 GiB child RSS cap, cgroup and affinity telemetry, zero
observed steal, and at most 10% before/after calibration median drift per
pair. Cap each IC child at 1,200 seconds, rho and calibration children at
120 seconds, each producer at 300 minutes, and each cloud job at 330 minutes.
The two-arm schedule has 32 IC, 32 same-point rho, and 192 calibration
children. All arms run on every point and host; no candidate is dropped
after observing an early result.

The primary measure is independently verified one-target online IC wall time
after reusable preparation, paired with independently verified rho online
wall time on the same public point. Fresh-process cold cost is supplementary.
Charge all target-dependent attempts and verification phases. For the two
distinct candidates, compute the mean of 16 paired
`log(second/first)` online times and enumerate all `2^16` sign flips for a
two-sided test. Select a faster candidate only if the signed mean favors it
and exact `p < 0.05`; otherwise leave the selected ID null with status
`BOUNDED_UNKNOWN`. Report the geometric mean online cost, pointwise paired
ratios to rho, per-host intervals, setup/rank/index costs, and the separate
paired cold difference. A missing, capped, failed, unverified or noisy row
leaves the bridge selection null. All rows use canonical `IC1...W...R1`
identifiers with the exact public point, host and run receipt.

The two-arm bridge identifies the fastest admitted online candidate within
the finalist and root4 families actually screened. Any later candidate
family requires its own frozen, same-point challenger against that selected
candidate before it enters the result.
