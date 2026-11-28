# PKM degree-32 wide-transfer construction and cost gate — 2026-11-24

## Question

Does the repeated 13-bit degree-32 PKM relation-extraction lead support a concrete experiment at 59-bit primes, P-192, or the NIST prime fields? Cover both `p-1` Kummer and `p+1` norm-one/Dickson towers, and distinguish them from the PKM §3.4 auxiliary-curve isogeny construction.

## Frozen calculations

For `m=2`, use the paper's heuristic expected decomposition count
`lambda = d^2 / (2p)` for a membership map of degree `d`. Define the target
degree `d_req = ceil(sqrt(2p))` and the least power-of-two tower height
`t_req = ceil(log2(d_req))`. For each prime, compute exact `v2(p-1)` and
`v2(p+1)`, whether either side supplies `2^t_req`, and the missing tower bits.
Also compute the degree-32 `log2(lambda)` and its reciprocal expected-trial
exponent. These are derived heuristic quantities, not measured relation rates.

Inputs are the named NIST field primes from repository curve constructors and
the three existing 59-bit pilot primes. Recompute valuations from the integers;
do not copy the earlier table as data. Include a degree-32 bit-size boundary
grid at 10, 13, 14, 16, 20, 24, 32, 40, 48, 59 and 192 bits.

## Construction and implementation gates

Report separately:

1. `p-1` Kummer: `t` squarings, degree `2^t`, requires `2^t | p-1`.
2. `p+1` Dickson/norm-one torus: `t` quadratic Dickson steps, degree `2^t`, requires `2^t | p+1`; this is not PKM §3.4.
3. Auxiliary elliptic isogeny: `t` degree-2 Vélu maps, degree `2^t`, no direct condition on `p`; the current repository implementation is u64-only, restricts `1 <= t <= 20`, and explicitly builds/enumerates the `2^t` fiber.

For every target, state whether the required height is mathematically available
from each `p±1` valuation, whether the current implementation can represent it,
and the number of low-degree steps per coordinate. A valuation pass alone is
not a feasible attack: explicit `2^t` enumeration and an `m*t`-scale tower
system must also be priced.

## Claim boundary and stopping rule

Stop at a construction/cost verdict. Do not run a wide F4 relation benchmark
when the required map cannot be built by the current implementation or when a
degree-32 map has exponentially negligible heuristic density. A future wide
experiment needs a non-enumerative map representation, a frozen system-size
budget, actual target witnesses, complete relation/rank costs and cold
same-target rho. Preserve derived versus measured labels.
