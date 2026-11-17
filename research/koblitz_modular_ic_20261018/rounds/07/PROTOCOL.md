# Round07 protocol — frozen before new arithmetic

Parent b582a94c2b9275e538218b55c0eff67fd5f18a80. Client and host date
2026-10-19; client timezone America/Los_Angeles. Preserve rounds01–06.
Use native Rust for all research arithmetic, search, assembly and checking.
No timing or IC efficiency assessment; retain unsuccessful attempts.

## N4 — independent forward norms

Use the actual round06 positive split and negative degree11 point-algebra
seeds over pinned F2^120. Reconstruct all12 modular target coordinates
from their seven/eleven kernel coordinates, independently of the Gram
matrix. Generalize exact Velu x transport and modular reconstruction to
the degree11 quotient fields. Check curve equations, commuting squares,
the requested fixed target and torsion line permutations. Bound polynomial
gcd degrees by8 and deterministic searches by the prior4096/96 limits.
Use cached inverse Frobenius on each degree11 field for fourth roots.
Trace target elliptic points over each coordinate algebra by its11
absolute Frobenius^120 conjugates; verify the result is constant over F120.
Apply exact Frobenius and determinant1 line changes to all7200 entries of
the20-fiber even divisor, summing the signed pointwise forward images.
Compare with both the source point and the previously recorded Gram result.
The Gram matrix must not enter generation of the new forward norms.

## O4 — odd reverse divisor

Construct an order11 point of E0(F32), explicitly twist it to E49 over
F1024, and record its Frobenius eigenvalue and order. Screen all12 reverse
neighbors, deriving the full11 field from its Frobenius matrix order;
allow a polynomial-basis field of degree at most1320, irreducible low<4096.
Split seven kernels by exact factorization, rather than a quadratic search
over all F1024 coefficient pairs. Attempt both split and degree11 etale
paired fibers with the N4 evaluator, retaining unsupported factor patterns.

Use an antisymmetric target divisor [8T]−[−8T], which represents5T on
11-torsion and is formally odd under T↦−T. This avoids a noninvariant
translated representative when the odd descent wraps. Construct all20
signed pullbacks, check their Frobenius permutation and degree accounting,
and independently norm-evaluate when the seed representation is complete.
An unreduced divisor remains distinct from Jacobian reduction.

## Q5 — determinant19 mixed quotients

Retain the four frozen first anti-isometries and the two determinant19
Hermitian forms that admit gluing. The cofactors have trace−2 or+2,
represented by y²+y=x³+x+c over F2, c=0 or1. Derive the exact common
torsion field from Frobenius orders modulo19, including the five-factor
splitting field, degree at most1440. Search at most4096 abscissas for
actual ordinary and supersingular19-torsion bases. Bind the companion
and CM coordinates explicitly; check all361 points of each graph and its
Frobenius permutation. Attempt explicit invariant quotient coordinates
using the finite translation action, retaining denominators and chart
coverage. An action or lattice presentation alone is partial; do not call
it expanded quotient equations or a Jacobian construction.

## M8 — relations beyond whole degree7 fibers

For the even seed P0, consider

    f = product_(i=0..4) (x(phi_0) + x(Frob^i P0)).

This is F2-rational. Its ordinary zeros are the five conjugates of the
positive and negative seed fibers, total degree6600; its poles are the
corresponding degree660 pullback of the elliptic pole with multiplicity10.
Enumerate closed components using actual Frobenius5 on both seed formats;
evaluate every new closed image with N4. Verify complete support and the
projected principal relation, including the cusp image. Join columns to
earlier records only with verified geometric identities, not equal images.
Report rank on all columns and separately on nonzero-image columns. Retain
zero-image cancellations, duplicates and missing cusp support explicitly.
If a new obstruction appears, amend this protocol before expanding scope.

## Proofs, evidence and publication

Add feasible Lean proofs of norm/trace and line-change implications,
antisymmetric representatives and descent, plus quotient/kernel identities.
Keep geometric hypotheses explicit; no custom axioms or native kernel bypass.
Pin and audit the actual modules, bind polynomial certificates to source
hashes with64 issuer and128 extra checks. Fresh-replay all new outputs and
round06, run required root/native/site/workflow validation, preserve an
immutable archive and a source-linked illustrated report/PDF. Update the
existing PR and canonical status narratives without promoting IC speed.

## Amendment A — explicit odd seed kernel bound

N4 and M8 succeeded. M8 has55 degree60 columns from the split seed and
five degree660 columns from the etale seed; nonzero-image ranks rise to5.
For the odd F1024 seed, full seven-torsion may need a larger field than
the selected eleven-torsion field. Permit the maintained exhaustive
kernel-polynomial solver on the two specified seven edges, at most
2*1024*1024 monic cubic candidates per edge. Retain that bound rather
than assuming all24 roots of psi7 split in the eleven-torsion field.
Split only the two recovered cubics by the deterministic factor routine.

## Amendment B — latest requested continuation

The user additionally requests expanded determinant19 equations and
nonzero-image relation rank beyond5. Test
Norm_F1024/F2(x(phi_1)+x(P0)), using all ten line conjugates2^j.
Recover its complete13200-degree zero support from the same two seed
fibers by exact deck/Frobenius transformations. Join split columns by
complete geometric basis keys. For degree11 columns check the nonconstant
P-coordinate obstruction to overlap with prior rational-P seed columns;
do not join solely by equal projected images. Retain multiplicities and
check the full principal sum and ranks after joining.

For expanded determinant19 geometry, compute the supersingular[19] map
as reduced rational functions X=Nx/Dx and Y=y+Nr/Dr over F2(x), with
polynomial degree cap2048 and exact Euclidean cancellation. Verify its
curve identity and all361 kernel points in the actual torsion field.
The cap applies to reduced map fractions; the cleared identity may have
degree up to4096 without altering the map search bound.
Expand the resulting cofactor-cover chart equations and retain a compact
polynomial certificate in Lean. These cover equations supplement the
mixed translation atlas; the full mixed invariant image ideal remains
an additional obligation unless actually eliminated.

The first odd attempts used both the wrong q-mod11 Frobenius entry and
the old degree5 embedding modulus. They produced no accepted fiber.
Use the degree10 modulus and verify the full group order before any
torsion extraction; bound every eleven-primary loop by the field degree.
