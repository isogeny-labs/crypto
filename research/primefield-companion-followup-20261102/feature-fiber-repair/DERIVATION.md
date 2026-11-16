# Two character fibers explain the saved exclusion and admit a targeted repair

2026-11-03. Derived theorem, exact certificate and bounded local software checks.
No new recovery panel, sampling experiment or performance measurement.

## Evidence and the stronger obstruction

The saved feature-span-dual/certificate.json has two annihilators of all
width-15 cubic features over F479. Both have support only at 25,173,292,440:

    w0 = (84,395,94,385),  w0*X=1
    w1 = (159,320,320,159), w1*X=0.

All omitted coordinates are zero. Direct linear combinations modulo479 give

    w0 + 338*w1 = 178*(e25-e173),
    w0 + 126*w1 = 178*(e292-e440).

Since 178*(25-173)=178*(292-440)=1 modulo479, both are normalized
exclusion certificates. The local unit check verifies these identities against
all 479 entries of the saved vectors, not just a copied support summary.

The independent square-set checker confirms every singleton character agrees
within each pair for offsets0..14. Hence the entire 15-character signature
agrees in each pair. Every function of that signature, of any polynomial
degree or even nonpolynomial form, is equal within each pair. X is not.
Thus increasing degree while keeping these 15 characters cannot repair this
specific exclusion. This strengthens the earlier cubic-only interpretation;
it does not modify the earlier frozen report or its scoped evidence.

This argument uses explicit supplied pairs from the saved certificate. It
performs no pair search or elliptic-point collision search. The quadratic
character definition, including zero, is standard: [NIST DLMF27.9](https://dlmf.nist.gov/27.9).
F479 is the scalar coefficient field of the retained source E/F509, not an
extension-field elliptic source. No unknown scalar labels enter the calculation.

## General two-fiber repair lemma

Let A be the old r-row feature matrix. Suppose the independent difference
functionals n1=e_a-e_b and n2=e_c-e_d annihilate A. Suppose further that
rank(A)=r-2; then these span its full left kernel. For any new columns U with
M=[n1;n2]U invertible and any target b, solve

    M*c = [n1;n2]b.

The residual b-Uc has zero pairing with the full left kernel, so lies in the
column space of A. A consistent old-span solve A*z=b-Uc then yields a full
representation A*z+U*c=b. Moreover [A,U] has full row rank r. This is ordinary
quotient-space linear algebra, specialized here using compact pair functionals.
The current constructor checks the two obstructions and M but not the global
old-span rank. Without that premise, residual membership is not concluded.

For character fibers, let u(x)=chi(x+e) split both pairs, and choose an old
offset d whose character values a1,a2 differ between the two fibers. Such an
offset exists when their old signatures differ. Set

    U0(x)=u(x), U1(x)=chi(x+d)*u(x),
    delta1=u(a)-u(b), delta2=u(c)-u(d),
    M = [[delta1,delta1*a1],[delta2,delta2*a2]].

Its determinant delta1*delta2*(a2-a1) is nonzero. Both added columns are in
the wider degree-at-most-three family (their degrees are only one and two).
No Gram weights or preconditioners are changed by this identity.

## Checked F479 repair and target-specific simplification

The constructor uses supplied new offset15 and finds old offset1 distinguishes
the two old fibers. Independent evaluation gives

    U = [chi(X+15), chi(X+1)*chi(X+15)],
    M = [[2,-2],[2,2]], det(M)=8,
    [n1;n2]X = [331,331], c = [405,0].

Thus the target-specific residual is particularly simple:

    b_new(X) = X - 405*chi(X+15).

It is equal within each old pair. The saved independent rank computation
reports rank(A)=477, so the repair lemma implies b_new is in the old cubic
span. Consequently X admits a representation using old width-15 cubic
features plus JUST the single new linear character 405*chi(X+15). Mixed
new-shift terms are not necessary for this target. The two-column matrix is
still useful because it spans the entire missing two-dimensional quotient.

The old rank477 is inherited from the preserved frozen rank summary (two
independently ordered eliminations), not recomputed by these local checks.
The present exact checker verifies primality, all30 pairwise singleton
equalities, quotient rank2 and zero residual pairings. It does not construct
z or certify an old-span solver. Reusing an old Gram inverse is not justified
merely by this theorem: a finite-field weighted Gram can have extra nullity.
Use a verified rectangular rank factorization/consistent solve, or establish
the required Gram image equality before integrating a Gram backend.

## Implementation and costs

`repair.construct` takes a caller-certified prime, old window, two supplied
pairs and one supplied new offset. It validates distinct points, verifies
old signatures, finds a distinguishing old offset, verifies both new splits,
and solves the explicit 2x2 projected system. It returns a compact packet.
`repair.residual` evaluates the corrected RHS without allocating r entries.
It is not a solver for the old span or an automatic separator search.

Construction evaluates 4q+4 quadratic characters, stores O(q) field entries,
and tests at most q old offsets. Beyond character evaluation it uses ten
field products, seven subtractions and one inversion in the displayed
implementation. Python pow operations are not a measured Meter schedule;
integer reductions, comparisons, copying and allocation are additional.
General residual evaluation uses two characters and three products; the
saved c1=0 permits a specialized one-character residual but the current
function deliberately retains its general formula. No runtime gain is measured.

The independent checker deliberately builds a square set and trial-divides
for primality, and refuses r>10000. Its O(r) reference work is charged
separately; it is not a scalable final checker. Both production and checker
costs, and the consistent old-span solve, are required in a complete route.
No full coefficient program or new scalar recovery is produced in this round.

## Local exact and software validation

Sources/check scope were committed before the single bounded unittest run
at5ed039e. Five test methods passed, with scalar/pair subcases and rejection
controls. They cover the saved-dual transformations, independent repair,
mutated coefficients/projection/offsets/pairs, invalid supplied separators,
F7q1 Schur updates and exact operation counters, normalized witness lifting,
singular/duplicate/used pivots, wrong frame provenance, streaming independent
and dependent outcomes, finite exhaustion, altered sparse witnesses and
altered theorem records. Both new primitives from the preceding rounds now
have these specific F7 software checks; earlier static-only records remain
unchanged. Root-pivot integration still has no numerical instance validation.

The fixed repair packet was then materialized and independently checked for
publication; this did not repeat the unittest suite. See repair-certificate.json,
certificate-check.json, unit-check.log, unit-check-receipt.json and check-lock.json.
The initial shell parse message said four sources although only three Python
sources existed at that point; the message was a literal count, not an audit.
The later static manifest audit enumerates the files and supersedes that count.

This is deterministic certificate/identity verification and local software
validation, not a comparative study or a newly executed frozen research panel.
The local suite was limited to one CPU,512MiB,30seconds and one attempt, with
no compilation or cloud job. Current budget telemetry is retained separately;
no budget, quota, role or infrastructure setting was changed. The repository
isolab-eks skill explicitly treats local unit tests as development work.
No empirical sample-size, distribution, timing or speedup claim is made.

## Next construction

Construct a reusable rectangular rank factorization or consistent old-span
solve, using the saved rank477 input and compact residual X-405*chi(X+15).
Recover and independently verify the old-feature coefficients, then compare
complete admitted compiler work against the retained full width-16 path.
Retain its fixed weights/inputs, include factorization reuse/setup and all
failed attempts, and keep the original full cohort. Before a new cloud run,
reconcile live job/cost reservations and publish a frozen launch protocol.
