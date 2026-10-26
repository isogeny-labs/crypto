# Matched order-four rho control for the GLS twist study

Frozen 22 October 2026 before implementing or running the producer. This is
a separate control experiment; the v1/v2 index-calculus runs remain immutable.

## Question

Does the same `psi²=[-1]` symmetry that folds the factor base also improve a
single-target Pollard rho baseline? Measure both transitions and complete
counted finite-field work. This control calibrates the GLS index-calculus
result; it does not test a polynomial relation solver or imply asymptotic
speedup.

## Instances and arms

Use the three exact twist models and prime subgroups from
`GLS_SYMMETRY_PROTOCOL.md`: `(p,r,cofactor)=(101,401,25),
(211,44221,1),(307,509,185)`. Recompute the generator and eigenvalue as
specified there, and verify `psi(G)=[lambda]G` and `lambda²=-1 mod r`.
Each target is one of `[37]G,[113]G,[271]G`; the walk receives only the
point. Use 32 paired public seeds `0..31`. No state or collision table is
shared across seeds or targets.

* `rho-negation`: canonicalise each state among `X,-X`.
* `rho-psi4`: canonicalise each state among `X,psi(X),-X,-psi(X)`.

Both arms use 16 identical public jump coefficients and the same starting
coefficients per `(p,target,seed,restart)`. The random partition hashes the
canonical point. After each jump, track both coefficients of `X=aG+bQ`;
canonicalisation multiplies each by the same `-1` or `lambda` power as the
point. A repeated canonical point gives a candidate scalar only when the
coefficient denominator is nonzero; verify every candidate directly on the
curve. On a fruitless collision or a cap of `8r` walk states, restart from
the next independent salt, up to 64 restarts. Retain any failure as a
failure. Rho is one-target, cold, and has no cross-target table.

## Prespecified measurements and interpretation

Record every target and seed, recovered scalar, success, states inspected,
collisions, restarts, group additions, `Fp²` multiplications and inversions,
`psi` evaluations, canonicalisation comparisons, table probes, modular
inversions and wall time. Charge a fresh `lambda` determination and
verification in every `rho-psi4` run. The public curve/generator/target
construction is input preparation for both arms and outside their clocks.
Independently replay the recovered scalar, the canonical orbit action and
raw-record invariants; report all 32 paired outcomes, failures, medians,
means, and paired ratios by target and curve. State whether quotient savings
in steps survive the extra `psi` field work. Compare to the earlier folded
IC only as an exact-count diagnostic, since that run did not use the same
rho binary or isolation envelope. Do not promote a leaderboard/benchmark
speed row without its separate validation gate and same-process IC pairing.

The null is no reduction in complete field-operation count despite fewer
expected quotient states. A transition reduction alone is a stage result.
The model prediction for ideal random quotient walks is `sqrt(2)` fewer
states for order four versus negation; small toy-group fruitless cycles can
depart from it. The three small subgroups and fixed seeds do not estimate
cryptographic-size behaviour.

## Post-run v2 control amendment

The complete v1 raw run is retained. Its negation arm failed on 14 of the
96 `(target,seed)` workloads at `p=211`, reaching the 64-restart cap;
`rho-psi4` solved all 96. This is a pathologically weak negation reference,
so no rho4 speed claim will be made from v1. Before coding or running v2,
freeze a stronger walk with **64** jump-table entries, **256** allowed
restarts, the same 32 seeds and all three target scalars. Keep every other
state update, charge, field model, collision check, and pairing rule above.
Retain v1 failures and full costs rather than censoring or replacing them.
If v2 still fails any target, retain that failure and do not use a
success-only speed ratio. A further reference must be specified separately.
