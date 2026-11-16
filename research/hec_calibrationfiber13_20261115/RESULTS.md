# Degree-18 calibration fiber replaces the cyclic point search

The fixed source family produces TWO nonzero source/image calibration pairs
without scanning multiples of D1 and without computing their relative discrete
logarithms. Exact function-field restriction, bounded-degree root factoring,
source-jet decoding and post-selection Kummer alignment checks all pass in V4.
This is a verified replacement mechanism for the old cyclic calibration search
on the registered instance, not a timed full-pipeline gain. Classification:
engineering plus a derived bounded-degree selection argument; novelty unverified.

## Exact identity and inputs

Source genus3 curve y²=f(x), ascending coefficients
[31,89,73,48,91,41,123,1] over F127; #J=2057786=2*ell, ell=1028893.
The SAME prior quotient kernel and normalization are supplied by KERNELS.txt and
ALPHA.txt, copied from the committed alignment atlas. Kernel construction is a
setup dependency. Field extension: degree13, polynomial T^13+T+10. The target
quartic descending homogeneous coefficients are
[1,58,103,47,66,88,51,80,84,123,34,83,109,1,111].

Fix Q=(0,44), R=(3,21) and vary P=(x,y) on the source curve. Put
E=P+Q+R-3infinity. These anchors, curve and input files never change between
attempts. Factor a polynomial to find E on the pullback section, decode its
image as a point difference, then double both source and image to clear the
known cofactor2. Any resulting nonzero ell-torsion source point generates that
prime subgroup. Its scalar relative to the original D1 is irrelevant to
calibration. The existing alignment matrix is consulted ONLY after decoding,
for validation; it never selects a root or a Hessian pair.

## Exact restriction and degree derivation

For a nonzero kernel divisor T=(uT,vT) with Weil function h=a+b*y, the existing
bounds are degree(uT)=3, degree(vT)<=2, degree(a)<=4, degree(b)<=1. Its evaluation
on E uses a divided difference. With d=(x-q)(x-r), the numerator is

    N = y+vT(x)+uT(x)*(aq*(x-r)+ar*(x-q)),
    aq = -(yq+vT(q))/uT(q)/(q-r),
    ar = -(yr+vT(r))/uT(r)/(r-q).

The restricted function is

    (N^3*(a+b*y)/uT^3) * h(Q)*h(R) / d^3.

Each division by uT^3 is exact COEFFICIENTWISE after reduction by y²=f.
This is checked for every nonzero kernel function. The identity function uses
the same denominator. The resulting numerator has degrees A<=7, B<=4.
Quadratic section combinations and coefficientwise field traces therefore give
(A+B*y)/d^6, with Adegree<=15, Bdegree<=11. Its raw norm A²-f*B² has degree<=30.

On each ORIGINAL anchor branch, N vanishes to order at least1. Each Weil
numerator vanishes to order at least3, and a quadratic section numerator to
order at least6. Thus the norm contains
S=(x-q)^6*(x-r)^6 of degree12. The code divides EXACTLY by S=d^6 and checks
that the quotient has degree at most18. The rational norm roots are obtained
by Frobenius gcd and bounded splitting, never a field grid. Extraneous poles
and points failing the original scalar section are excluded explicitly.

Observed Adegree15, Bdegree11, saturated norm degree18; raw degree30 is derived
from the exact degree12 factor and degree18 quotient. Saturated norm ascending:
[111,24,33,111,42,85,92,124,85,81,90,77,68,78,98,14,110,108,81].
Its rational x-roots are54and112, yielding exact regular zeros(54,56),(112,65).
An independent exhaustive verifier checks124regular source-point evaluations
and the COMPLETE regular zero set. This scan is verification only.

## Partial-chord decoder and actual calibration pairs

A decoder previously required all four chord intersections rational. Both
new points instead give chords with exactly TWO rational intersections.
Restricting the target quartic to the gradient line and factoring degree<=4
recovers these rational hits without a target point grid. The Hessian selects
a unique pair; both endpoint differential scales agree and negation reverses
the orientation. Lines at infinity remain explicitly unsupported; vertical
and affine graph lines are implemented.

| Source P | Decoded target pair | Doubled source u | Doubled source v |
|---|---|---|---|
| (54,56) | (9,13)-(89,44) | [14,25,16,1] | [103,116,48] |
| (112,65) | (10,1)-(99,112) | [120,67,71,1] | [124,42,111] |

The doubled target is twice the indicated difference. For BOTH pairs the
source and target are nonzero, killed by ell, and their Kummer coordinates
match the old alignment matrix in the final check. No relative scalar is
supplied. Erasing the Hessian leaves only ONE rational pair when there are
two rational hits: that control is correctly labeled non-discriminating,
not claimed as a successful corruption rejection.

## Retained attempts and custody

| Version | Frozen source | Pre-run checkpoint UTC, 2026-11-15 | Result |
|---|---|---|---|
| V1 | 3d5268c7819b; head015b005a5db9 | 224218 | exit101: incorrect Bdegree<=3 bound |
| V2 | 94b992677480 | 224346 | exact degree18 restriction/roots pass; old four-hit decoder rejects both |
| V3 | 1570b8f582e6 | 224619 | first partial chord/unique pair/endpoint scales pass; erased-Hessian control wrongly expects six pairs |
| V4 | d8d3ca3d599e | 224740 | exit0, empty stderr; BOTH calibration pairs pass final checks |

Each version ran ONCE after its own source freeze/checkpoint/gate. Prior output
was never overwritten. The original bound was wrong, not silently relaxed:
V2 re-derives the bound and adds exact fixed-anchor saturation. V3 admits
partially split chords; V4 corrects only the control cardinality. A pre-freeze
compiler API-arity failure is also retained. Source/binary hashes and locked
Cargo dependencies are adjacent. Local macOS correctness execution, unisolated;
there is no new wall-time ratio or benchmark claim.

## Formalization and evidence boundary

DegreeBound.lean proves `CalibrationFiber.saturated_norm_degree`: over an
arbitrary field, Adegree<=15, Bdegree<=11, fdegree<=7, a nonzero degree12 S,
nonzero norm and EXACT identity norm=S*Q imply Qdegree<=18. Checked with
Lean4.19.0/commit6caaee842e94 and mathlib
c44e0c8ee63ca166450922a373c7409c5d26b00b; exit0. Only standard axioms
propext/Classical.choice/Quot.sound; no sorryAx or custom axiom. The restriction
and geometric divisibility are explicit hypotheses, not formalized geometry.
Exact Rust coefficient divisions and independent value checks supply this
instance's evidence. See [proof map](PROOF-MAP.md) and LEAN.txt.

[Raw V4](RUN-V4.txt), [protocol](PROTOCOL.md), [bound amendment](PROTOCOL-V2.md),
[partial-chord amendment](PROTOCOL-V3.md), [control correction](PROTOCOL-V4.md),
[implementation](fiber.rs), [diagram](flow.svg), [PDF](REPORT.pdf).

Next use the first selected calibration pair to refit the8x8matrix, lift BOTH
original D1 and B via known-calibrator translations, resolve signs by adjacency,
and compute their point logs with the conic descent. Then replace reused
quotient inputs with fresh cold construction and point-log solving. This
removes the need for a known D1image in setup. No guarantee of a rational fiber
point for every curve/anchor family is proved, and no genus2/4 extension is
established. Existing performance charts remain unchanged because these runs
add correctness and structural cost evidence, not comparable timing samples.
The old full cold failure and full10x/genus2+goal remain open.

## Primary-source comparison

[Tian2020 sections4.2.2–4.2.3](https://arxiv.org/html/2007.03172) already computes
coordinate alignment using labeled two-torsion/Weil structures and lifts general
Kummer points. The cyclic scan is a bottleneck of THIS implementation, not a
necessary cost of Tian's algorithm. The new fiber selection is a concrete
alternative setup route; mathematical novelty has not been established.
