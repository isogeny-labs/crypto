# PKM degree-32 transfer gate: s59, P-192 and NIST prime fields

Date: 2026-11-24. Status: exact construction arithmetic and derived cost gate; no wide relation run. The frozen calculation is in `PROTOCOL.md`, executable in `wide_transfer.py`, and raw result in `results.json` (SHA-256 `0cc5faa68e8a3458416623888289323a0b5c9f09ebe2ff13f2bcef0bdfca0961`). Script SHA-256 recorded by the result is `7ff02a29bb330bb76fd57b5eb663d521f47176463509022286be7e2d2b8afe00`.

## Construction identities

The [Petit–Kosters–Messeng paper](https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf), §§3.1–3.4, asks for a high-degree membership map decomposed into low-degree maps. Three families must remain distinct:

| Family | Low-degree map | High-degree map | Condition |
|:--|:--|:--|:--|
| Kummer | `y -> y²` | `(x/g)^(2^t)-1`, degree `2^t` | `2^t | p-1` |
| Norm-one/Dickson | `y -> y²-2` | `D_(2^t)(x)-c`, degree `2^t` | `2^t | p+1`; separate from PKM §3.4 |
| Auxiliary elliptic isogeny | degree-2 Vélu x-map | composition of `t` maps, degree `2^t` | search an auxiliary curve with a point of order `2^t`; no direct `p±1` condition |

The repeated 13-bit lead used the third family with **five degree-2 maps, total degree 32**. The older wide pilot used the first two families at depth at most two and therefore did not execute the same map.

## Exact m=2 degree gate

For the paper's `m=2` heuristic, expected decompositions per target are approximately `lambda=d²/(2p)`. A constant expected count asks for `d >= ceil(sqrt(2p))`; for a power-of-two tower the required height is `t_req=ceil(log2(ceil(sqrt(2p))))`.

| Field | bits | `t_req` | `v2(p-1)` | `v2(p+1)` | best `p±1` side | missing height | derived reciprocal trials at best available `p±1` height | derived reciprocal trials at degree 32 |
|:--|--:|--:|--:|--:|:--|--:|--:|--:|
| s59 minus | 59 | 30 | 16 | 1 | `p-1`, t=16 | 14 | about `2^27` | about `2^49` |
| s59 plus | 59 | 30 | 1 | 18 | `p+1`, t=18 | 12 | about `2^23` | about `2^49` |
| s59 registry | 59 | 30 | 7 | 1 | `p-1`, t=7 | 23 | about `2^45` | about `2^49` |
| NIST P-192 | 192 | 97 | 1 | 64 | `p+1`, t=64 | 33 | about `2^65` | about `2^183` |
| NIST P-224 | 224 | 113 | 96 | 1 | `p-1`, t=96 | 17 | about `2^33` | about `2^215` |
| NIST P-256 | 256 | 129 | 1 | 96 | `p+1`, t=96 | 33 | about `2^65` | about `2^247` |
| NIST P-384 | 384 | 193 | 1 | 32 | `p+1`, t=32 | 161 | about `2^321` | about `2^375` |
| NIST P-521 | 521 | 261 | 1 | 521 | `p+1`, t=261 | 0 | order one heuristically | about `2^512` |

All entries are derived from exact primes and valuations; the expected-trial columns are heuristic exponents, not measurements. P-521 is the only named NIST field whose `p±1` 2-primary subgroup reaches the m=2 target height. That mathematical availability requires a 261-step Dickson tower per coordinate and a factor base of scale `2^261`; the current builder explicitly enumerates the fiber, so it is not computationally available.

The 59-bit primes do not reach the required t=30 on either side. Even the favorable `p+1` prime at t=18 has heuristic density around one relation per `2^23` targets before F4, rank, or descent costs. Degree 32 itself loses 49 bits of heuristic density at this scale. At P-192, the degree-32 shortfall is 92 tower levels and its heuristic density is around one per `2^183` targets.

## Implementation gate

The repository's `prime-isogeny` implementation uses `u64`, restricts `1 <= t <= 20`, explicitly constructs the subgroup/fiber of size `2^t`, and builds every base abscissa. The wide Kummer/Dickson pilot supports big primes but also explicitly enumerates its factor base. Therefore:

- The repeated t=5 component map is runnable only in the small-prime `ic` path.
- The current code cannot instantiate the t=30 s59 isogeny-height target, much less t=97 for P-192.
- The `p±1` valuations do not repair the degree-32 map's density.
- P-521's valuation pass is defeated by explicit `2^261` enumeration and at least 522 low-degree tower steps across two coordinates before pricing F4.

## Verdict and next falsifier

There is no justified direct transfer experiment at s59, P-192, or the NIST fields using the current degree-32 map. Running t=5 there would measure a construction whose m=2 heuristic relation density is exponentially below the small-field panel. This is a stopped construction/cost gate, not negative evidence about every possible PKM representation.

A wide continuation becomes meaningful only after implementing a **non-enumerative** high-degree map representation and freezing limits for tower variables, equations, F4 matrix size, relation collection, rank, descent and cold same-target rho. The first practical boundary should be below 59 bits: increase field size while holding t=5 to measure the observed crossover, then increase t only when the system representation avoids materializing `2^t` base elements. P-192 should remain a derived feasibility row until a t≈97 representation has a concrete memory and solver-cost bound.
