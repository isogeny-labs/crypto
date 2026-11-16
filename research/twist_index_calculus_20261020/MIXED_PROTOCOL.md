# Mixed-component extension-field experiment: frozen protocol

This protocol was written before the mixed-component runs. The earlier
`results.json` and `isogeny_control.json` remain historical evidence.

## Question

For a DLP in `E(F_p)` with odd prime order, can a structured factor base of
genuinely mixed points of `E(F_{p²})` increase the number of usable relations,
their independent rank, or the complete cost of solving one public target?

## Fixed workload

Use the six curves `(p,a,b,#E,#T)` below, with
`E:y²=x³+ax+b`, `T:d*y²=x³+ax+b`, and the least quadratic nonsquare `d`.
Recount both groups and check primality of `#E` before each run.

| p | a | b | #E | #T |
| ---: | ---: | ---: | ---: | ---: |
| 101 | 3 | 17 | 107 | 97 |
| 211 | 1 | 1 | 223 | 201 |
| 307 | 1 | 6 | 331 | 285 |
| 509 | 1 | 17 | 503 | 517 |
| 1009 | 1 | 14 | 1013 | 1007 |
| 2003 | 2 | 15 | 1933 | 2075 |

Choose the smallest-abscissa signed rational points `A_i` and the first
nonzero twist points `T_1,T_2`; set `T_3=T_1+T_2`, retrying `T_2` if the
three are zero or equal up to sign. The mixed points are `A_i+φ(T_j)`,
where `φ(x,y)=(x,u*y)` and `u²=d`. Check each factor's two Frobenius
components, its trace, and its actual extension-field coordinates.

For each `p`, run base sizes `B=12,24,36` where the curve has enough
distinct signed rational points. Use three one-target workloads, with
target scalar derived by a fixed integer mixing function from `(p,B,seed)`;
the solver receives only target coordinates. Each workload has four arms:

1. `native-B`: `B` distinct `A_i`, no twist component.
2. `native-trace`: the `B/3` distinct `A_i` of the rectangular mixed base.
3. `mixed-rect`: all `B/3 × 3` pairs `(A_i,T_j)`.
4. `mixed-unique`: `B` distinct `A_i`, assigned to the three twist colors in
   blocks of `B/3`.

The native arms use the same exact signed width-three pair-table oracle as
the mixed arms. Report both raw mixed relation multiplicity and distinct
trace-projected rows, and matrix ranks on raw and trace-collapsed columns.

## Complete DLP trial

For each arm, build the factor base and pair table, then query the
target-independent sequence `[a]P`, `a=1,2,...` until the trace-column
matrix has full rank. Count every trial, signed point addition, pair-table
entry, probe, hit, row reduction, modular inverse, and relation witness.
Fail after `r-1` trials. Only then start the online interval: query
`Q+[b]P`, `b=0,1,...`, until one exact extension-field decomposition is
found, derive `k` from solved trace logs, and verify `[k]P=Q` independently.
Fail after `r` online queries. Include setup, relation collection, linear
algebra, descent, and verification in the complete cost. Report online cost
separately. No known target scalar enters the solver.

The exact group law is computed in the `(E(F_p),T(F_p))` representation of
`E(F_{p²})`; this is an isomorphism here because both group orders are odd.
Check the representation against direct `F_{p²}` curve arithmetic on all
factor points and sampled relation witnesses. Count both component group
laws separately; also record wall time, which is descriptive until paired
inside the repository's isolated `ecbench` harness.

## Acceptance and interpretation

- Every recovered scalar must independently reproduce its target.
- Every collected mixed relation must satisfy both component equations.
- All matrices must be reduced modulo the stated prime group order.
- Preserve failures and rank stalls. No speed claim from an incomplete run.
- Compare raw relation yield, unique projected rows, rank, setup, online,
  and complete cost per one-target workload. Do not pool targets as one DLP.
- A formal `vs_rho` speed claim additionally needs the repository's
  `ecbench` sealed same-target session and independent replay; standalone
  measurements alone do not satisfy that gate.

## Recorded amendment after the first run

The frozen full-rank rule failed for every `mixed-unique` case. The exact
all-target census showed rank `B-2` for that arm, caused by the three
twist-color balance equations. The complete original outputs are kept in
`mixed_full_rank_attempt.ndjson` (SHA-256
`8ec1c26c7905ec38f771d6b70023ec9c6247d9044a27f80af0658921109074b2`).
The arm was not silently removed or counted as a completed DLP.

The finalized source has a `--full-rank-control` mode that reproduces the
same rank failure under the final arithmetic and accounting implementation;
its output is `mixed_full_rank_control.ndjson`. The earlier raw file is
kept separately because its intermediate source revision was not saved.

A second, explicitly different run will stop setup when the known
twist-color row-space dimension `B-2` is reached and evaluate a target
decomposition only when its trace row reduces to zero against the setup
basis. This is a target-row span attack: it need not solve all factor-base
logs. All other arms retain the full-rank criterion. The original
full-rank failure remains an outcome of the requested rank experiment.
