# Reflection-odd reduction of the cubic coefficient solve

## Exact symmetry and feasibility

Let r be an odd prime, q<r, h=(r-1)/2, c=(q-1)/2 in F_r, and Z=X+c.
For S subset {0,...,q-1} with |S|<=3, put

    f_S(X)=product_{a in S} chi(X+a),
    R(S)={q-1-a : a in S}, epsilon=chi(-1)=(-1)^h.

The involution tau(X)=-X-(q-1) gives

    f_S(tau(X)) = epsilon^|S| f_R(S)(X).

This includes zeros because chi(0)=0. Hence the feature span V is invariant
under tau. As 2 is invertible, P_-=(I-tau)/2 projects V onto its odd part.
Constants lie in V, and X+c=Z is odd. Thus

    X in V iff Z in P_-(V).

Forward: add c then project. Reverse: reconstruct Z from projected features
and subtract c. No feasibility is lost; this is not an assumption that an
arbitrary RHS is odd. Other RHS functions would also need their even sector.

Translation preserves the function-ring modulus:
(X+c)^r-(X+c)=X^r-X. In the centered variable, the unique degree-below-r
representative of an odd function has only powers Z,Z^3,...,Z^(r-2).
There are h coefficient rows. This is an exact subspace reduction, unlike
changing all r value coordinates to all r polynomial coordinates.

## Orbit columns and reconstruction

For each two-element reflection orbit choose one representative S. Its odd
column is the odd coefficient vector of product(Z+a-c)^h. In original
feature coordinates this column is

    (f_S - epsilon^|S| f_R(S))/2.

If S=R(S), keep it exactly when epsilon^|S|=-1; then its odd projection is
f_S itself. Discard fixed even features, including the empty subset.
After solving the h-row system for Z, expand paired coefficients with the
formula above and add constant -c. Centered shifts are sorted as canonical
field elements before calling the unchanged Miller/Frobenius builder.
The reconstructed packet is evaluated using the original shifts 0..q-1.

With m=1+q+binom(q,2)+binom(q,3), the number of orbit columns is:

* even q: (m-1-q/2)/2;
* odd q, epsilon=-1: m/2;
* odd q, epsilon=+1: (m-q-1)/2.

For even q, the fixed subsets are the empty set and q/2 reflected pairs;
all are even under tau. For odd q=2k+1, fixed subsets consist of the empty
set, central singleton, k reflected pairs and k triples containing the center
and a reflected pair. Their intrinsic character signs give the formulas.

For r479/q16, h=239 and m_odd=344, so the augmented system contains
239*345=82455 entries. The full system had 479*698=334342 entries. These
sizes follow from the algebraic construction. Gaussian elimination still has
cubic-type dense work in the dimensions; no asymptotic solver-class change
or elapsed-time gain is asserted. The source still supplies 16 signs.
The new packet has479 terms rather than the previous478; this cost is retained.

## Implementation, limits and exact controls

lib.rs uses the existing Miller builder, projects to odd rows, performs exact
forward elimination/back substitution and recomposes all h odd coefficients.
Its max_rows input still bounds the field characteristic r and inverse table,
while max_columns bounds the retained orbit columns. Primality is caller-
certified. The original fixed-width bounds and oracle/source checks remain.

A bounded optimized Rust build with overflow-checks=yes passes all99 tests,
none filtered. This includes the49-test full reference suite, retained source
and decoder tests, and36 comparisons of full/odd feasibility for primes
{3,5,7,11,17,31}, q=1..min(8,r-1). All successful small-field reconstructions
are checked at every input. This exercises both values of epsilon and both
parities of q, including reflection-fixed odd features.

At r479/q15 the odd solve correctly rejects with observed rank238. At q16
it succeeds with observed rank239,344 columns,479 output terms and7227137
inner elimination updates. This counter covers only row-update iterations;
it excludes construction, inverses, back substitution, certification and
source costs. These are exact-run diagnostics, not a preregistered comparative
performance panel. No timing inference is made from test-suite durations.
The packet passes every one of479 field inputs and all eight saved public-
point source integrations through the existing centered decoder.

A separate hash-verified checker executable reads the saved packet, expands
ordinary polynomials without producer imports, reduces the total modulo
X^479-X and verifies the exact coefficient vector X. It independently checks
all479 field values. Negating one reflected partner's coefficient in a saved
negative packet is rejected at the polynomial-identity assertion. This checks
the reconstruction in the original variable, not just odd-coordinate residuals.
Twenty source/data hashes verify before and after build; checker source,
binary and both packet hashes are bound in checker-receipt.json. Eight compiler
warnings are preserved. Original4563-case cohort and prior packets unchanged.

## Prior work and next task

The quadratic-character sign law is standard: NIST DLMF27.9.1,
https://dlmf.nist.gov/27.9 (read2026-11-05). Odd/even projection is the elementary
order-two representation decomposition; the proof above is explicit. A finite-
group representation course-note PDF was searched but its fetch timed out,
so no claim relies on it. Read the prior Miller builder and generator-compression
derivations before selecting this reduction. No novelty priority claim is made.

Next preserve this target-specific sector reduction while investigating
recurrence-aware elimination inside the odd system. The remaining solve has
239 rows on this fixture, and its columns still arise from low-degree powers.
Full recovery accounting must include the new packet and setup/source work.
