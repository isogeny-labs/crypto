#set page(paper:"a4",margin:16mm)
#set text(size:9pt)
= Find-root cardinality native replay
Terminal producer report · 2026-11-11

Eight modules compiled with exit 0: GraphAlgebra, Find, Cycles, Union, Forest, Composition, UnionSize and FindRootCardinality. Controls exited 1 on four Decidable synthesis errors involving RootMap and Forest predicates. Readback and ExpectedFailure were unattempted; no 14-name axiom readback or False-control result is recorded. All nine invoked processes were reaped with empty remaining groups.

#image("timing.svg",width:100%)

== Custody, time and accounting
Pre and post checks passed on identical maps of 16,736 dependency files totaling 3,111,892,318 bytes; runtime maps also matched. Native pre: 41.079102 s; compiler phase: 127.747860 s; post: 33.885975 s. Caller: 203.739195 s, exit 1. Nested intervals are not additive. All phase owners were terminal. Shared-cache pre/post checks do not exclude transient concurrent changes.

The 2,400-second conservative charge brings this terminal boundary to 22,980 charged and 106,620 remaining; deadline November 11 at 12:00 UTC. Native phases used 1 CPU and 8 GiB memory, zero swap. Exact logs, receipts, source inputs and eight compiled artifacts are archived with hashes. These are producer self-checks; no independent result review or separate binary kernel recheck is claimed. Prior failures remain preserved.
