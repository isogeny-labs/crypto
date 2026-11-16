# Degree-three closure on retained n83 m3 systems

This batch continues the fixed-base PDP dataset with twelve retained systems:
original, paired baseline and spike40, each with Koblitz/general-binary and
planted/natural targets. Every presentation has164 Boolean variables and the
same per-family2^27-candidate base used across m2..9. No targets, points or
input equations are regenerated. Original and paired planted targets differ;
natural targets repeat across original/baseline. These are provenance records,
not independent statistical samples. No base-quality ranking is inferred.

Literature checked before implementation: Caminata and Gorla, *Solving degree,
last fall degree, and related invariants*, Definitions1.3–1.5 and the Macaulay
closure construction, https://arxiv.org/html/2112.05579. The ordinary-degree
budget controls multiplication. Full original Boolean field equations justify
squarefree normal forms but never permit multiplying degree-three rows within
V3. The frozen m2 implementation `src/bin/degree3.rs` and saved V2 traces are
the implementation reference. The new instrument is `src/bin/degree3_m3.rs`;
the old instrument and evidence are retained unchanged.

Direct graded-colex indices replace the former64-bit monomial mask. Through
degree3 there are735295 columns; each row has11489 little-endian u64 words
(91912 bytes). Indices through degree2 agree exactly with saved V2 rows.
All735295 column indices are checked for bijection. Independent set-based
product checks cross variable boundaries63/64 and127/128. Missing field
equations, duplicate equation IDs, over-degree multiplication, modified trace
sources and false completion markers must be rejected.

First replay the complete original V2 trace and validate its saved verdict.
Then insert nonzero original generators of degree at most3. Reduce each
candidate with distinct highest pivots. Enqueue all variable multiples of
each newly inserted row of degree below3. Immutable inserted rows and every
source/reducer reference are retained. A fresh process reconstructs the exact
queue and every row, including the remaining queue on capped runs.

Stop after queue exhaustion,32768 inserted rows or131072 processed candidates,
whichever occurs first. Row/operation caps are checked before popping the next
source. Every completed operation is recorded; rows and operations flush each
128 operations. The termination record includes the next source and pending
count. Fresh replay verifies these and the stated stopping condition. A capped
trace proves membership of admitted rows only. It is not a complete V3 basis.
Externally interrupted partial files remain raw data and acquire no new claims
unless a later separately specified verifier certifies them.

A row of degree at most2 outside verified complete V2 certifies LFD>=3 even
if the V3 queue is unfinished. Save its row ID, exact polynomial columns,
V2 reductions and nonzero remainder. A unit or exhaustive affine-completion
certificate supplies a global upper bound3; it uses only proved memberships
and does not require queue exhaustion. Exact LFD3 needs both bounds. A plateau,
row cap, operation cap, timeout or missing witness supplies no global bound.
No new degree-of-regularity, FFD or solving-degree claim is made by this batch.

Native positive controls require exactLFD3 for xy+1,yz+1,xz and exactLFD2 for
xy,xy+1. The xy-only negative control must not certify a global upper bound.
Both cap mechanisms are exercised and replayed on toy systems; modifying an
incomplete trace to claim completion must fail. No n83 production input is used
before the protocol, source and executable hashes are frozen.

Resource envelope: one native process,8GiB virtual memory,600seconds for each
production and replay phase plus5seconds termination grace;18000seconds outer,
no retries. Attempt all12 cases even after individual limits. Preserve every
input snapshot, V2 reference, trace, partial file, status, source, executable
and control in S3 with Git manifests and a file-format map. Existing-host
fallback follows the retained EKS shared-bucket denial; the2026-10-29 inventory
has no active duplicate. No IAM/capacity change or timing comparison is made.
