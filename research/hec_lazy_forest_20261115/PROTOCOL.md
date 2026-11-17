# Lazy affine label gate, 2026-11-15

Class: engineering prerequisite for a joint structural cost case. This is not
a mathematical novelty claim or a timed experiment. Goal remains complete
>=10x genus-2/3/4 index calculus with justified mathematics and independent replay.

## Sources and current evidence

Reviewed newest dated log docs/session-logs/2026-11-15.md and scoped
research/hec_anchor_relations_20261114/LINEAR-SESSION.md and LINEAR-HANDOFF.md.
Previous goal turn was a verified wait on current-head PR4274 jobs. At this
turn's poll native, records and PCC passed; validation is still live.
Source inspected at de220aa93c: dense.rs, anchor.rs, rational_solve.rs.
DenseForest.find/insert loops over H on endpoint traversals and stored merges.
Echelon remains dense; removing forest expansion alone does not establish 10x.

KB ready, lexical only; revision 9141e386254340a9bdba3036f2bb706a,
last_refresh 2026-11-14T21:44:48Z, predates latest local work. Queries:
`"hyperelliptic" "sparse" "cycles"`, `"large prime" "union-find"`,
`"sparse" "expression DAG"`. Hits include existing HEC sparse elimination
notes and prior free-leg/large-prime proposals; none supplies this gate.
No absence or novelty inference follows from retrieval.

Primary source read: Gaudry--Thome--Theriault--Diem,
https://cacr.uwaterloo.ca/techreports/2004/corr2004-29.pdf, Sections2.3,3.1--3.3.
Weighted large-prime cycles, pins and union-find are established prior art.
In odd characteristic a cycle can pin a component rather than emit a full row.
This implementation must preserve that distinction, including arbitrary weights.

## Invariant and cost derivation

Over F_ell store L_u = w_u L_parent(u) + B_u dot A, root0=0.
B_u is an immutable linear expression DAG, not an expanded H-vector.
Every internal edge points to an older expression. A leaf is an input sparse
anchor row; sums carry field coefficients. Traversing a union-find path yields
L_start = alpha L_root + sum_i alpha_i B_i dot A. Compress the starting node
by storing this expression and alpha, exactly as the dense reference does.

For incoming sum c_i L_i + on dot A=0, substitute both endpoints; combine
root coefficients and discard zeros. Zero remaining roots emits the exact
anchor vector. One root pins it to0. Two roots attach the smaller component
with scale s=-1/a, weight s*b and label s*hot. Ties and endpoint order must
match dense reference. These identities prove exactness by induction; the gate
checks implementation and malformed corner cases separately.

Materialize a row only when emitted. Process reachable expression IDs in
descending order and accumulate coefficients before visiting each node;
children are strictly older, so each reachable node is processed at most once
per emission, including shared subexpressions and exact cancellation.
Use ordered sparse maps for the initial prototype, charging their overhead.

For M inputs, K total traversed union-find links, S input leaf terms, D stored
expression arcs and R total reachable nodes/arcs/leaf terms across emissions,
construction uses O(M+K+S+D) field work plus map overhead; expansion costs
O(R log(D+H)) conservatively. Storage is O(N+S+D), with no H*N beta array.
This is NOT an O(M) time theorem: R can be large, retained unreachable nodes
can grow, and dense emitted rows/LA remain. Record node/arc/leaf/visit counts.
Do not assume random graph cycle lengths or a 10x factor from this formula.

## Frozen exactness gate before implementation

Standalone native Rust source, existing hec-native-ic rlib; no scientific Python.
Reference include points to existing dense.rs without edits. Compare Option<Row>
byte-for-byte after EVERY insert, plus union/pin counters. Also evaluate every
emitted row against planted values for consistent streams. Test fields101,1009;
anchor widths0,1,3,19,127; endpoint counts1,7,63; seeds0..3; 512inserts each.
All random coefficients include0; endpoints may repeat, include0 and pin/merge.
Use both unconstrained streams and consistent planted streams (H>0).
Explicit controls: empty/full row, repeated endpoint cancellation, nonunit
weights, component pin, duplicate pin, zero-weight expression and shared DAG
cancellation. Fail on any output mismatch; no timing or speed claim.

If it passes, report exact scope and cost counters, then integrate an optional
forest arm only after a separate complete-cost protocol. Do not change previous
closed records, historical raw, benchmark admission, defaults or shared resume.
No cloud work or new timed scientific run is included in this gate.

## Session

### What was done
Read current source/logs and primary prior art; checked/claimed T-6078 for this
new directory. Wrote this derivation and gate before implementation.
### Decisions made
Test symbolic deferral first; retain dense solver and historical comparisons.
### Open threads
Implement the exactness instrument; count deferred work before a timed arm.
### Errors
KB search `limit` was unsupported; retried with query only. No measurement ran.
