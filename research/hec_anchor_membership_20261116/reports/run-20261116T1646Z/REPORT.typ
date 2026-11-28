#set page(paper:"a4",margin:18mm)
#set text(size:10pt)
= Anchor membership and paid support-equation reuse
16 November2026 · hec_anchor_membership_20261116 · frozen120policy screen

== Fixed comparison
Sourcea76de6a7ffe8, prior checkpoint20261116T164618Z. Once-only180second gate exits0/empty stderr; source/import/binary/lock/protocol frozen. Same12source/target inputs as the preceding core screen: g2/F251,g3/F211,g4/F61; two original targets and sourceD1or[37]D1. Actual role1source degrees2,3,4. This is deterministic coverage on reused fields, not a fresh holdout or seed search.

Each input has half/original H and five membership modes. Prefix, half-catalogue rotation and fixed shuffle require no preparatory descent. Modes3/4use known source/target-support membership plus canonical fill; they pay all preparatory random-walk work. Mode3discards those equations, mode4reuses them in the joint core. Support exceeding H is a retained rejection, never an enlarged core. Chosen/nonchosen points are canonically ordered after selection.

#image("../../COUNTS.svg",width:100%)

All77returned scalars verify in the original group and by independent sparse replay. There are14support-too-large,4missing-input-row and25other noncompletion outcomes. All12Compact controls recover. The24prefix cases reproduce prior matching count vectors; input/control bytes match. No failed policy, anchor mask, seed or bound is replaced.

#pagebreak()
== Exact finite outcomes
Each cell is verified pairs out of four(two sources × two targets).

#table(columns:6,inset:4pt,
[g / H],[Prefix],[Rotation],[Shuffle],[Support],[Support + reuse],
[2 / 7],[0/4],[0/4],[0/4],[0/4],[0/4],
[2 / 13],[4/4],[4/4],[4/4],[4/4],[4/4],
[3 / 18],[4/4],[4/4],[4/4],[3/4],[3/4],
[3 / 36],[4/4],[4/4],[4/4],[4/4],[4/4],
[4 / 11],[0/4],[0/4],[3/4],[0/4],[0/4],
[4 / 22],[4/4],[4/4],[4/4],[2/4],[2/4])

Modes3/4prepare the same support, costing total11/41/93preparatory additions across four cases in genus2/3/4respectively, at each H. These are per-policy costs, not amortized shared setup. Reuse removes further collision scans, without changing recovery counts here. Prepared principal-output ranges at g2H13are176–288without reuse and176–228with reuse; g3H36ranges776–1176and776–848. These are selected counts, not timing ratios or complete operation accounting.

Membership changes equation supply. Genus4H11shuffle recovers3cases that prefix misses, but needs3824–4672outputs versus688–832atH22. Genus3H18shuffle needs1224–1768versus800–1024atH36. Every tested genus2H7membership fails. These finite tradeoffs do not establish an all-genus core-halving gain.

== Rank checks and the omitted branch column
Full-catalogue coefficient/target-deleted/augmented rank checks agree with scalar availability. Preparatory support rejected for exceeding H is not admitted to that matrix; trivial unobservability there is a policy restriction, not a statement about the discarded equations' information.

Four successes appear to have rank37with38nominal columns. All are rotated genus3H36. Post-run native input analysis finds one branch anchor,(153,0),inside that set. Its odd-prime projected log is known zero. The solver omits its coefficients but retains it in nominal H. There are therefore37effective unknown columns, equal to observed rank. This is not recovery with an unresolved genuinely unknown anchor direction. Raw nominal counts are unchanged; BRANCH-CHECK.txt records this interpretation.

#pagebreak()
== Decision at the full objective
The prefix's failure is not universal across memberships: the shuffled genus4small core can recover three cases. However no frozen membership recovers a genus2H7case, and successful smaller cores demand much more principal generation. Target-support modes sometimes exceed the strict core budget. Enlarging the core silently or selecting a better seed after output would change the protocol.

An implementation that already pays for exact source/target decomposition should reuse those equations; the existing Compact front end already does so. That is ordinary reuse, not a new mathematical mechanism. The branch-column accounting correction also does not provide a speed result.

All anchor preparation, failed descents, collision scans/shifts, duplicate draws, prepared tails, roots, graph/potential work, linear algebra, allocation, catalogue/table/inverse setup and final verification remain costs. Counters are selected categories, not complete operation accounting. Extra subgroup certificates, sparse replay and full-catalogue rank calculations are correctness-gate work, not a production timing comparator.

A useful successor needs an inexpensive new supply of usable constraints with a complete cost case against the incumbent. Matrix dimensions, nominal rank deficits or a component count alone do not establish tenfold total improvement. No timing campaign, exponent change, mathematical novelty or performance promotion follows. The mathematical10xobjective stays open.

== Reproducibility and scope
PROTOCOL.md and FREEZE.sha256 identify source/lock/import/binary versions; CONTROL.tsv/stderr/EXIT.txt preserve every policy. BASELINE.tsv/INPUTS.txt reproduce preceding bytes. Native reduce.rs checks24prefix count vectors and renders the recorded counts; branch_check.rs is post-run deterministic geometry analysis, not a new scalar-recovery sample. All3PDFpages visually inspected.

Shared native field/group arithmetic remains a dependency. No independent scientific validation, machine-checked full group proof, benchmark record, benchmark-index/leaderboard entry or wall-time claim. Existing sealed reports/canonical performance graphs unchanged. KB lexical snapshot and primary-PDF gaps are explicit in PROTOCOL.md; no priority claim. Shared daily log remains reserved by another verifying task; scoped SESSION.md preserves the handoff. PR4357 remains unmerged.
