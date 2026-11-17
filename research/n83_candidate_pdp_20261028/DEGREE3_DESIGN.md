# Degree-three closure on preserved n83 m2 systems

This batch continues the full FFD/LFD and elite-base objective. It addresses the
four spike40 m2 Koblitz/general-binary cases whose V2 linear rank is17 and exact
LFD is unknown, with their four exact-target baseline controls. Larger m, prime
LFD, regularity and solving degree remain separate unfinished requirements.

Before implementation, read Caminata and Gorla, *Solving degree, last fall degree,
and related invariants*, https://arxiv.org/html/2112.05579, Definitions1.5 and the
Macaulay closure construction. V_i permits multiplication only within the
ordinary degree budget. The Boolean field equations justify multilinear normal
forms; they do not justify multiplying a full-degree row by arbitrary variables.
Existing V2 code and the paired-r1 raw source/trace data are the implementation
reference. No novelty or universal heuristic claim is made.

Reproduce exactly the saved V2 row space, including all queued operations, before
using it. Construct V3 by inserting original nonzero generators of degree<=3,
eliminating with graded pivots, and enqueueing every variable multiple of each
new row of degree<3. Save every generated/reduced row operation and immutable
inserted row. Fresh-process replay checks the source queue, all previous-row
references, exact row bytes, distinct pivots and queue exhaustion.

If a completed V3 row of degree<=2 has nonzero remainder in the verified complete
V2 row space, save its row ID, columns, V2 reductions and nonzero remainder. This
is a degree3 fall and gives LFD>=3. It does not by itself give an upper bound.
A unit or exhaustive affine-completion certificate gives a global upper bound3.
ExactLFD3 requires both. Existing V2 exactLFD2 is retained when valid and must not
be contradicted by a strict V3 witness. Plateaus and resource limits imply no
new upper bound. Keep all original equations and targets.

Controls: the Boolean system xy+1,yz+1,xz has a degree3-only unit certificate;
xy alone must not produce a global upper certificate through this detector;
xy,xy+1 must retain exactLFD2. Reject missing field equations, over-degree
multiplication and modified traces. All54-variable squarefree columns through
degree3 are checked for a bijective contiguous index (26290 columns).

Frozen resource envelope: one native process at a time,8GiB virtual memory,
600seconds per production and replay phase plus5seconds grace,10800seconds outer,
no retries. All eight cases are attempted even after an individual limit or
error. Raw rows and progress flush every128 steps; no checkpoint resume is
claimed. Native source/executable and prior input/V2 hashes are pinned before
new data. Existing-host fallback follows the recorded EKS shared-bucket denial;
refreshed inventory has no duplicate V3 job. No IAM/capacity change or timing
comparison. Append-only raw S3 prefix and Git artifact pointers retain all data.
