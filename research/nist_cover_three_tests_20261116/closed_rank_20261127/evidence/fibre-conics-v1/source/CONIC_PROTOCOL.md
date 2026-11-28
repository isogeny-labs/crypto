# Degree-eight conic-section search, frozen 27 November 2026

Use the established P-192 d7/kernel3 smooth plane quartic C, actual degree-six
elliptic map, and652computed rational preimages of the original225signed
elliptic columns/O. Keep baseline rank6, all225columns and reference H fixed.
For a quadratic plane form Q, a completely supported divisor C intersect Q
has degree8, and Q/H squared supplies a genuine principal divisor. Push its
image row against twice the reference image vector. This extends the
canonical-line family; it is not a Smith probability or complexity claim.

Choose64deterministic pencils of conics through four distinct pool points in
general position (no three collinear). First32pencils are distributed over
the21pairs of original fully split sections; choose two anchors from each
section, cycling the fixed six unordered pairs of its four indices. Include
every pair of original sections in the first21pencils. Deduplicate anchor
quadruples. Complete the list to64using lexicographic four-subsets of an
eight-point bank chosen greedily in pool order with no three collinear.
No parameter or anchor search is conditioned on resulting elliptic rank.

For each pencil compute the exact two-dimensional nullspace of its4by6
Veronese constraint matrix. Every other pool point determines one conic
parameter. Also include each base point's tangency parameter. These cover
all conics in the pencil with entire intersection supported in the pool:
such a conic has either another support point or excess multiplicity at a
base point. Require no extra base points and verify every candidate's
incidence and local multiplicities. Use smooth-chart series through order8,
precision9, when a tangent coefficient vanishes; transverse points have
order1. Sum orders over every incident pool point; accept only total8.
Quartic/conic degrees and proper intersection then leave no omitted support.
The quartic has no rational infinity, checked again independently of the pool.
[pcc:unverified]

Global deduplication uses normalized six conic coefficients, not row rank.
Retain all supported conics, including reducible/repeated ones and zero rows;
bound4096supported conics. Check actual elliptic row sums, unchanged old-point
and image bindings and baseline rank. Append225-column rows, compute old
rank, and separately append their429-column zero-padded versions to the
original108-row matrix. Report full/auxiliary/closed ranks and gain over6.
Positive gain is a candidate pending actual KM and separate native replay.

Controls: all21products of distinct original section lines must be recovered
in the declared pencils and their rows must span the original rank6; directly
check the repeated reference line Q=H squared has four support points of
multiplicity2and zero projected row. A changed conic coefficient and a missing
support must reject. No synthetic rows enter the actual matrix.

One CPU,1GiB address space,300s strict build and600s native worker. Retain
all source/input/binary hashes, command arguments, terminal exits, raw stdout/
stderr and costs. No scalar recovery, new PCC or speedup comparison. The
scope is exactly the frozen64pencils on this652-point pool, not all conics or
all curve points. Prior context: Diem canonical divisors/line collection and
the recorded negative line/trace tests; no novelty claim.
