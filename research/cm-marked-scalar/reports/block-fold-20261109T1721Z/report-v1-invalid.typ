#set page(paper:"a4",margin:16mm)
#set text(font:"Libertinus Serif",size:10pt)
#set par(leading:0.5em)
#align(center)[#text(size:18pt,weight:"bold")[Block folding with complete cold costs] #linebreak() P-192 primary; P-224/P-256/P-384/P-521 #linebreak() CM-BLOCK-COLD-V3 · attempt-001 · 2026-11-09T17:21Z · PR 3674]
= Recorded five-curve result
The frozen 125-row untimed diagnostic completed. Every configuration preserves the original $[mu]P$ output. The native summary validates; fresh archive analysis is byte-identical. Relative to the previous retained-bank implementation, the fixed block candidate saves one doubling on P-192, P-224 and P-521, and no group call on P-256 or P-384. It still exceeds the selected direct D+A on every curve. These are deterministic group-call counts, including prefix/table construction, not CPU time or e2e ratios.

#table(columns:(1fr,1fr,1fr,1fr,1fr),inset:5pt,align:(left,right,right,right,right),
 [Curve],[Selected direct],[Previous cached],[Block folded],[Folded binary],
 [P-192],[228],[232],[231],[229],
 [P-224],[260],[266],[265],[260],
 [P-256],[300],[303],[303],[300],
 [P-384],[449],[452],[452],[450],
 [P-521],[606],[613],[612],[606])

The direct plan is the earlier count-selected artifact; its load/parse is charged if timing runs. Block/default windows and offset0 were fixed before data. All six regular widths, API references, binary controls and installed modes remain in the full summary; no native timing selected these rows. Folded-block counters include zero-column blocks and are not operations-saved counters.

#image("delta.svg",width:100%)

= Implemented point-action optimization
For adjacent columns with integer coefficient $u$, a block updates $A_(new)=2^h A_(old)+[u]P$ only when $u$ is zero or an already retained signed multiple. A leading block can initialize directly from that multiple. The bounded dynamic program considers all original single-column choices and blocks of at most16columns; its lexicographic objective is D+A, negations, D, then steps. The unchanged bank and original fallback establish the finite-program group-count bound; they do not establish global addition-chain optimality or a runtime improvement.

The same optimizer is applied to the binary control. The signed-kernel/closed-map certificates and $a+b=mu mod n$ labels remain the original fixtures. This continuation implements their rational-subgroup action; global noninteger quotient polynomials remain unexpanded.

#pagebreak()
= Planning and setup costs
Every arm dispatch and object initialization starts after the first cold timestamp. The five exclusive phases are serialized input/fresh group and point setup; plan preparation; arithmetic; symmetric output verification; and remaining destruction. Planning includes quotient/remainder, both digit expansions, bank-label generation, DP states/transitions/allocations and program reconstruction. No plan or point table is reused across calls. Program/group destruction is charged. For block arms, point-bank construction/normalization and loop/bank destruction are recorded as subphases that sum exactly to arithmetic; they are not added twice to total.

#table(columns:(1fr,1.4fr,1fr,1fr,1fr),inset:5pt,align:(left,right,right,right,right),
 [Curve],[Candidate D,A,N,T],[Bank objects],[Normalized],[DP transitions],
 [P-192],[190,41,16,6],[10],[4],[671],
 [P-224],[220,45,18,10],[14],[8],[843],
 [P-256],[251,52,25,10],[14],[8],[926],
 [P-384],[379,73,27,18],[23],[16],[1554],
 [P-521],[520,92,34,18],[21],[16],[2153])

T counts logical table entries; bank objects and normalized entries are separate physical/event counts. Transitions count offers, not all CPU instructions, allocator operations or bytes. Unknown internal OpenSSL counts/subphases remain null. Event totals cannot supply missing planning, normalization or memory time. Process/library startup, compilation/publication and known-answer workload generation are symmetric explicit exclusions. Supplied curve/scalar parameters are common inputs; no per-point planner/table derivation is excluded.

= Native admission outcome; e2e timing absent
#image("flow.svg",width:100%)
The one calibration launcher returned3 before fork/exec of the worker: measured other CPU6.3seconds during the two-second preflight exceeds0.2. CPU/memory PSI avg10 are0. There are zero timed workers/rows, no held-out launch and no retry. Per-function wall/CPU costs, phase times, RSS, A/A envelope and e2e speed ratios are absent. The counted diagnostic's zero clock fields mean untimed, not zero-cost planning or setup.

The frozen timing design retains four calibration and eight held-out points per curve,25configurations, one warmup and six rounds (3500/7000rows). Per curve, block-default must beat the fastest of eleven reproduced regular/binary reference medians at every held-out point beyond max(1.01,the paired A/A envelope). That analysis cannot be run on absent timing rows. The present count profile does not show a group-call win over the selected direct schedules.

= Verification, publication and resources
Final warning-free native qualification passes2820 independent GMP programs,300 exhaustive small minima,2520 point outputs,26820 retained labels and282 changed-program rejections. The shared cold interface passes625 untimed paths and five bad expected outputs; analyzer passes30 synthetic controls. Synthetic clocks are fabricated software tests, never experimental observations. Source/qualification3b7fe627df89 and freeze f43454319f8d were verified hosted, with PR updated, before data. Protocol digest72609205fba2fa0c5f16387ade18663276b538dadb7042013d329d444ce40860. Each invocation had120seconds/2GiB/thread1; no cloud job, budget/cap/role change or generator/controller restart. No root Cargo result is claimed; no Rust implementation changed.

The5,244-byte raw archive, SHA256a0eda0859cc46f8821d403d2361581f54735c1250c1e5572b5f9a9f59fc7a124, verifies on fresh extraction and deterministic repack. The export is schema-valid. Source: ../../block-v3/README.md, count-summary.json, admission-receipt.json, export.json and manifests; ../../qualification/block-build-v4/; ../../protocol/block-cold-v3/. KB unavailable as retained; Bernstein–Lange precomp.pdf introduction/sections3–4 and prior contracts checked. No novelty claim or official scientific promotion.
