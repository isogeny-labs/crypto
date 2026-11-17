# Actual S3 yield, homogeneous regularity and transfer — frozen experiment

2026-10-24, run `s3-yield-20261024`, starting head
`5fed419c1a9818a524abc66f7912fbe7c35672c5`.

The requested next step is actual better useful yield and a lower relevant
degree on a twist system, with a verified reduction for the original main DLP.
A positive result is not presumed. The preceding conditional cost theorem
does not establish these hypotheses.

## Frozen population and systems

Use all 942 nonsingular short models over p=5,7,11,13,17,19, their least-positive
nonsquare d and monic twists (d^2*a,d^3*b). Consider every shared odd prime r>=5.
Use the first lexicographic nonzero rational r-torsion point as generator.
Enumerate the nonzero subgroup and take the least y representative for each x.
For each feasible size b=2,3, take the first b distinct x coordinates in each
arm. These are unmatched ground-field bases, not transported bases.

As a separately labelled additional fixture use the already registered F43
models (a,b)=(1,0),(4,0), d=2, r=11, main P=(31,18). Enumerate every size-2
and size-3 subset of the five subgroup x coordinates on BOTH arms. Compare
the two prefix bases as the primary fixture comparison; report all subsets,
including ties/losses, so selecting a different base is visible.

For each nonzero scalar target n=1,...,r-1 use the actual affine target R=nP
and the exact prime-field system

    S3_A,B(x,y,x(R)) = 0, g_B(x) = 0, g_B(y) = 0,
    g_B(t) = product_{u in selected x-base} (t-u).

No coefficient-field descent or extension-field rationality constraint is
silently substituted. The factor-base equations are explicit monic polynomials.
Every polynomial root pair is lifted to actual signed point decompositions;
support collisions and repeated x coordinates are checked, not discarded.

## Quantities and falsification criteria

Record ordered root-pair counts, actual signed relation rows, rank progression
modulo r and the target indices at which rank increases. A useful row means
an actual increase of the coefficient matrix rank for the unknown base logs;
the right side n must agree with independently recomputed scalar labels.
Report both full target sequence and preregistered first min(3,r-1) targets;
do not identify raw decompositions with independent rows or use all-target
throughput as a one-target runtime. Record first full-rank target index too.

For each target, build ALL monomial multiples of the three input polynomials
of total degree at most D, for D=max(4,b),...,12. Use exact modular elimination.
Record the first degree deriving 1 for an unsatisfiable instance, and the first
degree at which the row rank equals the full degree<=D vanishing-space
dimension, computed by independent evaluation on all actual root pairs.
The latter is a labelled finite vanishing-space closure degree, NOT claimed to
be the intrinsic degree of regularity or a particular F4/F5 solving degree.
A cap miss stays null; never replace a miss with the cap.

Independently compute the homogeneous leading-form ideal of the given encoding:
<x^2*y^2,x^b,y^b>. Record its standard-monomial Hilbert function and first zero
degree. Prove the coefficient-independent S3 leading form and the elementary
standard-monomial cutoff in Lean if feasible. Distinguish homogeneous regularity,
finite closure/refutation degree, matrix work and actual runtime throughout.

Success needs a labelled actual comparison with strictly better useful yield
and lower relevant degree, plus an applicable scalar-preserving reduction.
Absence in this population is a bounded negative result, not a universal
impossibility. Equal homogeneous regularity is a result about this encoding.

## Reduction and controls

Look exhaustively for a base-field scaling s with s^4*a=d^2*a and
s^6*b=d^3*b. If it exists, independently check (x,y)->(s^2*x,s^3*y) on every
r-subgroup point and every subgroup sum/scalar. Choose the twist generator
as the image of P, so all unknown scalars are preserved without computing
them. Otherwise mark this reduction unavailable; do not construct a fake
efficient map by a discrete-log table. The old F43 pairing reduction remains
available but does not by itself supply a reduction to an arbitrary twist DLP.

For every available scaling also use a matched transported factor base as an
equality control. It must preserve all relation coefficient rows and matrix
ranks/closure degrees. Reject source-bound controls with a false rank/yield,
false degree, omitted case and false scaling. Valid points only; no invalid-
curve attack scope.

## Execution, verification and delivery

C++17 producer may reuse unchanged full.cpp field/point arithmetic. Independent
Rust audit uses its own modular arithmetic, affine addition, S3 construction,
Macaulay matrices and elimination; no producer curve/field routines are shared.
Each native command has a 600-second cap. No wall-time or speedup is measured.
Retain exact small per-case records, pins, hashes, commands, failures, controls,
Lean sources/audit, diagram and inspected PDF. Any arithmetic identity gets a
source-bound IDC1 certificate with false-identity/source controls.

Follow current artifact thresholds; do not rewrite historical frozen evidence.
Run the unmodified scoped proof gate, mandatory quiet release library suite,
native replay and relevant workflow metadata tests. Extend PR #2381 normally,
without force/rebase, registry/leaderboard changes or conflict resolution.
Preserve both catalog provenance dependencies and all existing CI/review,
merge/integration, mathematical and shared-Conductor completion holds.

Actual `conductor check` before edits exited127 (CLI unavailable); no task
reservation or replacement coordinator is claimed. Lead-atlas match was run
before proposing this comparison; relevant existing leads include chained
Semaev regularity, factor-base choice and model representation. An atlas handoff
will retain their existing dispositions for the next review.
