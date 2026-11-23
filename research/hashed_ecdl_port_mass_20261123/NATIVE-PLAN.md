# Actual child-constructor controls, before execution

Question: does the current Rust constructor realize the multiset endpoint
partition, with sum of group edge counts exactly2M for each fixed tuple,
independently of beta? Does it preserve old tensor identities/creation
precision and create new packet tensors at the common working precision?

Use the real `packing::pack_labels`, `deformation::modes`, `Cauchy::new` and
`deformation::child`, with new cfg(test) controls only. Original counter,
packing and child algorithm bodies stay byte-identical. An independent
oracle counts original-edge endpoints by digit; a second oracle sums
selected old vertex degrees. Compare every group and retained port against
actual constructed graph objects and `Bipartite::validate`.

Deterministic base contexts: eight geometries (radix,a)=(2,1),(2,2),(2,3),
(3,1),(3,2),(4,1),(4,2),(64,1), with length2^a,arrayradix^(2a),u=a;
each coordinate0..a-1; M in0,1,2,3,4,8,12,16; five graph families
(matching,parallel,leftstar,rightstar,shared3-by2 incidence); four explicit
label schedules (zero,cyclic,affine,reverse). Exactly2080 base contexts.
Old tensors have creation precision3; working ring has degree4,precision7,
z=1,t=2^coordinate. These are forced component geometries, not Setup-admitted
production calls. Packing balance is disabled in this base sweep; exact
coverage/graph validity remain checked by source.

For each context use the distinct admissible tuples among all-zero,
first retained mode exponent1, and the largest first-mode exponent with
weighted sum<7. Deduplicate identical tuples and report both base-context
and actual distinct-tuple/group-call denominators. At most46,560 group
constructions; zero-mode contexts legitimately have one tuple.

One separate balanced actual-radix64 fixture uses8192 matching edges,
a1,L2,array4096, row labels e%64 and column labels(e/64)%64. Use z0 so the
actual mode generator gives zero modes. `pack_labels` enforces its native
coverage, packet and port-balance checks. Expect all64 children to have256
edges, sum16384, and child degree<=4; check the source strict shrinkage
inequality on their actual values. This still constructs no production
Setup call or complete contraction result.

Controls: positive-M witnesses must reject the edge-once modelM and the
row-only histogram; malformed beta length must return an error; removing
or duplicating a packet occurrence must violate actual child graph validity
for an affected group (these mutants intentionally bypass pack_labels's
coverage guarantee). Include empty groups/isolated vertices/parallel edges.
Check nonnegative prefix masses<=2M and common precision-weighted mass
equals2M*7, explicitly a structural accounting quantity, not bit work.

Raw native JSONL and summary are written outside the repository and retained
locally, with exact source/lock/compiler/binary hashes before and after.
Run copied read-only test binary single-threaded, then required existing
crate library checks. One offline build worker, official Rust1.99.1,
600s phase ceiling,120s native executable ceiling, fixed sampled whole-cgroup
stop15,032,385,536bytes. Any panic/assertion, wrong census, timeout, hash
mutation or physical stop fails the attempt. Keep failed outputs. No timing
comparison, solver success, production recursive admission, S3 or cloud spend.
