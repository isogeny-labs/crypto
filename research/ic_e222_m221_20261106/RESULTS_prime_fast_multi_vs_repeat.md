# Multi-target IC vs a real run of `ic_solve` once per target (no extrapolation)

Status: [pcc:unverified] producer's own measurement; exploratory (not frozen before the data); same machine, 1 thread, wall clock; not independently replayed.

Purpose: the multi-target gain over repeated single-target `ic_solve` (134x to 570x at T = 1000) used a reference extrapolated from 5 solves. `examples/prime_fast_multi_vs_repeat.rs` runs `ic_solve` (summands 3, c = 0.5, +6 extra relations) for every one of T = 200 targets and times the total, then times `ic_multi_solve` (sweep c = 1.0; table c = 2.0) on the identical planted targets. Every target of every arm is verified `k G = Q_t` (200/200 in all 12 arm-runs).

| bits | res | repeated `ic_solve` ms (T = 200) | multi sweep ms | multi table ms | repeated / sweep | repeated / table |
|---|---|---|---|---|---|---|
| 25 | 3 | 3715 | 38.5 | 47.4 | 97x | 78x |
| 25 | 1 | 3899 | 31.8 | 41.2 | 122x | 95x |
| 27 | 3 | 12746 | 146.3 | 134.5 | 87x | 95x |
| 27 | 1 | 12682 | 132.8 | 124.7 | 95x | 102x |

Result: with the repeat arm really run, multi-target IC is 78x to 122x faster than repeating `ic_solve` at T = 200 (25 and 27 bits, 4 instance-rows). The amortization gain is real; it is smaller than the extrapolated figure at T = 1000 only because T is smaller here (the setup is paid once and the per-target cost is the floor). At T = 200 the table collector is not faster than the sweep at 25 bits (its pair-table build costs more); it is faster at 27 bits and at T = 1000 in the frozen evaluation.

Limits: 4 instance-rows, T = 200, 25 and 27 bits only; one repeat of each; no validator replay; the comparison does not touch the distance to BSGS or rho (see the tuned BSGS results: 4.6x to 15x behind).
