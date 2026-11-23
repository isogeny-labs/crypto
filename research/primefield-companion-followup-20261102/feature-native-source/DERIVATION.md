# Native normalized source integration

The source now computes a raw character representative from a public curve point
using native phase, scalar-value division polynomials, prescribed-multiplier
geometry and the original polynomial backend. A Source stores private curve/base
parameters and the inverse of raw(P), so a public-point query produces 0 at O or
normalizes a finite raw value to +1/-1 with explicit rejection of other values.
The window API generates Q,Q+P,... itself through the existing joint geometry;
it accepts no scalar labels or precomputed source signs.

For h=(r-1)/2, select the original clipped ceiling-square-root width and partition
h=2wk+2s+epsilon. With seed c(Q), scaled(n)=c(Q)^(n² mod(p-1))*W_n(Q).
The recurrence updates pairs (N,D), (Nprev,Dprev) to
((x(S)-x(T))*N²*c(S)²*Dprev, D²*Nprev). It performs no recurrence division.
The baby and center products B and A stay as pairs. Their root polynomial and
center evaluations supply resultant R. The main ratio is
((-1)^(wk)*R*A_num^(2w)*B_num^(2k), A_den^(2w)*B_den^(2k)).
The original prefix polynomial, signed scaled(s+epsilon), prefix pair and
singleton -scaled((r+1)/2) complete the tail, followed by one final inversion.
See original shared-projective-grid/evaluator.py, hashed in original-inputs.sha256.

## Checks and actual scope

One build and one filtered invocation passed three methods. The three historical
setup records at subgroup orders 359,479,683 agree exactly in raw value, phase
seed, numerator/denominator, resultant and partial polynomial value. Window tests
at O,P,-P on F509 produce three signs each, including identity crossings; expected
Euler signs use scalar labels only inside the checker. This is 14 successful fresh
finite raw evaluations including four calibrations and nine window outputs (two
identity outputs), counted from the fixed test call graph rather than instrumented.
Partition/rejection checks also pass. Sixteen dependency test methods were filtered.
The compiler emits an unused-fields warning for imported geometry diagnostics;
its full text is preserved. No elapsed-time result is promoted.

## Assumptions and remaining cost

The caller certifies prime p and prime subgroup order r. Inherited word bounds
and curve validation apply; phase checks r-torsion and the coprime normalization
domain. The API rejects r<7 and failures rather than using a scalar-search fallback.
Tests cover partial-plus-singleton source formulas on three saved fixtures;
no-partial/no-singleton partition arithmetic is checked but those raw branches
are not separately exercised in this invocation.

Calibration and prepared strides are setup work; the current window constructs
prepared strides per call. Each finite point still computes phase and separately
initializes the value-only division engine. Geometry is shared across the window,
but those normalization costs are not yet shared. These costs remain explicit;
there is no complete-cost or speed claim. Full4563-case validation is unchanged.

## Next construction

Port the original outer seed propagation in mixed-projective-strides-v2/run.py:
anchors O,P,-P,2P; at most two direct initial phases; then
c(T+P)=(x(P)-x(T))*c(P)²*c(T)²/c(T-P), retaining anchor resets across O.
Keep prepared strides in reusable setup, bind source signs to the repaired native
decoder, and add complete counted accounting before the full authorized run.

## Prior work

Historical shared-projective-grid and normalized-partial-block sources determine
these formulas. This integrates established constructions, not a novelty claim.
FLINT modular-polynomial documentation was checked as context for the retained
polynomial backend: https://flintlib.org/doc/nmod_poly/index.html . Its present page
labels an ongoing documentation revision; no complexity or performance claim here
is inferred from that page. Exact saved sources and unit checks support this port.
