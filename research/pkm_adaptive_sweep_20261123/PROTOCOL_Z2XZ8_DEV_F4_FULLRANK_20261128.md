# Development F4 full-rank follow-up

Frozen before launching these six runs. This is a development-only follow-up
to the corrected 64-trial equal-size panel
`RESULT_Z2XZ8_DEV_F4_EQUAL_SIZE_20261128.json` (SHA-256
`4437722a94f0a4954175107e0eebef5027248dc19a09f9be72788fc913e6872c`).
Its three `t5_c0..c2` curves, first target on each, and relation seeds are
fixed by `RESULT_Z2XZ8_JOIN_BUILDERS_20261128.json` (SHA-256
`f6ea41b675302fc64a618c59c51fc771b54c6d43eca216d36c5b4c9faf3cbbba`).
They are development curves and cannot replace the six held-out curves.

Use corrected source `5f8ddfd0ff43d151d7b35c5d81003905c59d12fc` and
binary SHA-256
`efe776236bea16b4f0bb31cbdaa2c8e83216b5914a337b4054a8f317106b9bf8`.
For each curve launch fresh processes for the selected equal-size
`prime-isogeny:t=5,seed=0,aux=ordinary,family=z2xz8,selection=equal-size`
and exact interval `prime-interval:t=5,lo=0`, both with the m=2 F4 chain
oracle, full-rank checked incremental Gaussian elimination, the identical
target and relation seed, and one matched same-target rho run. Run the
selected arm first, then the interval arm. Each process has at most 20,000
relation trials, a five-second cap per solver call, 120 seconds wall, and
2 GiB RSS. Keep every failed and censored arm. Verify base sizes before
comparing. A recovered scalar is accepted only after independent point
verification and full-rank checks.

The host is contended. Record process wall, CPU and RSS but do not use them
as qualified cold time or pool them with the held-out panel. Report solver
operations, all trials and relations, rank, native unpriced work and
same-target rho separately. The result can guide the next map iteration;
it cannot satisfy the two-rung promotion gate or establish complete-cost
advantage.
