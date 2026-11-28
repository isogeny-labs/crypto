#set page(paper:"a4",margin:18mm)
#set text(size:10pt)
= Pivot control native replay
Terminal producer report · 2026-11-11

EliminationField, PivotSwap, Controls and Readback compiled with exit 0. ExpectedFailure exited 1 with an unsolved False goal, as specified. All five processes were reaped with empty remaining groups. Readback printed all 17 requested declarations; their reported axioms were confined to propext, Classical.choice and Quot.sound. canonical_preserved reported no axioms. Four compiled artifacts are hash-bound in summary.json.

#image("timing.svg",width:100%)

== Custody and timings
Pre and post checks passed on identical maps of 16,736 dependency files totaling 3,111,892,318 bytes; runtime maps also matched. Native pre: 43.208809 s; compiler phase: 83.963936 s; post: 34.731056 s. Outer completion: 162.773863 s; caller: 162.874211 s, exit 0. Outer/caller intervals contain phase work and are not additive. All three phase owners were terminal. Shared-cache pre/post checks do not exclude transient concurrent changes.

== Accounting and evidence
The 2,400-second conservative charge brings this terminal boundary to 20,580 charged and 109,020 remaining, deadline November 11 at 12:00 UTC. Native phases use 1 CPU and 8 GiB memory, zero swap. Exact commands, receipts, logs and compiled artifacts are archived with hashes. This report's parsing and custody checks are producer self-checks; independent result review and separate binary kernel rechecking are not claimed. Prior failed pivot attempts remain preserved.
