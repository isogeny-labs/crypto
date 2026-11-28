# Optimistic affine-label peeling oracle

Six unchanged incidence-oracle workloads (p101/251/601,source multiples1/17),
all5785retained signed residual supports,16anchors each; no producer rerun.
Seed2026112500^p reproduces original two targets per model. Canonical orbit
columns and scalar weights as before. Initial core0point C has public label
C=(1/sm)D from exact chosen source; no hidden point log supplied.

A point label is alpha*T+beta*D. Repeatedly traverse all immutable rows in
original order, admitting any previously unused row with <=1unknown orbit
column after exact coefficient aggregation. One unknown yields its affine
label; zero unknowns yields a constraint on k. Reject inconsistency; verify
every derived label in original Jacobian. Each admitted prefix must agree
for Sparse/nativeFast target result; any scalar reconstructs unchanged target.
Repeat until a pass derives no new label, not until a favorable solve. Output
all admissions and six terminal summaries including unresolved cases.

This grants free knowledge of every split support, ignores discovery cost,
and permits reuse of all16anchors after closure. It is an optimistic finite-
panel reachability gate, not cheap production, independent rank theorem,
full-cost result or tenfold gain. Stall excludes only algorithms restricted
to these same rows and one-unknown peeling from the same initial labels.
It does not exclude adding multi-unknown LPcycle elimination or new sections.

Candidate physical interpretation: for genus2anchor R, known signed points
share t=(y-vR(x))/uR(x). Two distinct roots in same bucket leave one linear
factor of cubic residual and hence a new affine label. Degenerate/cancelled
supports in oracle may be easier than this producer; no reverse implication
or free access to rejected residuals is assumed. Existing incidence bound
still applies to scan yield, so a positive gate needs full adaptive cost.
Classical sieving/peeling, no novelty claim; no genus3/4qualification.

Freeze source/protocol/import/native/ROOTS/binary hashes and push before one
120CPU-second invocation. Git raw/exit/hashes, no overwrite/retry/S3/cloud.
