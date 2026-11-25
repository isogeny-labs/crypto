# Frozen recorded-pool hypothesis audit

Scope: static/development analysis of the immutable226 quartic points and
their already-recorded elliptic images in the main fixed replay45221063854.
No new curve points, random line scan, parameter search, unknown scalar
recovery or performance campaign. Native bounded exact checks follow the
existing development/static-replay scope. Old data and reports stay unchanged.

H-A: effective canonical divisors supported on this pool include principal
relations beyond the seven retained vertical fibers/reference differences.
H-B: degree2 pair-sum equalities or degree4 norm-to-reference identities
give additional projected coefficient rank, including nonprincipal classes
in the elliptic norm kernel. Do not call such a class a principal relation.

Inputs: unchanged kernel3 model and cover input from the SHA-checked archived
kernel3 replay;the exact fixed-main relations/result.json;existing target-KM
and rank sources. Check all points on the model and elliptic equation, all
projection signs/indices against the recorded base, and the original rank6.
Select the same first vertical fiber as reference. No anchor/window change.

Enumerate all unordered elliptic-image pairs i<=j, including diagonal pairs.
Use OpenSSL named P-192 arithmetic and, separately, direct GMP affine group
arithmetic with the exact published field/order/curve constants. Require the
complete ordered pair-fingerprint stream and reference sums to agree. One
search uses grouped pair keys;the second uses sorted direct/complement keys
and a merge join. Their complete pair-equality and degree4 multiset outputs
must agree. Every effective degree4 multiset decomposes into two such pairs.
Classify degree2 equalities between distinct pair divisors and all degree4
multisets whose image sum equals that of the reference fiber. Include the
single recorded identity image;do not count zero coefficient rows as useful.

For canonical sections, enumerate lines through every pair of distinct
recorded quartic points and the tangent at every recorded point. Lines not
tangent at any recorded point need four distinct recorded supports to be
fully supported. Inspect all remaining candidates by exact restriction of
the quartic, repeated division by the known support roots, and exact product
reconstruction. Any canonical divisor with at least two distinct supports
has its line in the pair list;an all-one-point divisor is a hyperflex and
its tangent is in the tangent list. A lower affine degree has missing
infinity support and cannot be a degree4 divisor supported on this affine
pool. Do not infer new rational roots or silently fill missing support.

Check all resulting rows' elliptic sums and signed canonical projection.
Compute old rank, old-plus-canonical rank and old-plus-all-norm rank modulo
the standard P-192 subgroup order. Record all candidates/witnesses, including
the reference's zero row. Compare canonical sets against the frozen fibers.

The target checker uses frozen Khuri-Makdisi arithmetic on the same pool.
Extend the existing complete rational-point Newton-jet encoder to arbitrary
effective degree12 divisors and multiplicities<=12, with the same x/y local
chart selection, exact curve-jet replay and RR/evaluation dimensions. Compare
this encoder against the old effective encoder on all retained simple
vertical fibers. Classify each degree4 divisor minus the reference as
zero/nonzero and require agreement with exact canonical-line geometry.
Distinct degree2 pair divisors with equal norm must give a nonzero target
class on this smooth nonhyperelliptic quartic. Pad both pairs with twice the
first reference point, then with twice the reference fiber, before comparing
classes. Padding cannot change their difference. Preserve unsupported cases.

Controls before data: corrupt one recorded curve coordinate;reject an
off-curve elliptic point(0,0);remove a pair from the fingerprint stream;
replace one reference support by the first outside point;change a nonzero
old row coefficient. Each must reject or be reported as inconclusive if a
declared coordinate mutation remains on the curve. No outcome-based retry.

Bounds:4096 distinct candidates per norm domain and256 target-class checks;
exceeding either is a retained bounded failure, not an exhaustive negative.
Native audit cap120seconds/1GiB;target checker600seconds/1GiB. Strict native
compilation, fresh write-once output directory, frozen source/input/binary
hashes and exact exits. Freeze all constants before the first data call.
No elapsed-time comparison or admitted benchmark result. [pcc:unverified]

Sources reviewed: existing README torsion negative controls, relation and
closure reports/protocols, Diem plane-curve method and official Magma
principal-divisor/canonical-section documentation already read in this
thread. The two exhaustive searches are ordinary incidence and pair-sum
methods;no new general algorithm or global absence claim. KB unavailable
(repository wrapper has no executable). Findings apply only to this pool.
