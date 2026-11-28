# H-S: lift the fixed elliptic base, then test closed rank

Question: does completing the rational curve preimages of the original signed
elliptic factor base expose principal line relations with positive additional
rank on that **unchanged** elliptic base? The competing outcome is that every
new section projects into the original rank-six row span. More curve points,
more sections, or a larger full matrix rank alone is insufficient.
[pcc:unverified]

Fixed inputs are the original P-192 d7/kernel3 quartic, actual degree-six map,
226-point original curve pool, its 225 sign-canonical nonzero elliptic columns,
six baseline rows, and the original 108-row tangent batch. Preserve all old
files. No new elliptic point is promoted to an old column. No scalar recovery,
new PCC identifier or performance comparison is part of this experiment.

First compute rational points in

$$S=f^{-1}(\{O,\pm B_1,\ldots,\pm B_{225}\})\cap C(\mathbb F_p).$$

The source infinity polynomial must have no rational root; its geometric
points must still be accounted for in the projective intersection algebra.
For each old elliptic x-coordinate u, intersect the homogeneous quartic with
A-uB, where x_E=A/B. Also intersect with B for O. The given stored numerator
and denominator have degree at most five. Use degree-nine and degree-ten
Macaulay quotients and require Hilbert dimensions 4*deg(G) at both degrees.
Reject a zero reduced G or an inconsistent quotient dimension. A linear form
L=Z+tX, t=0..20, must induce an invertible multiplication map. In its chart,
localize at Z using the stable image of (Z/L)^20; retain and check the complete
invariant affine subspace. Compute the characteristic polynomial of x=X/Z.

Extract all rational x-roots by gcd with x^p-x and complete fixed-schedule
linear factor splitting, then extract all rational y-roots of the quartic at
each x. Check both original equations. Evaluate the actual map at every
candidate; common numerator/denominator zeros use the retained point's own
smooth local chart at precision 45. Keep only images in the specified fibre.
Thus fixed base points of A,B cannot be counted as spurious fibre points.
Output every intersection/fibre diagnostic and every retained coordinate.

Bounds: first qualify one pilot with old column 0 and the O fibre. Before
that execution, freeze the full follow-up as 15 sequential batches covering
old columns [0,16),...,[208,224),[224,225); include O only in the first batch.
The pilot's overlapping work is a separate executed cost. Polynomial splits
use the existing shifts 0..1023. Per-call failures, incomplete splitting and
timeouts reject and remain in fresh evidence directories. No cap increase.

Pilot controls require the known original points in each selected fibre,
reject a changed characteristic-polynomial constant via matrix evaluation,
and reject omission of a rational linear factor. The final combined pool must
recover every original curve point with its exact original image/sign. These
are arithmetic/control obligations, not a prediction of positive rank.

After the fibre qualification, freeze and implement the section stage: all
lines through two distinct points of S plus every tangent on S; complete
restriction factor products retain multiplicities. Deduplicate complete
divisors. The original reference stays fixed. Construct rows only in the
original 225 columns, count their rank modulo the unchanged prime subgroup
order, and include the original private-column tangent batch in the full
elimination accounting. Positive candidates require actual KM principality
and separately checked elliptic images/sums and rank witnesses before they
count toward the goal. Synthetic/planted controls never enter the data matrix.

Every strict build is capped at 300 s and every native worker at 600 s,
one allowed CPU and 1 GiB virtual address space. Retain source/input/binary
hashes, argv, all stdout/stderr, exact exits and wall/user/system costs; record
outer setup/checking intervals separately and avoid adding nested costs twice.
All field/group/polynomial/matrix work is native C++; Python is limited to
file, JSON, hash and cost bookkeeping.

Prior work: original recorded-pool H-A/H-B/H-C audits, the local tangent
collector and independent 2026-11-25 closure report; Diem,
[JTNB 24 (2012), pp.645 and658](https://www.numdam.org/item/10.5802/jtnb.815.pdf),
on line relations and staged large-prime graphs; and Gaudry/Theriault/Thome,
[2005 preprint](https://cacr.uwaterloo.ca/techreports/2004/corr2004-29.pdf),
on combining partial relations. The quotient-algebra computation is standard
finite-intersection linear algebra. KB tools were unavailable; scoped
repository/primary-source searches are logged. No novelty is asserted.
