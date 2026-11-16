# Exact calibration fiber instead of a cyclic scan

The previous cold gate's generator calibration costs about1.38s at F127.
This gate tests a specific way to remove its q-sized cyclic search: use ANY
nonzero source/image calibration pair, not necessarily original D1. No scalar
relative to D1 is needed to fit the Kummer coordinate transformation.

Frozen source family: E=P+(0,44)+(3,21)-3infinity on the SAME genus3/F127 curve.
Reuse the already constructed quotient kernel/normalization in copied input
files. These are setup dependencies, not a full cold run. For fixed anchors
q,r and variable P=(x,y), restrict each of27Weil functions exactly to the source
function field F_(127^13)(x)[y]/(y²-f). The divided-difference numerator N is
(y+vT(x))+uT(x)*(aq*(x-r)+ar*(x-q)), where
 aq=-(yq+vT(q))/uT(q)/(q-r), ar=-(yr+vT(r))/uT(r)/(r-q).
The known Weil norm identity makes N³*(hA+hB*y) divisible coefficientwise by
uT³. All27functions then have common denominator d³, d=(x-q)(x-r).
A numerator has degree<=7 and B degree<=3. Their quadratic Kummer-section
combination has Adegree<=14/Bdegree<=10. Its norm A²-f*B² has degree<=28.
This follows algebraically; no sampled interpolation or fitted degree bound.
Require EVERY exact division, base-field coefficient descent and degree bound.

Factor gcd(norm,x^127-x) and use bounded deterministic splitting to list all
rational x-roots. At each root solve y²=f(x), reject poles/unsupportedsource
charts and require actual scalar_g zero. No field-point or generator-multiple
scan in the candidate. If A=B=0, test both y signs; retain all candidates and
exclusions, without changing anchors after outcome. Independent verifier scans
ALLsource rational points and compares every regular restricted value, as well
as the exact zero set. Its scan is verification only, never a candidate source.

Try the existing source-jet decoder at all admitted zeros in x/y order, cap28x
roots. Retain unsupported charts. For a supported zero E, double sourceE and
decoded targetdifference to clear the known cofactor2. Require nonzero orderell
source and target calibration pair, and negation orientation control. Expected
alignment matrix is available only as a FINAL projective-image check, not in
root construction/selection or decoding. No source/target DLP supplied.

First execution only after source/protocol/binary freeze and S3checkpoint/gate.
Retain failure, no rerun or anchor/degree adjustment without amendment.
No speed ratio from this correctness gate. If successful, integrate fresh
quotient setup and alignment calibration, then full cold map/sign/descent/LA.
The mathematical selection idea may be standard divisor/pullback arithmetic;
novelty is unverified. Comparators: Tian2020section4.2.3 and Diem2012plane-curve
index calculus. Existing cold results and genus2/4 obligations remain.
