#set page(paper: "a4", margin: 1.7cm)
#set text(size: 10pt)
#set par(leading: 0.65em)
= T20 retained-record review
2026-10-20 · Crypto PR \#2318 · review addendum

This addendum qualifies the historical n = 73, K = 600 comparison. Original
protocol, measurements and reports remain unchanged. The native analyzer
reproduces the retained JSON, and the three IC answers independently replay.

== Timing provenance correction
The historical prose says rho draw 1 was kept from before IC. The retained
rho_01.time instead matches its second isolation receipt:

#table(columns: (1fr, 1.4fr, 1fr), inset: 5pt,
[Receipt], [UTC start], [Wrapper wall seconds],
[Earlier], [Oct 19, 19:08:28], [163.95028786],
[Retained], [Oct 20, 03:17:31], [105.25283527])

Draws *1–12* have duplicate receipts. All twenty retained timings match their
final receipts and follow the completed IC runs. None brackets IC. The 41
original isolation receipts remain present, with exit zero and contended=false
under the guest-local monitor.

== Interpretation
The 2.027 old/new wall ratio reproduces from the retained data. Its bootstrap
interval is conditional on those runs and their columns. Two same-arm runs
provide one observed A/A difference; they do not bound run-to-run noise. There
is only one old-arm run.

Recorded probes, attempts, relations and answers agree across the three arms.
The three-operation-per-probe proxy is reproducible. Complete charged-cost
equality remains *unmeasured*: IC group additions and operation-kind counts
were not emitted. The live leaderboard separates the proxy ratio from the
null complete-cost ratio. A zero lower bound at weight zero is not a measurement
of zero total work or evidence for the cross-method ordering.

== Analyzer validation
The original analyzer returned success for six deliberately invalid inputs:
unverified IC, altered N2 relation, altered N2 target, unverified rho, altered
rho target and a missing rho draw. It still emitted kill_triggered=false. The
repair requires verified fixed-identity inputs, all twenty rho draws, unique
rank columns and complete finite timing/counter fields before emitting analysis.
The six controls now fail; the retained dataset's JSON is unchanged and four
native regression tests pass.

This review does not authorize another timing run. Parent \#2156, native producer
validation and current-head CI remain merge gates. See T20_REVIEW_20261020.md
for source paths and the precise evidence boundary.
