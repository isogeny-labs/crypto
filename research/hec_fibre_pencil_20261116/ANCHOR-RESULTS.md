# Constructive anchor elimination: exhaustive algebra and curve result

The graph component construction now passes independent full/off-column modular rank and row-containment checks. It emits all anchor-supported fibre-difference relations, including the component-size correction when the coefficient characteristic divides component cardinalities. Classification: classical incidence algebra implemented and checked; no new exponent, novelty, or timing claim. [pcc:unverified]

## Frozen execution

Source5e05b98b844e, prior S3 checkpoint20261116T064255Z-5e05b98b844e. Both binaries built successfully and their source/lock/native-import/binary hashes were frozen before once-only execution. ANCHOR-SYNTHETIC.exit and ANCHOR-CURVE.exit are0. Synthetic stderr empty. Curve stderr deliberately contains ANCHOR diagnostics. Original curve stdout compares byte-for-byte with CONTROL.tsv. No replaced inputs, adaptive masks, reruns or dropped zero cases.

All179424synthetic configurations pass. Each n=1..4 simple graph edge and ground column has three states(absent/off-anchor/anchor), exhaustively enumerated for primes3,5,1000003. Native counters record11742cases with a full-graph closed component and5432cases with an off-component size divisible by the coefficient prime. These are overlapping finite categories, not independent samples. Each check compares component output rank to rank(R)−rank(R_off), verifies off-column zero, and verifies row containment in the complete difference-row space. Empty columns, zero rank and characteristic-sensitive cases are included. This checks the stated bounded graph family, not arbitrary implementation correctness.

The curve gate passes690policies: all69eligible original inputs, five bounded anchor sizes with canonical/half-rotated catalogue orders. Three original input exclusions remain. Regular-fibre identities and point-add checks remain active. No component closed in the full graph appears in this curve panel; nontrivial closed cases are supplied by the synthetic gate. Same native field/Jacobian dependencies remain; no independent scientific validation or machine-checked group proof.

## What survives at anchor scale

Table entries show pencils with nonzero homogeneous intersection rank, followed by pencils with an available off-graph ungrounded component. Counts are per eligible input. Canonical/rotated orders are reported separately. Complete data: ANCHOR-SUMMARY.tsv.

| Genus / field | Eligible | H | Canonical: positive rank / available component | Rotated: positive rank / available component |
|---|---:|---:|---:|---:|
|2/F101|24|8|0 / 0|0 / 0|
|2/F101|24|16|0 / 6|3 / 8|
|2/F101|24|59(all)|24 / 24|24 / 24|
|3/F61|23|8|0 / 0|0 / 0|
|3/F61|23|16|0 / 3|0 / 2|
|3/F61|23|35(all)|20 / 23|20 / 23|
|4/F31|22|8|0 / 0|0 / 0|
|4/F31|22|16|0 / 3|0 / 3|
|4/F31|22|18(all)|0 / 5|0 / 5|

All H=0 and H=g+1 policies have zero intersection rank and no available component; these negative controls remain. Rank sums are not pooled multi-pencil rank. These are exact deterministic finite counts, without uncertainty estimates or wall-time measurement. Post-run Rust reducer generates all summaries and graph directly from frozen output.

## Affine consequence and decision

A homogeneous relation must cancel the supplied-divisor column, hence the zero-sum component-size constraint. A target decomposition need not cancel it. For each ungrounded off-component C, apply an odd-prime cyclic projection pi to the original equations (killing branch-place 2-torsion) to obtain

    |C| pi([D]) + sum_anchor w_C(P) pi([P-infinity]) = 0.

One component therefore yields an anchor decomposition under an appropriate cyclic prime projection when |C| is invertible. Curve diagnostics have component sizes below1000003, so k>0 gives an available equation for that algebra-control prime. That prime is not the actual group order; this is not a scalar-recovery or original-group division claim. The affine consequence and reporting of available components were derived after the frozen homogeneous gate, using its retained k diagnostics; they were not a pre-registered solver-success endpoint.

This distinction matters: at genus2/H16 canonical order, six pencils have a component equation while none has homogeneous rank. Zero rank alone would incorrectly dismiss descent availability. Conversely H8 produces neither for any tested genus. The full-catalogue sweep still pays setup, point scanning, inverses, fibre handling and component assembly, while Compact uses smaller target-seeded anchors. These finite observations do not establish a tenfold complete-cost premise. No timing campaign follows from them.

The next useful construction is affine component descent integrated with the actual subgroup projection and target-seeded anchors, together with a cost case that charges failed pencils and all source/target work. It must compete with existing random point-add descent and Compact under one boundary. The exact component law is now a checked building block; a mathematical tenfold result remains unresolved.

## Artifacts

[Theory](THEORY.md), [frozen protocol](ANCHOR-PROTOCOL.md), [synthetic raw](ANCHOR-SYNTHETIC.tsv), [curve diagnostics](ANCHOR-CURVE-DIAGNOSTICS.tsv), [native summary](ANCHOR-SUMMARY.tsv), [graph](ANCHOR-COUNTS.svg), [report PDF](reports/run-20261116T0642Z/REPORT.pdf). Report/graph scope is finite count and algebra checks. No timing record, benchmark-index/leaderboard entry or canonical performance graph change. Original freeze/report bytes remain preserved. Shared daily log is reserved; SESSION.md carries this task's handoff. Primary external PDF access gap from the preceding round remains, with no priority claim. PR remains unmerged.
