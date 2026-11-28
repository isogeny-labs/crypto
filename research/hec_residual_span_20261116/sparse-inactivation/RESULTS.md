# Actual sparse producer completes six fixed workloads

Pushed freeze60e22f8e941, one invocation exits0; all input hashes pass.
No retained ROOTS/support file is read. Seed rule always orbit1, with formal
unknown S, not a supplied seed log. Exact original-group rows/affine labels,
constraint-vs-originalSparse prefixes, terminal nativeFast and final scalars
all pass on the six unchanged original inputs.

|p|source multiple|anchor scans|point visits|singular skips|rows|labeled orbits at stop|k|
|---|---|---|---|---|---|---|---|
|101|1|4|55|0|6|6|71|
|101|17|2|25|1|5|5|6092|
|251|1|5|90|4|11|10|8453|
|251|17|5|120|0|10|10|44840|
|601|1|16|385|2|24|24|104551|
|601|17|7|165|0|12|12|188934|

Each run stops during first16anchor pass, so no extra whole-panel rescan was
needed here. All16anchors are constructed before scanning even for early
completion; this setup is real work. Point visits count canonical abscissas
of current labeled pool members, with both signs evaluated after one batched
inverse. No hidden fullFB filtering per anchor: orbit-member arrays provide
direct iteration. Sparse buckets need two distinct known abscissas; exact
cubic division leaves a linear factor and extends the symbolic label pool.
All observed >=2point buckets yielded admitted rows on these inputs.

This removes the free-support oracle dependency and gets past the initial
p601source1stopping set using symbolic seed1. It also covers source1p601
without waiting for the earlier target-bearing rational promotion event.
It does not prove a runtime or tenfold gain. Complete factor-base/action
setup, all16anchor generation, batch multiplications, bucket allocation,
symbolic labels/constraints, row/reference work and final verification are
not priced by point visits. Singular exclusions remain explicit; no general
completeness or fresh-target success rate follows from six reused fixtures.

Next: freeze a cold native-counter comparison with the completing strong
full-scan Sparse control on fresh targets. Equal public-source information,
fixed seed1, all producer/setup/LA/final work charged. Diagnostic reference
checks outside count interval in both arms. Include failures and no retries;
count both base and subgroup operations separately, no arbitrary weighting.
Only a genuine complete-cost advantage warrants a wall experiment or genus3/4
extension. Classical interpolation/inactivation, no novelty claim.
