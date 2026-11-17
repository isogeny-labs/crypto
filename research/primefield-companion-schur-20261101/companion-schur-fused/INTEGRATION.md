# Fused Schur integration and residual certificate path

2026-11-01. Derived adapter and exact checker source; syntax checked only.
No integration numerical trials. The separate full fused kernel validation
is live and must reach its terminal checks before this panel is frozen/run.

## Operator injection

FusedSchur inherits the existing Schur construction and changes only its
_source implementation. Source-direction counters remain in the same place.
Leading solve, transpose solve, obstruction propagation, correction signs,
wrap guards and generated width s+2 are inherited unchanged. The historical
Schur source and its capped archive are immutable.

Let N=n+m be source dimension, s its generator width. The leading block E
has dimension n; S has dimension m. Inherited schedules give:

| operation | source calls | leading oracle calls | extra field additions | extra field products |
|---|---:|---:|---:|---:|
| Schur apply (either direction) | 2, selected direction | 1, selected direction | m | 0 |
| Schur generators | s+3 each direction | s+2 each direction | m(2s+5) | m |
| reduced RHS | 1 forward | 1 forward | m | 0 |
| primal lift | 1 forward | 1 forward | n | 0 |
| dual lift | 1 transpose | 1 transpose | n | 0 |

Each source call adds its complete fused ledger. For generators, q=2s+6:
field products m, field additions q+m(2s+5), field inversions q, source
coefficient reductions qN, native integer products q(4s+8), input encodings
q(3s+1), packing bytes q(3s+1)Nd. Here d is the full-source radix digit width,
not the generated Schur operator's possibly different width. Coordinate
reversal slots sum (s+3)N+(s+3)(2s+1)N. Data-dependent bit volumes must be
summed from all actual events, never inferred from one representative query.
Reference leading-oracle work remains 2(s+2)n^2 dense terms and 2(s+2)n
coefficient reductions, separate from source-kernel reductions.

## Exact residual transport

For b=[b1;b2], h=b2-C E^-1 b1,

    x=[E^-1(b1-Bz);z],     Ax-b=[0;Sz-h].
    w=[-E^-t C^t v;v],    w^t A=[0;v^t S],    w^t b=v^t h.

certificates.py checks all three identities against explicit independent A,S
references using unbounded integer sums reduced modulo r. It also checks the
trailing coordinates. A normalized reduced witness v^tS=0,v^th=1 yields a
nonzero full normalized witness. Feature-annihilation is a separate function
accepting the actual feature matrix F; it is never inferred from Gram nullness.

For a normalized test on a singular S, obtain a nonzero left-null vector v
from the reference RREF of S^t. Choose its first nonzero coordinate j and set
h_j=v_j^-1, all other coordinates zero. With b1=0,b2=h, the reduced RHS is
exactly h and the witness pairing must be1. This uses public reference data,
not a scalar label. All nonnormalized and nonsingular cases are also retained.

## Full integration panel to freeze after the kernel panel

Retain all321 original Schur contexts and both arms:642rows, including every
source context, zero/singular Schur case, leading-block obstruction, and the
96 full-invertible/leading-singular controls across arms. Dense independent
Schur identities, generated operators and all original vector schedules stay.
Never substitute the42 completed historical rows for the original full plan.

For every valid leading block add zero, ones and seeded b,z,v residual checks;
for every singular S add the normalized witness above. Check modified leading
coordinates in x and w (detectable because E is invertible), changed reduced
RHS, changed normalized pairing and altered source/oracle counters. Keep
zero-Schur branches. A zero witness must fail normalized acceptance. Verify
all nested field/native/byte costs, and reference-oracle work separately.

For every leading obstruction verify the original failure schedule plus each
lift adapter's propagation: RHS reduction fails on its first oracle call,
primal lift fails after one forward source call, dual lift after one transpose
source call. No full-matrix rank verdict follows from any such failure.

Complete checker, analyzer, expanded seed schedule and locks before freezing
this fresh invocation. The historical baseline archive is only a completed
prefix; any comparison against it is explicitly matched to that prefix.
No fresh baseline timing or complete baseline cohort is implied.
