#set page(paper: "a4", margin: 16mm)
#set text(font: "Arial", size: 10pt)
#set par(justify: true)

= Torsion-row root-finding check
_Local correctness and operation-counter record — 2026-11-16_

Source commit: `c6766ee4772ff03091553298ce1c397ee7a9ab96`. This report records observations only. It makes no wall-time or end-to-end speedup claim. The pre-implementation protocol is `research/rootfind_torsion_seed_20261116/PROTOCOL.md`.

#image("mechanism.svg", width: 100%)

On the binary model with `b=1`, `T=(0,1)` has order two and `x(T+P)=1/x(P)` for nonzero `x(P)`. Signed twins share the first-row key. The new opt-in engine retains the first witness, checks all other collisions in the full group, and seeds the one torsion relation.

#image("counts.svg", width: 100%)

_Fixed cells: Koblitz E0 over GF(2^13), GF(2^19), GF(2^23); dimensions 5, 7, 8. One deterministic release assertion per cell. Original candidates are partitioned into retained full-group checks and torsion twins. Data and SVG generator are beside this PDF._

== Controls and results

The source-pinned `ecbench` release binary suite passed 21 tests; one was ignored. The focused library test passed: every table key retained the same first witness and probe result on all three cells; corrupt negative-index and unsupported-model controls refused. Modular ranks with one torsion seed were 5, 11, 23.

The fixed K13 same-target pipeline verified scalar 641 under all three arms. The torsion arm reported 23 target trials, 13 target-bearing rows, five independent seed pivots, 11 full collision checks, 16 torsion twins, 305 setup group additions, and 2,262 total group-addition equivalents. The prior harvest arm reported 23 trials, 13 target-bearing rows, five seed pivots, 386 setup additions, and 2,316 total group-addition equivalents. These counters do not price the new field batch inversion and are not wall times.

The complete release library suite hit its 180-second watchdog after 2,502 passes, eight ecbench failures, and 22 ignored tests. Its final suite outcome is incomplete. The focused library test and full ecbench binary suite passed. Exact commands, output, hashes, source, compiler and host are in the adjacent JSON and stdout/stderr files.

The frozen K13 EKS F4/F5/F6 panel remains unrun under the live account budget gate. A matched whole-call timing panel and independent replay are the next checks.
