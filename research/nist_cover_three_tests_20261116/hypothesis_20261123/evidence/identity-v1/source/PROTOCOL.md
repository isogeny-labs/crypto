# Proposed next experiment: a rational-point pencil

Status: hypothesis and frozen proposed experiment; no new curve experiment
has run. This proposal preserves the actual P-192 d7 kernel3 quartic and its
degree6 backmap. It changes the relation collector, not the isogeny or group.

## Question and prediction

Does conditioning each line to contain a known rational point increase
independent, checked projected relations per unit of complete collector work?
Hypothesis H1 predicts a component gain from cubic rather than quartic
residual splitting. Competing H0 predicts that mapping, checking, dependencies
or factor-base growth consume that gain. H2, a separate stronger hypothesis,
predicts additional relations on the original projected base after every new
column has been eliminated. H1 does not imply H2.

The forecast uses full symmetric geometric and arithmetic monodromy and
uniform unramified sampling: a generic quartic splits with frequency1/24,
a cubic with frequency1/6. Their ratio4 is only a conditional splitting
heuristic. The fixed small slope window need not be uniformly distributed.
Neither monodromy nor either rate is established here for this quartic/pencil.

## Fixed construction

Use the original pipeline's first fully split vertical reference H_b. Choose
R=(x_R,y_R) to be its first point in the original retained support order;
retain that exact choice and original coordinates in the implementation
receipt, without searching for a better anchor. The reference is an input
common to both arms and its acquisition is charged explicitly in a cold run.

Arm V reproduces the original256 lines X=aZ, a=0..255. Arm P uses the256
lines Y-y_R Z=t(X-x_R Z), t=0..255, with H_b as the common vertical reference.
For each P line, substitute x=x_R+u, y=y_R+t*u in the exact quartic and
require its constant coefficient to vanish. Divide by u exactly once and
require reconstruction of the complete restricted polynomial. Generically
the residual has degree3. Never call this a degree3 isogeny.

Record every line and its factorization type, failed bounded split, repeated
root, lower affine degree, infinity support, tangent or unsupported chart.
Simple completely rational projective intersections qualify for primary
row collection. Exceptions remain in the denominator and raw data. Separate
forced tangent and vertical-reference controls exercise their explicit
classification; do not quietly retry with another anchor or slope.

For a qualifying line, use its actual linear function divided by X-bZ.
The common R contribution cancels with the same multiplicity in the simple
case, leaving at most three positive and three negative support entries.
The implementation must check complete intersection multiplicities and a
target-Jacobian zero class, then transport through the frozen degree6 map.
Check signed duplicate/identity canonicalization, the elliptic sum and row
rank with the existing separate arithmetic backend. Retain the map's local
exceptional chart; membership or a sum alone does not check principality.

Keep every rational point found in either arm's declared scan. Do not trim
the base after seeing rank. Record separate arm bases and their union, with
the same mandatory reference supports. Do not compare raw ranks while
concealing extra columns. Eliminate all columns outside the original225
projected points and report the independent surviving rows separately.
H2 fails this screen if no new independent common-base row survives.

## Observables, controls and interpretation

Record attempted/exceptional lines, factor degrees, simple complete splits,
native/checker exits, target-zero and elliptic-sum checks, nonzero/duplicate
rows, all matrix dimensions/ranks, sparsity and new-column elimination.
Report complete splits per attempt, independent rows per attempt, rank/base
size, surviving common-base rank and all-in wall/CPU/memory. A bigger matrix
is not, by itself, evidence for H2.

Corrupt the canceled-root coefficient, one residual root, a row coefficient
and a projected image; every control must reject at the appropriate check.
Include a valid tangent to prevent accepting a lost multiplicity. Freeze
all constants, limits, source/input/binary hashes and output location before
the new curve run. Initial cap:256 candidates per arm,64 existing splitting
shifts per polynomial,1GiB and600seconds per native worker. Preserve failures
and timeouts. A subsequent larger or statistical experiment needs a separate
protocol; no outcome-dependent window expansion.

The256-line screen may reject a lead or justify a larger experiment; it is
not a statistically established speed result. Cold accounting must include
reference acquisition, map synthesis, source preparation and compilation,
factorization, target arithmetic, transport, checks and matrix elimination.
Record common setup separately as well as in each cold total. Existing
machine isolation, budget admission and independent-signature gates apply
to any performance claim. This proposal itself launches no paid compute.

## Prior art and novelty boundary

Known: Diem's plane-curve index calculus and its Magma implementation use
lines through factor-base points. Proposed here: quantify this specific
change on the existing transferred P-192 fixture with projected-rank,
factor-base-growth and common-base-elimination checks. No new general
algorithm, new torsion shortcut, complexity bound or curve-selection
separator is claimed. A positive H1 result would first be a collector
component improvement; broader conclusions require H2 and full accounting.

The companion identity qualification is only a native issuer check of the
elementary polynomial root-removal identity, over the certificate's declared
field2^61-1. It does not evaluate P-192, establish the new principal rows or
check the monodromy assumptions. Those remain future implementation tests.
