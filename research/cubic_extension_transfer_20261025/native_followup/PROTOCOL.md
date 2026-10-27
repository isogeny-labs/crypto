# Native follow-up protocol

Requested scope: prove or disprove the universal extra-step assertion after
torsion preparation using joint obstruction square classes; complete exceptional
input handling; run isolated repeated comparisons. A partial diagnostic does not
discharge any of these requirements.

## Fixed mathematical scope

Ordinary non-subfield E/F_(p^3), p >= 5, p = 1 mod 3, prime full group
order r > 3 with r = 4 mod 9. A prepared curve has scalar -I Frobenius
on E[3]. The assertion concerns one additional rational degree-3 isogeny to
another prepared curve and the twelve transverse quotient kernels. It does
not concern arbitrary covers or arbitrary isogeny degrees.

## Frozen execution plan, before new computation

1. Implement a native FLINT evaluator of the theta obstruction, using historical
   exact curves as fixtures. Check usable-kernel counts, all-negative centers,
   and eligible neighbors. Investigate square-class dependencies; record a
   universal theorem only with an algebraic proof, or a counterexample only
   with exact subgroup and quotient-order certificates. Neither a finite search
   nor a fitted sign relation establishes a universal identity.
2. Implement projective closed-divisor recovery with multiplicities and local
   jets, including support at infinity. Specify separately the unimplemented
   inverse-map strata and any proof needed to guarantee common-multiplier
   success. Do not describe divisor recovery alone as complete transfer.
3. Reuse the std-only native quartic arithmetic from PR #2170, pinned at
   e5d1f02aeee957f2517d2cee5afe6a63bb36c803, blob
   94a2465cfffe10d533b8e9f13a5629837b3bd908. Native fixture adapters and
   counters must verify every recovered scalar. Request at least five independent
   seeds per implemented arm and matched A/A repetitions. Preserve every failure.
   Full cold pipeline includes construction, sections, inversion, recovery,
   target solving, verification, retries, and failed attempts. Stage replays
   must be labeled stage replays. No unavailable phase is assigned zero cost.

Native execution only: preserve all Sage sources and receipts unchanged.
Record compiler, host, source/binary/input digests, CPU affinity, exclusive
lock, PSI, other-process CPU, context switches, faults and contention.
Never pool contended and uncontended timings. Use a native replacement for
the repository's historical isolation harness if necessary. Do not claim
external-tool superiority, strongest-rho qualification, independent replay,
leaderboard admission, or a speedup unless the corresponding gate is satisfied.

Initial per-process diagnostic bounds: 30 minutes and 4 GiB. Before timed
measurements, freeze exact fixtures and arms in a separate run manifest.
No paid cloud instances are requested or launched. Do not build or run other
experiments on this host during a timed stage.

## Repository integration

Conductor check attempted before edits; executable is unavailable. The workflow
reference .conductor/WORKFLOW.md is absent at current main (API 404). Edits are
scoped to this new follow-up directory and the existing admission problem.
Update existing PR #2545 rather than replacing the historical evidence. Existing
benchmark/leaderboard rows stay unchanged without validation-gate evidence.

The historical 10.87/91.43/1.0352-second observations are not native baselines
and are not used as a matched comparison in this protocol.
