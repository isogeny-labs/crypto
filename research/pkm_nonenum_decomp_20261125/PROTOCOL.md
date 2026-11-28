# PKM compact three-summand prefix sieve, frozen protocol

Date: 2026-11-25. Task: T-12451. Source baseline: `e8e289a04c9e5b131f0e5f959c0d5548e8641983`.

## Question and prior work

Can a solver reject most partial exponent classes of a high-degree PKM map
without expanding all its roots, and return a checked three-point witness at
less complete cost than the existing enumerative joins? This tests the
early-pruning possibility identified in
`research/subexp_routes_20261125/route1/ROUTE1_MEMO.md` section 7. The
existing `route2_m3.py` tests complete-element enumeration, a pair table, and
random triples. Its results are controls, not prefix-sieve measurements.

The construction follows Petit, Kosters and Messeng, *Algebraic approaches
for the Elliptic Curve Discrete Logarithm Problem over prime fields*, sections
3.1-3.4: replace a high-degree membership equation by low-degree constraints
and couple them to Semaev summation equations. Paper:
https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf

## Map and exact witness contract

Use odd prime `p` with `2^t | p-1`, a fixed `g in F_p^*`, and a primitive
`2^t`-th root `zeta`. The factor-base abscissae are
`V = {g*zeta^a : 0 <= a < 2^t}`, represented by the compact equation
`x^(2^t) = g^(2^t)` or its `t` quadratic tower equations. Each retained
abscissa must lift to a point on the chosen curve. No list of `V` is supplied
to the candidate solver. The comparator may enumerate the same points and
must charge that construction and its memory.

For a public target point `R`, solve
`S3(x1,x2,u)=0` and `S3(u,x3,x(R))=0` with each `xi in V`. A candidate answer
is accepted only after recovering three actual curve points `P1,P2,P3`,
checking their abscissae satisfy the map, and checking `P1+P2+P3=R` by the
curve group law. Polynomial solutions without such a witness count as failed
queries. Targets are sampled independently of the candidate map.

## Concrete algorithm

For variable `i` at prefix depth `k_i`, the exponent class `a_i mod 2^k_i`
has the compact polynomial
`x_i^(2^(t-k_i)) = (g*zeta^a_i)^(2^(t-k_i))`. Start with all three `k_i=0`.
For each node of three exponent prefixes, substitute those prefix polynomials
into the two Semaev chain equations. An exact finite-field ideal solver may
prune a node only after returning a checkable inconsistency certificate.
Otherwise split one exponent class into its two children in cyclic variable
order. When all three `k_i=t`, extract all surviving solutions, lift their signs, and
verify the group-law witness. Memoize identical residual ideals by a
canonical digest; charge canonicalization, solver work, cache storage and
all failed nodes. A timeout is an unresolved node and contributes its spent
resources. Any fallback leaf enumeration is counted explicitly.

The initial implementation may use a generic finite-field Groebner engine
for the certificate and extraction. It must record its actual field
operations, wall, CPU, peak memory, node depth and residual-system shape.
It must not infer savings from the count of true prefixes computed by an
enumerative oracle: only certificates found within the resource cap count as
pruning.

## Prediction and falsifier frozen before implementation

Let `B=2^t`, `C_j` be the mean charged cost of one certificate after `j`
total exponent bits have been fixed, `N_j` the number of tested nodes at that
depth, and `L` the surviving leaf
work. The candidate's measured query cost is
`C_query = sum(j=0..3t, N_j*C_j) + L + extraction + verification`.
Memory is the maximum resident certificate/cache state. The generic full
three-summand enumeration control is `Theta(B^2)` work per target, while the
pair-table control charges its `Theta(B^2)` cold build plus target queries.
The research prediction is that certified early rejection makes
`C_query = o(B^2)` at growing `t`, with enough surviving witnesses for full
rank. A finite panel cannot prove the little-o claim. Its falsifier is a
measured nondecreasing `C_query/B^2` across three consecutive feasible `t`
values, or resource censoring before the third value. A promising panel
requires decreasing ratios on each of three independent curves per size,
successful witness verification, and a complete cold-cost advantage over
both controls and same-target rho at the largest measured size.

## First bounded batch and records

Use three deterministic independent prime-order curves at each feasible
`t=3,4,5` with `2^t | p-1`. Freeze their exact `(p,a,b,r,G)`, root `zeta`,
`g`, target scalar list, software commit and binary hashes in a selection
record before timing. Select curves without consulting solver performance.
Use 16 independent cold targets per curve, candidate/control/control/candidate
mirrors, and a 60-second and 2-GiB cap per target. Record every attempted
curve, unmatched base, failed target, timeout, and censored arm in append-only
JSONL. Charge factor-base/map setup, all relation attempts, dependency and
rank, matrix work, linear algebra, and target descent. Run a matched optimized
rho on each identical target and record wall, CPU, counted operations and
memory. Both controls use the identical factor base and target list.

Before timing, validate the solver's witness output against independent
exhaustive enumeration on `t=3` and a second implementation of the group law.
Freeze any amendment in a new file and store; never rewrite a measured row.

## Wide-field decision boundary

For s59, P-192, and each NIST prime, separately record `v2(p-1)` Kummer and
`v2(p+1)` norm-one/Dickson availability, auxiliary-isogeny availability,
required degree for the measured `m=3` relation rate, predicted system size,
peak memory and complete cost. A wide implementation is admitted only after
the compact solver produces checked witnesses and its extrapolated resource
interval fits the named field. The `p+1` family is a separate map construction,
not an automatic transfer of this `p-1` experiment.
