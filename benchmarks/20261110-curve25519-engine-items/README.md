# 20261110-curve25519-engine-items

Copies of the figure and report of `research/curve25519_engine_items_20261110/` (two engine options
on the Curve25519-family miniatures, k = 20 and 24, single-target ecbench: (A) `wwalk cv=0`, the
target-in-walk arm without per-collision recomputation; (B) `rho.negation_chained`, a chained
jump set-up for the rho-neg reference). Not a `benchmarks/README.md` row: no validation gate was
passed and the arm is not decomposition-based index calculus. (A) measured 0.879x and 0.949x the
current arm; (B) measured 1.53x and 2.74x rho-neg and is rejected as a reference. Records:
`benchmarks/records/20261110T0357Z-curve25519-engine-items-k20.json`,
`benchmarks/records/20261110T0357Z-curve25519-engine-items-k24.json` [pcc:unverified].
