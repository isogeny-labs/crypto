# Runtime quartic chart and exact Kummer section gate

Goal: replace archived target anchors/chart in the current cold path with an
algorithm taking ONLY a runtime quartic. This correctness gate stays at F127;
field portability and fullcold integration follow, not claimed by this gate.
Literature: Tian2020section4.2.3, https://arxiv.org/html/2007.03172 . Its
pullback/diagonal-section construction motivates this tangent-residual RR
implementation. Arbitrary regular O is a derived chart adaptation checked here,
not a novelty claim or assertion of universal chart existence.
KB search2026-11-16 returned adjacent RR/classgroup pointers, no direct
implementation; absence is not novelty.

Input INPUT-QF.txt is the actually constructed prior quotient quartic.
Frozen cohort: original and affine translations F(X+aZ,Y+bZ,Z) for shifts
(0,0),(1,1),(7,11). Full127²evaluation identities certify the coefficient
transforms (degrees below field size), with corrupted constant-term rejection.
These are THREE REPRESENTATIONS OF ONE ISOMORPHISM CLASS, not independent
curves. No discarded case or shift tuning after outcome.

Candidate computes rational points by127univariate restrictions/root factoring;
an exhaustive point enumerator is verifier-only. Among first32affine points,
choose FIRST O with nonvertical tangent, exactly double tangency, degree2
squarefree residual D, no overlap O and unit Fy on D. Find first four-distinct
rational canonical-line intersections among pairs of first32affine points.
Select four rational, Fy-regular, distinct-x points with no three collinear for
tensor controls. No archived candidate anchor/point table or targetscalar.

Construct cubic numerators vanishing on D: RRrank2/dimension8; multiplication
rank18; exact diagonal coefficientrank112/nullity8. Require all coefficient
identities and corrupt-section rejection. Retain480quotient determinant and
480ordinary split tensor controls with newly discovered points. Add120exact
base-divisor cancellation controls: E=D+S with S off tangent, gcd degree2,
Hensel quotient values equal independent derivative limits in Fp² (or split
Fp roots). Symmetric tensors descend to Fp and projective vector nonzero.

Center using 2beta=2Dinf-2O-K, where K is the computed split canonical line;
require H in oddell-subgroup and 2beta=H. Seed is twice a fresh point difference.
Test all64scalar pairs ±X with original/XZswap graph charts: each supported pair
must be projectively equal, reject corrupted coordinates, retain all exclusions,
require >=12supported and overlap>0 plus irreducible-cubic/mixed coverage.

Independent comparator ONLY: the old chart on the ORIGINAL quartic, transported
by the certified affine isomorphism. It is not used by candidate selection or
section construction. First16jointly supported pairs from64fixed n:12fit an
8x8basis-change matrix,4withheld. Require rank63, matrixrank8, all checks and
matrixcorruption rejection. This prevents a constant/degenerate invariant
passing only negation. No old matrix/image/log supplied. The old comparator
does contain archived points explicitly as a comparator; don't call the whole
binary free of archived points or a complete generic Kummer atlas.

Each scientific case once, sequential, stop on firstfailure, retain raw
stdout/stderr/status and failed outputs. Source/lockeddependencies/input/binary
hashes committed, S3checkpoint/gate BEFORE executing even the cohort input
transform. No silent rerun/pointcap/shift adjustment. No timing or10xclaim.
Unsupported charts and all correctness failures need a separate amendment.
Known grouporder2*1028893 follows from prior quotient and certified isomorphism;
this gate is not a point counter or independent-host replay.
