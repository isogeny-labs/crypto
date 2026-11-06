# N83 batched-root extended finalist holdout

This experiment compares the earlier K992 and K1024 root11 factor bases and
the historical K3300 wide3 online finalist on the current batched-root
implementation with the independently selected online and cold finalists
from the sealed six-arm panel. It uses 16 new
public points of `EC1N83Ckb1a1haacb69ee9f37` (polynomial-basis
`GF(2^83)`, modulus terms `[0,2,4,7,83]`, curve
`y² + xy = x³ + x² + 1`, subgroup order `8569786107849059`, cofactor
`1128547018`). The [fixture](fresh_manifest.json) fixes the public points
before the six-arm panel reports a result. Its domain is
`n83-final-extended16-batched-roots-20261018-v1`; the scalar law is
`1 + little_endian(SHA256(domain || u32_le(i))) mod (r-1)` for `i=0..15`.
The answer sidecar is used only for independent scalar replay.

The [exclusion audit](fresh_exclusion_audit.json) found zero overlap with
31,488 prior public N83 points, including the six-arm panel's 32 points,
and zero overlap with the separately archived 32-point finalist fixture.
The target-file SHA-256 is
`10a0cb758b3dfcff8b79e06a2437c3786ee03049dba7e096e7858024cdd9e811`.

The 32-point finalist fixture is retained byte-for-byte in
[`historical_finalist_targets.jsonl`](historical_finalist_targets.jsonl),
from commit `9b9a0f3842de6cf921e38573cb99fcaed2a4683c` at
`research/n83_root11_finalists_tier1_20261017/targets.jsonl`. Its SHA-256 is
`4eb139e73c99f0921041a911d30698997913d1e71155a8e09035ffa647e02a58`.
The exclusion audit checks those bytes before comparing points, so the sealed
runner needs only the current shallow checkout.

The exact implementation is the same IC source as the six-arm panel:
SHA-256 `4a94e4e570f0cc4e39ba5c0846e411a6c43820c28119f7b9802e549023596507`.
All arms use `KIC_INDEX_ROOT_BATCH=1`, two target workers, cooperative
cancellation, and the same rank and root-index policy. The three new exact
candidate manifests are:

| Arm | Usable points | Folded columns | Candidate ID |
| --- | ---: | ---: | --- |
| K992 root11 | 164,672 | 992 | `IC1N83Ckb1a1fb164672PDP4rootRCguidedLAgaussTDdirectISO0ha6a31a64b52b` |
| K1024 root11 | 169,984 | 1,024 | `IC1N83Ckb1a1fb169984PDP4rootRCguidedLAgaussTDdirectISO0h644bda8bfc36` |
| K3300 wide3 | 547,800 | 3,300 | `IC1N83Ckb1a1fb547800PDP4rootRCguidedLAgaussTDdirectISO0h8d1e886f8586` |

K3300 enters because it led an earlier 4,000-point online comparison under
an older source. This amendment was made before any of the 16 holdout points
were measured. Its current-source manifest retains the same ordered base
digest and wide3 layout. The separate 636.799-second full-process pilot and
fresh archive extraction independently replayed its 547,800 base points,
four-point relation and scalar on an already used point. These pilot timings
do not enter the fresh-point selector.

Before dispatch, independently replay the four-stratum six-arm provider
archives with `research/n83_online_finalists_20261018/aggregate.py`, then
recompute its frozen `select.py` output. Choose these holdout arms in order:

1. K992 root11, K1024 root11 and K3300 wide3, using the exact manifests above.
2. The six-arm `online.selected_candidate_id` only if its status is
   `SELECTED`; otherwise its prespecified K2304 root11 control.
3. The six-arm `cold.selected_candidate_id` only if its status is
   `SELECTED`; otherwise its prespecified K1107 root11 control.
4. Deduplicate exact candidate IDs while preserving order. The resulting
   panel has four or five arms. Do not choose an arm from descriptive means
   when a selector status is `BOUNDED_UNKNOWN`.

Freeze the chosen IDs, source, public points, per-arm resource caps, rho
policy, runner, verifier, selector and workflow hashes in a dispatch file
before cloud timing. Use two distinct, overlapping Tier 1 cloud allocations:
points 0–7 and 8–15. Rotate arm order by point index and alternate IC/rho
order by point and fixed-arm parity. Each candidate-point pair starts a
fresh IC process with empty process caches and solves that same public point
as a separately seeded rho process. Run three fixed-point rho calibrations
before and three after each pair. Preserve complete raw receipts, including
timeouts, noise, OOM and partial production. Require at least four visible
CPUs, enough memory for a 15.25 GiB child RSS cap, cgroup and affinity
telemetry, zero observed steal, and at most 10% drift between the two
calibration medians for every pair.

Cap each K992, K1024 or K1107 IC child at 600 seconds;
K2304, K2816 or K3072 at 900 seconds; and K3300, K3450 or K3500 at 1,200
seconds. Cap rho and calibration children at 120 seconds, each producer at
330 minutes, and each cloud job at 360 minutes. A four-arm panel schedules
64 IC, 64 same-point rho and 384 calibration children; a five-arm panel
schedules 80, 80 and 480 respectively.

The primary measure is verified one-target online IC wall time after reusable
base, index and log setup, paired with verified rho online wall time for the
same point. Fresh-process cold cost is a separate selector. Charge every
target-dependent attempt and verification phase; retain setup, rank,
relation, target, resource and correctness receipts. For every unordered
candidate pair, compute the mean of 16 paired `log(second/first)` online
times and the mean of paired `second-first` cold milliseconds. Enumerate all
`2^16` sign flips for each two-sided test. Select the fastest geometric-mean
online candidate only if it beats every other arm and each of its pairwise
tests passes `0.05 / binomial(arm_count, 2)`. Apply the same rule to cold
arithmetic means and paired cold differences. A missing, failed, capped,
unverified or noisy row leaves both selections null. Report descriptive
paired intervals and each candidate's same-point rho/IC ratios by host.

The existing K992/K1024/K1107 ten-point Tier 1 panel selected K1107 for
fresh-process cold time under an older source. This holdout tests the two
smaller bases and the historical K3300 online contender on the current
source and new points. The six-arm result is a selection among its six
declared bases; this extension adds both earlier frontiers under matched
source and fresh inputs.
