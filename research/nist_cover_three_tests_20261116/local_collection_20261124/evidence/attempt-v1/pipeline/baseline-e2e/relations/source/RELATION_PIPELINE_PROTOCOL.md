# P-192 kernel3 relation pipeline: functional qualification

Frozen before new calls. Keep the existing kernel3 quartic, degree6 map,
and affine x-window 0..255. Recompute the complete rational point pool by
gcd with Y^p-Y, with the original 64-shift splitting limit; keep all points
and every fully split simple vertical fiber. Compare the complete ordered
pool/fibers with the archived predecessor as an input-consistency control.
No widening, relation replacement, or outcome-based selection.

Emit one principal divisor H_a-H_b for each split vertical fiber after the
first. Check each full fiber polynomial as a product of its four distinct
linear factors, and each signed row against its defining line ratio.
Use the existing actual target KM backend to encode H_a+2H_b relative to
D0=3H_b and check that its class is zero. This checks each relation through
the target Jacobian representation, not just coefficient cancellation.

Evaluate the existing degree6 quartic-to-P192 morphism on all retained
points, including its existing local chart for vanishing denominators.
Canonicalize nonzero elliptic images by sign, merge duplicates, retain the
complete index/sign map, and check every transported row. Compute rank
modulo the exact P192 subgroup order, and report zero rows, base size and
rank deficiency. Retain the complete sparse matrix. These are a vertical
subcollector's setup relations, not the full Diem algorithm.

A separate executable links OpenSSL EC_GROUP/EC_POINT for P192 equations,
subgroup membership, signed sums, and retained public composition checks.
It must not include or call the producer's elliptic arithmetic. Independently
evaluate finite map fractions and reconstruct the exceptional local image
from fixed 16-term series; reject unresolved valuations. Both implementations
may share the read-only JSON syntax parser, not field/group arithmetic.
Cross-library agreement is not an independent signed PCC verdict.

Controls: change a point coordinate; change a nonempty relation coefficient;
change a finite map image; change a public [6] companion. Each must reject.
Do not substitute a hand-written always-false predicate for a corrupted row.

Record native wall and process CPU time per stage, build time, total worker
time, input/output byte counts, candidates, accepted fibers, relations, rank,
and all failures. These local development diagnostics are not isolated EKS
measurements, an admitted performance comparison, or a speedup claim. Preserve
pre-call source/input/binary hashes and failed attempts in fresh directories.
Native execution limit: 600 seconds, 1 GiB per executable. No paid launch.

The full goal additionally requires independent signed verification of the
map/relations and an authorized, budgeted EKS run for admitted costs. This
functional qualifier does not waive either requirement. Prior quotient,
forward/dual and opaque-decoder reports stay immutable.

Sources reviewed: BACKMAP.md, BACKMAP_RELATIONS_PROTOCOL.md,
DESTINATION_RELATION_PROTOCOL.md, POLARIZED_QUOTIENT.md, MAPS_STATUS.md and
the retained native source/evidence; Khuri-Makdisi arXiv:math/0105182v3,
Lemma/Algorithm2.3 and the representation/equality constructions. The
principal-line divisor identity is the existing method, not a novelty claim.
KB MCP unavailable; scripts/research-kb status reports no executable.
