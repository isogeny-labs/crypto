# Minimum-spanning-tree relation elimination

Native Rust continuation after the retained stage22 censor. This is a classical
filtering construction with an odd-prime coefficient port. It has no measured
wall-speed result at this checkpoint. [pcc:unverified]

Sources checked (KB): MCP unavailable and wrapper queries have no executable;
receipts /tmp/ic-fill-cost-kb-search{1,2}.log. Read Cavallar, *Strategies in
Filtering in the Number Field Sieve*, [CWI primary report](https://ir.cwi.nl/pub/4456/04456D.pdf),
sections on controlled merges, spanning trees and dimension/nonzero cost,
SHA2562e511f1130d420143161357d50aaae129ebd51554270e3d09a61265e0c102dc5.
Read canonical CADO-NFS [merge.cpp](https://github.com/cado-nfs/cado-nfs/blob/master/filter/merge.cpp),
blob0e41569a1e097a413ad9a25ce23372f9c3aeaae4, algorithm references and actual
MST calls at lines1035–1050; [mst.h](https://github.com/cado-nfs/cado-nfs/blob/master/filter/mst.h)
blobe967a31f37a63260b2ef82eba19fc236870a3ccf. The paper considers binary
factorization matrices and both dimension and nonzero weight. Neither its
runtime constants nor its measured gains transfer to this implementation.
[pcc:unverified]

For one temporary column, let participating rows have nonzero coefficients
$a_i$. Normalize each complete affine row, including its RHS and source
certificate, by $a_i^{-1}$ modulo the prime subgroup order. Any difference
between two normalized rows cancels this column. Treat rows as graph vertices
and these exact differences as edges. An oriented spanning tree supplies
$k-1$ independent differences: its incidence rows span the zero-sum coefficient
space. Thus the differences span all combinations that cancel the chosen
column. The argument relies on invertible coefficients and preserving every
tree edge; it is an unverified design argument, not a signed formal proof.
[pcc:unverified]

Prim's algorithm chooses a minimum tree among feasible edges, using actual
remaining small/large nonzeros as weight. Source-certificate size breaks ties.
Edges violating row/source limits are absent; a disconnected feasible graph
refuses the whole projection. Every emitted complete row is replayed from
original input combinations before the elliptic pipeline uses it. The target
log stays in protected small columns, and RHS values follow all operations.
[pcc:unverified]

Limits are4096input rows,64incident rows per eliminated column,1000000edge
candidates per attempt and1000000persistent small/large/source terms, with
existing4096small/source row limits. Initial singleton peeling and later
minimum-degree column choice are deterministic. This also changes ordering
relative to the original streaming pivot filter; a future complete-path
comparison cannot isolate the tree choice alone. [pcc:unverified]

The cover adapter additionally refuses a result when projected dimension times
nonzeros exceeds that of the original selected system. This is a sparse
matrix-vector work proxy. It does not predict full solve iterations, memory
costs, planning cost or wall time. Refusal and unsuccessful projected recovery
use the original solve, with spent work included in complete cold wall.
Legacy operation totals omit graph arithmetic. [pcc:unverified]

Integration is additive: `run_cover_sieve_dlp_on_with_mst_filter` and the
`--mst-graph` comparison flag; plan metadata uses `graph_strategy: "mst"`.
Original function signatures and option fields remain; optional MST statistics
and zero guard counts are omitted from old default reports. The analyzer binds
explicit strategy metadata and checks tree counters; a valid fallback recovery
does not satisfy the nontrivial-projection gate. [pcc:unverified]

Standalone release tests pass all ten checks, including five inherited tests,
576complete projected-rowspace comparisons over three odd primes,64exhaustive
four-vertex tree minima, a tree lighter than every star, cap refusal, nonunits,
duplicate IDs and damaged witnesses. Release build and strict Clippy pass;
initial parent tuple-type lint is retained and resolved by an identical alias.
These are producer checks. Native API compilation/group validation and any
new performance protocol remain separate obligations at this checkpoint.
[pcc:unverified]

Native source checks subsequently pass:10generic/MSTfilter tests,17sieve/graph
tests(two existing ignored),19cover tests(one existing ignored),1public-input
worker test, actual production/analyzer builds and strict Clippy. Analyzer
46executions include10imported duplicates;36distinct checks pass. Historical
20/21/22summaries reproduce byte-exactly. Prospective full-group validation is
frozen separately in Amendment18; no stage23scientific result exists here.
[pcc:unverified]
