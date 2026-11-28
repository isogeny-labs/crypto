#set page(paper:"a4",margin:18mm)
#set text(size:10pt)
= Joint LP core: source coverage and target observability
16 November 2026 · hec_collision_core_20261116 · combined screen/replay report

== Original screen and fixed inputs
Source273d1c564833, pre-run checkpoint20261116T163306Z. Once-only180second bounded gate exits0, empty stderr. Source/import/binary/protocol/lock freeze passes. Three reused curves g2/F251,g3/F211,g4/F61; sourceD1and[37]D1, each paired with the same two public targets. Role1has actual source degrees2,3,4. Expected relative scalar is computed outside the solver API. This is deterministic coverage on reused fields, not a fresh holdout or arbitrary-source proof.

Four pre-registered core sizes per input: g+1;8/9/10;ceil(original H/2);original H13/36/22. Prefix anchors, fixed shifts/RNG/offset equations, at most8anchor additions/input,1024unique principal tuples/4096draws. All48candidate outcomes and12existing Compact controls remain. No cap extensions or input replacement. Candidate successes verify in the original group and by independent sparse elimination.

#image("../../COUNTS.svg",width:100%)

All12distinct inputs recover at the original core sizes, including6full-degree-source cases. Four additional successes occur only at genus3H18. There are16successful policies,10missing-input-row outcomes and22other no-k outcomes. Every success reaches full core rank H+2. Counts are finite deterministic observations, not timing, population estimates or uncertainty intervals.

#pagebreak()
== Full finite outcome table
#table(columns:6,inset:4pt,
[g / field],[H],[Cases],[Verified],[Missing row],[Other no-k],
[2 / F251],[3],[4],[0],[0],[4],
[2 / F251],[7],[4],[0],[0],[4],
[2 / F251],[8],[4],[0],[0],[4],
[2 / F251],[13],[4],[4],[0],[0],
[3 / F211],[4],[4],[0],[3],[1],
[3 / F211],[9],[4],[0],[0],[4],
[3 / F211],[18],[4],[4],[0],[0],
[3 / F211],[36],[4],[4],[0],[0],
[4 / F61],[5],[4],[0],[3],[1],
[4 / F61],[10],[4],[0],[3],[1],
[4 / F61],[11],[4],[0],[1],[3],
[4 / F61],[22],[4],[4],[0],[0])

All12Compact controls recover, with source/target identities unchanged. The16candidate successes include8role1policies but only6distinct role1source/target inputs. Prefix membership differs from Compact's target-seeded anchors; these algorithms are not expected to have identical counts.

Genus3H18reduces core columns from38to20but needs1576or2208prepared principal outputs, compared with768–896atH36. Smaller cores fail throughout genus2and4. Every success has full core rank, so this panel shows no target recovery with a free anchor-core direction. Matrix size alone is not a total-cost argument.

== Second run: full-catalogue observability replay
Sourced34e82f88f19, prior checkpoint20261116T163558Z. Separately frozen instrumented replay of the SAME48streams, without changing rows/selection/bounds or counting them as independent samples. Once-only watchdog run exits0, empty stderr.

Retain every admitted raw equation before forest elimination. Compute dense modular rank(A), rank(A without target column), and augmented rank over actual ell. All48systems are consistent; rank difference1matches16successes, difference0matches32noncompletions. This is independent of the forest's emitted core equations; rejected greater-than-two-endpoint rows are outside its scope.

All24original fields reproduce byte-for-byte in every replayed policy. Input and Compact-control files also match. Six original-source/original-Hcount vectors reproduce the preceding shifted candidate exactly. This is deterministic-work reproduction, not wall-time A/A evidence.

#pagebreak()
== Why the finite failures are real equation-supply failures
A consistent system has the form A_other*x+a*k=b. If a is in the column span of A_other, choose w with A_other*w=a. The null vector(-w,1)changes k while preserving all equations. RankA−rankDrop=0therefore means the admitted stream does not determine k. Difference1fixes k. This is standard linear algebra, not an algorithmic novelty or universal DLP lower bound.

Several failed streams exhaust their entire selected principal family: genus2H7/H8visit all35/56unsigned triples, and applicable genus4H11streams visit all462unsigned5tuples, each full sign orbit. Their rank result cannot be repaired by reordering the same admitted equations or using a different pivot heuristic. Additional target rows, anchor points, functions or endpoint policies would be new data/algorithms. Missing-input-row cases contain no usable target coefficient.

The full-catalogue rank replay shares native field arithmetic and uses source-generated rows in memory; it is producer validation, not an externally signed proof certificate or independent scientific validation. It does not validate information from uncollected/rejected rows.

== Complete-cost decision
All scans/shifts, failed pencils, duplicate draws, prepared tails, roots, graph/potential work, linear algebra, allocation and catalogue/table/inverse setup remain costs. Counters are selected categories, not complete arithmetic accounting. Multiplier certificates, independent sparse replay and dense observability replay are extra correctness-gate work; they are not a production timing comparator.

Original core sizes now have tested full-degree source coverage on these fields. Shrinking the core does not yield an all-genus tenfold premise: it loses recovery in genus2/4and costs more generation at genus3. No elapsed-time result, core-cube-to-speed conversion, exponent improvement or performance campaign follows. A successor must improve the supply of inexpensive useful constraints while charging changed off-anchor elimination. The full mathematical10xobjective remains open.

== Reproducibility and limits
PROTOCOL.md/CERT-PROTOCOL.md, both freeze manifests, original/replay stdout/stderr/exits, BASELINE.tsv/CERT-BASELINE.tsv and INPUTS.txt/CERT-INPUTS.txt preserve every outcome. Original Cargo bytes are kept in ORIGINAL-Cargo.toml after adding replay binary. Native reduce.rs checks fixed counter/input reproduction and renders the graph. Identical combined reports under bothrunids; all3pages visually inspected.

No scientific Python, timing benchmark, benchmark-index/leaderboard admission, independent VALIDATION.json or machine-checked full group theorem. Prior sealed reports/canonical timing graphs unchanged. KB lexical snapshot/access limitations remain in PROTOCOL.md; no priority claim. Today's shared daily log remains reserved; scoped SESSION.md holds handoff. PR4357 remains unmerged.
