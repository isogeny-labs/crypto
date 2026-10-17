# Fresh N83 high-K base and target-policy comparison

This prospective Tier 1 study compares K2816 and K3072 root11 factor bases
under both two-worker join and cooperative cancellation. The primary question
is which **factor base under cancellation** has the shorter verified
one-target online interval. The join arms measure whether the factor-base
ordering depends on target-worker policy. Complete cold process cost is a
separate supplementary objective.

The four [exact candidate manifests](CANDIDATES.md) share source SHA-256
`8109fb33374ce2aa66c6a5a0b6e144d3b0143168ac756f9e0bd70af60a911829`.
The public 24-point file is [fresh_targets.jsonl](fresh_targets.jsonl), SHA-256
`5a234d93d748fafdf6ee0d00714d42446a309bfbbdfba9669b75831a87ad8090`.
The independent replay sidecar is [fresh_answers.jsonl](fresh_answers.jsonl),
SHA-256
`6ca85ac4f16622142ae7b4a56135bdca57077ba55ef01d90efd250dc9395e1f6`.
The deterministic scalar law, exact GF(2^83) curve/subgroup, and domain are
bound by [fresh_manifest.json](fresh_manifest.json). The
[exclusion audit](fresh_exclusion_audit.json) checked the 24 points against
46 prior public-target files containing 31,376 distinct points and the
separate 32-point finalist fixture at commit
`9b9a0f3842de6cf921e38573cb99fcaed2a4683c`; both overlaps were zero.
The answer sidecar is unavailable to the producer.

Run four independent Tier 1 cloud instances, each on six public points:
indices 0–5, 6–11, 12–17, and 18–23. Every candidate-point execution starts
with empty factor-base, index, and log caches. Rotate the four-arm order by
point index; alternate IC then same-point rho and rho then IC by point and
fixed arm index. The paired rho seed is
`2026110500 + 4*point_index + fixed_arm_index`, where fixed arm indices
0–3 are K2816 join, K2816 cancel, K3072 join, K3072 cancel. For each pair,
run three independent solves of the same
frozen calibration rho point before and three after. Compare the two medians;
the absolute difference divided by their mean must be at most 10%. Every
calibration and same-point rho answer must independently replay.

Each IC process has a 900-second cap and a 15.25 GiB RSS cap on a host whose
real cgroup limit and physical memory pass preflight. Each rho/calibration
process has a 120-second cap. The host producer cap is 300 minutes, and the
cloud job cap is 350 minutes. Record exact source/binary hashes, commands,
flags, target and answer separation, event order, topology, affinity, cgroup
limits, throttling, steal time, peak memory, OOM events, and provider runner
identity. Preserve every timeout, noisy pair, resource failure, and
incomplete stratum as recorded; none becomes a zero-cost observation.

Independent replay checks all 96 IC base constructions, ranks, four-point
relations and scalars, all 96 matching rho solves, and all 576 calibration
solves. It binds every extracted result to its provider ZIP and verifies
four distinct overlapping cloud runner allocations. Admit a stratum only
when every pair, calibration, resource envelope, and replay gate passes.
Pool points only after all four strata are admitted.

For the primary cancellation base choice, compute the 24 paired differences
`log(K3072_cancel_online_ms / K2816_cancel_online_ms)`. Select K2816 when
their mean is positive and the exact two-sided paired sign-flip p value is
below 0.05; select K3072 when the mean is negative and p is below 0.05.
Otherwise keep the selected candidate ID null with `BOUNDED_UNKNOWN`. Report
the paired mean, descriptive 95% interval using `t(0.975,23) =
2.06865761041904`, exact p over all `2^24` sign assignments with the
absolute mean as the two-sided statistic, point win count, every
online interval, probes, and same-point rho/IC ratio. Apply the same fixed
rule to complete cold process millisecond differences separately.

For secondary policy analysis, calculate the paired join-versus-cancel log
differences at each K and the pointwise interaction
`log(cancel3072/join3072) - log(cancel2816/join2816)`. Use the same exact
two-sided sign-flip test for these three secondary contrasts with Bonferroni
threshold `0.05/3`. Keep these policy results distinct from the primary
factor-base choice. Stage diagnostics include actual base sizes, folded
columns, exact index entries and allocation, setup/rank costs, target probe
counts, verified relation yield, online phases, and final scalar replay.

The fixture and decision rule are frozen here before dispatch. The versioned
runner and independent verifier bind the exact source and workload. The
provider binder and dispatch file must pass their own preflight before any
of these 24 public points are sent to a producer.
